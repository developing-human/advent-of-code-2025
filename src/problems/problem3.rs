use crate::shared::Answer;
// use rayon::prelude::*;

pub fn solve(input: &str) -> Answer {
    input
        .split("\n")
        .filter(|s| !s.trim().is_empty())
        // .collect::<Vec<&str>>()
        // .par_iter()
        .map(solve_one)
        .sum()
}

fn solve_one(battery_bank: &str) -> Answer {
    print!("processing: {battery_bank}");
    // iterate over digits, right to left
    let mut digit_iter = battery_bank.chars().rev().map(|c| c.to_digit(10).unwrap());

    // start with the rightmost digits
    let mut ones_digit = digit_iter.next().unwrap();
    let mut tens_digit = digit_iter.next().unwrap();

    for this_digit in digit_iter {
        // if this digit is bigger than the current tens, replace it
        if this_digit >= tens_digit {
            if tens_digit > ones_digit {
                ones_digit = tens_digit;
            }
            tens_digit = this_digit;
        }
    }
    println!(" -> {}", tens_digit * 10 + ones_digit);

    Answer {
        part1: (tens_digit * 10 + ones_digit) as usize,
        part2: 0,
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
    }

    #[test]
    fn solve_example_one() {
        let result = solve_one("987654321111111");

        assert_eq!(result.part1, 98);
    }

    #[test]
    fn solve_example_two() {
        let result = solve_one("811111111111119");

        assert_eq!(result.part1, 89);
    }

    #[test]
    fn solve_example_three() {
        let result = solve_one("234234234234278");

        assert_eq!(result.part1, 78);
    }

    #[test]
    fn solve_example_four() {
        let result = solve_one("818181911112111");

        assert_eq!(result.part1, 92);
    }

    #[test]
    fn solve_example_five_mine() {
        let result = solve_one("818191911112111");

        assert_eq!(result.part1, 99);
    }
}
