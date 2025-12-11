use std::collections::HashSet;

use itertools::Itertools;

use crate::shared::Answer;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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
    area: usize,
}

impl Rect {
    fn new(top_left: &Point, bottom_right: &Point) -> Self {
        Self {
            top_left: *top_left,
            bottom_right: *bottom_right,
            area: Self::area(top_left, bottom_right),
        }
    }

    fn area(top_left: &Point, bottom_right: &Point) -> usize {
        let width = usize::abs_diff(top_left.x, bottom_right.x) + 1;
        let height = usize::abs_diff(top_left.y, bottom_right.y) + 1;

        width * height
    }

    fn lines(&self) -> Vec<Line> {
        let top_right = Point::new(self.bottom_right.x, self.top_left.y);
        let bottom_left = Point::new(self.top_left.x, self.bottom_right.y);
        vec![
            Line::new(&self.top_left, &top_right),
            Line::new(&top_right, &self.bottom_right),
            Line::new(&self.bottom_right, &bottom_left),
            Line::new(&bottom_left, &self.top_left),
        ]
    }

    fn perimeter_points(&self) -> HashSet<Point> {
        self.lines().iter().flat_map(|l| l.points()).collect()
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: &Point, end: &Point) -> Self {
        Self {
            start: *start,
            end: *end,
        }
    }

    fn points(&self) -> Vec<Point> {
        if self.start.x == self.end.x {
            let min_y = self.start.y.min(self.end.y);
            let max_y = self.start.y.max(self.end.y);

            (min_y..=max_y)
                .map(|y| Point::new(self.start.x, y))
                .collect()
        } else {
            let min_x = self.start.x.min(self.end.x);
            let max_x = self.start.x.max(self.end.x);

            (min_x..=max_x)
                .map(|x| Point::new(x, self.start.y))
                .collect()
        }
    }
}

struct Polygon {
    borders: Vec<Line>,
    concave_vertices: HashSet<Point>,
}

impl Polygon {
    fn new(points: &[Point]) -> Self {
        let borders = points
            .iter()
            .tuple_windows()
            .map(|(start, end)| Line::new(start, end))
            // add line for end to start
            .chain(std::iter::once(Line::new(
                points.last().unwrap(),
                points.first().unwrap(),
            )))
            .collect::<Vec<_>>();

        let concave_vertices = borders
            .iter()
            .tuple_windows()
            .chain(std::iter::once((
                borders.last().unwrap(),
                borders.first().unwrap(),
            )))
            .map(|(first, second)| {
                // first.end and second.start are the same. Is that point convex?
                let vertex = first.end;
                let first_dir: Direction = first.into();
                let second_dir: Direction = second.into();
                (vertex, first_dir, second_dir)
            })
            .filter_map(
                |(vertex, first_dir, second_dir)| match (first_dir, second_dir) {
                    (Direction::Right, Direction::Down) => None,
                    (Direction::Right, Direction::Up) => Some(vertex),
                    (Direction::Down, Direction::Left) => None,
                    (Direction::Down, Direction::Right) => Some(vertex),

                    (Direction::Left, Direction::Down) => Some(vertex),
                    (Direction::Left, Direction::Up) => None,
                    (Direction::Up, Direction::Left) => Some(vertex),
                    (Direction::Up, Direction::Right) => None,
                    _ => panic!("Impossible turn encountered: {first_dir:?} -> {second_dir:?}"),
                },
            )
            .collect::<HashSet<_>>();
        Self {
            borders,
            concave_vertices,
        }
    }
}

