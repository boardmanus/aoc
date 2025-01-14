use aoc_utils::graph::{Graph, Node};

pub fn part1(input: &str) -> usize {
    let graph: Graph<&str, u8> = Graph::parse(input, "-").unwrap();
    let cycles = graph.find_cycles(3, |node: &&Node<&str, u8>| node.id.starts_with("t"));
    cycles.len()
}

pub fn part2(input: &str) -> String {
    let graph: Graph<&str, u8> = Graph::parse(input, "-").unwrap();
    let mut max_clique = graph.find_maximum_clique(|node| true);
    max_clique.sort();
    max_clique.join(",")
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 7;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: &str = "co,de,ka,ta";

    #[test]
    fn test_any_3_cycle() {
        let graph: Graph<&str, u8> = Graph::parse(TEST_INPUT, "-").unwrap();
        let cycles = graph.find_cycles(3, |_: &&Node<&str, u8>| true);
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
}
