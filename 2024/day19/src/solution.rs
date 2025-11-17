use std::collections::HashMap;

struct Puzzle<'a> {
    towels: Vec<&'a str>,
    designs: Vec<&'a str>,
}

impl<'a> Puzzle<'a> {
    fn parse(input: &'a str) -> Option<Puzzle<'a>> {
        let mut lines = input.lines();
        let towels = lines.next()?.split(", ").collect::<Vec<_>>();
        let designs = lines.skip(1).collect::<Vec<_>>();
        Some(Puzzle { towels, designs })
    }
}

fn is_feasible_design(puzzle: &Puzzle, design: &str) -> bool {
    let mut possies: Vec<&str> = vec![design];
    while let Some(design) = possies.pop() {
        for &towel in puzzle.towels.iter() {
            if let Some(rest) = design.strip_prefix(towel) {
                if rest.is_empty() {
                    return true;
                }
                possies.push(rest)
            }
        }
    }

    false
}

fn num_possible_arrangements_r<'a>(
    design: &'a str,
    towels: &[&'a str],
    visited: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        0
    } else if let Some(&val) = visited.get(design) {
        val
    } else {
        let sum = towels
            .iter()
            .map(|&towel| {
                if design == towel {
                    1
                } else if let Some(rest) = design.strip_prefix(towel) {
                    num_possible_arrangements_r(rest, towels, visited)
                } else {
                    0
                }
            })
            .sum();
        visited.insert(design, sum);
        sum
    }
}

fn num_possible_arrangements(design: &str, towels: &[&str]) -> usize {
    let mut visited: HashMap<&str, usize> = HashMap::new();
    num_possible_arrangements_r(design, towels, &mut visited)
}

fn total_possible_arrangements(puzzle: &Puzzle) -> usize {
    puzzle
        .designs
        .iter()
        .map(|d| num_possible_arrangements(d, &puzzle.towels))
        .sum()
}

fn num_feasible_designs(puzzle: &Puzzle) -> usize {
    puzzle
        .designs
        .iter()
        .filter(|d| is_feasible_design(puzzle, d))
        .count()
}

pub fn part1(input: &str) -> usize {
    let puzzle = Puzzle::parse(input).expect("Failed to parse input");
    num_feasible_designs(&puzzle)
}

pub fn part2(input: &str) -> usize {
    let puzzle = Puzzle::parse(input).expect("Failed to parse input");
    total_possible_arrangements(&puzzle)
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 6;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 16;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
}
