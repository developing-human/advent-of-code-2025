/// Counts paths through a graph by recurisvely searching the graph, and tallying up how many times
/// specific nodes are passed through.
use std::{cell::OnceCell, collections::HashMap, ops::AddAssign, rc::Rc};

use crate::shared::Answer;

#[derive(Debug)]
struct Device {
    label: String,

    // reference counting makes the references to the other devices play nice with the compiler
    outputs: OnceCell<Vec<Rc<Device>>>,
}

impl Device {
    fn new(label: String) -> Self {
        Self {
            label,
            outputs: OnceCell::new(),
        }
    }

    // more ergonomic way to get the outputs
    fn outputs(&self) -> &Vec<Rc<Device>> {
        self.outputs.get_or_init(Vec::new)
    }

    fn set_outputs(&self, outputs: Vec<Rc<Device>>) {
        self.outputs
            .set(outputs)
            .expect("set_outputs should only be called once");
    }
}

#[derive(Debug, Clone, Default)]
struct PathTally {
    // how many paths go through this node?
    out: usize,

    // how many paths have gone through dac to reach this node?
    dac: usize,

    // how many paths have gone through fft to reach this node?
    fft: usize,

    // how many paths have gone through dac AND fft to reach this node?
    dac_and_fft: usize,
}

impl PathTally {
    fn update_for_specific_devices(&mut self, device_label: &str) {
        match device_label {
            // start counting 'out'. it was accumulate as the stack unwinds.
            "out" => self.out = 1,

            // every path from here to out has gone through dac, so dac = out.
            // if fft is already set, we know the value for dac_and_fft
            "dac" => {
                self.dac = self.out;
                if self.fft > 0 {
                    self.dac_and_fft = self.fft
                }
            }
            // every path from here to out has gone through fft, so fft = out.
            // if dac is already set, we know the value for dac_and_fft
            "fft" => {
                self.fft = self.out;
                if self.dac > 0 {
                    self.dac_and_fft = self.dac
                }
            }
            _ => (),
        }
    }
}

impl AddAssign<PathTally> for PathTally {
    fn add_assign(&mut self, rhs: Self) {
        self.out += rhs.out;
        self.dac += rhs.dac;
        self.fft += rhs.fft;
        self.dac_and_fft += rhs.dac_and_fft;
    }
}

fn find_paths_to_out<'d>(
    device: &'d Device,
    all_tallies: &mut HashMap<&'d str, PathTally>,
) -> PathTally {
    // is there already an answer for this device? use it rather than exploring it again.
    if all_tallies.contains_key(device.label.as_str()) {
        return all_tallies.get(&device.label.as_str()).unwrap().clone();
    }

    // tally up the results from this node + its children
    let mut tallies = PathTally::default();
    for output in device.outputs().iter() {
        tallies += find_paths_to_out(output, all_tallies)
    }

    tallies.update_for_specific_devices(&device.label);

    // remember the answer in case we find ourselves here again
    all_tallies.insert(&device.label, tallies.clone());

    tallies
}

pub fn solve(input: &str) -> Answer {
    let (you, svr) = parse(input);

    let part1 = you.map(|you| {
        // how many paths exist from you to out?
        let tallies = find_paths_to_out(&you, &mut HashMap::new());

        tallies.out
    });

    let part2 = svr.map(|svr| {
        // how many paths exist from svr, through dac/fft, to out?
        let tallies = find_paths_to_out(&svr, &mut HashMap::new());

        tallies.dac_and_fft
    });

    Answer {
        part1: part1.unwrap_or_default(),
        part2: part2.unwrap_or_default(),
    }
}

/// Loads all devices, then returns references to the 'you' and 'svr' devices.
fn parse(input: &str) -> (Option<Rc<Device>>, Option<Rc<Device>>) {
    let mut devices: Vec<Rc<Device>> = Vec::new();
    let mut connections: Vec<Vec<&str>> = Vec::new();
    let mut label_to_device: HashMap<&str, Rc<Device>> = HashMap::new();

    // In the first pass create all devices, parse their connections, and map labels to devices
    for line in input.lines() {
        let (label, connections_str) = line.split_once(": ").unwrap();
        let device = Rc::new(Device::new(label.to_string()));

        label_to_device.insert(label, Rc::clone(&device));
        devices.push(device);
        connections.push(connections_str.split(" ").collect());
    }

    // The implicit "out" device doesn't exist in the input explicitly, so add it.
    let out_device = Rc::new(Device::new("out".to_string()));
    label_to_device.insert("out", Rc::clone(&out_device));
    devices.push(out_device);

    // In the second pass, add the outputs to each device
    for (device, connections) in devices.iter().zip(connections) {
        let outputs = connections
            .iter()
            .map(|&c| Rc::clone(&label_to_device[c]))
            .collect();

        device.set_outputs(outputs);
    }

    (
        label_to_device.get("you").map(Rc::clone),
        label_to_device.get("svr").map(Rc::clone),
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn solve_basic_input() {
        let input = r#"
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;
        let result = solve(input.trim());
        assert_eq!(result.part1, 5);

        let input = r#"
you: aaa
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;

        let result = solve(input.trim());
        assert_eq!(result.part2, 2);
    }

    #[test]
    fn can_parse_input() {
        let input = r#"
aaa: you hhh
you: bbb ccc
svr: hhh aaa
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;

        let (you, svr) = parse(input.trim());

        let you = you.unwrap();
        assert_eq!(you.label, "you");
        assert_eq!(you.outputs()[0].label, "bbb");
        assert_eq!(you.outputs()[1].label, "ccc");

        let svr = svr.unwrap();
        assert_eq!(svr.label, "svr");
        assert_eq!(svr.outputs()[0].label, "hhh");
        assert_eq!(svr.outputs()[1].label, "aaa");
    }
}
