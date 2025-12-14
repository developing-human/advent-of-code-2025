use std::str::FromStr;

use itertools::Itertools;

use crate::shared::Answer;

#[derive(Clone, Debug)]
struct Button {
    // which lights this button will toggle
    connections: Vec<usize>,

    // the position of this button the machine
    #[allow(dead_code)]
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

    // TODO: something with joltages that isn't used yet
    #[allow(dead_code)]
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
}

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

    let total_button_presses: usize = machines
        .iter()
        // .collect::<Vec<&Machine>>()
        // .par_iter()
        .map(|m| m.calculate_minimal_configuration_instructions())
        .map(|ci: ConfigurationConstraints| ci.total_button_presses())
        .map(|count| count as usize)
        .sum();

    Answer {
        part1: total_button_presses,
        part2: 0,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn solve_basic_input() {
        let input = r#"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

        let result = solve(input.trim());
        assert_eq!(result.part1, 7);
        assert_eq!(result.part2, 0);
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
}
