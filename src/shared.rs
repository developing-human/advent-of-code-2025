use std::iter::Sum;

/// Splits a string into partitions of the requested size
pub struct PartitionIterator<'a> {
    pub remaining: &'a str,
    pub partition_size: usize,
}

impl<'a> PartitionIterator<'a> {
    pub fn new(to_split: &'a str, partition_size: usize) -> Self {
        PartitionIterator {
            remaining: to_split,
            partition_size,
        }
    }
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

#[derive(Debug)]
pub struct Answer {
    pub part1: usize,
    pub part2: usize,
}

/// Enables calling .sum() on an iterator of Answers
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
