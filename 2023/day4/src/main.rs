use lazy_static::lazy_static; // 1.3.0
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref WHITESPACE_RE: Regex = Regex::new(r"\s+").unwrap();
    static ref CARD_NUM_RE: Regex = Regex::new(r"Card\s+(\d+)").unwrap();
}

fn parse_line(line: &str) -> (usize, HashSet<u8>, HashSet<u8>) {
    let mut split = line.split(':');
    let card_num_str = split.next().unwrap();

    let card_num = CARD_NUM_RE
        .captures(card_num_str)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();

    let nums: Vec<HashSet<u8>> = split
        .next()
        .unwrap()
        .trim()
        .split(" | ")
        .map(|s| {
            WHITESPACE_RE
                .replace_all(s.trim(), " ")
                .split(' ')
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();
    (card_num, nums[0].clone(), nums[1].clone())
}

fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let res = parse_line(line);
            let num_matches = res.1.intersection(&res.2).count();
            if num_matches == 0 {
                0
            } else {
                1 << (num_matches - 1)
            }
        })
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let lines = input.lines();
    let max_card_num = lines.count() + 1;
    let card_freqs = input
        .lines()
        .fold(HashMap::<usize, usize>::default(), |mut acc, line| {
            let ticket = parse_line(line);
            let card = ticket.0;
            let num_matches = ticket.1.intersection(&ticket.2).count();
            *acc.entry(card).or_insert(0) += 1;

            let num_copies = *acc.get(&card).unwrap();
            for copy in (card + 1)..max_card_num.min(card + num_matches + 1) {
                *acc.entry(copy).or_insert(0) += num_copies;
            }
            acc
        });
    card_freqs.iter().fold(0, |acc, (_, count)| acc + count)
}

fn main() {
    let part1 = solve_part1(include_str!("input.txt"));
    println!("Part1: {part1}");
    let part2 = solve_part2(include_str!("input.txt"));
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
    const TEST_INPUT_2: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 30);
    }

    #[test]
    fn test_parseline() {
        assert_eq!(
            parse_line("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            (
                3,
                HashSet::from([1, 21, 53, 59, 44]),
                HashSet::from([69, 82, 63, 72, 16, 21, 14, 1])
            )
        );
    }
}
