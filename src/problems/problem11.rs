use std::{cell::OnceCell, collections::HashMap, rc::Rc};

use crate::shared::Answer;

#[derive(Debug)]
struct Device {
    label: String,
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
        self.outputs.get().unwrap()
    }

    fn set_outputs(&self, outputs: Vec<Rc<Device>>) {
        self.outputs
            .set(outputs)
            .expect("set_outputs should only be called once");
    }
}

pub fn solve(input: &str) -> Answer {
    let devices = [
        Rc::new(Device::new("a".to_string())),
        Rc::new(Device::new("b".to_string())),
        Rc::new(Device::new("c".to_string())),
    ];

    devices[0].set_outputs(vec![devices[0].clone()]);

    Answer { part1: 0, part2: 0 }
}

fn parse(input: &str) -> Vec<Rc<Device>> {
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
            .map(|&c| Rc::clone(&label_to_device[dbg!(c)]))
            .collect();

        device.set_outputs(outputs);
    }

    devices
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
    }

    #[test]
    fn can_parse_input() {
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

        let devices = parse(input.trim());

        assert_eq!(devices.len(), 11);
        assert_eq!(devices[0].label, "aaa");
        assert_eq!(devices[0].outputs()[0].label, "you");
        assert_eq!(devices[0].outputs()[1].label, "hhh");

        assert_eq!(devices[1].label, "you");
        assert_eq!(devices[1].outputs()[0].label, "bbb");
        assert_eq!(devices[1].outputs()[1].label, "ccc");

        // do the outputs point to the memory location of other nodes in the list?
        assert!(std::ptr::eq(
            devices[0].outputs()[0].as_ref(),
            devices[1].as_ref()
        ));
    }
}
