use std::collections::HashSet;

use crate::shared::Answer;

pub fn solve(input: &str) -> Answer {
    let mut active_beams: HashSet<usize> = HashSet::new();

    let mut splits = 0;
    for line in input.lines() {
        for (idx, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    active_beams.insert(idx);
                }
                '^' => {
                    if active_beams.contains(&idx) {
                        active_beams.remove(&idx);
                        active_beams.insert(idx - 1);
                        active_beams.insert(idx + 1);
                        splits += 1;
                    }
                }
                '.' => {}
                _ => panic!("unexpected character"),
            };
        }
    }

    Answer {
        part1: splits,
        part2: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_basic_input() {
        let input = r#"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

        let result = solve(input.trim());
        assert_eq!(result.part1, 21);
    }
}
