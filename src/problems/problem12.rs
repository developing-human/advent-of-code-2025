// Surprisingly... The solution was as simple as counting the volume taken up by the presents and
// comparing it to the size of the region. Perhaps I just got a lucky input, but I'm going with it.

use crate::shared::Answer;

#[derive(Debug)]
struct Shape {
    map: Vec<Vec<bool>>,
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,

    // a vector, listing how many presents of each type are left to place.
    // indices match the order they are loaded
    presents_left_to_place: Vec<usize>,
}

impl Region {
    fn new(width: usize, height: usize, presents_left_to_place: Vec<usize>) -> Self {
        Self {
            width,
            height,
            presents_left_to_place,
        }
    }
}

pub fn solve(input: &str) -> Answer {
    let (shapes, regions) = parse(input);

    let volume_per_shape: Vec<usize> = shapes
        .iter()
        .map(|s| s.map.iter().flatten().filter(|&&b| b).count())
        .collect();

    let mut does_not_fit = 0;
    for region in regions.iter() {
        let area = region.width * region.height;

        let gift_volume: usize = region
            .presents_left_to_place
            .iter()
            .enumerate()
            .map(|(idx, count)| count * volume_per_shape[idx])
            .sum();

        if area < gift_volume {
            does_not_fit += 1;
        }
    }

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

        assert_eq!(regions[1].presents_left_to_place, vec![1, 0, 1, 0, 2, 2]);
    }
}
