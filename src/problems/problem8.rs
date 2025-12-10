use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::shared::Answer;

#[derive(Eq, PartialEq, Debug, Hash)]
struct Junction {
    location: (usize, usize, usize),
}

impl Junction {
    fn new(location: (usize, usize, usize)) -> Self {
        Self { location }
    }
}

#[derive(Debug, Clone, Copy)]
struct Length(f64);

impl Ord for Length {
    fn cmp(&self, other: &Self) -> Ordering {
        // f64's bit pattern gives a total ordering that matches the heap semantics.
        self.0.to_bits().cmp(&other.0.to_bits())
    }
}

impl PartialOrd for Length {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Length {}
impl PartialEq for Length {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

#[derive(Eq, Debug)]
struct StringOfLights<'a> {
    start: &'a Junction,
    end: &'a Junction,

    length: Length,
}

impl<'a> StringOfLights<'a> {
    fn new(start: &'a Junction, end: &'a Junction) -> Self {
        Self {
            start,
            end,
            length: Length(StringOfLights::calculate_length(
                start.location,
                end.location,
            )),
        }
    }

    fn calculate_length(start: (usize, usize, usize), end: (usize, usize, usize)) -> f64 {
        let diff_0 = start.0 as i64 - end.0 as i64;
        let diff_1 = start.1 as i64 - end.1 as i64;
        let diff_2 = start.2 as i64 - end.2 as i64;

        f64::sqrt(((diff_0).pow(2) + (diff_1).pow(2) + (diff_2).pow(2)) as f64)
    }
}

impl<'a> Ord for StringOfLights<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        // reversed, so heap will be a min heap.
        other.length.cmp(&self.length)
    }
}

impl<'a> PartialOrd for StringOfLights<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// impl<'a> Eq for StringOfLights<'a> {}
impl<'a> PartialEq for StringOfLights<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length
    }
}

pub fn solve(input: &str) -> Answer {
    let junctions: Vec<Junction> = input
        .lines()
        .map(|line| line.splitn(3, ',').collect::<Vec<&str>>())
        .map(|strs| {
            (
                strs[0].parse::<usize>().unwrap(),
                strs[1].parse::<usize>().unwrap(),
                strs[2].parse::<usize>().unwrap(),
            )
        })
        .map(Junction::new)
        .collect();

    let mut heap: BinaryHeap<StringOfLights> = BinaryHeap::new();
    for (idx_a, junction_a) in junctions.iter().enumerate() {
        for (idx_b, junction_b) in junctions.iter().enumerate() {
            if idx_b > idx_a && junction_a != junction_b {
                heap.push(StringOfLights::new(junction_a, junction_b));
            }
        }
    }

    // Maps every junction to a hash set of connected junctions
    let mut connected_junctions: HashMap<&Junction, HashSet<&Junction>> = HashMap::new();
    let mut checked_junctions: HashSet<&Junction> = HashSet::new();

    for _ in 0..1000 {
        let lights = heap.pop();
        if lights.is_none() {
            break;
        }

        let lights = lights.unwrap();

        connected_junctions
            .entry(lights.start)
            .or_default()
            .insert(lights.end);

        connected_junctions
            .entry(lights.end)
            .or_default()
            .insert(lights.start);
    }

    let mut circuits: Vec<HashSet<&&Junction>> = vec![];
    for (junction, connections) in connected_junctions.iter() {
        // while connected_junctions.len() > checked_junctions.len() {
        // for (junction, connections) in connected_junctions {
        // let (junction, connections) = connected_junctions.iter().next().unwrap();
        if checked_junctions.contains(junction) {
            continue;
        }

        println!("outer");

        let mut current_circuit = HashSet::new();
        let mut junctions_to_explore = HashSet::new();

        junctions_to_explore.extend(connections);
        current_circuit.insert(junction);
        checked_junctions.insert(junction);
        // connected_junctions.remove(junction);

        while !junctions_to_explore.is_empty() {
            println!("inner");
            let exploring = *junctions_to_explore.iter().next().unwrap();
            current_circuit.insert(exploring);
            junctions_to_explore.remove(exploring);

            dbg!(exploring);
            if checked_junctions.contains(exploring) {
                continue;
            }

            checked_junctions.insert(exploring);

            let extend_with = connected_junctions.get(exploring).unwrap();
            junctions_to_explore.extend(extend_with);
        }

        circuits.push(current_circuit);
    }

    let mut sizes = circuits.iter().map(|c| c.len()).collect::<Vec<usize>>();
    sizes.sort();

    let product = sizes.iter().rev().take(3).product();
    Answer {
        part1: product,
        part2: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_basic_input() {
        let input = r#"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

        let result = solve(input.trim());
        assert_eq!(result.part1, 40);
        assert_eq!(result.part2, 0);
    }
}
