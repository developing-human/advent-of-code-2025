use std::str::FromStr;

use itertools::Itertools;
use peroxide::fuga::{Matrix, MatrixTrait, MutMatrix, Scalable, Shape::Row, Vector, zeros};

use crate::shared::Answer;

#[derive(Clone, Debug)]
struct Button {
    // which lights this button will toggle
    connections: Vec<usize>,

    // the position of this button the machine
    position: usize,
}

#[derive(Clone, Debug)]
struct ConfigurationConstraints {
    // how many times to push each button. None implies no constraint on this button.
    button_presses: Vec<Option<u8>>,
}

impl ConfigurationConstraints {
    fn new(presses_per_button: Vec<Option<u8>>) -> Self {
        Self {
            button_presses: presses_per_button,
        }
    }

    fn total_button_presses(&self) -> u8 {
        self.button_presses
            .iter()
            .filter_map(|bp| bp.as_ref())
            .sum()
    }
}

struct Machine {
    // the pattern of lights which must be activated
    indicator_light_diagram: Vec<bool>,

    // describes which buttons affect which lights
    button_wiring_schematics: Vec<Button>,

    joltage_requirements: Vec<usize>,
}

impl Machine {
    fn calculate_minimal_configuration_instructions(&self) -> ConfigurationConstraints {
        let light_to_buttons: Vec<Vec<&Button>> = self.map_lights_to_buttons();

        // For each light, this walks through each possible valid configuration. The solution ends
        // up constrained by the solution to the lights which came before it.
        //
        // For example (X = no constraint, # = must be this many button presses):
        // light 0     | off | EF  | XXXX00
        //   light 1   | on  | BF  | X1XX00 must press B to turn light on
        //   light 2   | on  | CDE | X11000
        //     light 3 | off | AD  | 011000 can't press A
        //   light 2   | on  | CDE | X10100
        //     light 3 | off | AD  | 110100 must press A

        // Start with one candidate that has no constraints
        let mut current_candidates = vec![ConfigurationConstraints::new(
            vec![None; self.button_wiring_schematics.len()],
        )];
        let mut next_candidates: Vec<ConfigurationConstraints> = vec![];

        for (light_idx, buttons_for_light) in light_to_buttons.iter().enumerate() {
            while let Some(candidate) = current_candidates.pop() {
                let new_candidates = generate_candidates_for_constraints(
                    candidate,
                    buttons_for_light,
                    self.indicator_light_diagram[light_idx],
                );

                next_candidates.extend(new_candidates);
            }

            // next candidates become current, reset next
            current_candidates = next_candidates;
            next_candidates = vec![];
        }

        current_candidates
            .into_iter()
            .min_by_key(|c| c.total_button_presses())
            .expect("all machines should be solvable")
    }

    fn calculate_fewest_presses_for_joltage_requirements(&self) -> usize {
        // calculates which buttons to press by first reducing the number of variables through the
        // magic of linear algebra, calculating rough ranges for the remaining variables, then
        // looping over all possible values.

        let joltage_matrix = JoltageMatrix::new(self);

        // if this machine has no free buttons (generally, more buttons than joltages), then we can
        // simply compute the answer and be done.
        let free_button_indices = &joltage_matrix.free_button_indices;
        if free_button_indices.is_empty() {
            return joltage_matrix
                .calculate_button_presses(&[])
                .expect("should always calculate result if no free buttons")
                .iter()
                .sum();
        }

        // Limits can be calculated through the free buttons. Look at the free button's connections, and
        // take the max of those. At most, you can hit a free button that many times.
        let ranges = free_button_indices.iter().map(|&free_button_index| {
            let max = self.button_wiring_schematics[free_button_index]
                .connections
                .iter()
                .map(|&c| self.joltage_requirements[c])
                .max()
                .unwrap();

            0..max
        });

        let mut min_presses = usize::MAX;
        for free_buttons in ranges.multi_cartesian_product() {
            let all_button_presses = joltage_matrix.calculate_button_presses(&free_buttons);

            // may get none, if non-integers were encountered
            if all_button_presses.is_none() {
                continue;
            }

            let all_button_presses = all_button_presses.unwrap();

            // if !self.validate_joltage_requirements(&all_button_presses) {
            //     continue;
            // }

            let total_presses = all_button_presses.iter().sum();
            if total_presses < min_presses {
                min_presses = total_presses;
            }
        }

        if min_presses == usize::MAX {
            panic!("no solution found?? {:?}", self.indicator_light_diagram)
        }

        min_presses
    }

