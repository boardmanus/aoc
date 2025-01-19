use std::collections::{HashMap, HashSet};

type Wire = str;
type Wires<'a> = HashMap<&'a str, usize>;
type OpType = fn(usize, usize) -> usize;

fn and_op(a: usize, b: usize) -> usize {
    a & b
}
fn or_op(a: usize, b: usize) -> usize {
    a | b
}
fn xor_op(a: usize, b: usize) -> usize {
    a ^ b
}

#[derive(Debug, Copy, Clone)]
struct Gate<'a> {
    in0: &'a Wire,
    in1: &'a Wire,
    out: &'a Wire,
    op: fn(usize, usize) -> usize,
}

impl<'a> Gate<'a> {
    fn apply(&self, wires: &Wires) -> Option<(&str, usize)> {
        let &a = wires.get(self.in0)?;
        let &b = wires.get(self.in1)?;
        Some((self.out, (self.op)(a, b)))
    }

    fn has_input(&self, input: &str) -> bool {
        self.in0 == input || self.in1 == input
    }

    fn has_output(&self, output: &str) -> bool {
        self.out == output
    }
}

#[derive(Debug, Clone)]
struct Device<'a> {
    wires: HashMap<&'a str, usize>,
    gates: Vec<Gate<'a>>,
}

fn filter_wires<'a>(wire_name: &str, outputs: &HashMap<&'a str, usize>) -> Vec<(&'a str, usize)> {
    let mut zs = outputs
        .iter()
        .filter(|(w, _)| w.starts_with(wire_name))
        .map(|(&w, &o)| (w, o))
        .collect::<Vec<_>>();
    zs.sort_by(|a, b| b.0.cmp(a.0));
    zs
}

fn val_wires(wires: &Vec<(&str, usize)>) -> usize {
    wires.iter().fold(0, |val, (_, bit)| (val << 1) | bit)
}

fn bit_from_name(wire: &str) -> (&str, usize) {
    let split = wire.split_at(1);
    (split.0, split.1.parse().unwrap())
}

impl<'a> Device<'a> {
    fn parse(input: &str) -> Device {
        let mut it = input.split("\n\n");
        let wires = it
            .next()
            .unwrap()
            .lines()
            .fold(HashMap::new(), |mut acc, line| {
                let mut it = line.split(": ");
                let wire = it.next().unwrap();
                let val = it.next().unwrap().parse::<usize>().unwrap();
                acc.insert(wire, val);
                acc
            });
        let gates = it
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let mut it = line.split_whitespace();
                let a = it.next().unwrap();
                let op = match it.next().unwrap() {
                    "AND" => and_op,
                    "OR" => or_op,
                    "XOR" => xor_op,
                    _ => panic!(),
                };
                let b = it.next().unwrap();
                let _ = it.next();
                let x = it.next().unwrap();
                Gate {
                    in0: a,
                    in1: b,
                    out: x,
                    op,
                }
            })
            .collect();
        Device { wires, gates }
    }

    fn new(wires: HashMap<&'a str, usize>, gates: Vec<Gate<'a>>) -> Device<'a> {
        Device { wires, gates }
    }

    fn new_inputs(&self, xs: usize, ys: usize) -> Device<'a> {
        let wires = self.wires.keys().fold(HashMap::new(), |mut wires, &wire| {
            let (prefix, bit) = bit_from_name(wire);
            let input = match prefix {
                "x" => xs,
                "y" => ys,
                _ => panic!(),
            };
            wires.insert(wire, (input >> bit) & 1);
            wires
        });
        let gates = self.gates.clone();
        Device::new(wires, gates)
    }

    fn gate_with_inputs(&self, op: OpType, i0: &str, i1: &str) -> Option<&Gate<'a>> {
        self.gates
            .iter()
            .find(|g| g.op == op && g.has_input(i0) && g.has_input(i1))
    }

    fn gate_with_any_inputs(&self, op: OpType, i0: &str, i1: &str) -> Option<&Gate<'a>> {
        assert_eq!(
            self.gates
                .iter()
                .filter(|g| g.op == op && (g.has_input(i0) || g.has_input(i1)))
                .count(),
            1
        );
        self.gates
            .iter()
            .find(|g| g.op == op && (g.has_input(i0) || g.has_input(i1)))
    }

    fn outputs(&self) -> HashMap<&str, usize> {
        let mut gates = self.gates.iter().map(|g| g).collect::<Vec<_>>();
        let mut wires = self.wires.clone();
        loop {
            gates = gates.iter().fold(vec![], |mut acc, gate| {
                if let Some(res) = gate.apply(&wires) {
                    wires.entry(res.0).or_insert(res.1);
                } else {
                    acc.push(gate);
                }
                acc
            });
            if gates.is_empty() {
                break;
            }
        }
        wires.iter().map(|(&o, &v)| (o, v)).collect()
    }

    fn num_bits(&self, prefix_name: &str) -> usize {
        self.gates
            .iter()
            .filter(|gate| gate.out.starts_with(prefix_name))
            .count()
    }
    fn zs(&self) -> Vec<(&str, usize)> {
        filter_wires("z", &self.outputs())
    }

    fn z(&self) -> usize {
        val_wires(&self.zs())
    }

    fn sum(&self) -> usize {
        let num_zs = self.num_bits("z");
        let xs = filter_wires("x", &self.wires);
        let x = xs.iter().fold(0, |val, (_, bit)| (val << 1) | bit);
        let ys = filter_wires("y", &self.wires);
        let y = ys.iter().fold(0, |val, (_, bit)| (val << 1) | bit);
        (x + y) & ((1 << num_zs) - 1)
    }
}

