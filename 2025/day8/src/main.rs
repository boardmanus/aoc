use std::collections::HashSet;

use aoc_utils::pos3d;
use aoc_utils::{
    grif::{
        simple::{SimpleGraph, SimpleGraphBuilder},
        Builder, Graph,
    },
    vecnd::VecSize,
};

type Pos3d = pos3d::Pos3d<i64>;

fn parse_pos3d(line: &str) -> Option<Pos3d> {
    let mut i = line.split(',').filter_map(|s| s.parse::<i64>().ok());
    Some(Pos3d {
        x: i.next()?,
        y: i.next()?,
        z: i.next()?,
    })
}

fn parse_input(input: &str) -> Vec<Pos3d> {
    input.lines().filter_map(parse_pos3d).collect()
}

fn shortest_wires(boxes: &[Pos3d], n: usize) -> Vec<(&Pos3d, &Pos3d)> {
    let mut dists: Vec<(&Pos3d, &Pos3d)> = (0..boxes.len()).fold(vec![], |dists, i| {
        (i + 1..boxes.len()).fold(dists, |mut dists, j| {
            dists.push((&boxes[i], &boxes[j]));
            dists
        })
    });
    dists.sort_by_key(|(&a, &b)| (b - a).mag_sqr());
    dists.into_iter().take(n).collect::<Vec<_>>()
}

fn connect_wires(boxes: &[Pos3d], n: usize) -> SimpleGraph<Pos3d> {
    let shortest_nodes = shortest_wires(boxes, n);
    let mut g = SimpleGraphBuilder::new("circuts");
    shortest_nodes
        .iter()
        .for_each(|(&a, &b)| _ = g.add_edge(a, b, 1));

    g.build()
}

fn circuit_prod(boxes: &[Pos3d], n: usize, np: usize) -> usize {
    let g = connect_wires(boxes, n);
    let mut visited: HashSet<Pos3d> = HashSet::new();
    let mut circuit_size: Vec<usize> = vec![];
    for j in boxes {
        if visited.contains(j) {
            continue;
        }
        if g.node(j).is_some() {
            let s = g.dfs(*j, |_x| true).fold(0, |s, x| {
                visited.insert(x);
                s + 1
            });
            circuit_size.push(s);
        }
    }
    circuit_size.sort();
    circuit_size.iter().rev().take(np).product()
}

fn last_boxes(boxes: &[Pos3d]) -> (Pos3d, Pos3d) {
    let mut used: HashSet<Pos3d> = HashSet::new();
    let mut dists: Vec<(&Pos3d, &Pos3d)> = (0..boxes.len()).fold(vec![], |dists, i| {
        (i + 1..boxes.len()).fold(dists, |mut dists, j| {
            dists.push((&boxes[i], &boxes[j]));
            dists
        })
    });
    dists.sort_by_key(|(&a, &b)| (b - a).mag_sqr());
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
