use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use crate::shared::Answer;

#[derive(Clone)]
struct Circuit {
    id: CircuitId,
    junctions: Vec<JunctionId>,
}

impl Circuit {
    fn new(first_junction_id: JunctionId) -> Self {
        Self {
            id: CircuitId(first_junction_id.0),
            junctions: vec![first_junction_id],
        }
    }

    fn merge(&mut self, other: Circuit) {
        self.junctions.extend(other.junctions);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct CircuitId(usize);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct JunctionId(usize);

#[derive(Eq, PartialEq, Clone)]
struct Junction {
    id: JunctionId,
    circuit_id: CircuitId,
    location: (usize, usize, usize),
}

impl Junction {
    fn new(location: (usize, usize, usize), id: JunctionId, circuit_id: CircuitId) -> Self {
        Self {
            location,
            id,
            circuit_id,
        }
    }
}

#[derive(Eq, PartialEq)]
struct StringOfLights {
    start: JunctionId,
    end: JunctionId,

    length: Length,
}

impl StringOfLights {
    fn new(start: &Junction, end: &Junction) -> Self {
        Self {
            start: start.id,
            end: end.id,
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

impl Ord for StringOfLights {
    fn cmp(&self, other: &Self) -> Ordering {
        // reversed, so heap will be a min heap.
        other.length.cmp(&self.length)
    }
}

impl PartialOrd for StringOfLights {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Length(f64);

impl Ord for Length {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.to_bits().cmp(&other.0.to_bits())
    }
}

impl PartialOrd for Length {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Length {}

pub fn solve(input: &str, connections_to_make: usize) -> Answer {
    let mut junctions: Vec<Junction> = input
        .lines()
        .map(|line| line.splitn(3, ',').collect::<Vec<&str>>())
        .map(|strs| {
            (
                strs[0].parse::<usize>().unwrap(),
                strs[1].parse::<usize>().unwrap(),
                strs[2].parse::<usize>().unwrap(),
            )
        })
        .enumerate()
        .map(|(id, location)| Junction::new(location, JunctionId(id), CircuitId(id)))
        .collect();

    let circuits: Vec<Circuit> = junctions.iter().map(|j| Circuit::new(j.id)).collect();

    let mut id_to_circuit: HashMap<CircuitId, Circuit> =
        circuits.iter().map(|c| (c.id, c.clone())).collect();

    let mut heap: BinaryHeap<StringOfLights> = BinaryHeap::new();
    for (idx_a, circuit_a) in circuits.iter().enumerate() {
        let junction_a_id = &circuit_a.junctions[0];
        for (idx_b, circuit_b) in circuits.iter().enumerate() {
            let junction_b_id = &circuit_b.junctions[0];
            if idx_b > idx_a && junction_a_id != junction_b_id {
                let junction_a = junctions[junction_a_id.0].clone();
                let junction_b = junctions[junction_b_id.0].clone();
                heap.push(StringOfLights::new(&junction_a, &junction_b));
            }
        }
    }

    let mut circuits_remaining = junctions.len();
    let mut connections_made = 0;
    let mut part1_answer = 0;
    let mut part2_answer = 0;

    // connect junctions until both anwers are calculated
    while let Some(lights) = heap.pop() {
        // when enough connections are made, calculate the answer to part1 (but keep going)
        if connections_made == connections_to_make {
            let mut sizes = id_to_circuit
                .values()
                .map(|c| c.junctions.len())
                .collect::<Vec<_>>();

            sizes.sort();

            part1_answer = sizes.iter().rev().take(3).product();
        }

        connections_made += 1;

        let new_circuit_id;
        let junctions_to_update;

        // merges two circuits together, if they need to be merged
        {
            let junction_start = &junctions[lights.start.0];
            let junction_end = &junctions[lights.end.0];

            if junction_start.circuit_id == junction_end.circuit_id {
                continue;
            }
            // remove the "other" circuit
            let other_circuit = id_to_circuit
                .remove(&junction_end.circuit_id)
                .expect("junction_end.circuit_id should exist")
                .clone();

            new_circuit_id = junction_start.circuit_id;
            junctions_to_update = other_circuit.junctions.clone();

            id_to_circuit
                .entry(junction_start.circuit_id)
                .and_modify(|c| c.merge(other_circuit));
        }

        // assign everything in the other circuit to "this" circuit
        for junction_id in junctions_to_update {
            junctions[junction_id.0].circuit_id = new_circuit_id;
        }

        // when only one circuit remains, calculate the answer to part 2
        circuits_remaining -= 1;
        if circuits_remaining == 1 {
            let junction_start = &junctions[lights.start.0];
            let junction_end = &junctions[lights.end.0];
            part2_answer = junction_start.location.0 * junction_end.location.0;
            break;
        }
    }

    Answer {
        part1: part1_answer,
        part2: part2_answer,
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

        let result = solve(input.trim(), 10);
        assert_eq!(result.part1, 40);
        assert_eq!(result.part2, 25272);
    }
}
