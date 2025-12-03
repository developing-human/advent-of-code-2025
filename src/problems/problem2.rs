use crate::shared::{Answer, PartitionIterator};
use rayon::prelude::*;

/// A product id, which implements validity checks.
pub struct ProductId {
    text: String,
    num: usize,
}

impl ProductId {
    fn new(id: usize) -> Self {
        ProductId {
            text: format!("{id}"),
            num: id,
        }
    }

    fn has_two_matching_partitions(&self) -> bool {
        if !self.text.len().is_multiple_of(2) {
            return false; // odd length strings can't match
        }

        self.has_matching_partitions_of_size(self.text.len() / 2)
    }

    fn has_n_matching_partitions(&self) -> bool {
        // try split sizes up to half the length, past that it can't divide evenly
        let mut split_sizes_to_try = 1..=(self.text.len() / 2);

        split_sizes_to_try.any(|split_size| self.has_matching_partitions_of_size(split_size))
    }

    fn has_matching_partitions_of_size(&self, split_size: usize) -> bool {
        // if s can't be split evenly, it won't have matching partitions
        if !self.text.len().is_multiple_of(split_size) {
            return false;
        }

        // split off the first partition, all others must match this
        let mut partitions = self.partitions(split_size);
        let first_partition = partitions.next().unwrap();

        partitions.all(|this_partition| this_partition == first_partition)
    }

    fn partitions<'a>(&'a self, split_size: usize) -> PartitionIterator<'a> {
        PartitionIterator::new(&self.text, split_size)
    }
}

pub fn solve(input: &str) -> Answer {
    input
        .split(",")
        .collect::<Vec<&str>>()
        .par_iter()
        .map(|s| solve_one_range(s))
        .sum()
}

fn solve_one_range(range: &str) -> Answer {
    let split: Vec<&str> = range.trim().split("-").collect();
    let start: usize = split[0].parse().expect("start of range should be integer");
    let end: usize = split[1].parse().expect("end of range should be integer");

    (start..=end)
        .map(|num| {
            let id = ProductId::new(num);
            let two_matches = id.has_two_matching_partitions();
            let n_matches = id.has_n_matching_partitions();

            Answer {
                part1: if two_matches { id.num } else { 0 },
                part2: if n_matches { id.num } else { 0 },
            }
        })
        .sum()
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
