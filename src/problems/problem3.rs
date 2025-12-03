use crate::shared::Answer;

struct BatteryBank<'a> {
    joltages: &'a str,
}

impl<'a> BatteryBank<'a> {
    fn new(joltages: &'a str) -> Self {
        BatteryBank { joltages }
    }

    fn maximum_joltage(&self, max_batteries: usize) -> usize {
        // CALCULATE INDIVIDUAL JOLTAGES (iterates right to left)
        let mut digit_iter = self.joltages.chars().rev().map(|c| c.to_digit(10).unwrap());

        // INITIALIZE JOLTAGES
        let mut selected: Vec<u32> = digit_iter.by_ref().take(max_batteries).collect();
        selected.reverse(); // 0 is highest order digit, last is ones digit

        // DETERMINE MAXIMIZED JOLTAGE ARRAY
        for this_digit in digit_iter {
            // a battery that is available as a replacement
            let mut available_battery = this_digit;

            for selected_battery in selected.iter_mut() {
                if available_battery >= *selected_battery {
                    // swap out a selected battery for a better one
                    // making the old battery available
                    std::mem::swap(&mut *selected_battery, &mut available_battery);
                } else {
                    break; // no battery will be available to swap
                }
            }
        }

        // CALCULATE MAXIMUM JOLTAGE
        selected
            .into_iter()
            .enumerate()
            .map(|(idx, joltage)| {
                let exponent = (max_batteries - idx - 1) as u32;
                10_usize.pow(exponent) * joltage as usize
            })
            .sum()
    }
}

pub fn solve(input: &str) -> Answer {
    input
        .split("\n")
        .filter(|s| !s.trim().is_empty())
        .map(BatteryBank::new)
        .map(solve_one)
        .sum()
}

fn solve_one(battery_bank: BatteryBank) -> Answer {
    Answer {
        part1: battery_bank.maximum_joltage(2),
        part2: battery_bank.maximum_joltage(12),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_basic_input() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

        let result = solve(input);
        assert_eq!(result.part1, 357);
        assert_eq!(result.part2, 3121910778619);
    }

    #[test]
    fn solve_example_one() {
        let result = solve_one(BatteryBank::new("987654321111111"));

        assert_eq!(result.part1, 98);
        assert_eq!(result.part2, 987654321111);
    }

    #[test]
    fn solve_example_two() {
        let result = solve_one(BatteryBank::new("811111111111119"));

        assert_eq!(result.part1, 89);
        assert_eq!(result.part2, 811111111119);
    }

    #[test]
    fn solve_example_three() {
        let result = solve_one(BatteryBank::new("234234234234278"));

        assert_eq!(result.part1, 78);
        assert_eq!(result.part2, 434234234278);
    }

    #[test]
    fn solve_example_four() {
        let result = solve_one(BatteryBank::new("818181911112111"));

        assert_eq!(result.part1, 92);
        assert_eq!(result.part2, 888911112111);
    }

    #[test]
    fn solve_example_five_mine() {
        let result = solve_one(BatteryBank::new("818191911112111"));

        assert_eq!(result.part1, 99);
    }
}
