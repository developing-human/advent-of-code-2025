use crate::shared::Answer;

const TOO_MANY_NEIGHBORS_TO_MOVE: usize = 4;
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

struct HelpfulDiagram {
    neighbor_counts: Vec<Vec<usize>>,
    rolls: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl HelpfulDiagram {
    pub fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let width = lines[0].len();
        let height = lines.len();

        let rolls = vec![vec![false; height]; width];
        let neighbor_counts = vec![vec![0_usize; height]; width];
        let mut diagram = Self {
            width,
            height,
            rolls,
            neighbor_counts,
        };

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '@' {
                    diagram.add_roll(x, y);
                }
            }
        }

        // for y in 0..height {
        //     for x in 0..width {
        //         print!("{} ", diagram.neighbor_counts[x][y])
        //     }
        //     println!();
        // }

        diagram
    }

    pub fn add_roll(&mut self, x: usize, y: usize) {
        self.rolls[x][y] = true;

        let (x, y) = (x as i32, y as i32);
        for (dx, dy) in NEIGHBOR_DELTAS {
            let neighbor_x = x + dx;
            let neighbor_y = y + dy;

            if self.in_bounds(neighbor_x, neighbor_y) {
                self.neighbor_counts[neighbor_x as usize][neighbor_y as usize] += 1;
            }
        }
    }

    //TODO: Deduplicate w/ add_roll if this approach works nicely
    pub fn remove_roll_recursive(&mut self, x: usize, y: usize) -> usize {
        self.rolls[x][y] = false;
        let mut removed_count = 1;

        let mut buffer = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.has_roll_at(x as i32, y as i32) {
                    buffer.push('@');
                } else {
                    buffer.push(' ');
                }
            }
            buffer.push('\n');
        }

        print!("\x1B[2J"); // clear?
        println!("{buffer}");
        // sleep(Duration::from_millis(10));

        let (x, y) = (x as i32, y as i32);
        for (dx, dy) in NEIGHBOR_DELTAS {
            let neighbor_x = x + dx;
            let neighbor_y = y + dy;

            if self.in_bounds(neighbor_x, neighbor_y) {
                let (neighbor_x, neighbor_y) = (neighbor_x as usize, neighbor_y as usize);
                self.neighbor_counts[neighbor_x][neighbor_y] -= 1;

                if self.has_roll_at(neighbor_x as i32, neighbor_y as i32)
                    && self.neighbor_counts[neighbor_x][neighbor_y] < TOO_MANY_NEIGHBORS_TO_MOVE
                {
                    removed_count += self.remove_roll_recursive(neighbor_x, neighbor_y);
                }
            }
        }

        removed_count
    }

    // pub fn try_remove_neighbors(&mut self, x: usize, y: usize) -> usize {
    //     for (dx, dy) in NEIGHBOR_DELTAS {
    //         let neighbor_x = x + dx;
    //         let neighbor_y = y + dy;
    //
    //         if self.in_bounds(neighbor_x, neighbor_y) {
    //             self.neighbor_counts[neighbor_x as usize][neighbor_y as usize] -= 1;
    //         }
    //     }
    // }

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    // Checks if a roll is present. Returns false if out of bounds.
    fn has_roll_at(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return false;
        }

        self.rolls[x as usize][y as usize]
    }

    fn count_adjacent_rolls(&self, x: i32, y: i32) -> usize {
        self.neighbor_counts[x as usize][y as usize]
    }
}

pub fn solve(input: &str) -> Answer {
    let mut diagram = HelpfulDiagram::parse(input);

    let mut can_initially_remove = 0;
    for y in 0..diagram.height {
        for x in 0..diagram.width {
            let adjacent_rolls = diagram.count_adjacent_rolls(x as i32, y as i32);

            if diagram.has_roll_at(x as i32, y as i32)
                && adjacent_rolls < TOO_MANY_NEIGHBORS_TO_MOVE
            {
                can_initially_remove += 1;
            }
        }
    }

    let mut can_eventually_remove = 0;
    for y in 0..diagram.height {
        for x in 0..diagram.width {
            let adjacent_rolls = diagram.count_adjacent_rolls(x as i32, y as i32);

            // print!("{adjacent_rolls} ");
            if diagram.has_roll_at(x as i32, y as i32)
                && adjacent_rolls < TOO_MANY_NEIGHBORS_TO_MOVE
            {
                can_eventually_remove += diagram.remove_roll_recursive(x, y);
            }
        }
        // println!();
    }

    Answer {
        part1: can_initially_remove,
        part2: can_eventually_remove,
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
        assert_eq!(result.part2, 43);
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
