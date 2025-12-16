use std::fmt::Display;

use crate::shared::Answer;

#[derive(Debug)]
struct Shape {
    map: Vec<Vec<bool>>,
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.map.iter().rev() {
            let row_str = row
                .iter()
                .map(|&occupied| if occupied { '#' } else { '.' })
                .collect::<String>();

            writeln!(f, "{}", row_str)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
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
struct Present {
    rotate_0: Shape,
    rotate_90: Shape,
    rotate_180: Shape,
    rotate_270: Shape,
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,

    // a boolean map of the space to place presents, updated as presents are placed
    // access with map[y][x] / map [row][col]
    map: Vec<Vec<bool>>,

    // a vector, listing how many presents of each type are left to place.
    // indices match the order they are loaded
    presents_left_to_place: Vec<usize>,
}

impl Region {
    fn new(width: usize, height: usize, presents_left_to_place: Vec<usize>) -> Self {
        Self {
            width,
            height,
            map: vec![vec![false; width]; height],
            presents_left_to_place,
        }
    }

    // checks if a shape can be placed in the region without overlap
    // shape is assumed to be 3x3, and location is the center of the shape
    fn check_placement(&self, shape: &Shape, location: Point) -> bool {
        if location.x == 0
            || location.x >= self.width - 1
            || location.y == 0
            || location.y >= self.height - 1
        {
            // 3x3 shape can't be placed on the border
            return false;
        }

        // this will be a 3x3 grid on the map, determined by location.
        for shape_x in 0..3 {
            let map_x = location.x + shape_x - 1;
            for shape_y in 0..3 {
                let map_y = location.y + shape_y - 1;

                // if the shape and map both occupy this location, placement is invalid
                if shape.map[shape_y][shape_x] && self.map[map_y][map_x] {
                    return false;
                }
            }
        }

        true
    }

    // assumes the placement is valid, call check_placement first.
    fn place_present(&mut self, shape: &Shape, location: Point) {
        for shape_x in 0..3 {
            let map_x = location.x + shape_x - 1;
            for shape_y in 0..3 {
                let map_y = location.y + shape_y - 1;

                if shape.map[shape_y][shape_x] {
                    self.map[map_y][map_x] = true
                }
            }
        }
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.map.iter().rev() {
            let row_str = row
                .iter()
                .map(|&occupied| if occupied { '#' } else { '.' })
                .collect::<String>();

            writeln!(f, "{}", row_str)?;
        }

        Ok(())
    }
}

pub fn solve(input: &str) -> Answer {
    let (shapes, regions) = parse(input);

    let volume_per_shape: Vec<usize> = shapes
        .iter()
        .map(|s| s.map.iter().flatten().filter(|&&b| b).count())
        .collect();

    let mut stats = vec![];
    let mut does_not_fit = 0;
    for region in regions.iter() {
        let area = region.width * region.height;

        // Some can be ruled out because the gifts have more volume than the region.
        // Surprisingly, this is over half of the main inputs.
        let gift_volume: usize = region
            .presents_left_to_place
            .iter()
            .enumerate()
            .map(|(idx, count)| count * volume_per_shape[idx])
            .sum();

        if area < gift_volume {
            does_not_fit += 1;
        }

        stats.push(((gift_volume as f64 / area as f64) * 10000.0) as usize);

        println!(
            "area: {}, volume: {:5}, % occupied: {:5.2} {}",
            area,
            gift_volume,
            (gift_volume as f64 / area as f64) * 100.0,
            if area < gift_volume { "<----" } else { "" }
        );
    }

    stats.sort();

    dbg!(stats);

    Answer {
        part1: regions.len() - does_not_fit,
        part2: 0,
    }
}

fn parse(input: &str) -> (Vec<Shape>, Vec<Region>) {
    let mut lines = input.lines();

    // this assumes there's six 3x3 shapes, which is true for both inputs :shrug:
    let shapes = (0..=5)
        .map(|_| {
            // skip the header line "0:", etc
            lines.next();

            let map: Vec<Vec<bool>> = (0..3)
                .map(|_| lines.next().unwrap().chars().map(|c| c == '#').collect())
                .rev() // flip upside down, for rendering
                .collect();

            // skip the blank line between shapes
            lines.next();

            Shape { map }
        })
        .collect();

    let regions = lines
        .map(|line| {
            let (size_str, presents_str) = line.split_once(": ").unwrap();

            let (width, height) = size_str.split_once("x").unwrap();
            let (width, height) = (
                width.parse::<usize>().unwrap(),
                height.parse::<usize>().unwrap(),
            );

            let present_counts = presents_str
                .split(" ")
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            Region::new(width, height, present_counts)
        })
        .collect();

    (shapes, regions)
}

#[cfg(test)]
mod tests {

    use super::*;

    //     #[test]
    //     fn solve_basic_input() {
    //         let input = r#"
    // 0:
    // ###
    // ##.
    // ##.
    //
    // 1:
    // ###
    // ##.
    // .##
    //
    // 2:
    // .##
    // ###
    // ##.
    //
    // 3:
    // ##.
    // ###
    // ##.
    //
    // 4:
    // ###
    // #..
    // ###
    //
    // 5:
    // ###
    // .#.
    // ###
    //
    // 4x4: 0 0 0 0 2 0
    // 12x5: 1 0 1 0 2 2
    // 12x5: 1 0 1 0 3 2
    // "#;
    //         let result = solve(input.trim());
    //         assert_eq!(result.part1, 2);
    //     }

    #[test]
    fn can_parse_input() {
        let input = r#"
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"#;
        let (shapes, regions) = parse(input.trim());

        assert_eq!(shapes.len(), 6);
        assert_eq!(regions.len(), 3);

        assert_eq!(
            shapes[0].map,
            vec![
                vec![true, true, true],
                vec![true, true, false],
                vec![true, true, false]
            ]
        );

        assert_eq!(regions[1].width, 12);
        assert_eq!(regions[1].height, 5);
        assert_eq!(regions[1].map.len(), 5);
        assert_eq!(regions[1].map[0].len(), 12);

        assert_eq!(regions[1].presents_left_to_place, vec![1, 0, 1, 0, 2, 2]);
    }

    #[test]
    fn can_check_placement() {
        let mut region = Region::new(20, 10, vec![]);

        // ###
        // ##.
        // ##.
        //
        // ###
        // ###
        // ..#
        let shape = Shape {
            map: vec![
                vec![true, true, false],
                vec![true, true, false],
                vec![true, true, true],
            ],
        };

        println!("{shape}");

        region.map[0][2] = true;
        region.map[1][2] = true;

        println!("{region}");

        assert!(!region.check_placement(&shape, Point::new(0, 0)));
        assert!(region.check_placement(&shape, Point::new(1, 1)));
        assert!(region.check_placement(&shape, Point::new(1, 2)));
        assert!(!region.check_placement(&shape, Point::new(2, 1)));

        region.place_present(&shape, Point::new(1, 1));
        println!("{region}");

        panic!()
    }

    // #[test]
    // fn check_region_display() {
    //     let mut region = Region::new(20, 10, vec![]);
    //
    //     // check origin
    //     region.map[0][0] = true;
    //
    //     // check axes (should be up 5 over 1)
    //     region.map[5][1] = true;
    //
    //     println!("{}", region);
    //     panic!("make test fail to see output");
    // }
}
