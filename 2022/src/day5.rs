use crate::aoc;
use lazy_static::lazy_static;
use regex::Regex;

fn partition_input(lines: &Vec<String>) -> (&[String], &[String]) {
    for i in 0..(lines.len() - 1) {
        let line = &lines[i];
        if line == "" {
            return (&lines[..i - 1], &lines[i + 1..]);
        }
    }
    panic!();
}

fn stacks_from_strs(stack_strs: &[String]) -> [Vec<char>; 9] {
    let mut stacks: [Vec<char>; 9] = Default::default();
    stack_strs.iter().rev().for_each(|row_str| {
        row_str.chars().enumerate().for_each(|(idx, c)| {
            let col = idx / 4;
            let i = idx % 4;
            if i == 1 && c >= 'A' && c <= 'Z' {
                stacks[col].push(c);
            }
        });
    });
    stacks
}

fn moves_from_strs(move_strs: &[String]) -> Vec<(usize, usize, usize)> {
    lazy_static! {
        static ref MOVE_REGEX: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    }
    move_strs
        .iter()
        .map(|move_str| {
            let matches = MOVE_REGEX.captures(move_str).unwrap();
            let n = matches.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let cola = matches.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
            let colb = matches.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;
            (n, cola, colb)
        })
        .collect::<Vec<(usize, usize, usize)>>()
}

pub struct Day5_1;
impl aoc::Aoc<u32> for Day5_1 {
    fn day(&self) -> u32 {
        5
    }
    fn puzzle_name(&self) -> &str {
        "Supply Stacks"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        let (stack_strs, move_strs) = partition_input(lines);
        let mut stacks = stacks_from_strs(stack_strs);
        let moves = moves_from_strs(move_strs);
        for m in moves {
            let (n, s, e) = m;
            for i in 0..n {
                let c = stacks[s].pop().unwrap();
                stacks[e].push(c);
            }
        }
        stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect::<String>()
    }
}

pub struct Day5_2;
impl aoc::Aoc<u32> for Day5_2 {
    fn day(&self) -> u32 {
        5
    }
    fn puzzle_name(&self) -> &str {
        "Supply Stacks 2"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        let (stack_strs, move_strs) = partition_input(lines);
        let mut stacks = stacks_from_strs(stack_strs);
        let moves = moves_from_strs(move_strs);
        for m in moves {
            let (n, s, e) = m;
            let start = &mut stacks[s];
            let tail = start.split_off(start.len() - n);
            stacks[e].extend(tail);
        }
        stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stuff() {}
}
