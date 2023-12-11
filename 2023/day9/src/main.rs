fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn dline_end(pattern: &mut [i64]) -> &mut [i64] {
    let len = pattern.len();
    assert!(len > 1);
    for i in 0..(pattern.len() - 1) {
        let dp = pattern[i + 1] - pattern[i];
        pattern[i] = dp;
    }
    &mut pattern[0..(len - 1)]
}

fn dline_start(pattern: &mut [i64]) -> &mut [i64] {
    let len = pattern.len();
    assert!(len > 1);
    for i in (1..pattern.len()).rev() {
        let dp = pattern[i] - pattern[i - 1];
        pattern[i] = dp;
    }
    &mut pattern[1..len]
}

fn has_variation(pattern: &[i64]) -> bool {
    !pattern.iter().all(|i| *i == 0)
}

fn next_val(pattern: &[i64]) -> i64 {
    let mut workspace = Vec::from(pattern);
    let mut dpattern = &mut workspace[..];
    while has_variation(dpattern) {
        println!("dpattern: {:?}", dpattern);
        dpattern = dline_end(dpattern);
    }
    workspace.iter().sum()
}

fn prev_val(pattern: &[i64]) -> i64 {
    let mut workspace = Vec::from(pattern);
    let mut dpattern = &mut workspace[..];
    while has_variation(dpattern) {
        println!("dpattern: {:?}", dpattern);
        dpattern = dline_start(dpattern);
    }
    println!("workspace: {:?}", workspace);
    let s = workspace.iter().rev().fold(0, |acc, x| x - acc);
    println!("s: {:?}", s);
    s
}

fn solve_part1(input: &str) -> i64 {
    let lines = parse(input);
    lines
        .iter()
        .map(|l| {
            println!("Line: {:?}", l);
            next_val(l)
        })
        .sum()
}

fn solve_part2(input: &str) -> i64 {
    let lines = parse(input);
    lines
        .iter()
        .map(|l| {
            println!("Line: {:?}", l);
            prev_val(l)
        })
        .sum()
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
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
        assert_eq!(solve_part1(TEST_INPUT), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 2);
    }

    #[test]
    fn test_parse() {
        let expected: Vec<Vec<i64>> = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ];
        assert_eq!(parse(TEST_INPUT), expected);
    }
}