pub fn solve(input: &str) -> Answer {
    let points = build_points(input);
    let mut all_rects = build_rects(&points);
    let polygon = Polygon::new(&points);

    all_rects.sort_by(|a, b| b.area.cmp(&a.area));

    let max_rect_area = all_rects.iter().map(|r| r.area).next().unwrap();

    // Processing in sorted order, so the first rectangle to pass the filter
    // will be the largest that fits.
    let max_in_bound_rect_area = all_rects
        .iter()
        .enumerate()
        .inspect(|(idx, rect)| {
            println!(
                "processing rect: {} of {} (area = {})",
                idx,
                all_rects.len(),
                rect.area
            )
        })
        .map(|(_, rect)| rect)
        .filter(|r| rect_in_bounds(r, &polygon))
        .map(|r| r.area)
        .next()
        .unwrap();

    Answer {
        part1: max_rect_area,
        part2: max_in_bound_rect_area,
    }
}

fn build_rects(points: &[Point]) -> Vec<Rect> {
    points
        .iter()
        .combinations(2)
        .map(|v| Rect::new(v[0], v[1]))
        .collect::<Vec<_>>()
}

fn build_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .map(|(x, y)| Point::new(x, y))
        .collect::<Vec<_>>()
}

fn rect_in_bounds(rect: &Rect, polygon: &Polygon) -> bool {
    let top_right = Point::new(rect.bottom_right.x, rect.top_left.y);
    let bottom_left = Point::new(rect.top_left.x, rect.bottom_right.y);

    // if the "implied" corners aren't in bounds, the rect isn't in bounds
    if !point_in_bounds(&top_right, polygon) || !point_in_bounds(&bottom_left, polygon) {
        return false;
    }

    // the corners are all in bounds, now walk the perimeter
    rect.perimeter_points()
        .iter()
        .all(|p| point_in_bounds(p, polygon))
}

