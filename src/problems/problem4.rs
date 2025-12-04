use crate::shared::Answer;

struct HelpfulDiagram {
    rolls: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl HelpfulDiagram {
    fn parse(input: &str) -> Self {
        let rolls: Vec<Vec<bool>> = input
            .lines()
            .map(|line| line.chars().map(|c| c == '@').collect())
            .collect();

        Self {
            width: rolls[0].len(),
            height: rolls.len(),
            rolls,
        }
    }

    // Checks if a roll is present. Returns false if out of bounds.
    fn has_roll_at(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return false;
        }

        self.rolls[y as usize][x as usize]
    }

    fn count_adjacent_rolls(&self, x: i32, y: i32) -> usize {
        let deltas = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        deltas
            .iter()
            .filter(|d| self.has_roll_at(x + d.0, y + d.1))
            .count()
    }
}

pub fn solve(input: &str) -> Answer {
    let diagram = HelpfulDiagram::parse(input);

    let mut total = 0;
    for y in 0..diagram.height {
        for x in 0..diagram.width {
            let adjacent_rolls = diagram.count_adjacent_rolls(x as i32, y as i32);

            // print!("{adjacent_rolls} ");
            if diagram.has_roll_at(x as i32, y as i32) && adjacent_rolls < 4 {
                total += 1;
            }
        }
        // println!();
    }

    Answer {
        part1: total,
        part2: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_basic_input() {
        let input = r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

        let result = solve(input.trim());
        assert_eq!(result.part1, 13);
    }

    #[test]
    fn can_parse_input() {
        let input = r#"
..@@
@@@."#;

        let diagram = HelpfulDiagram::parse(input.trim());
        assert!(!diagram.has_roll_at(0, 0));
        assert!(!diagram.has_roll_at(1, 0));
        assert!(diagram.has_roll_at(2, 0));
        assert!(diagram.has_roll_at(3, 0));
        assert!(diagram.has_roll_at(0, 1));
        assert!(diagram.has_roll_at(1, 1));
        assert!(diagram.has_roll_at(2, 1));
        assert!(!diagram.has_roll_at(3, 1));
    }

    #[test]
    fn can_count_neighbors() {
        let input = r#"
@.@
.@@
@.@"#;

        let diagram = HelpfulDiagram::parse(input.trim());
        assert_eq!(diagram.count_adjacent_rolls(0, 0), 1);
        assert_eq!(diagram.count_adjacent_rolls(2, 0), 2);
        assert_eq!(diagram.count_adjacent_rolls(1, 1), 5);
        assert_eq!(diagram.count_adjacent_rolls(2, 1), 3);
        assert_eq!(diagram.count_adjacent_rolls(0, 2), 1);
        assert_eq!(diagram.count_adjacent_rolls(2, 2), 2);
    }
}
