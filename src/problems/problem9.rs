// Checks if rectangles are inside a polygon by checking if the borders of the rectangle intersect
// the borders of the polygon.
//
// There were tricky edge cases around:
// 1. concave vertices (which cause an intersection, but don't exit the polygon). I opted to track
//    which vertices were concave, and special case them.
// 2. rectangle edges stopping on borders, which I handled by imagining rectangle edges travel down
//    the center of a cell, but polygon edges are on the sides. Which side the polygon edge is on
//    is determined by the direction it is pointing (up/down/left/right).
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
}

#[derive(Debug, Clone)]
struct Line {
    start: Point,
    end: Point,
    direction: Direction,
}

impl Line {
    fn new(start: &Point, end: &Point) -> Self {
        let direction = if start.x == end.x {
            if end.y > start.y {
                Direction::Down
            } else {
                Direction::Up
            }
        } else if end.x > start.x {
            Direction::Right
        } else {
            Direction::Left
        };

        Self {
            start: *start,
            end: *end,
            direction,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
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

struct Polygon {
    vertical_borders: Vec<Line>,
    horizontal_borders: Vec<Line>,
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

        let vertical_borders = borders
            .clone()
            .into_iter()
            .filter(|l| l.start.x == l.end.x)
            .collect::<Vec<_>>();

        let horizontal_borders = borders
            .clone()
            .into_iter()
            .filter(|l| l.start.y == l.end.y)
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
                (vertex, first.direction, second.direction)
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
            vertical_borders,
            horizontal_borders,
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
        // .enumerate()
        // .inspect(|(idx, rect)| {
        //     println!(
        //         "processing rect: {} of {} (area = {}) {rect:?}",
        //         idx,
        //         all_rects.len(),
        //         rect.area
        //     )
        // })
        // .skip(47694)
        // .map(|(_, rect)| rect)
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
    let min_x = rect.top_left.x.min(rect.bottom_right.x);
    let max_x = rect.top_left.x.max(rect.bottom_right.x);
    let min_y = rect.top_left.y.min(rect.bottom_right.y);
    let max_y = rect.top_left.y.max(rect.bottom_right.y);

    let top_line = Line::new(&Point::new(min_x, min_y), &Point::new(max_x, min_y));
    let bottom_line = Line::new(&Point::new(min_x, max_y), &Point::new(max_x, max_y));
    let left_line = Line::new(&Point::new(min_x, min_y), &Point::new(min_x, max_y));
    let right_line = Line::new(&Point::new(max_x, min_y), &Point::new(max_x, max_y));

    !has_intersections(&top_line, polygon)
        && !has_intersections(&bottom_line, polygon)
        && !has_intersections_vertical(&left_line, polygon)
        && !has_intersections_vertical(&right_line, polygon)
}

fn has_intersections(line: &Line, polygon: &Polygon) -> bool {
    polygon
        .vertical_borders
        .iter()
        // border is at or after start of this line
        .filter(|b| {
            if b.direction == Direction::Down {
                b.start.x >= line.start.x
            } else {
                b.start.x > line.start.x
            }
        })
        // border is at or before the end of this line
        .filter(|b| {
            if b.direction == Direction::Down {
                b.start.x < line.end.x
            } else {
                b.start.x <= line.end.x
            }
        })
        // line intersects this border
        .filter(|b| {
            let min = b.start.y.min(b.end.y);
            let max = b.start.y.max(b.end.y);
            line.start.y >= min && line.start.y <= max
        })
        // is not intersecting a border at a concave vertex
        //
        // this is an edge case where crossing these vertices keeps the line on the inside of the
        // polygon.
        .any(|b| {
            let vertex_to_check = if line.start.y == b.start.y {
                b.start
            } else if line.start.y == b.end.y {
                b.end
            } else {
                return true;
            };

            !polygon.concave_vertices.contains(&vertex_to_check)
        })
}

// It might be possible to merge this with has_intersections... but I suspect keeping them separate
// is easier to read & reason about.
fn has_intersections_vertical(line: &Line, polygon: &Polygon) -> bool {
    polygon
        .horizontal_borders
        .iter()
        // border is at or after start of this line
        .filter(|b| {
            if b.direction == Direction::Right {
                b.start.y > line.start.y
            } else {
                b.start.y >= line.start.y
            }
        })
        // border is at or before the end of this line
        .filter(|b| {
            if b.direction == Direction::Right {
                b.start.y <= line.end.y
            } else {
                b.start.y < line.end.y
            }
        })
        // line intersects this border
        .filter(|b| {
            let min = b.start.x.min(b.end.x);
            let max = b.start.x.max(b.end.x);
            line.start.x >= min && line.start.x <= max
        })
        // is not intersecting a border at a concave vertex
        //
        // this is an edge case where crossing these vertices keeps the line on the inside of the
        // polygon.
        .any(|b| {
            let vertex_to_check = if line.start.x == b.start.x {
                b.start
            } else if line.start.x == b.end.x {
                b.end
            } else {
                return true;
            };

            !polygon.concave_vertices.contains(&vertex_to_check)
        })
}

// The amount of tests below may suggest edge cases were kicking my butt.

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
    fn check_has_intersections() {
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

        // this is the top line of the polygon, going backwards
        // meaning the rect corner used was "1", and the "0" is implied.
        // landing on the 0-7 edge shouldn't count, since it started in bounds.
        assert!(!has_intersections(
            &Line::new(&Point::new(7, 1), &Point::new(11, 1)),
            &poly,
        ));

        assert!(has_intersections(
            &Line::new(&Point::new(6, 1), &Point::new(7, 1)),
            &poly,
        ));

        assert!(has_intersections(
            &Line::new(&Point::new(5, 1), &Point::new(8, 1)),
            &poly,
        ));
        assert!(has_intersections(
            &Line::new(&Point::new(5, 2), &Point::new(8, 2)),
            &poly,
        ));
        assert!(!has_intersections(
            &Line::new(&Point::new(5, 3), &Point::new(8, 3)),
            &poly,
        ));
        assert!(has_intersections(
            &Line::new(&Point::new(5, 1), &Point::new(12, 1)),
            &poly,
        ));
        assert!(has_intersections(
            &Line::new(&Point::new(0, 3), &Point::new(4, 3)),
            &poly,
        ));
        assert!(!has_intersections(
            &Line::new(&Point::new(7, 1), &Point::new(11, 1)),
            &poly,
        ));
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
