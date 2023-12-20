use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Display, Formatter},
    rc::Rc,
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
enum ParseError {
    Module,
    Node,
    Input,
    Output,
}

#[derive(Debug, Clone, Copy)]
enum Module {
    Broadcaster,
    FlipFlop(bool),
    Conjunction,
}

impl Module {
    fn input(self, signal: bool, inputs: &HashMap<&str, bool>) -> (Self, Option<bool>) {
        match self {
            Module::Broadcaster => (self, Some(signal)),
            Module::FlipFlop(state) => {
                if signal {
                    (self, None)
                } else {
                    (Module::FlipFlop(!state), Some(!state))
                }
            }
            Module::Conjunction => {
                if inputs.iter().all(|input| *input.1) {
                    (self, Some(false))
                } else {
                    (self, Some(true))
                }
            }
        }
    }
}

impl FromStr for Module {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "b" => Ok(Module::Broadcaster),
            "%" => Ok(Module::FlipFlop(false)),
            "&" => Ok(Module::Conjunction),
            _ => Err(ParseError::Module),
        }
    }
}

#[derive(Debug, Clone)]
struct Node<'a> {
    name: &'a str,
    module: Module,
    outputs: Vec<&'a str>,                   //&'a str>,
    inputs: RefCell<HashMap<&'a str, bool>>, //&'a str, bool)>,
}

impl<'a> Node<'a> {
    fn new(name: &'a str, module: Module, outputs: Vec<&'a str>) -> Self {
        Node {
            name,
            module,
            outputs,
            inputs: RefCell::new(HashMap::new()),
        }
    }

    fn from_str(s: &'a str) -> Self {
        let mut parts = s.split(" -> ");
        let ntype = parts.next().unwrap();
        let module = Module::from_str(&ntype[0..1]).unwrap();
        let name = &ntype[1..];
        let output_strs = parts.next().unwrap().split(", ");
        Node::new(name, module, output_strs.collect())
    }

    fn input(&mut self, from: &'a str, signal: bool) -> Option<bool> {
        self.inputs
            .borrow_mut()
            .entry(from)
            .and_modify(|input| *input = signal);

        let (module, out_signal) = self.module.input(signal, &self.inputs.borrow());
        self.module = module;
        out_signal
    }
}

impl<'a> Display for Node<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: in[", self.name)?;
        self.inputs.borrow().iter().for_each(|input| {
            write!(f, "({}, {}),", input.0, input.1).unwrap();
        });
        Ok(())
    }
}

fn parse(input: &str) -> HashMap<&str, Node> {
    let mut nodes = Vec::new();
    for line in input.lines() {
        let node = Node::from_str(line);
        nodes.push(node);
    }

    update_inputs(nodes)
}

fn update_inputs(nodes: Vec<Node>) -> HashMap<&str, Node> {
    let node_map: HashMap<&str, Node> =
        nodes.iter().map(|node| (node.name, node.clone())).collect();

    for node in nodes {
        for output in node.outputs.iter() {
            if let Some(output_node) = node_map.get(output) {
                output_node.inputs.borrow_mut().insert(node.name, false);
            }
        }
    }
    node_map
}

fn push_button(push: usize, nodes: &mut HashMap<&str, Node>) -> (usize, usize) {
    let mut lo_pulses = 0;
    let mut hi_pulses = 0;
    let mut queue = vec![(false, "button", "roadcaster")];
    while let Some(pulse) = queue.pop() {
        let (signal, from, to) = pulse;
        match signal {
            true => hi_pulses += 1,
            false => lo_pulses += 1,
        }

        if to == "rx" {
            if nodes[from].inputs.borrow().iter().all(|input| *input.1) {
                println!(
                    "{} {signal} --> pulses=({}, {}), push={push}",
                    nodes[from], lo_pulses, hi_pulses
                );
            }
        }

        if let Some(rcvr) = nodes.get_mut(to) {
            let out_signal = rcvr.input(from, signal);
            if let Some(signal) = out_signal {
                rcvr.outputs.iter().for_each(|output| {
                    queue.push((signal, to, output));
                });
            }
        }
    }
    (lo_pulses, hi_pulses)
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    last_press: usize,
    first_match_press: Option<usize>,
    repeat: Option<usize>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Key<'a> {
    name: &'a str,
    lo: usize,
    hi: usize,
    d_push: usize,
}

