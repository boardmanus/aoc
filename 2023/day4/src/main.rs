use std::collections::HashSet;

fn parse_line(line: &str) -> (HashSet<u8>, HashSet<u8>) {
    let ws_re = regex::Regex::new(r"\s+").unwrap();
    let nums: Vec<HashSet<u8>> = line
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" | ")
        .map(|s| {
            ws_re
                .replace_all(s.trim(), " ")
                .split(' ')
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();
    (nums[0].clone(), nums[1].clone())
}

fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let res = parse_line(line);
            let num_matches = res.0.intersection(&res.1).count();
            if num_matches == 0 {
                0
            } else {
                1 << (num_matches - 1)
            }
        })
        .sum()
}

fn solve_part2(_input: &str) -> u64 {
    0
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
        assert_eq!(solve_part2(TEST_INPUT_2), 467835);
    }

    #[test]
    fn test_parseline() {
        assert_eq!(
            parse_line("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            (
                HashSet::from([1, 21, 53, 59, 44]),
                HashSet::from([69, 82, 63, 72, 16, 21, 14, 1])
            )
        );
    }
}
