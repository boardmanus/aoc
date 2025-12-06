use std::iter::once;

use aoc_utils::str::AocStr;

struct Sequence {
    nums: Vec<usize>,
    op: char,
}

impl Sequence {
    fn solve(&self) -> usize {
        match self.op {
            '+' => self.nums.iter().sum(),
            '*' => self.nums.iter().product(),
            _ => panic!("illegal op"),
        }
    }
}

fn parse_cols(input: &str) -> Vec<Vec<&str>> {
    let rows = input.lines().collect::<Vec<_>>();
    let op_str = rows.last().unwrap();
    let col_iter = op_str
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if c.is_whitespace() { None } else { Some(i) })
        .chain(once(op_str.len() + 1));
    let col_next_iter = col_iter.clone().skip(1);
    col_iter
        .zip(col_next_iter)
        .map(|(s, e)| {
            rows.iter()
                .map(|row_str| &row_str[s..e - 1])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn parse_input(input: &str) -> Vec<Sequence> {
    let h = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let len = h[0].len();
    assert!(h.iter().all(|v| v.len() == len));

    let v = (0..len)
        .map(|i| h.iter().map(|s| s[i]).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    v.iter()
        .map(|s| Sequence {
            nums: s[0..s.len() - 1]
                .iter()
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect(),
            op: s[s.len() - 1].first().unwrap(),
        })
        .collect()
}

fn parse_input2(input: &str) -> Vec<Sequence> {
    let cols = parse_cols(input);

    println!("cols={:?}", cols);
    cols.iter()
        .map(|s| Sequence {
            nums: nums_from_str(&s[0..s.len() - 1]),
            op: s.last().unwrap().first().unwrap(),
        })
        .collect()
}

fn nums_from_str(num_strs: &[&str]) -> Vec<usize> {
    let len = num_strs[0].len();
    let nums = (0..len)
        .rev()
        .map(|i| {
            num_strs
                .iter()
                .filter_map(|row| {
                    let c = row.nth(i);
                    if c.is_whitespace() {
                        None
                    } else {
                        Some(c.to_digit(10).unwrap() as usize)
                    }
                })
                .fold(0, |num, digit| num * 10 + digit)
        })
        .collect();
    println!("nums={:?}", nums);
    nums
}

pub fn part1(input: &str) -> usize {
    let questions = parse_input(input);
    questions.iter().map(|q: &Sequence| q.solve()).sum()
}

pub fn part2(input: &str) -> usize {
    let questions = parse_input2(input);
    questions.iter().map(|q: &Sequence| q.solve()).sum()
}

const INPUT: &str = include_str!("data/input");
fn main() {
    aoc_utils::run::main(INPUT, part1, part2);
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 4277556;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 3263827;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
}
