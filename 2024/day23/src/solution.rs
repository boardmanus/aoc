use aoc_utils::grif::{
    algorithms::find_cycles, algorithms::find_maximum_clique, simple::SimpleGraphBuilder,
};

pub fn part1(input: &str) -> usize {
    let graph = SimpleGraphBuilder::<&str>::parse("day23", input, "-").unwrap();
    let cycles = find_cycles(&graph, 3, |node| node.starts_with("t"));
    cycles.len()
}

pub fn part2(input: &str) -> String {
    let graph = SimpleGraphBuilder::<&str>::parse("day23", input, "-").unwrap();
    let mut max_clique = find_maximum_clique(&graph).unwrap();
    max_clique.sort();
    max_clique.join(",")
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;
    use graphviz_rust::{cmd::Format, exec, printer::PrinterContext};

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 7;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: &str = "co,de,ka,ta";

    #[test]
    fn test_any_3_cycle() {
        let graph = SimpleGraphBuilder::<&str>::parse("test-day23", TEST_INPUT, "-").unwrap();
        let cycles = find_cycles(&graph, 3, |_| true);
        println!("cycles: {:?}", cycles);
        assert_eq!(cycles.len(), 12);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }

    #[test]
    fn test_print_graph() {
        let graph = SimpleGraphBuilder::<&str>::parse("test-day23", TEST_INPUT, "-").unwrap();

        let viz = graph.to_viz(false);
        println!("{:?}", viz);

        let graph_svg = exec(
            viz,
            &mut PrinterContext::default(),
            vec![Format::Svg.into()],
        )
        .unwrap();
        fs::write("graph.svg", graph_svg).expect("Unable to write file");
    }
}
