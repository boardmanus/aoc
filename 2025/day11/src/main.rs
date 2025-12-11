use std::collections::HashMap;

use aoc_utils::{
    grif::{
        simple::{SimpleGraph, SimpleGraphBuilder},
        Builder, Graph,
    },
    lust::Lust,
};

type Path<'a> = Lust<&'a str>;
type MachineGraph<'a> = SimpleGraph<&'a str>;

fn parse_input<'a>(input: &'a str) -> MachineGraph<'a> {
    let mut b = SimpleGraphBuilder::new("machines");
    input.lines().for_each(|line| {
        let (node, rest) = line.split_once(':').unwrap();
        b.add_node(node);
        rest.split_ascii_whitespace().for_each(|n| {
            b.add_directed_edge(node, n, 1);
        })
    });
    b.build()
}

fn num_paths_r<'a>(
    g: &MachineGraph<'a>,
    path: Path<'a>,
    must_visit: &[&str],
    visited: &mut HashMap<&'a str, usize>,
) -> usize {
    let &n = path.data().unwrap();
    if n == "out" {
        return if must_visit.iter().all(|n| path.contains(n)) {
            1
        } else {
            0
        };
    }

    if let Some(&paths) = visited.get(n) {
        println!("Existing path at {path}: {paths}");
        return paths;
    }
    let num_paths = g
        .node_neighbours(n)
        .map(|nb| {
            if !path.contains(&nb) {
                num_paths_r(g, path.append(nb), must_visit, visited)
            } else {
                0
            }
        })
        .sum();

    visited.insert(n, num_paths);

    num_paths
}

fn num_paths(g: &MachineGraph, start: &str, must_visit: &[&str]) -> usize {
    let mut visited: HashMap<&str, usize> = HashMap::new();
    let np = num_paths_r(g, Path::new(start), must_visit, &mut visited);
    println!("visited: {:?}", visited);
    np
}

fn num_paths2_r<'a>(
    g: &MachineGraph<'a>,
    path: Path<'a>,
    did_visit: [bool; 2],
    must_visit: [&str; 2],
    visited: &mut HashMap<&'a str, (usize, [bool; 2])>,
) -> (usize, [bool; 2]) {
    let &n = path.data().unwrap();
    let did_visit = must_visit
        .iter()
        .enumerate()
        .fold(did_visit, |mut dv, (i, v)| {
            dv[i] = dv[i] || (n == must_visit[i]);
            dv
        });
    if n == "out" {
        println!("Found out: {path}");
        return (1, [false, false]);
    }

    if let Some(&paths) = visited.get(n) {
        println!("Existing path at {path}: {:?} : {:?}", paths, paths);
        return paths;
    }
    let mut num_paths = g
        .node_neighbours(n)
        .map(|nb| {
            if !path.contains(&nb) {
                num_paths2_r(g, path.append(nb), did_visit, must_visit, visited)
            } else {
                (0, [false, false])
            }
        })
        .fold((0, [false, false]), |r, nb| match nb {
            (size, [true, true]) => match r {
                (size2, [true, true]) => (size + size2, [true, true]),
                _ => nb,
            },
            (size, [true, false]) => match r {
                (_, [true, true]) => r,
                (size2, [true, false]) => (size + size2, [true, false]),
                (_, [false, false]) => nb,
                _ => panic!("unexpected this is"),
            },
            (size, [false, true]) => match r {
                (_, [true, true]) => r,
                (size2, [false, true]) => (size + size2, [false, true]),
                (_, [false, false]) => nb,
                _ => panic!("unexpected"),
            },
            (size, [false, false]) => match r {
                (size2, [false, false]) => (size + size2, [false, false]),
                _ => r,
            },
        });
    if n == must_visit[0] {
        num_paths.1[0] = true;
    } else if n == must_visit[1] {
        num_paths.1[1] = true;
    }

    println!("Inserting {n}, {:?}", num_paths);
    visited.insert(n, num_paths);

    num_paths
}

fn num_paths2(g: &MachineGraph, start: &str, must_visit: [&str; 2]) -> usize {
    let mut visited: HashMap<&str, (usize, [bool; 2])> = HashMap::new();
    let np = num_paths2_r(
        g,
        Path::new(start),
        [false, false],
        must_visit,
        &mut visited,
    );
    println!("visited: {:?}", visited);
    assert!(np.1[0] && np.1[1]);
    np.0
}

pub fn part1(input: &str) -> usize {
    let g = parse_input(input);
    num_paths(&g, "you", &[])
}

pub fn part2(input: &str) -> usize {
    let g = parse_input(input);
    num_paths2(&g, "svr", ["fft", "dac"])
}

const INPUT: &str = include_str!("data/input");
fn main() {
    aoc_utils::run::main(INPUT, part1, part2);
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 5;
    pub const TEST_INPUT_2: &str = include_str!("data/input_example2");
    pub const TEST_ANSWER_2: usize = 2;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
}