    /// Flips buttons -> lights into lights -> buttons
    fn map_lights_to_buttons(&self) -> Vec<Vec<&Button>> {
        (0..self.indicator_light_diagram.len())
            .map(|light_idx| {
                self.button_wiring_schematics
                    .iter()
                    .filter(|&b| b.connections.contains(&light_idx))
                    .collect()
            })
            .collect()
    }

    #[allow(dead_code)]
    fn validate_joltage_requirements(&self, button_presses: &[usize]) -> bool {
        let mut joltages = vec![0; self.joltage_requirements.len()];

        for (button_idx, presses) in button_presses.iter().enumerate() {
            for &connection in &self.button_wiring_schematics[button_idx].connections {
                joltages[connection] += presses;
            }
        }

        if joltages != self.joltage_requirements {
            println!("Joltages don't match!");
            println!("expected: {:?}", self.joltage_requirements);
            println!("actual:   {:?}", joltages);
            false
        } else {
            true
        }
    }
}

/// Stores data about joltage requirements, and which buttons affect which joltage registers.
/// Reduces number of variables to only the "free" buttons (which is often 1-2) and calculates
/// other button presses based on those.
struct JoltageMatrix {
    matrix: Matrix,
    joltage_requirements: Vec<f64>,

    basic_button_indices: Vec<usize>,
    free_button_indices: Vec<usize>,
}

impl JoltageMatrix {
    fn new(machine: &Machine) -> Self {
        // columns are buttons, and rows are connections
        let mut matrix = zeros(
            machine.joltage_requirements.len(),
            machine.button_wiring_schematics.len(),
        );

        // populate the buttons/connections on the matrix
        for (b_idx, button) in machine.button_wiring_schematics.iter().enumerate() {
            for connection in button.connections.iter() {
                matrix[(*connection, b_idx)] = 1.0;
            }
        }

        // convert joltages to float, so they can be used in the matrix
        let joltage_requirements = machine
            .joltage_requirements
            .iter()
            .map(|&j| j as f64)
            .collect();

        // compute the row reduced echelon form, to identify the basic vs free variables
        // joltage requirements are also adjusted as rows are swapped/subtracted
        let reduced_matrix = rref(&matrix);
        let matrix_with_joltages = matrix.add_col(&joltage_requirements);
        let reduced_matrix_with_joltages = rref(&matrix_with_joltages);
        let joltage_requirements =
            reduced_matrix_with_joltages.col(reduced_matrix_with_joltages.col - 1);

        // basic variables are well defined, and can be calculate in terms of the free variables
        // they are the columns which contain the first 1 in a row.
        let basic_button_indices: Vec<usize> = (0..matrix.row)
            .filter_map(|row| (0..matrix.col).position(|col| reduced_matrix[(row, col)] == 1.0))
            .collect();

        // free variables can have a range of values, and the solution will be defined in terms of
        // those
        let free_button_indices: Vec<usize> = (0..matrix.col)
            .filter(|i| !basic_button_indices.contains(i))
            .collect();

        Self {
            matrix: reduced_matrix,
            basic_button_indices,
            free_button_indices,
            joltage_requirements,
        }
    }

