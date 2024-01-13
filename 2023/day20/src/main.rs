use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fmt::{Display, Formatter},
    ops::{Add, AddAssign},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
enum ParseError {
    Module,
}

#[derive(Debug, Clone)]
enum Module<'a> {
    Button,
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>),
    Terminator,
}

impl<'a> Module<'a> {
    fn signal(&self, from: &'a str, pulse: bool) -> Option<bool> {
        match self {
            Module::Button => Some(false),
            Module::Broadcaster => Some(pulse),
            Module::FlipFlop(state) => match pulse {
                false => Some(!*state),
                true => None,
            },
            Module::Conjunction(inputs) => {
                Some(
                    !inputs
                        .iter()
                        .all(|input| if *input.0 == from { pulse } else { *input.1 }),
                )
            }
            _ => None,
        }
    }
    fn input(&mut self, from: &str, signal: bool) -> Option<bool> {
        let output = self.signal(from, signal);
        match self {
            Module::FlipFlop(state) => *state = output.unwrap_or(*state),
            Module::Conjunction(inputs) => *inputs.get_mut(from).unwrap() = signal,
            _ => (),
        }
        output
    }

    fn state(&self) -> usize {
        match self {
            Module::FlipFlop(state) => *state as usize,
            Module::Conjunction(ref inputs) => inputs
                .iter()
                .enumerate()
                .fold(0, |state, (i, input)| state | (*input.1 as usize) << i),
            _ => 0,
        }
    }

    fn update_inputs(&mut self, name: &'a str, signal: bool) {
        match self {
            Module::Conjunction(ref mut inputs) => {
                inputs.insert(name, signal);
            }
            _ => (),
        }
    }
}

impl<'a> Display for Module<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Module::Button => write!(f, "button"),
            Module::Broadcaster => write!(f, "broadcaster"),
            Module::FlipFlop(state) => write!(f, "flipflop({})", state),
            Module::Conjunction(ref inputs) => {
                write!(f, "conjunction(")?;
                inputs.iter().for_each(|input| {
                    write!(f, "{}, ", input.0).unwrap();
                });
                write!(f, ")")
            }
            Module::Terminator => write!(f, "terminator"),
        }
    }
}

impl<'a> FromStr for Module<'a> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "b" => Ok(Module::Broadcaster),
            "%" => Ok(Module::FlipFlop(false)),
            "&" => Ok(Module::Conjunction(HashMap::new())),
            _ => Err(ParseError::Module),
        }
    }
}

#[derive(Debug, Clone)]
struct Node<'a> {
    name: &'a str,
    module: RefCell<Module<'a>>,
    inputs: Vec<&'a str>,
    outputs: Vec<&'a str>, //&'a str>,
    last_pulse: RefCell<Option<bool>>,
}

impl<'a> Node<'a> {
    fn new(name: &'a str, module: Module<'a>, inputs: Vec<&'a str>, outputs: Vec<&'a str>) -> Self {
        Node {
            name,
            module: RefCell::new(module),
            inputs,
            outputs,
            last_pulse: RefCell::new(None),
        }
    }

    fn input(&self, from: &str, signal: bool) -> Option<bool> {
        self.module.borrow_mut().input(from, signal)
    }

    fn state(&self) -> usize {
        self.module.borrow().state()
    }

    fn propagate(&self, signal: Signal<'a>, queue: &mut VecDeque<Signal<'a>>) -> Option<bool> {
        let maybe_pulse = self.input(signal.from, signal.pulse);
        if let Some(pulse) = maybe_pulse {
            let from: &str = signal.to;
            queue.extend(self.outputs.iter().map(|to| Signal { from, to, pulse }));
        }
        maybe_pulse
    }
}

impl<'a> Display for Node<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.module.borrow())?;
        Ok(())
    }
}

impl<'a> TryFrom<&'a str> for Node<'a> {
    type Error = ParseError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut parts = s.split(" -> ");
        let ntype = parts.next().unwrap();
        let module_type = ntype.chars().next().unwrap();
        let module = Module::from_str(&ntype[0..1]).unwrap();
        let name = match module_type.is_alphabetic() {
            true => ntype,
            false => &ntype[1..],
        };
        let output_strs = parts.next().unwrap().split(", ");
        Ok(Node::new(name, module, vec![], output_strs.collect()))
    }
}

struct Signal<'a> {
    from: &'a str,
    to: &'a str,
    pulse: bool,
}

impl<'a> Signal<'a> {
    fn new(from: &'a str, to: &'a str, pulse: bool) -> Self {
        Signal { from, to, pulse }
    }
}

impl<'a> Display for Signal<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pulse_str = match self.pulse {
            true => "high",
            false => "lo",
        };
        write!(f, "{} -{pulse_str}-> {}", self.from, self.to)
    }
}

struct Circuit<'a> {
    nodes: HashMap<&'a str, Node<'a>>,
}

