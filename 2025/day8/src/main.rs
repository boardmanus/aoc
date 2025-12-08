use std::{collections::HashSet, fmt::Display, ops::Sub};

use aoc_utils::grif::{
    simple::{NodeIdFromStr, SimpleGraph, SimpleGraphBuilder},
    Builder, Graph,
};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Pos3d {
    x: i64,
    y: i64,
    z: i64,
}

impl Display for Pos3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{},{}]", self.x, self.y, self.z)
    }
}

impl<'a> NodeIdFromStr<'a> for Pos3d {
    fn node_id_from_str(_s: &'a str) -> Option<Self> {
        None
    }
}

impl Pos3d {
    fn parse(line: &str) -> Option<Pos3d> {
        let mut i = line.split(',').filter_map(|s| s.parse::<i64>().ok());
        Some(Pos3d {
            x: i.next()?,
            y: i.next()?,
            z: i.next()?,
        })
    }
}

impl Sub for Pos3d {
    type Output = Vec3d;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3d {
            x: rhs.x - self.x,
            y: rhs.y - self.y,
            z: rhs.z - self.z,
        }
    }
}

struct Vec3d {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3d {
    fn new(x: i64, y: i64, z: i64) -> Vec3d {
        Vec3d { x, y, z }
    }

    fn mag2(&self) -> i64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

fn parse_input(input: &str) -> Vec<Pos3d> {
    input
        .lines()
        .filter_map(|line| Pos3d::parse(line))
        .collect()
}

fn shortest_wires(boxes: &Vec<Pos3d>, n: usize) -> Vec<(&Pos3d, &Pos3d)> {
    let mut dists: Vec<(&Pos3d, &Pos3d)> = (0..boxes.len()).fold(vec![], |dists, i| {
        (i + 1..boxes.len()).fold(dists, |mut dists, j| {
            dists.push((&boxes[i], &boxes[j]));
            dists
        })
    });
    dists.sort_by_key(|(&a, &b)| (b - a).mag2());
    dists.into_iter().take(n).collect::<Vec<_>>()
}

fn connect_wires(boxes: &Vec<Pos3d>, n: usize) -> SimpleGraph<Pos3d> {
    let shortest_nodes = shortest_wires(boxes, n);
    let mut g = SimpleGraphBuilder::new("circuts");
    shortest_nodes
        .iter()
        .for_each(|(&a, &b)| _ = g.add_edge(a, b, 1));

    g.build()
}

fn circuit_prod(boxes: &Vec<Pos3d>, n: usize, np: usize) -> usize {
    let g = connect_wires(&boxes, n);
    let mut visited: HashSet<Pos3d> = HashSet::new();
    let mut circuit_size: Vec<usize> = vec![];
    for j in boxes {
        if visited.contains(&j) {
            continue;
        }
        if let Some(_) = g.node(&j) {
            let s = g.dfs(*j, |_x| true).fold(0, |s, x| {
                visited.insert(x);
                s + 1
            });
            circuit_size.push(s);
        }
    }
    circuit_size.sort();
    let prod = circuit_size.iter().rev().take(np).product();
    println!("prod={prod}, circuit_sizes={:?}", circuit_size);
    prod
}

fn last_boxes(boxes: &Vec<Pos3d>) -> (Pos3d, Pos3d) {
    let mut used: HashSet<Pos3d> = HashSet::new();
    let mut dists: Vec<(&Pos3d, &Pos3d)> = (0..boxes.len()).fold(vec![], |dists, i| {
        (i + 1..boxes.len()).fold(dists, |mut dists, j| {
            dists.push((&boxes[i], &boxes[j]));
            dists
        })
    });
    dists.sort_by_key(|(&a, &b)| (b - a).mag2());
    let last = dists
        .iter()
        .find(|(&a, &b)| {
            used.insert(a);
            used.insert(b);
            boxes.iter().all(|b| used.contains(b))
        })
        .unwrap();
    (*last.0, *last.1)
}

pub fn part1(input: &str) -> usize {
    let boxes = parse_input(input);
    circuit_prod(&boxes, 1000, 3)
}

pub fn part2(input: &str) -> usize {
    let boxes = parse_input(input);
    let last = last_boxes(&boxes);
    (last.0.x * last.1.x) as usize
}

const INPUT: &str = include_str!("data/input");
fn main() {
    aoc_utils::run::main(INPUT, part1, part2);
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 40;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 25272;

    #[test]
    fn test_part1() {
        let boxes = parse_input(TEST_INPUT);
        //assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
        assert_eq!(circuit_prod(&boxes, 10, 3), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
}