    /// Given values for the "free" buttons, calcualte the values for all buttons.
    fn calculate_button_presses(&self, free_button_presses: &[usize]) -> Option<Vec<usize>> {
        let mut all_button_presses = vec![0; self.matrix.col];

        // add the free variables into the answer, so others can be computed from them
        for (&idx, &presses) in self.free_button_indices.iter().zip(free_button_presses) {
            all_button_presses[idx] = presses;
        }

        for (row_idx, &button_idx) in self.basic_button_indices.iter().enumerate() {
            // for row_idx in 0..self.basic_button_indices.len() {
            let row = self.matrix.row(row_idx);

            // start with the joltage requirement, then subject any presses by the 'free' buttons
            let mut button_presses = self.joltage_requirements[row_idx];
            for &col_idx in self.free_button_indices.iter() {
                let presses = all_button_presses[col_idx] as f64;
                button_presses -= row[col_idx] * (presses as f64);
            }

            // Fractional values are no good.
            // TODO: Would it be better to sort this on the matrix itself?
            let rounded_button_presses = button_presses.round();
            if (button_presses - rounded_button_presses).abs() > 0.000001 {
                return None;
            }

            if rounded_button_presses < 0.0 {
                return None;
            }

            all_button_presses[button_idx] = rounded_button_presses as usize;
        }

        Some(all_button_presses)
    }
}

// I can't believe I had to do this... but this is a more numerically stable (for my purposes, at
// least) version of peroxide's rref algorithm. It's a copy paste, with the two changes noted
// below.
fn rref(matrix: &Matrix) -> Matrix {
    let mut lead = 0usize;
    let mut result = matrix.clone();
    'outer: for r in 0..matrix.row {
        if matrix.col <= lead {
            break;
        }
        let mut i = r;

        // check based on epsislon, rather than == 0.0
        while result[(i, lead)].abs() < 0.000000001 {
            i += 1;
            if matrix.row == i {
                i = r;
                lead += 1;
                if matrix.col == lead {
                    break 'outer;
                }
            }
        }
        unsafe {
            result.swap(i, r, Row);
        }
        let tmp = result[(r, lead)];
        // check based on epsislon, rather than == 0.0
        if tmp.abs() >= 0.000000001 {
            unsafe {
                result.row_mut(r).iter_mut().for_each(|t| *(*t) /= tmp);
            }
        }
        for j in 0..result.row {
            if j != r {
                let tmp1 = result.row(r).mul_scalar(result[(j, lead)]);
                let tmp2 = result.row(j).sub_vec(&tmp1);
                result.subs_row(j, &tmp2);
            }
        }
        lead += 1;
    }
    result
}

/// Given a set of constraints and the buttons for a specific light, determines which constraints
/// should be checked next. This takes into consideration if the light should be on, and how many
/// related lights are already on.
fn generate_candidates_for_constraints(
    constraints: ConfigurationConstraints,
    buttons_for_light: &Vec<&Button>,
    is_on: bool,
) -> Vec<ConfigurationConstraints> {
    let constraints = constraints.button_presses;

    // filter to buttons that are not constrained
    let unconstrained_buttons = buttons_for_light
        .iter()
        .filter(|b| constraints[b.position].is_none())
        .collect::<Vec<_>>();

    // based on constraints, how many lights are already on?
    let current_button_count_for_light = buttons_for_light
        .iter()
        .map(|b| b.position)
        .map(|p| constraints[p].unwrap_or(0))
        .sum::<u8>() as usize;

    // determine if the number of buttons pressed should be even or odd
    // taking into consideration the number of lights already on
    let mod_target = if is_on { 1 } else { 0 };
    let mod_target = (mod_target + current_button_count_for_light) % 2;

    let mut candidates = vec![];

    // starting at 0 or 1, count by twos up to the number of unconstrained buttons
    // then permute over possible indices for buttons to press
    for indices_to_choose in (mod_target..=unconstrained_buttons.len()).step_by(2) {
        for buttons_to_press in (0..unconstrained_buttons.len()).combinations(indices_to_choose) {
            // create a candidate to suggest, based on the starting candidate
            let mut candidate_constraints = constraints.clone();

            // PUSH THE BUTTONS!
            for (idx, unconstrained_button) in unconstrained_buttons.iter().enumerate() {
                let times_to_push_button = if buttons_to_press.contains(&idx) {
                    1
                } else {
                    0
                };

                candidate_constraints[unconstrained_button.position] = Some(times_to_push_button);
            }

            candidates.push(ConfigurationConstraints::new(candidate_constraints));
        }
    }

    candidates
}

