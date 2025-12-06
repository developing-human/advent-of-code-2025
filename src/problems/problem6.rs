use crate::shared::Answer;

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
pub fn solve(input: &str) -> Answer {
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
    let answers = lines
        .last()
        .unwrap()
        .split_whitespace()
        .enumerate()
        .map(|(idx, op)| problems[idx].calculate(op));

    Answer {
        part1: answers.sum(),
        part2: 0,
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
        assert_eq!(result.part1, 3263827);
        // assert_eq!(result.part2, 14);
    }
}