fn keep_pushing<'a>(check_name: &str, nodes: &mut HashMap<&'a str, Node<'a>>) -> usize {
    let mut num_pushes = 0;
    let mut check_state = nodes[check_name]
        .inputs
        .borrow()
        .iter()
        .map(|(name, _)| {
            (
                *name,
                State {
                    last_press: 0,
                    first_match_press: None,
                    repeat: None,
                },
            )
        })
        .collect::<HashMap<&'a str, State>>();
    let mut memory = HashMap::<Key, usize>::new();

    while !check_state.iter().all(|s| s.1.repeat.is_some()) {
        num_pushes += 1;

        let mut lo_pulses = 0;
        let mut hi_pulses = 0;

        let mut queue = vec![(false, "button", "roadcaster")];
        while let Some(pulse) = queue.pop() {
            let (signal, from, to) = pulse;
            match signal {
                true => hi_pulses += 1,
                false => lo_pulses += 1,
            }

            if to == "rx" {
                nodes[check_name].inputs.borrow().iter().filter(|input| *input.1).for_each(|input| {
                    let state = check_state.get_mut(*input.0).unwrap();
                    if state.first_match_press.is_none() {
                        state.first_match_press = Some(num_pushes);
                    }
                    let last_press = state.last_press;
                    let d_push = num_pushes - last_press;
                    state.last_press = num_pushes;
                    println!(
                        "{} {signal} --> pulses=({lo_pulses}, {hi_pulses}), push={num_pushes}, dPush={d_push}",
                        *input.0);
                    let key = Key::<'a> { name: *input.0, lo: lo_pulses, hi: hi_pulses, d_push};
                    if let Some(last_push) = memory.get(&key) {
                        if state.repeat.is_none() {
                            println!("Found repeat for key: {:?}, lastPush={last_push}, Push={num_pushes}", key);
                            state.repeat = Some(num_pushes - *last_push);
                        }
                    } else {
                        memory.insert(key, num_pushes);
                    }
                });
            }

            if let Some(rcvr) = nodes.get_mut(to) {
                let out_signal = rcvr.input(from, signal);
                if let Some(signal) = out_signal {
                    rcvr.outputs.iter().for_each(|output| {
                        queue.push((signal, to, output));
                    });
                }
            }
        }
    }

    let prod: usize = check_state
        .iter()
        .map(|s| {
            println!("{}: {:?}", s.0, s.1);
            s.1.repeat.unwrap()
        })
        .product();

    let lcm = check_state.iter().fold(1, |lcm, x| {
        let new_lcm = num_integer::lcm(lcm, x.1.repeat.unwrap());
        lcm.max(new_lcm)
    });
    println!("state={:?}", check_state);

    println!("prod={prod}, lcm={lcm}");

    let offset: usize = check_state
        .iter()
        .map(|s| s.1.first_match_press.unwrap())
        .sum();

    println!("Offset={offset}");
    lcm - offset
}

fn solve_part1(input: &str) -> usize {
    let mut nodes = parse(input);
    let (lo, hi) = (0..1000).fold((0, 0), |sum, push| {
        let (lo, hi) = push_button(push, &mut nodes);
        (sum.0 + lo, sum.1 + hi)
    });
    lo * hi
}

fn solve_part2(input: &str) -> usize {
    let mut nodes = parse(input);

    keep_pushing("lx", &mut nodes)
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //let part1 = solve_part1(INPUT);
    //println!("Part1: {part1}");
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
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 467835);
    }

    #[test]
    fn test_keep_pushing() {
        let mut nodes = parse(TEST_INPUT_3);
        keep_pushing("con", &mut nodes);
    }

    #[test]
    fn test_push_button() {
        let mut nodes = parse(TEST_INPUT_3);
        (0..100).for_each(|i| {
            push_button(i, &mut nodes);
        });
    }

    #[test]
    fn test_parse() {
        let nodes = parse(TEST_INPUT);
        assert_eq!(nodes.len(), 5);
        assert_eq!(nodes["roadcaster"].outputs, vec!["a", "b", "c"]);
        assert_eq!(nodes["a"].outputs, vec!["b"]);
        //assert_eq!(nodes["b"].inputs, vec![]);
        assert_eq!(nodes["b"].outputs, vec!["c"]);
        //assert_eq!(nodes["c"].inputs, vec![]);
        assert_eq!(nodes["c"].outputs, vec!["inv"]);
        //assert_eq!(nodes["inv"].inputs, vec![]);
        assert_eq!(nodes["inv"].outputs, vec!["a"]);
    }
}
