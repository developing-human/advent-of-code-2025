use crate::shared::Answer;

struct TachyonParticleAnalyzer {
    possible_timelines: Vec<usize>,
    splits: usize,
}

impl TachyonParticleAnalyzer {
    fn new() -> Self {
        Self {
            possible_timelines: vec![],
            splits: 0,
        }
    }

    /// Analyzes a single row of tachyon particles, tabulating splits and possible timelines.
    fn analyze(&mut self, row: &str) {
        if self.possible_timelines.is_empty() {
            self.possible_timelines.resize(row.len(), 0);
        }

        for (idx, c) in row.chars().enumerate() {
            match c {
                'S' => {
                    self.possible_timelines[idx] = 1;
                }
                '^' => {
                    if self.possible_timelines[idx] > 0 {
                        // If a particle comes into this splitter, it's possibilities are applied
                        // to both split beams.
                        self.possible_timelines[idx - 1] += self.possible_timelines[idx];
                        self.possible_timelines[idx + 1] += self.possible_timelines[idx];
                        self.possible_timelines[idx] = 0;
                        self.splits += 1;
                    }
                }
                '.' => {}
                _ => panic!("unexpected character"),
            };
        }
    }

    fn splits(&self) -> usize {
        self.splits
    }

    fn possibilities(&self) -> usize {
        self.possible_timelines.iter().sum()
    }
}

pub fn solve(input: &str) -> Answer {
    let mut analyzer = TachyonParticleAnalyzer::new();

    input.lines().for_each(|l| analyzer.analyze(l));

    Answer {
        part1: analyzer.splits(),
        part2: analyzer.possibilities(),
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
        assert_eq!(result.part2, 40);
    }
}