impl<'a> TryFrom<&'a str> for Circuit<'a> {
    type Error = ParseError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        // Extract nodes from the input string.
        // A button is the first component.
        let mut v_nodes = vec![Node::new(
            Circuit::BUTTON,
            Module::Button,
            vec![],
            vec![Circuit::BROADCASTER],
        )];
        for line in s.lines() {
            let node = Node::try_from(line)?;
            v_nodes.push(node);
        }

        // Copy nodes into a hashmap to allow modification
        let mut nodes: HashMap<&str, Node> = v_nodes
            .iter()
            .map(|node| (node.name, node.clone()))
            .collect();

        // Update the nodes in the hashmap
        // Note: we're iterating over the vector, to allow modification of the hashmap.
        // If we iterate of the hashmap, it can't be modified (because it's borrowed for the
        // iteration).
        for node in v_nodes {
            for output in node.outputs.iter() {
                let output_node = nodes.entry(output).or_insert(Node::new(
                    output,
                    Module::Terminator,
                    vec![],
                    vec![],
                ));
                output_node.inputs.push(node.name);
                output_node
                    .module
                    .borrow_mut()
                    .update_inputs(node.name, false);
            }
        }
        Ok(Circuit { nodes })
    }
}

struct PulseCount {
    lo: usize,
    hi: usize,
}

impl Add for PulseCount {
    type Output = PulseCount;
    fn add(self, rhs: Self) -> Self {
        PulseCount {
            hi: self.hi + rhs.hi,
            lo: self.lo + rhs.lo,
        }
    }
}

impl AddAssign<bool> for PulseCount {
    fn add_assign(&mut self, rhs: bool) {
        match rhs {
            true => self.hi += 1,
            false => self.lo += 1,
        }
    }
}

impl PulseCount {
    fn new() -> Self {
        PulseCount { lo: 0, hi: 0 }
    }
}

impl<'a> Circuit<'a> {
    const BUTTON: &'static str = "button";
    const BROADCASTER: &'static str = "broadcaster";

    fn push_button(&self) -> PulseCount {
        let mut pulse_count = PulseCount::new();
        let mut queue: VecDeque<Signal<'a>> =
            VecDeque::from([Signal::new(Circuit::BUTTON, Circuit::BROADCASTER, false)]);
        while let Some(signal) = queue.pop_front() {
            pulse_count += signal.pulse;
            if let Some(node) = self.nodes.get(signal.to) {
                *node.last_pulse.borrow_mut() = node.propagate(signal, &mut queue);
            }
        }
        pulse_count
    }

    fn keep_pushing(&mut self, conjunction: &'a str) -> PulseCount {
        let mut pulse_count = PulseCount::new();
        let inputs = self.nodes[conjunction]
            .inputs
            .iter()
            .copied()
            .collect::<Vec<_>>();
        //while inputs.iter().any(|x| !x) {
        for i in 0..1000 {
            pulse_count = self.push_button();
            if inputs
                .iter()
                .all(|&input| *self.nodes[input].last_pulse.borrow() == Some(true))
            {
                println!("{}: {}", i, self.nodes[conjunction]);
            }
        }

        pulse_count
    }
}

fn solve_part1(input: &str) -> usize {
    let mut circuit = Circuit::try_from(input).unwrap();
    let pulse_count = (0..1000).fold(PulseCount::new(), |sum, _| sum + circuit.push_button());
    pulse_count.lo * pulse_count.hi
}

fn solve_part2(input: &str) -> usize {
    let mut _circuit = Circuit::try_from(input).unwrap();

    //keep_pushing("lx", &mut nodes)
    0
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let part1 = solve_part1(INPUT);
    println!("Part1: {part1}");
    let part2 = solve_part2(INPUT);
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
    const TEST_INPUT_2: &str = include_str!("test_input2.txt");
    const TEST_INPUT_3: &str = include_str!("test_input3.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 32000000);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(solve_part1(TEST_INPUT_2), 11687500);
    }

    #[test]
    fn test_data2() {
        let mut circuit = Circuit::try_from(TEST_INPUT_2).unwrap();

        for i in 0..10 {
            println!("\n{i}:");
            let pulse_count = circuit.push_button();
        }
    }

    #[test]
    fn test_part2() {
        let mut circuit = Circuit::try_from(TEST_INPUT_2).unwrap();
        let conjunction = circuit.nodes["output"].inputs[0];

        let res = circuit.keep_pushing(conjunction);
        //assert_eq!(res /*solve_part2(TEST_INPUT)*/, 467835);
    }

    #[test]
    fn test_push_button() {
        let mut circuit = Circuit::try_from(TEST_INPUT_2).unwrap();
        (0..100).for_each(|i| {
            circuit.push_button();
        });
    }

    #[test]
    fn test_parse() {
        let mut circuit = Circuit::try_from(TEST_INPUT).unwrap();
        assert_eq!(circuit.nodes.len(), 5);
        assert_eq!(circuit.nodes["roadcaster"].outputs, vec!["a", "b", "c"]);
        assert_eq!(circuit.nodes["a"].outputs, vec!["b"]);
        //assert_eq!(nodes["b"].inputs, vec![]);
        assert_eq!(circuit.nodes["b"].outputs, vec!["c"]);
        //assert_eq!(nodes["c"].inputs, vec![]);
        assert_eq!(circuit.nodes["c"].outputs, vec!["inv"]);
        //assert_eq!(nodes["inv"].inputs, vec![]);
        assert_eq!(circuit.nodes["inv"].outputs, vec!["a"]);
    }
}
