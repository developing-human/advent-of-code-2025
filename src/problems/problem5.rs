use std::{num::ParseIntError, str::FromStr};

use crate::shared::Answer;

struct ComplicatedInventoryManagmentSystem {
    fresh_ingredients: Vec<IngredientRange>,
}

impl ComplicatedInventoryManagmentSystem {
    fn load(fresh_ingredients: &str) -> Self {
        let mut fresh_ingredients: Vec<IngredientRange> = fresh_ingredients
            .lines()
            .map(|s| s.parse().unwrap())
            .collect();

        fresh_ingredients.sort_unstable();

        // Pull out one range before looping, so the previous entry always exists.
        let mut iter = fresh_ingredients.into_iter();
        let mut fresh_ingredients_merged = vec![iter.next().unwrap()];

        for range in iter {
            let previous_idx = fresh_ingredients_merged.len() - 1;
            let previous = fresh_ingredients_merged.get_mut(previous_idx).unwrap();

            // When a new range starts inside the previous, extend previous
            // Otherwise, add a new range
            if range.start <= previous.end {
                if range.end > previous.end {
                    previous.end = range.end;
                }
            } else {
                fresh_ingredients_merged.push(range)
            }
        }

        Self {
            fresh_ingredients: fresh_ingredients_merged,
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
            Err(0) => false,
            Err(pos) => self.fresh_ingredients[pos - 1].contains(id),
        }
    }

    fn count_all_fresh_ingredients(&self) -> usize {
        self.fresh_ingredients.iter().map(|r| r.len()).sum()
    }

    fn count_requested_fresh_ingredients(&self, requested_ingredients: Vec<IngredientId>) -> usize {
        requested_ingredients
            .into_iter()
            .filter(|&ingredient| self.is_ingredient_fresh(ingredient))
            .count()
    }
}

type IngredientId = usize;

#[derive(Eq, PartialEq, Debug)]
struct IngredientRange {
    start: IngredientId,
    end: IngredientId,
}

impl IngredientRange {
    fn contains(&self, id: IngredientId) -> bool {
        id >= self.start && id <= self.end
    }

    fn len(&self) -> usize {
        self.end - self.start + 1
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

impl FromStr for IngredientRange {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once("-").expect("all ranges should have a hyphen");
        let start: IngredientId = start.parse()?;
        let end: IngredientId = end.parse()?;

        Ok(IngredientRange { start, end })
    }
}

pub fn solve(input: &str) -> Answer {
    let (fresh_ingredients, ingredients_to_check) = input.split_once("\n\n").unwrap();

    let requested_ingredients = ingredients_to_check
        .lines()
        .map(|line| line.parse::<IngredientId>().unwrap())
        .collect();

    let cims = ComplicatedInventoryManagmentSystem::load(fresh_ingredients);
    Answer {
        part1: cims.count_requested_fresh_ingredients(requested_ingredients),
        part2: cims.count_all_fresh_ingredients(),
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
        assert_eq!(result.part2, 14);
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
