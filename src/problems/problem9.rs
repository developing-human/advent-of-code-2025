use itertools::Itertools;

use crate::shared::Answer;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Rect {
    top_left: Point,
    bottom_right: Point,
}

impl Rect {
    fn new(top_left: Point, bottom_right: Point) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }

    fn area(&self) -> usize {
        let width = usize::abs_diff(self.top_left.x, self.bottom_right.x) + 1;
        let height = usize::abs_diff(self.top_left.y, self.bottom_right.y) + 1;

        width * height
    }
}

pub fn solve(input: &str) -> Answer {
    let points = input
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .map(|(x, y)| Point::new(x, y));

    let rects = points
        .combinations(2)
        // .collect::<Vec<Vec<Point>>>()
        // .iter()
        .map(|v| Rect {
            top_left: v[0],
            bottom_right: v[1],
        });

    // dbg!(&rects.clone().collect::<Vec<_>>());

    let part1 = rects.map(|r| r.area()).max().unwrap();

    Answer {
        part1: part1,
        part2: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_basic_input() {
        let input = r#"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

        let result = solve(input.trim());
        assert_eq!(result.part1, 50);
        assert_eq!(result.part2, 0);
    }

    #[test]
    fn calculate_area() {
        let rect = Rect::new(Point::new(2, 5), Point::new(9, 7));
        assert_eq!(rect.area(), 24);
    }
}
