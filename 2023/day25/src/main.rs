use rustworkx_core::{
    connectivity::stoer_wagner_min_cut,
    petgraph::{graph::NodeIndex, graph::UnGraph},
    Result,
};
use std::collections::HashMap;

fn parse<'a>(input: &'a str) -> UnGraph<&'a str, ()> {
    let mut graph = UnGraph::new_undirected();
    let mut nodes = HashMap::<&str, NodeIndex>::new();
    input.lines().for_each(|line| {
        let mut parts = line.split(": ");
        let a_name: &str = parts.next().unwrap();
        let rest = parts.next().unwrap().split_whitespace().collect::<Vec<_>>();

        let a = *nodes
            .entry(a_name)
            .or_insert_with(|| graph.add_node(a_name));
        for b_name in rest {
            let b = *nodes
                .entry(b_name)
                .or_insert_with(|| graph.add_node(b_name));
            graph.add_edge(a, b, ());
        }
    });
    graph
}

fn solve_part1(input: &str) -> usize {
    let graph = parse(input);
    let res: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&graph, |_| Ok(1));
    if let Ok(Some(min_cut)) = res {
        let num_min_cut_nodes = min_cut.1.len();
        num_min_cut_nodes * (graph.node_count() - num_min_cut_nodes)
    } else {
        panic!("No min cut found!");
    }
}

fn solve_part2(input: &str) -> usize {
    0
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let part1 = solve_part1(INPUT);
    println!("Part1: {part1}");
    let part2 = solve_part2(INPUT);
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
    const TEST_INPUT_2: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 54);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 467835);
    }

    #[test]
    fn test_parse() {
        let graph = parse(TEST_INPUT);
        //assert_eq!(graph["jqt"], vec!["rhn", "xhk", "nvd", "ntq"]);
        //assert_eq!(graph["nvd"], vec!["jqt", "cmg", "pzl", "qnr", "lhk"]);
        println!("{:?}", graph);
    }
}
