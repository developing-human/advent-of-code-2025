use std::{collections::BinaryHeap, ops::RangeInclusive};

use crate::shared::Answer;

struct ComplicatedInventoryManagmentSystem {
    fresh_ingredients: Vec<IngredientRange>,
}

impl ComplicatedInventoryManagmentSystem {
    fn load(fresh_ingredients: &str) -> Self {
        let mut heap_of_ingredients = BinaryHeap::new();
        for line in fresh_ingredients.lines() {
            let (start, end) = line.split_once("-").unwrap();
            let start: IngredientId = start.parse().unwrap();
            let end: IngredientId = end.parse().unwrap();

            heap_of_ingredients.push(IngredientRange::new(start..=end));
        }

        Self {
            fresh_ingredients: heap_of_ingredients.into_sorted_vec(),
        }
    }

    fn is_ingredient_fresh(&self, id: IngredientId) -> bool {
        match self
            .fresh_ingredients
            .binary_search_by_key(&id, |r| r.start)
        {
            // If Ok (found), the start of a range was hit directly.
            Ok(_) => true,

            // If Err (not found), pos is either within the range or after it, so check the prior range.
            // Err(0) => false,
            // Err(pos) => self.fresh_ingredients[pos - 1].contains(id),
            Err(mut idx) => {
                // search backwards, until the id is larger than the range's start
                // this handles overlapping ranges
                while idx > 0 && id >= self.fresh_ingredients[idx - 1].start {
                    if self.fresh_ingredients[idx - 1].contains(id) {
                        return true;
                    }

                    idx -= 1;
                }

                false
            }
        }
    }
}

type IngredientId = usize;

#[derive(Eq, PartialEq, Debug)]
struct IngredientRange {
    start: IngredientId,
    end: IngredientId,
}

impl IngredientRange {
    fn new(range: RangeInclusive<IngredientId>) -> Self {
        Self {
            start: *range.start(),
            end: *range.end(),
        }
    }

    fn contains(&self, id: IngredientId) -> bool {
        id >= self.start && id <= self.end
    }
}

impl PartialOrd for IngredientRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IngredientRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

pub fn solve(input: &str) -> Answer {
    let (fresh_ingredients, ingredients_to_check) = input.split_once("\n\n").unwrap();

    let cims = ComplicatedInventoryManagmentSystem::load(fresh_ingredients);

    let fresh_ingredient_count = ingredients_to_check
        .lines()
        .map(|line| line.parse::<IngredientId>().unwrap())
        .filter(|&ingredient| cims.is_ingredient_fresh(ingredient))
        .count();

    Answer {
        part1: fresh_ingredient_count,
        part2: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_basic_input() {
        let input = r#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

        let result = solve(input.trim());
        assert_eq!(result.part1, 3);
    }

    #[test]
    fn overlapping_ranges_inner_first() {
        let cims = ComplicatedInventoryManagmentSystem::load("3-4\n2-5");

        assert!(!cims.is_ingredient_fresh(1));
        assert!(cims.is_ingredient_fresh(2));
        assert!(cims.is_ingredient_fresh(3));
        assert!(cims.is_ingredient_fresh(4));
        assert!(cims.is_ingredient_fresh(5));
        assert!(!cims.is_ingredient_fresh(6));
    }

    #[test]
    fn overlapping_ranges_outer_first() {
        let cims = ComplicatedInventoryManagmentSystem::load("2-5\n3-4");

        assert!(!cims.is_ingredient_fresh(1));
        assert!(cims.is_ingredient_fresh(2));
        assert!(cims.is_ingredient_fresh(3));
        assert!(cims.is_ingredient_fresh(4));
        assert!(cims.is_ingredient_fresh(5));
        assert!(!cims.is_ingredient_fresh(6));
    }
}
