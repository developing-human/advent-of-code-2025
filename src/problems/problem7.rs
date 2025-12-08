use crate::shared::Answer;

pub fn solve(input: &str) -> Answer {
    let mut splits = 0;

    let line_length = input.find('\n').unwrap();
    let mut possible_timelines: Vec<usize> = vec![0; line_length];
    for line in input.lines() {
        for (idx, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    possible_timelines[idx] = 1;
                }
                '^' => {
                    if possible_timelines[idx] > 0 {
                        possible_timelines[idx - 1] += possible_timelines[idx];
                        possible_timelines[idx + 1] += possible_timelines[idx];
                        possible_timelines[idx] = 0;
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
        part2: possible_timelines.iter().sum(),
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