pub fn part1(input: &str) -> usize {
    let device = Device::parse(input);
    let z = device.z();
    println!("{:?}", z);
    z
}

fn print_info<'a>(device: &Device<'a>) {
    let sum = device.sum();
    let outputs = device.outputs();
    let zs = filter_wires("z", &outputs);
    let z = val_wires(&zs);
    let num_zs = zs.len();
    let xor = sum ^ z;
    println!("{sum:0num_zs$b} ^ {z:0num_zs$b} = {:0num_zs$b}", xor);
    println!("sum: {sum:0num_zs$b}");
    println!("zs : {z:0num_zs$b}");
    println!("xor: {:0num_zs$b}", xor);

    let diffs: (Vec<_>, Vec<_>) = zs
        .into_iter()
        .filter(|x| {
            let bit = bit_from_name(x.0);
            xor & (1 << bit.1) != 0
        })
        .partition(|x| x.1 == 0);

    println!("{:?}", diffs);
}

pub fn part2(input: &str) -> String {
    let device = Device::parse(input);
    print_info(&device);

    // The circuit results in the addition of x + y.
    // Addition can be implemented in terms of xor, and, or.
    // When applying by bit, the carry must be handled:
    // zi = (xi ^ yi) ^ ci
    // 1.) z'i = xi ^ yi
    // 2.) zi = z'i ^ ci
    // co = xi & yi | ci & (xi ^ yi)
    // => co = xi & yi | ci & z'i
    // 3.) c'o = xi & yi
    // 4.) c'i = ci & z'i
    // 5.) co = c'o | c'i
    // In order to implement an adder, all five gate operations must be present with
    // the correct inputs. Starting with bit0, find each gate, and if the outputs is
    // present, add the output to be swapped.
    //
    // There is no carry input for bit0, so handle it separately.
    let mut swappies = HashSet::<String>::new();
    let gate_z0 = device.gate_with_inputs(xor_op, "x00", "y00").unwrap();
    if !gate_z0.has_output("z00") {
        swappies.insert("z00".to_string());
    }

    // Determine the initial carry after bit0
    let gate_c1_ = device.gate_with_inputs(and_op, "x00", "y00").unwrap();
    // gate_c0_ not defined for z0
    let mut gate_ci = gate_c1_;

    for bit in 1..filter_wires("x", &device.wires).len() {
        let xi = format!("x{bit:02}");
        let yi = format!("y{bit:02}");
        let zi = format!("z{bit:02}");

        // 1.) z'i = xi ^ yi
        let gate_zi_ = device.gate_with_inputs(xor_op, &xi, &yi).unwrap();

        // 2.) zi = z'i ^ ci
        let gate_zi = device
            .gate_with_any_inputs(xor_op, gate_zi_.out, gate_ci.out)
            .unwrap();
        if !gate_zi.has_output(&zi) {
            swappies.insert(zi.to_string());
            swappies.insert(gate_zi.out.to_string());
        }
        if !gate_zi.has_input(gate_zi_.out) {
            swappies.insert(gate_zi_.out.to_string());
        }
        if !gate_zi.has_input(gate_ci.out) {
            swappies.insert(gate_ci.out.to_string());
        }

        // 3.) c'o = xi & yi
        let gate_co_ = device.gate_with_inputs(and_op, &xi, &yi).unwrap();

        // 4.) c'i = ci & z'i
        let gate_ci_ = device
            .gate_with_any_inputs(and_op, gate_ci.out, gate_zi_.out)
            .unwrap();
        if !gate_ci_.has_input(gate_ci.out) {
            swappies.insert(gate_ci.out.to_string());
        }
        if !gate_ci_.has_input(gate_zi_.out) {
            swappies.insert(gate_zi_.out.to_string());
        }

        // 5.) co = c'o | c'i
        let gate_co = device
            .gate_with_any_inputs(or_op, gate_co_.out, gate_ci_.out)
            .unwrap();
        if !gate_co.has_input(gate_co_.out) {
            swappies.insert(gate_co_.out.to_string());
        }
        if !gate_co.has_input(gate_ci_.out) {
            swappies.insert(gate_ci_.out.to_string());
        }

        // Update to the carry for the next bit
        gate_ci = gate_co;
    }

    let mut swappies: Vec<String> = swappies.into_iter().collect();
    swappies.sort();
    println!("swappies = {:?}", swappies);
    assert_eq!(swappies.len(), 2 * 4);

    swappies.join(",")
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 4;
    pub const TEST_INPUT_2: &str = include_str!("data/input_example_2");
    pub const TEST_ANSWER_2: usize = 2024;
    pub const TEST_INPUT_3: &str = include_str!("data/input_example_3");
    pub const TEST_ANSWER_3: &str = "z00,z01,z02,z05";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
        assert_eq!(part1(TEST_INPUT_2), TEST_ANSWER_2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_3), TEST_ANSWER_3);
    }
}
