use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter, Result},
    rc::Rc,
};

use regex::Regex;

#[derive(Debug, Clone)]
struct Node<'a> {
    name: &'a str,
    l: Option<Rc<RefCell<Node<'a>>>>,
    r: Option<Rc<RefCell<Node<'a>>>>,
}

type NodeRef<'a> = Rc<RefCell<Node<'a>>>;

impl<'a> Display for Node<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let lname = if let Some(l) = &self.l {
            l.borrow().name
        } else {
            "None"
        };
        let rname = if let Some(r) = &self.r {
            r.borrow().name
        } else {
            "None"
        };
        write!(f, "Node({} /{lname}/\\{rname}\\)", self.name)
    }
}

impl<'a> Node<'a> {
    fn empty(name: &str) -> Node {
        Node {
            name,
            l: None,
            r: None,
        }
    }
    fn new(name: &'a str, l: Option<NodeRef<'a>>, r: Option<NodeRef<'a>>) -> Node<'a> {
        Node { name, l: l, r: r }
    }

    fn left(&self) -> Option<NodeRef<'a>> {
        self.l.clone()
    }

    fn right(&self) -> Option<NodeRef<'a>> {
        self.r.clone()
    }
}

type NodeMap<'a> = HashMap<&'a str, Rc<RefCell<Node<'a>>>>;

// Parse AAA = (BBB, CCC)
fn parse_nodes(input: &str) -> NodeMap {
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    input.lines().fold(NodeMap::new(), |mut nodes, line| {
        let caps = re.captures(line).unwrap();
        let name = caps.get(1).unwrap().as_str();
        let l_name = caps.get(2).unwrap().as_str();
        let r_name = caps.get(3).unwrap().as_str();

        let l_node = nodes
            .entry(l_name)
            .or_insert(Rc::new(RefCell::new(Node::empty(l_name))))
            .clone();

        let r_node = nodes
            .entry(r_name)
            .or_insert(Rc::new(RefCell::new(Node::empty(r_name))))
            .clone();

        if let Some(root) = nodes.get(name) {
            root.borrow_mut().l = Some(l_node);
            root.borrow_mut().r = Some(r_node);
            //println!("Updated to {}", root.borrow());
        } else {
            let n = Rc::new(RefCell::new(Node::new(name, Some(l_node), Some(r_node))));
            //println!("Inserted {}", n.borrow());
            nodes.insert(name, n);
        }

        nodes
    })
}

fn parse(input: &str) -> (&str, NodeMap) {
    let mut parts = input.split("\n\n");
    let name = parts.next().unwrap();
    let nodes = parse_nodes(parts.next().unwrap());
    (name, nodes)
}

fn path_len(start_node: NodeRef, dirs: &str, at_end: fn(&str) -> bool) -> u64 {
    let mut count = 0;
    let mut i = dirs.chars().cycle();
    let mut node = start_node;
    while !at_end(node.borrow().name) {
        let c = i.next().unwrap();
        count += 1;
        node = match c {
            'L' => node.borrow().left().unwrap(),
            'R' => node.borrow().right().unwrap(),
            _ => panic!("Invalid direction"),
        };
    }
    count
}

fn solve_part1(input: &str) -> u64 {
    let (dirs, nodes) = parse(input);
    let node = nodes.get("AAA").unwrap().clone();
    path_len(node, dirs, |n| n == "ZZZ")
}

fn solve_part2(input: &str) -> u64 {
    let (dirs, nodes) = parse(input);
    let start_nodes = nodes
        .iter()
        .filter(|(_, n)| n.borrow().name.ends_with('A'))
        .map(|(_k, v)| v.clone())
        .collect::<Vec<_>>();

    println!(
        "Start nodes: {}",
        start_nodes
            .iter()
            .map(|n| n.borrow().name)
            .collect::<Vec<_>>()
            .join(",")
    );

    let counts = start_nodes
        .iter()
        .map(|n| path_len(n.clone(), dirs, |n| n.ends_with('Z')))
        .collect::<HashSet<_>>();

    println!("Counts: {:?}", counts);

    let mut it = counts.iter();
    let first = *it.next().unwrap();
    it.fold(first, |lcm, x| {
        let new_lcm = num_integer::lcm(lcm, *x);
        lcm.max(new_lcm)
    })
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
    const TEST_INPUT_1_2: &str = include_str!("test_input1_2.txt");
    const TEST_INPUT_2: &str = include_str!("test_input2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 2);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(solve_part1(TEST_INPUT_1_2), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 6);
    }

    #[test]
    fn test_parse() {
        let (dirs, nodes) = parse(TEST_INPUT);
        assert_eq!(dirs, "RL");
        for n in nodes.iter() {
            println!("{}", n.0,);
        }
        assert_eq!(nodes["AAA"].borrow().name, "AAA");
        assert!(nodes["AAA"].borrow().l.is_some());
        assert_eq!(nodes.len(), 7);
    }
}