fn point_in_bounds(point: &Point, polygon: &Polygon) -> bool {
    // Count how many borders are crossed between (0, point.y) and (point.x, point.y).
    let intersecting_borders = polygon
        .borders
        .iter()
        .filter(|b| b.start.x == b.end.x) // only vertical lines
        .filter(|b| point.x >= b.start.x) // point is to the right of this border
        .filter(|b| {
            let min = b.start.y.min(b.end.y);
            let max = b.start.y.max(b.end.y);
            point.y >= min && point.y <= max
        })
        .collect::<Vec<_>>();

    let point_on_border = intersecting_borders.iter().any(|b| {
        let min = b.start.y.min(b.end.y);
        let max = b.start.y.max(b.end.y);
        point.x == b.start.x && point.y >= min && point.y <= max
    });

    if point_on_border {
        return true;
    }

    // If the ray passed through a concave vertice, ignore this intersection.
    let intersecting_borders = intersecting_borders.iter().filter(|b| {
        let vertex_to_check = if point.y == b.start.y {
            b.start
        } else if point.y == b.end.y {
            b.end
        } else {
            return true;
        };

        !polygon.concave_vertices.contains(&vertex_to_check)
    });

    let count = intersecting_borders.count();

    // println!("count: {count}");

    count % 2 == 1
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<&Line> for Direction {
    fn from(line: &Line) -> Self {
        if line.start.x == line.end.x {
            if line.end.y > line.start.y {
                Self::Down
            } else {
                Self::Up
            }
        } else if line.end.x > line.start.x {
            Self::Right
        } else {
            Self::Left
        }
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
        assert_eq!(result.part2, 24);
    }

    #[test]
    fn calculate_area() {
        let rect = Rect::new(&Point::new(2, 5), &Point::new(9, 7));
        assert_eq!(rect.area, 24);
    }

    #[test]
    fn check_point_in_bounds() {
        // ..............
        // .......0XXX1..
        // .......X...X..
        // ..6XXXX7...X..
        // ..X........X..
        // ..5XXXXXX4.X..
        // .........X.X..
        // .........3X2..
        // ..............
        let input = r#"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

        let points = build_points(input.trim());
        let poly = Polygon::new(&points);

        assert!(point_in_bounds(&Point::new(10, 6), &poly)); // broken
        assert!(!point_in_bounds(&Point::new(12, 6), &poly)); // broken
        assert!(!point_in_bounds(&Point::new(13, 6), &poly)); // broken

        assert!(!point_in_bounds(&Point::new(2, 0), &poly));
        assert!(!point_in_bounds(&Point::new(3, 0), &poly));
        assert!(!point_in_bounds(&Point::new(4, 0), &poly));
        assert!(!point_in_bounds(&Point::new(5, 0), &poly));
        assert!(!point_in_bounds(&Point::new(6, 0), &poly));
        assert!(!point_in_bounds(&Point::new(7, 0), &poly));
        assert!(!point_in_bounds(&Point::new(8, 0), &poly));
        assert!(!point_in_bounds(&Point::new(9, 0), &poly));
        assert!(!point_in_bounds(&Point::new(10, 0), &poly));
        assert!(!point_in_bounds(&Point::new(11, 0), &poly));
        assert!(!point_in_bounds(&Point::new(12, 0), &poly));
        assert!(!point_in_bounds(&Point::new(13, 0), &poly));

        assert!(!point_in_bounds(&Point::new(2, 1), &poly));
        assert!(!point_in_bounds(&Point::new(3, 1), &poly));
        assert!(!point_in_bounds(&Point::new(4, 1), &poly));
        assert!(!point_in_bounds(&Point::new(5, 1), &poly));
        assert!(!point_in_bounds(&Point::new(6, 1), &poly));
        assert!(point_in_bounds(&Point::new(7, 1), &poly));
        assert!(point_in_bounds(&Point::new(8, 1), &poly));
        assert!(point_in_bounds(&Point::new(9, 1), &poly));
        assert!(point_in_bounds(&Point::new(10, 1), &poly));
        assert!(point_in_bounds(&Point::new(11, 1), &poly));
        assert!(!point_in_bounds(&Point::new(12, 1), &poly));
        assert!(!point_in_bounds(&Point::new(13, 1), &poly));

        assert!(!point_in_bounds(&Point::new(2, 2), &poly));
        assert!(!point_in_bounds(&Point::new(3, 2), &poly));
        assert!(!point_in_bounds(&Point::new(4, 2), &poly));
        assert!(!point_in_bounds(&Point::new(5, 2), &poly));
        assert!(!point_in_bounds(&Point::new(6, 2), &poly));
        assert!(point_in_bounds(&Point::new(7, 2), &poly));
        assert!(point_in_bounds(&Point::new(8, 2), &poly));
        assert!(point_in_bounds(&Point::new(9, 2), &poly));
        assert!(point_in_bounds(&Point::new(10, 2), &poly));
        assert!(point_in_bounds(&Point::new(11, 2), &poly));
        assert!(!point_in_bounds(&Point::new(12, 2), &poly));
        assert!(!point_in_bounds(&Point::new(13, 2), &poly));

        assert!(point_in_bounds(&Point::new(2, 3), &poly));
        assert!(point_in_bounds(&Point::new(3, 3), &poly));
        assert!(point_in_bounds(&Point::new(4, 3), &poly));
        assert!(point_in_bounds(&Point::new(5, 3), &poly));
        assert!(point_in_bounds(&Point::new(6, 3), &poly));
        assert!(point_in_bounds(&Point::new(7, 3), &poly));
        assert!(point_in_bounds(&Point::new(8, 3), &poly));
        assert!(point_in_bounds(&Point::new(9, 3), &poly));
        assert!(point_in_bounds(&Point::new(10, 3), &poly));
        assert!(point_in_bounds(&Point::new(11, 3), &poly));
        assert!(!point_in_bounds(&Point::new(12, 3), &poly));

        assert!(point_in_bounds(&Point::new(2, 4), &poly));
        assert!(point_in_bounds(&Point::new(3, 4), &poly));
        assert!(point_in_bounds(&Point::new(4, 4), &poly));
        assert!(point_in_bounds(&Point::new(5, 4), &poly));
        assert!(point_in_bounds(&Point::new(6, 4), &poly));
        assert!(point_in_bounds(&Point::new(7, 4), &poly));
        assert!(point_in_bounds(&Point::new(8, 4), &poly));
        assert!(point_in_bounds(&Point::new(9, 4), &poly));
        assert!(point_in_bounds(&Point::new(10, 4), &poly));
        assert!(point_in_bounds(&Point::new(11, 4), &poly));
        assert!(!point_in_bounds(&Point::new(12, 4), &poly));
        assert!(!point_in_bounds(&Point::new(13, 4), &poly));
        assert!(!point_in_bounds(&Point::new(13, 4), &poly));

        assert!(point_in_bounds(&Point::new(2, 5), &poly));
        assert!(point_in_bounds(&Point::new(3, 5), &poly));
        assert!(point_in_bounds(&Point::new(4, 5), &poly));
        assert!(point_in_bounds(&Point::new(5, 5), &poly));
        assert!(point_in_bounds(&Point::new(6, 5), &poly));
        assert!(point_in_bounds(&Point::new(7, 5), &poly));
        assert!(point_in_bounds(&Point::new(8, 5), &poly));
        assert!(point_in_bounds(&Point::new(9, 5), &poly));
        assert!(point_in_bounds(&Point::new(10, 5), &poly));
        assert!(point_in_bounds(&Point::new(11, 5), &poly));
        assert!(!point_in_bounds(&Point::new(12, 5), &poly));
        assert!(!point_in_bounds(&Point::new(13, 5), &poly));
        assert!(!point_in_bounds(&Point::new(13, 5), &poly));

        assert!(!point_in_bounds(&Point::new(2, 6), &poly));
        assert!(!point_in_bounds(&Point::new(3, 6), &poly));
        assert!(!point_in_bounds(&Point::new(4, 6), &poly));
        assert!(!point_in_bounds(&Point::new(5, 6), &poly));
        assert!(!point_in_bounds(&Point::new(6, 6), &poly));
        assert!(!point_in_bounds(&Point::new(7, 6), &poly));
        assert!(!point_in_bounds(&Point::new(8, 6), &poly));
        assert!(point_in_bounds(&Point::new(9, 6), &poly));
        assert!(point_in_bounds(&Point::new(11, 6), &poly));

        assert!(!point_in_bounds(&Point::new(2, 7), &poly));
        assert!(!point_in_bounds(&Point::new(3, 7), &poly));
        assert!(!point_in_bounds(&Point::new(4, 7), &poly));
        assert!(!point_in_bounds(&Point::new(5, 7), &poly));
        assert!(!point_in_bounds(&Point::new(6, 7), &poly));
        assert!(!point_in_bounds(&Point::new(7, 7), &poly));
        assert!(!point_in_bounds(&Point::new(8, 7), &poly));
        assert!(point_in_bounds(&Point::new(9, 7), &poly));
        assert!(point_in_bounds(&Point::new(10, 7), &poly));
        assert!(point_in_bounds(&Point::new(11, 7), &poly));
        assert!(!point_in_bounds(&Point::new(12, 7), &poly));
        assert!(!point_in_bounds(&Point::new(13, 7), &poly));
    }

    #[test]
    fn check_rect_in_bounds() {
        // ..............
        // .......0XXX1..
        // .......X...X..
        // ..6XXXX7...X..
        // ..X........X..
        // ..5XXXXXX4.X..
        // .........X.X..
        // .........3X2..
        // ..............
        let input = r#"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

        let points = build_points(input.trim());
        let point_11_1 = &points[1];
        let point_9_7 = &points[3];
        let point_9_5 = &points[4];
        let point_2_5 = &points[5];
        let point_2_3 = &points[6];
        let point_7_3 = &points[7];
        let poly = Polygon::new(&points);

        assert!(rect_in_bounds(&Rect::new(point_7_3, point_11_1), &poly));
        assert!(rect_in_bounds(&Rect::new(point_9_7, point_9_5), &poly));
        assert!(!rect_in_bounds(&Rect::new(point_2_3, point_11_1), &poly));
        assert!(!rect_in_bounds(&Rect::new(point_2_5, point_11_1), &poly));

        assert!(rect_in_bounds(&Rect::new(&points[7], &points[0]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[7], &points[1]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[7], &points[2]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[7], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[7], &points[4]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[7], &points[5]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[7], &points[6]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[7], &points[7]), &poly));

        assert!(!rect_in_bounds(&Rect::new(&points[6], &points[0]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[6], &points[1]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[6], &points[2]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[6], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[6], &points[4]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[6], &points[5]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[6], &points[6]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[6], &points[7]), &poly));

        assert!(!rect_in_bounds(&Rect::new(&points[5], &points[0]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[5], &points[1]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[5], &points[2]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[5], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[5], &points[4]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[5], &points[5]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[5], &points[6]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[5], &points[7]), &poly));

        assert!(rect_in_bounds(&Rect::new(&points[4], &points[0]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[4], &points[1]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[4], &points[2]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[4], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[4], &points[4]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[4], &points[5]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[4], &points[6]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[4], &points[7]), &poly));

        assert!(!rect_in_bounds(&Rect::new(&points[3], &points[0]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[3], &points[1]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[3], &points[2]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[3], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[3], &points[4]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[3], &points[5]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[3], &points[6]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[3], &points[7]), &poly));

        assert!(!rect_in_bounds(&Rect::new(&points[2], &points[0]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[2], &points[1]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[2], &points[2]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[2], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[2], &points[4]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[2], &points[5]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[2], &points[6]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[2], &points[7]), &poly));

        assert!(rect_in_bounds(&Rect::new(&points[0], &points[1]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[0], &points[2]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[0], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[0], &points[4]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[0], &points[5]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[0], &points[6]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[0], &points[7]), &poly));

        assert!(rect_in_bounds(&Rect::new(&points[1], &points[0]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[1]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[2]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[4]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[1], &points[5]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[1], &points[6]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[7]), &poly));
    }

    #[test]
    fn check_rect_in_bounds_edge_cases() {
        let input = r#"
2,2
4,2
4,1
6,1
6,2
7,2
7,4
6,4
6,5
4,5
4,4
2,4"#;

        // .......
        // ....2-3
        // ..0-1.45
        // ..|....|
        // ..B-A.76
        // ..  9-8
        let points = build_points(input.trim());
        let poly = Polygon::new(&points);
        assert!(rect_in_bounds(&Rect::new(&points[11], &points[0]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[11], &points[1]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[11], &points[2]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[11], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[11], &points[4]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[11], &points[5]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[11], &points[6]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[11], &points[7]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[11], &points[8]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[11], &points[9]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[11], &points[10]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[11], &points[11]), &poly));

        assert!(rect_in_bounds(&Rect::new(&points[10], &points[0]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[1]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[2]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[4]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[5]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[6]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[7]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[8]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[9]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[10]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[11]), &poly));

        assert!(!rect_in_bounds(&Rect::new(&points[9], &points[0]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[9], &points[1]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[9], &points[2]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[9], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[9], &points[4]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[9], &points[5]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[9], &points[6]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[9], &points[7]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[9], &points[8]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[9], &points[9]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[9], &points[10]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[9], &points[11]), &poly));

        assert!(!rect_in_bounds(&Rect::new(&points[8], &points[0]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[8], &points[1]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[8], &points[2]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[8], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[8], &points[4]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[8], &points[5]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[8], &points[6]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[8], &points[7]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[8], &points[8]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[8], &points[9]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[8], &points[10]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[8], &points[11]), &poly));

        assert!(rect_in_bounds(&Rect::new(&points[7], &points[0]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[7], &points[1]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[7], &points[2]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[7], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[7], &points[4]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[7], &points[5]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[7], &points[6]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[7], &points[7]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[7], &points[8]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[7], &points[9]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[7], &points[10]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[7], &points[11]), &poly));

        assert!(rect_in_bounds(&Rect::new(&points[6], &points[0]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[6], &points[1]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[6], &points[2]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[6], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[6], &points[4]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[6], &points[5]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[6], &points[6]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[6], &points[7]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[6], &points[8]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[6], &points[9]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[6], &points[10]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[6], &points[11]), &poly));

        assert!(rect_in_bounds(&Rect::new(&points[5], &points[0]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[5], &points[1]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[5], &points[2]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[5], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[5], &points[4]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[5], &points[5]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[5], &points[6]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[5], &points[7]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[5], &points[8]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[5], &points[9]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[5], &points[10]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[5], &points[11]), &poly));

        assert!(rect_in_bounds(&Rect::new(&points[4], &points[0]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[4], &points[1]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[4], &points[2]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[4], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[4], &points[4]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[4], &points[5]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[4], &points[6]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[4], &points[7]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[4], &points[8]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[4], &points[9]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[4], &points[10]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[4], &points[11]), &poly));

        assert!(!rect_in_bounds(&Rect::new(&points[3], &points[0]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[3], &points[1]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[3], &points[2]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[3], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[3], &points[4]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[3], &points[5]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[3], &points[6]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[3], &points[7]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[3], &points[8]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[3], &points[9]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[3], &points[10]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[3], &points[11]), &poly));

        assert!(!rect_in_bounds(&Rect::new(&points[2], &points[0]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[2], &points[1]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[2], &points[2]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[2], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[2], &points[4]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[2], &points[5]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[2], &points[6]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[2], &points[7]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[2], &points[8]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[2], &points[9]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[2], &points[10]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[2], &points[11]), &poly));

        assert!(rect_in_bounds(&Rect::new(&points[1], &points[0]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[1]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[2]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[4]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[5]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[6]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[7]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[8]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[9]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[10]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[11]), &poly));

        assert!(rect_in_bounds(&Rect::new(&points[0], &points[0]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[0], &points[1]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[0], &points[2]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[0], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[0], &points[4]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[0], &points[5]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[0], &points[6]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[0], &points[7]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[0], &points[8]), &poly));
        assert!(!rect_in_bounds(&Rect::new(&points[0], &points[9]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[0], &points[10]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[0], &points[11]), &poly));

        assert!(rect_in_bounds(&Rect::new(&points[8], &points[10]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[0]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[1]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[2]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[4]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[5]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[6]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[7]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[8]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[9]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[10], &points[11]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[0], &points[6]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[11], &points[5]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[11], &points[4]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[6], &points[0]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[6], &points[0]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[2], &points[4]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[4]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[5]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[6]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[7]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[8]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[9]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[10]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[11]), &poly));
        // assert!(rect_in_bounds(&Rect::new(point_4_1, point_6_4), &polygon));
    }

    #[test]
    fn check_rect_in_bounds_more_edge_cases() {
        let input = r#"
2,2
4,2
4,1
6,1
6,2
8,2
8,1
10,1
10,2
12,2
12,3
2,3
"#;

        // .............
        // ....2-3.6-7..
        // ..0-1.4-5.8-9
        // ..B---------A
        let points = build_points(input.trim());
        let poly = Polygon::new(&points);
        assert!(rect_in_bounds(&Rect::new(&points[0], &points[10]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[10]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[4], &points[10]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[5], &points[10]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[8], &points[10]), &poly));

        assert!(rect_in_bounds(&Rect::new(&points[0], &points[11]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[11]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[4], &points[11]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[5], &points[11]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[8], &points[11]), &poly));

        assert!(rect_in_bounds(&Rect::new(&points[2], &points[4]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[1], &points[3]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[6], &points[8]), &poly));
        assert!(rect_in_bounds(&Rect::new(&points[5], &points[7]), &poly));

        assert!(rect_in_bounds(&Rect::new(&points[11], &points[9]), &poly));
    }
}
