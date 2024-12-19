use std::collections::HashMap;

use aoc_utils::str::AocStr;

struct TowelEntry {
    towel_end: bool,
    others: HashMap<char, TowelEntry>,
}

impl TowelEntry {
    fn new(towel_end: bool, towels: &Vec<&str>) -> TowelEntry {
        let mut others: HashMap<char, TowelEntry> = HashMap::new();
        for stripe in "wubrg".chars() {
            let next_towels = towels
                .iter()
                .filter(|t| t.first() == Some(stripe))
                .map(|&s| &s[1..])
                .collect::<Vec<_>>();
            if !next_towels.is_empty() {
                let end = towels
                    .iter()
                    .any(|t| t.len() == 1 && t.first() == Some(stripe));
                others.insert(stripe, TowelEntry::new(end, &next_towels));
            }
        }

        TowelEntry { towel_end, others }
    }
}

struct Puzzle<'a> {
    towels: Vec<&'a str>,
    designs: Vec<&'a str>,
    towel_tree: TowelEntry,
}

impl<'a> Puzzle<'a> {
    fn parse(input: &str) -> Puzzle {
        let mut lines = input.lines();
        let towels = lines.next().unwrap().split(", ").collect::<Vec<_>>();
        let _ = lines.next();
        let designs = lines.collect::<Vec<_>>();
        let towel_tree = TowelEntry::new(false, &towels);
        Puzzle {
            towels,
            designs,
            towel_tree,
        }
    }
}

fn is_feasible_design(puzzle: &Puzzle, design: &str) -> bool {
    let mut possies: Vec<&str> = vec![design];
    while let Some(design) = possies.pop() {
        for towel in puzzle.towels.iter() {
            if design.starts_with(towel) {
                if design.len() == towel.len() {
                    return true;
                }
                possies.push(&design[towel.len()..])
            }
        }
    }

    false
}

fn num_possible_designs(
    design: &str,
    position: usize,
    possibilities: &mut [Option<usize>],
    towels: &[&str],
) -> usize {
    if design.len() == 0 {
        return 1;
    }

    if let Some(possibilities) = possibilities[position] {
        return possibilities;
    }

    let possible = towels
        .iter()
        .map(|towel| {
            if !design.starts_with(towel) {
                0
            } else {
                num_possible_designs(
                    &design[towel.len()..],
                    position + towel.len(),
                    possibilities,
                    towels,
                )
            }
        })
        .sum();

    possibilities[position] = Some(possible);

    possible
}

fn all_possible_designs(puzzle: &Puzzle) -> usize {
    puzzle
        .designs
        .iter()
        .map(|d| num_possible_designs(d, 0, &mut vec![None; d.len()], &puzzle.towels))
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
    let puzzle = Puzzle::parse(input);
    num_feasible_designs(&puzzle)
}

pub fn part2(input: &str) -> usize {
    let puzzle = Puzzle::parse(input);
    all_possible_designs(&puzzle)
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
