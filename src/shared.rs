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

/// Splits a string into partitions of the requested size
pub struct NumericPartitionIterator {
    pub remaining: usize,
    divisor: usize,
}

impl NumericPartitionIterator {
    /// Creates an iterator which breaks a number into partitions
    /// of the specified size.
    ///
    /// WARNING: This goes right to left.
    pub fn new(to_split: usize, partition_size: u32) -> Self {
        Self {
            remaining: to_split,
            divisor: 10_usize.pow(partition_size),
        }
    }
}

impl Iterator for NumericPartitionIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let partition = self.remaining % self.divisor;
        self.remaining /= self.divisor;

        Some(partition)
    }
}

/// Given a location (x, y) and limits, returns up to eight neighbors which are in bounds.
pub struct Neighborator {
    center: (usize, usize),
    dimensions: (usize, usize),

    index: usize,
}

impl Neighborator {
    pub fn new(center: (usize, usize), dimensions: (usize, usize)) -> Self {
        Self {
            center,
            dimensions,
            index: 0,
        }
    }
}

const NEIGHBOR_DELTAS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Iterator for Neighborator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < NEIGHBOR_DELTAS.len() {
            let delta = NEIGHBOR_DELTAS[self.index];
            self.index += 1;

            // Is x in bounds?
            let neighbor_x = self.center.0 as i32 + delta.0;
            if neighbor_x < 0 || neighbor_x >= self.dimensions.0 as i32 {
                continue; // try the next potential neighbor
            }

            // Is y in bounds?
            let neighbor_y = self.center.1 as i32 + delta.1;
            if neighbor_y < 0 || neighbor_y >= self.dimensions.1 as i32 {
                continue; // try the next potential neighbor
            }

            return Some((neighbor_x as usize, neighbor_y as usize));
        }

        None // no more neighbors :(
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_partition_by_1() {
        let mut iter = NumericPartitionIterator::new(12345, 1);

        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn numeric_partition_by_2() {
        let mut iter = NumericPartitionIterator::new(123456, 2);

        assert_eq!(iter.next(), Some(56));
        assert_eq!(iter.next(), Some(34));
        assert_eq!(iter.next(), Some(12));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn numeric_partition_by_3() {
        let mut iter = NumericPartitionIterator::new(123456, 3);

        assert_eq!(iter.next(), Some(456));
        assert_eq!(iter.next(), Some(123));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn numeric_partition_too_few_digits() {
        let mut iter = NumericPartitionIterator::new(23456, 3);

        assert_eq!(iter.next(), Some(456));
        assert_eq!(iter.next(), Some(23));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn neighborator_all_in_bounds() {
        let iter = Neighborator::new((1, 1), (3, 3));

        // collecting & using contains, because order doesn't matter
        let v: Vec<(usize, usize)> = iter.collect();
        assert!(v.contains(&(0, 0)));
        assert!(v.contains(&(0, 1)));
        assert!(v.contains(&(0, 2)));
        assert!(v.contains(&(1, 0)));
        assert!(v.contains(&(1, 2)));
        assert!(v.contains(&(2, 0)));
        assert!(v.contains(&(2, 1)));
        assert!(v.contains(&(2, 2)));
    }

    #[test]
    fn neighborator_all_top_left() {
        let iter = Neighborator::new((0, 0), (3, 3));

        // collecting & using contains, because order doesn't matter
        let v: Vec<(usize, usize)> = iter.collect();
        assert_eq!(v.len(), 3);
        assert!(v.contains(&(0, 1)));
        assert!(v.contains(&(1, 1)));
        assert!(v.contains(&(1, 0)));
    }

    #[test]
    fn neighborator_all_bottom_right() {
        let iter = Neighborator::new((2, 2), (3, 3));

        // collecting & using contains, because order doesn't matter
        let v: Vec<(usize, usize)> = iter.collect();
        assert_eq!(v.len(), 3);
        assert!(v.contains(&(1, 1)));
        assert!(v.contains(&(1, 2)));
        assert!(v.contains(&(2, 1)));
    }

    #[test]
    fn neighborator_all_one_row() {
        let iter = Neighborator::new((1, 0), (3, 1));

        // collecting & using contains, because order doesn't matter
        let v: Vec<(usize, usize)> = iter.collect();
        assert_eq!(v.len(), 2);
        assert!(v.contains(&(0, 0)));
        assert!(v.contains(&(2, 0)));
    }

    #[test]
    fn neighborator_all_one_column() {
        let iter = Neighborator::new((0, 1), (1, 3));

        // collecting & using contains, because order doesn't matter
        let v: Vec<(usize, usize)> = iter.collect();
        assert_eq!(v.len(), 2);
        assert!(v.contains(&(0, 0)));
        assert!(v.contains(&(0, 2)));
    }
}
