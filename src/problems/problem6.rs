use crate::shared::Answer;

#[derive(Debug)]
struct CephalopodMathProblem {
    lines: Vec<Vec<char>>,
}

impl CephalopodMathProblem {
    fn solve(&self) -> usize {
        let mut numbers: Vec<usize> = vec![];
        for x in 0..self.lines[0].len() {
            // concatenate these to get a number
            let mut number_string = String::new();
            for y in 0..(self.lines.len() - 1) {
                number_string.push(self.lines[y][x])
            }

            numbers.push(number_string.trim().parse().unwrap())
        }

        let operation = self.lines.last().unwrap().first().unwrap();
        match operation {
            '+' => numbers.iter().sum(),
            '*' => numbers.iter().product(),
            _ => panic!("unexpected operation: {operation}"),
        }
    }
}

struct CephalopodMathScroll {
    all_problem_chars: Vec<Vec<char>>,
}

impl CephalopodMathScroll {
    fn new(all_problem_text: &str) -> Self {
        Self {
            all_problem_chars: all_problem_text
                .trim()
                .lines()
                .map(|l| l.chars().collect())
                .collect(),
        }
    }

    fn problems(self) -> ProblemIterator {
        ProblemIterator::new(self)
    }
}

struct ProblemIterator {
    scroll: CephalopodMathScroll,
    cur_problem_index: Option<usize>,
}

impl ProblemIterator {
    fn new(scroll: CephalopodMathScroll) -> Self {
        Self {
            scroll,
            cur_problem_index: Some(0),
        }
    }

    fn find_next_problem_index(&self) -> Option<usize> {
        let cur_problem_index = self.cur_problem_index?;

        // finds the next problem, by looking for the math operation in the last line
        self.scroll
            .all_problem_chars
            .last()
            .unwrap()
            .iter()
            .skip(cur_problem_index + 1)
            .position(char::is_ascii_punctuation)
            .map(|i| i + cur_problem_index + 1)
    }

    fn find_next_problem_index_bad(
        lines: &[Vec<char>],
        last_problem_index: usize,
    ) -> Option<usize> {
        // String::new("hi").find(pat);
        // For each line, starting at last_problem_index, find the last digit.
        // Then take the max of that.
        // That's the end of this problem. Add two for start of next? Or change name of function to
        // be finding end of current.
        let next_problem_index = lines
            .iter()
            .map(|l| {
                let first_digit_index = l
                    .iter()
                    .skip(last_problem_index)
                    .position(char::is_ascii_digit)?
                    + last_problem_index;

                let digit_onward = &l[first_digit_index..];
                digit_onward.iter().position(|c| c.is_whitespace())
            })
            .max()?
            .map(|i| i + last_problem_index + 1);

        if let Some(next_problem_index) = next_problem_index
            && next_problem_index >= lines[0].len()
        {
            None
        } else {
            next_problem_index
        }
    }
}

impl Iterator for ProblemIterator {
    type Item = CephalopodMathProblem;

    fn next(&mut self) -> Option<Self::Item> {
        let curr_idx = self.cur_problem_index?;
        let next_idx = self.find_next_problem_index();

        // pulls out the text of a single problem from the scroll
        let problem_text = self
            .scroll
            .all_problem_chars
            .iter()
            .map(|line| {
                if let Some(next_idx) = next_idx {
                    &line[curr_idx..(next_idx - 1)]
                } else {
                    &line[curr_idx..]
                }
            })
            //TODO: Can I drop this copy?
            .map(|slice| slice.to_vec())
            .collect();

        self.cur_problem_index = next_idx;

        Some(CephalopodMathProblem {
            lines: problem_text,
        })
    }
}

struct MathProblem {
    values: Vec<usize>,
}

impl MathProblem {
    fn new(value: usize) -> Self {
        Self {
            values: vec![value],
        }
    }

    fn add_value(&mut self, value: usize) {
        self.values.push(value);
    }

    fn calculate(&self, operation: &str) -> usize {
        match operation {
            "+" => self.values.iter().sum(),
            "*" => self.values.iter().product(),
            _ => panic!("unexpected operation: {operation}"),
        }
    }
}

pub fn part1(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();

    // first line creates the math problems with one value
    let mut problems: Vec<_> = lines[0]
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .map(MathProblem::new)
        .collect();

    // middle lines add more values
    for line in &lines[1..(lines.len() - 1)] {
        line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .enumerate()
            .for_each(|(idx, value)| {
                problems[idx].add_value(value);
            });
    }

    // last line performs calculations
    lines
        .last()
        .unwrap()
        .split_whitespace()
        .enumerate()
        .map(|(idx, op)| problems[idx].calculate(op))
        .sum()
}

fn part2(input: &str) -> usize {
    // Create a 2D array of chars
    let scroll = CephalopodMathScroll::new(input);
    scroll.problems().map(|p| p.solve()).sum()
}
pub fn solve(input: &str) -> Answer {
    //TODO: I think these can merge once I parse them into problems by string, i can have two
    //different calculate functions, one for each part.
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_basic_input() {
        let input = r#"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +"#;

        let result = solve(input.trim());
        assert_eq!(result.part1, 4277556);
        assert_eq!(result.part2, 3263827);
    }

    #[test]
    fn find_next_problem_index() {
        let input = r#"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +"#;

        let scroll = CephalopodMathScroll::new(input);
        let mut problems = scroll.problems();

        assert!(problems.next().is_some());
        assert!(problems.next().is_some());
        assert!(problems.next().is_some());
        assert!(problems.next().is_some());
        assert!(problems.next().is_none());
    }

    #[test]
    fn solve_a_math_problem() {
        let input = r#"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +"#;

        let problem = CephalopodMathScroll::new(input).problems().next().unwrap();
        assert_eq!(problem.solve(), 8544);
    }
}
