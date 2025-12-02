use std::iter::Sum;

#[derive(Debug)]
pub struct Answer {
    part1: usize,
    part2: usize,
}

impl Sum for Answer {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut total = Answer { part1: 0, part2: 0 };
        for val in iter {
            total.part1 += val.part1;
            total.part2 += val.part2;
        }

        total
    }
}

pub fn solve(input: &str) -> Answer {
    input.split(",").map(solve_one_range).sum()
}

fn solve_one_range(range: &str) -> Answer {
    let split: Vec<&str> = range.trim().split("-").collect();
    let start: usize = split[0].parse().expect("start of range should be integer");
    let end: usize = split[1].parse().expect("end of range should be integer");

    // (start..=end).filter(is_invalid).sum()
    (start..=end)
        .map(|num| (num, is_invalid_part_1(&num), is_invalid_part_2(&num)))
        .map(|(num, part1, part2): (usize, bool, bool)| Answer {
            part1: if part1 { num } else { 0 },
            part2: if part2 { num } else { 0 },
        })
        .sum()
}

fn is_invalid_part_1(number: &usize) -> bool {
    let number_str = format!("{number}");

    if number_str.len() % 2 == 1 {
        return false; // odd length strings are always valid
    }

    let (first_half, second_half) = number_str.split_at(number_str.len() / 2);

    first_half == second_half
}

fn is_invalid_part_2(number: &usize) -> bool {
    // I need to check if parts repeat on a split of 1, 2, 3, etc
    // Up until when? If I have 10 digits, up to half the length rounded down
    //
    // TODO: There's certainly optimizations around knowing the result of the previous
    //       number, because only the last digit will change.
    //
    // I can use split_at to break a string into "this chunk" and the rest
    // then do it again to get the next.
    //
    // I can skip a split size if the length isn't divisible by it.
    let number_str = format!("{number}");

    'outer: for split_size in 1..=(number_str.len() / 2) {
        if !number_str.len().is_multiple_of(split_size) {
            continue;
        }

        let (first_partition, mut rest) = number_str.split_at(split_size);
        while !rest.is_empty() {
            let split = rest.split_at(split_size);

            let this_partition = split.0;
            rest = split.1;

            // if this doesn't match, continue outer and try next split size
            // if it does match, keep going. if all match, return true (is invalid) after inner.
            if this_partition != first_partition {
                continue 'outer;
            }
        }

        return true; // is invalid
    }

    false // must be valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_basic_input() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let result = solve(input);
        assert_eq!(result.part1, 1227775554);
        assert_eq!(result.part2, 4174379265);
    }

    #[test]
    fn solve_one_range_11_22() {
        let input = "11-22";
        let result = solve_one_range(input);
        assert_eq!(result.part1, 33);
        assert_eq!(result.part2, 33);
    }

    #[test]
    fn solve_one_range_95_115() {
        let input = "95-115";
        let result = solve_one_range(input);
        assert_eq!(result.part1, 99);
        assert_eq!(result.part2, 99 + 111);
    }

    #[test]
    fn solve_one_range_998_1012() {
        let input = "998-1012";
        let result = solve_one_range(input);
        assert_eq!(result.part1, 1010);
        assert_eq!(result.part2, 999 + 1010);
    }

    #[test]
    fn solve_one_range_1188511880_1188511890() {
        let input = "1188511880-1188511890";
        let result = solve_one_range(input);
        assert_eq!(result.part1, 1188511885);
        assert_eq!(result.part2, 1188511885);
    }

    #[test]
    fn solve_one_range_222220_222224() {
        let input = "222220-222224";
        let result = solve_one_range(input);
        assert_eq!(result.part1, 222222);
        assert_eq!(result.part2, 222222);
    }

    #[test]
    fn solve_one_range_1698522_1698528() {
        let input = "1698522-1698528";
        let result = solve_one_range(input);
        assert_eq!(result.part1, 0);
        assert_eq!(result.part2, 0);
    }

    #[test]
    fn solve_one_range_446443_446449() {
        let input = "446443-446449";
        let result = solve_one_range(input);
        assert_eq!(result.part1, 446446);
        assert_eq!(result.part2, 446446);
    }

    #[test]
    fn solve_one_range_38593856_38593862() {
        let input = "38593856-38593862";
        let result = solve_one_range(input);
        assert_eq!(result.part1, 38593859);
        assert_eq!(result.part2, 38593859);
    }
}
