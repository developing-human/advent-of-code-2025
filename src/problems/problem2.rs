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

    fn partitions<'a>(&'a self, split_size: usize) -> PartitionIterator<'a> {
        PartitionIterator {
            remaining: &self.text,
            partition_size: split_size,
        }
    }
}

pub struct PartitionIterator<'a> {
    remaining: &'a str,
    partition_size: usize,
}

impl<'a> Iterator for PartitionIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_empty() {
            return None;
        }

        let (partition, remaining) = self.remaining.split_at(self.partition_size);
        self.remaining = remaining;

        Some(partition)
    }
}

pub fn solve(input: &str) -> Answer {
    input.split(",").map(solve_one_range).sum()
}

fn solve_one_range(range: &str) -> Answer {
    let split: Vec<&str> = range.trim().split("-").collect();
    let start: usize = split[0].parse().expect("start of range should be integer");
    let end: usize = split[1].parse().expect("end of range should be integer");

    (start..=end)
        .map(ProductId::new)
        .map(|id| {
            (
                has_two_matching_halves(&id),
                has_matching_partitions(&id),
                id,
            )
        })
        .map(|(part1, part2, id): (bool, bool, ProductId)| Answer {
            part1: if part1 { id.num } else { 0 },
            part2: if part2 { id.num } else { 0 },
        })
        .sum()
}

fn has_two_matching_halves(id: &ProductId) -> bool {
    if id.text.len() % 2 == 1 {
        return false; // odd length strings can't match
    }

    has_matching_partitions_for_split_size(id, id.text.len() / 2)
}

fn has_matching_partitions(id: &ProductId) -> bool {
    // NOTE: There's certainly optimizations around knowing the result of the previous
    //       number, because often only the last digit will change.

    // try split sizes up to half the length, past that it can't divide evenly
    let mut split_sizes_to_try = 1..=(id.text.len() / 2);

    split_sizes_to_try.any(|split_size| has_matching_partitions_for_split_size(id, split_size))
}

fn has_matching_partitions_for_split_size(id: &ProductId, split_size: usize) -> bool {
    // if s can't be split evenly, it won't have matching partitions
    if !id.text.len().is_multiple_of(split_size) {
        return false;
    }

    // split off the first partition, all others must match this
    let mut partitions = id.partitions(split_size);
    let first_partition = partitions.next().unwrap();

    partitions.all(|this_partition| this_partition == first_partition)
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
