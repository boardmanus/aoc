use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
enum ParseError {
    Module,
    Input,
    Output,
}

#[derive(Debug, Clone)]
enum Module<'a> {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>),
}

impl<'a> Module<'a> {
    fn input(&mut self, from: &'a str, signal: bool) -> Option<bool> {
        match self {
            Module::Broadcaster => Some(signal),
            Module::FlipFlop(state) => {
                if signal {
                    None
                } else {
                    *state = !*state;
                    Some(*state)
                }
            }
            Module::Conjunction(inputs) => {
                if let Some(input) = inputs.get_mut(from) {
                    *input = signal;
                    if inputs.iter().all(|input| *input.1) {
                        Some(false)
                    } else {
                        Some(true)
                    }
                } else {
                    None
                }
            }
        }
    }
    fn state(&self) -> usize {
        match self {
            Module::Broadcaster => 0,
            Module::FlipFlop(state) => (if *state { 1 } else { 0 }) << 1,
            Module::Conjunction(ref inputs) => {
                inputs.iter().enumerate().fold(0, |state, (i, input)| {
                    state | if *input.1 { 1 << i } else { 0 }
                }) << 2
            }
        }
    }
    fn update_inputs(&mut self, name: &'a str, signal: bool) {
        match self {
            Module::Broadcaster => {}
            Module::FlipFlop(_) => {}
            Module::Conjunction(ref mut inputs) => {
                inputs.insert(name, signal);
            }
        }
    }
}

impl<'a> Display for Module<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Module::Broadcaster => write!(f, "broadcaster"),
            Module::FlipFlop(state) => write!(f, "flipflop({})", state),
            Module::Conjunction(ref inputs) => {
                write!(f, "conjunction(")?;
                inputs.iter().for_each(|input| {
                    write!(f, "{}, ", input.0).unwrap();
                });
                write!(f, ")")
            }
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
    module: Module<'a>,
    outputs: Vec<&'a str>, //&'a str>,
}

impl<'a> Node<'a> {
    fn new(name: &'a str, module: Module<'a>, outputs: Vec<&'a str>) -> Self {
        Node {
            name,
            module,
            outputs,
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
        self.module.input(from, signal)
    }

    fn state(&self) -> usize {
        self.module.state()
    }
}

impl<'a> Display for Node<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.module)?;
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
    let mut node_map: HashMap<&str, Node> =
        nodes.iter().map(|node| (node.name, node.clone())).collect();

    for node in nodes {
        for output in node.outputs.iter() {
            if let Some(output_node) = node_map.get_mut(output) {
                output_node.module.update_inputs(node.name, false);
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    last_press: usize,
    first_match_press: HashSet<usize>,
    repeat: Option<usize>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Key<'a> {
    name: &'a str,
    lo: usize,
    hi: usize,
    d_push: usize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Key2<'a> {
    name: &'a str,
    state: Vec<usize>,
}


fn keep_pushing<'a>(check_name: &'a str, nodes: &mut HashMap<&'a str, Node<'a>>) -> usize {
    let mut num_pushes = 0;
    let mut check_state = match nodes[check_name].module {
        Module::Conjunction(ref inputs) => inputs
            .iter()
            .map(|(name, _)| {
                (
                    *name,
                    State {
                        last_press: 0,
                        first_match_press: HashSet::new(),
                        repeat: None,
                    },
                )
            })
            .collect::<HashMap<&'a str, State>>(),
        _ => panic!("Not a conjunction"),
    };
    println!("check_state={:?}", check_state);

    //let mut memory = HashMap::<Key, usize>::new();
    let mut memory = HashMap::<Key2, usize>::new();

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

            if let Some(rcvr) = nodes.get_mut(to) {
                let out_signal = rcvr.input(from, signal);
                if let Some(signal) = out_signal {
                    rcvr.outputs.iter().for_each(|output| {
                        queue.push((signal, to, output));
                    });
                }
            }

            if to == "rx" {
                match nodes[check_name].module {
                    Module::Conjunction(ref inputs) => {
                        inputs.iter().filter(|input| *input.1).for_each(|input| {
                        let (last_press, d_push) = if let Some(state) = check_state.get_mut(*input.0) {
                            let last_press = state.last_press;
                            let d_push = num_pushes - last_press;
                            state.last_press = num_pushes;
                            (last_press, d_push)
                        } else {
                            panic!("No state for {}", *input.0);
                        };
                        if d_push == 0 {
                            return;
                        }
                        
                        //println!(
                        //"{} {signal} --> pulses=({lo_pulses}, {hi_pulses}), push={num_pushes}, dPush={d_push}",
                        //*input.0);

                        let key = Key2::<'a> {
                            name: *input.0,

                            state: nodes.iter().map(|n| if *n.0 == check_name { 9999999 } else { n.1.state()}).collect(),
                        };

                        if let Some(last_push) = memory.get(&key) {
                            println!("Found key: {:?}, input={}, lastPush={last_push}, Push={num_pushes}", key, *input.0);
                            if let Some(state) = check_state.get_mut(*input.0) {
                                if state.repeat.is_none() {
                                    println!("Found repeat for key: {:?}, lastPush={last_push}, Push={num_pushes}", key);
                                    state.repeat = Some(num_pushes - *last_push - 1);
                                }
                            }
                        } else {
                            //println!("Inserting key: {:?}, Push={num_pushes}", key);
                            memory.insert(key, num_pushes);
                        }

                        if let Some(state) = check_state.get_mut(*input.0) {

                            if state.repeat.is_none() {
                                state.first_match_press.insert(num_pushes);
                            }
                        }
                    })},
                    _ => panic!("Not a conjunction"),
                };
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

    check_state.iter().for_each(|s| {
        println!("Offsets for {}:", s.0);
        s.1.first_match_press
            .iter()
            .for_each(|p| println!("{}: {p}", s.0))
    });
    /*
    let mut index = 0;

    let input_pushes: Vec<(usize, usize)> = check_state
        .iter()
        .enumerate()
        .map(|(i, s)| {
            if *s.0 == "cl" {
                index = i
            }
            (3732, s.1.repeat.unwrap())
        })
        .collect();
    */
    /*
    input_pushes[index].0 += input_pushes[index].1;
    while !input_pushes
        .iter()
        .all(|(push, _)| *push == input_pushes[index].0)
    {
        (0..input_pushes.len()).for_each(|i| {
            let push_check = input_pushes[index].0;
            let (push, repeat) = input_pushes.get_mut(i).unwrap();
            if *push <= push_check {
                *push += *repeat;
            }
        });
        if (input_pushes[index].0 / input_pushes[index].1) % 1000000 == 0 {
            println!("input_pushes={:?}", input_pushes);
        }
    }
    println!("input_pushes={:?}", input_pushes);
    */
    lcm
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