#[derive(Debug)]
struct ParseError;

impl FromStr for Machine {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (light_str, rest) = s.split_once(" ").ok_or(ParseError)?;
        let (button_str, joltage_str) = rest.rsplit_once(" ").ok_or(ParseError)?;

        let lights: Vec<bool> = light_str[1..(light_str.len() - 1)]
            .chars()
            .map(|c| c == '#')
            .collect();

        let buttons: Vec<Button> = button_str
            .split(" ")
            // each like: (1,2)
            .map(|s| &s[1..(s.len() - 1)])
            // each like 1,2 (str)
            .map(|s| s.split(",").map(|s| s.parse::<usize>().unwrap()).collect())
            .enumerate()
            .map(|(position, connections)| Button {
                position,
                connections,
            })
            .collect();

        let joltages: Vec<usize> = joltage_str[1..(joltage_str.len() - 1)]
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();

        Ok(Self {
            indicator_light_diagram: lights,
            button_wiring_schematics: buttons,
            joltage_requirements: joltages,
        })
    }
}

pub fn solve(input: &str) -> Answer {
    let machines: Vec<Machine> = input.lines().map(|line| line.parse().unwrap()).collect();

    let total_button_presses_for_lights: usize = machines
        .iter()
        .map(|m| m.calculate_minimal_configuration_instructions())
        .map(|ci: ConfigurationConstraints| ci.total_button_presses())
        .map(|count| count as usize)
        .sum();

    let total_button_presses_for_joltage_requirements: usize = machines
        .iter()
        .map(|m| m.calculate_fewest_presses_for_joltage_requirements())
        .sum();

    Answer {
        part1: total_button_presses_for_lights,
        part2: total_button_presses_for_joltage_requirements,
    }
}

#[cfg(test)]
mod tests {

    use peroxide::fuga::MatrixTrait;

    use super::*;

    #[test]
    fn solve_basic_input() {
        let input = r#"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

        let result = solve(input.trim());
        assert_eq!(result.part1, 7);
        assert_eq!(result.part2, 33);
    }

    #[test]
    fn can_parse_one_input() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let parsed: Machine = input.parse().unwrap();
        assert!(!parsed.indicator_light_diagram[0]);
        assert!(parsed.indicator_light_diagram[1]);
        assert!(parsed.indicator_light_diagram[2]);
        assert!(!parsed.indicator_light_diagram[3]);

        assert_eq!(parsed.button_wiring_schematics[0].connections[0], 3);
        assert_eq!(parsed.button_wiring_schematics[1].connections[0], 1);
        assert_eq!(parsed.button_wiring_schematics[1].connections[1], 3);
        assert_eq!(parsed.button_wiring_schematics[2].connections[0], 2);
        assert_eq!(parsed.button_wiring_schematics[3].connections[0], 2);
        assert_eq!(parsed.button_wiring_schematics[3].connections[1], 3);
        assert_eq!(parsed.button_wiring_schematics[4].connections[0], 0);
        assert_eq!(parsed.button_wiring_schematics[4].connections[1], 2);
        assert_eq!(parsed.button_wiring_schematics[5].connections[0], 0);
        assert_eq!(parsed.button_wiring_schematics[5].connections[1], 1);

        assert_eq!(parsed.joltage_requirements[0], 3);
        assert_eq!(parsed.joltage_requirements[1], 5);
        assert_eq!(parsed.joltage_requirements[2], 4);
        assert_eq!(parsed.joltage_requirements[3], 7);
    }

    #[test]
    fn can_map_lights_to_buttons() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let parsed: Machine = input.parse().unwrap();
        let lights_to_buttons = parsed.map_lights_to_buttons();

        assert_eq!(lights_to_buttons.len(), 4);
        assert_eq!(lights_to_buttons[0].len(), 2);
        assert_eq!(lights_to_buttons[1].len(), 2);
        assert_eq!(lights_to_buttons[2].len(), 3);
        assert_eq!(lights_to_buttons[3].len(), 3);

        // the first button for light 0 is (0, 2)
        assert_eq!(lights_to_buttons[0][0].connections[0], 0);
        assert_eq!(lights_to_buttons[0][0].connections[1], 2);

        // the first button for light 0 is (0, 1)
        assert_eq!(lights_to_buttons[0][1].connections[0], 0);
        assert_eq!(lights_to_buttons[0][1].connections[1], 1);

        // the first button for light 1 is (1, 3)
        assert_eq!(lights_to_buttons[1][0].connections[0], 1);
        assert_eq!(lights_to_buttons[1][0].connections[1], 3);

        // the second button for light 1 is (0, 1)
        assert_eq!(lights_to_buttons[1][1].connections[0], 0);
        assert_eq!(lights_to_buttons[1][1].connections[1], 1);

        // the first button for light 2 is (2)
        assert_eq!(lights_to_buttons[2][0].connections[0], 2);

        // the second button for light 2 is (2,3)
        assert_eq!(lights_to_buttons[2][1].connections[0], 2);
        assert_eq!(lights_to_buttons[2][1].connections[1], 3);

        // the third button for light 2 is (0,2)
        assert_eq!(lights_to_buttons[2][2].connections[0], 0);
        assert_eq!(lights_to_buttons[2][2].connections[1], 2);

        // the first button for light 3 is (3)
        assert_eq!(lights_to_buttons[3][0].connections[0], 3);

        // the second button for light 3 is (1,3)
        assert_eq!(lights_to_buttons[3][1].connections[0], 1);
        assert_eq!(lights_to_buttons[3][1].connections[1], 3);

        // the third button for light 3 is (2,3)
        assert_eq!(lights_to_buttons[3][2].connections[0], 2);
        assert_eq!(lights_to_buttons[3][2].connections[1], 3);
    }

    #[test]
    fn check_generate_candidates_for_constraints() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let parsed: Machine = input.parse().unwrap();
        let lights_to_buttons = parsed.map_lights_to_buttons();

        let constraints = ConfigurationConstraints::new(vec![None; 6]);

        // get candidates for the first light, with no incoming constraints
        let first_candidates =
            generate_candidates_for_constraints(constraints, &lights_to_buttons[0], false);
        assert_eq!(first_candidates.len(), 2);
        assert_eq!(
            first_candidates[0].button_presses,
            vec![None, None, None, None, Some(0), Some(0)]
        );
        assert_eq!(
            first_candidates[1].button_presses,
            vec![None, None, None, None, Some(1), Some(1)]
        );

        // get candidates for the second light, based on the first candidate from light one
        let second_candidates = generate_candidates_for_constraints(
            first_candidates[0].clone(),
            &lights_to_buttons[1],
            true,
        );
        assert_eq!(second_candidates.len(), 1);
        assert_eq!(
            second_candidates[0].button_presses,
            vec![None, Some(1), None, None, Some(0), Some(0)]
        );

        // get candidates for the second light, based on the second candidate from light one
        let second_candidates = generate_candidates_for_constraints(
            first_candidates[1].clone(),
            &lights_to_buttons[1],
            true,
        );
        assert_eq!(second_candidates.len(), 1);
        assert_eq!(
            second_candidates[0].button_presses,
            vec![None, Some(0), None, None, Some(1), Some(1)]
        );
    }

    #[test]
    fn can_create_joltage_matrix() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let parsed: Machine = input.parse().unwrap();
        let matrix = JoltageMatrix::new(&parsed);

        assert_eq!(matrix.matrix.row(0), vec![1.0, 0.0, 0.0, 1.0, 0.0, -1.0]);
        assert_eq!(matrix.matrix.row(1), vec![0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        assert_eq!(matrix.matrix.row(2), vec![0.0, 0.0, 1.0, 1.0, 0.0, -1.0]);
        assert_eq!(matrix.matrix.row(3), vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0]);

        assert_eq!(matrix.free_button_indices, vec![3, 5]);
    }
    #[test]
    fn can_calculate_button_presses() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let parsed: Machine = input.parse().unwrap();
        let matrix = JoltageMatrix::new(&parsed);

        // these are exampled I worked through by hand ahead of time
        assert_eq!(
            matrix.calculate_button_presses(&[0, 0]).unwrap(),
            vec![2, 5, 1, 0, 3, 0]
        );
        assert_eq!(
            matrix.calculate_button_presses(&[3, 2]).unwrap(),
            vec![1, 3, 0, 3, 1, 2]
        );

        assert_eq!(
            matrix.calculate_button_presses(&[1, 0]).unwrap(),
            vec![1, 5, 0, 1, 3, 0]
        );
        assert_eq!(
            matrix.calculate_button_presses(&[1, 1]).unwrap(),
            vec![2, 4, 1, 1, 2, 1]
        );
        assert_eq!(
            matrix.calculate_button_presses(&[1, 2]).unwrap(),
            vec![3, 3, 2, 1, 1, 2]
        );
        assert_eq!(
            matrix.calculate_button_presses(&[2, 1]).unwrap(),
            vec![1, 4, 0, 2, 2, 1]
        );
        assert_eq!(
            matrix.calculate_button_presses(&[2, 2]).unwrap(),
            vec![2, 3, 1, 2, 1, 2]
        );
        assert_eq!(
            matrix.calculate_button_presses(&[3, 3]).unwrap(),
            vec![2, 2, 1, 3, 0, 3]
        );
        assert_eq!(
            matrix.calculate_button_presses(&[4, 3]).unwrap(),
            vec![1, 2, 0, 4, 0, 3]
        );
    }

    #[test]
    fn can_calculate_joltages_no_free_buttons() {
        let input = "[#.#.#.] (2,4) (0,4) (1,2,3,5) (0,1,3,4,5) {14,22,181,22,183,22}";
        let parsed: Machine = input.parse().unwrap();

        assert_eq!(
            parsed.calculate_fewest_presses_for_joltage_requirements(),
            195 //TODO: Be skeptical of this value, I don't know for sure that it's right
        );
    }

    #[test]
    fn can_calculate_joltages_with_free_buttons() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let parsed: Machine = input.parse().unwrap();

        assert_eq!(
            parsed.calculate_fewest_presses_for_joltage_requirements(),
            10
        );
    }

    #[test]
    fn can_calculate_failing_machine() {
        let input = "[##..#] (1,3,4) (0,2,4) (3,4) (2,3) (0,1,3,4) (0,2) {178,24,162,32,31}";

        let parsed: Machine = input.parse().unwrap();

        assert_eq!(
            parsed.calculate_fewest_presses_for_joltage_requirements(),
            186
        );
    }

    #[test]
    fn can_calculate_failing_machine_2() {
        let input = "[#.#.#...##] (0,1,3,4,7,8,9) (0,1,6) (1,2,3,4,5,6,7,9) (1,8,9) (3,5,6,7) (0,2,3,5,6,7,8,9) (1,2,4,5) (0,1,2,3,4,5,6,7) {31,185,165,32,174,171,27,32,20,23}";

        let parsed: Machine = input.parse().unwrap();

        assert_eq!(
            parsed.calculate_fewest_presses_for_joltage_requirements(),
            195
        );
    }

    #[test]
    fn can_calculate_failing_machine_3() {
        let input = "[.####...##] (4) (0,3,4,5,6) (1,2,3,4,5,6,7,8,9) (0,1,2,3,5,8,9) (0,1,2,3,5,6,7,8) (1,2,6) (6,8) (0,2,3,4,5,7) (0,3,8,9) {57,48,63,59,42,54,46,24,42,27}";

        let parsed: Machine = input.parse().unwrap();

        assert_eq!(
            parsed.calculate_fewest_presses_for_joltage_requirements(),
            101
        );
    }
}
