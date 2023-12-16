use core::hash;
use std::collections::LinkedList;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Operation<'a> {
    label: &'a str,
    focal_length: Option<usize>,
}

impl<'a> Operation<'a> {
    fn new(label: &'a str, focal_length: Option<usize>) -> Self {
        Self {
            label,
            focal_length,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct LensBox<'a> {
    ops: Vec<Operation<'a>>,
}

impl<'a> LensBox<'a> {
    fn new() -> Self {
        Self { ops: Vec::new() }
    }
    fn push(&mut self, op: &Operation<'a>) {
        match self.ops.iter().position(|inop| inop.label == op.label) {
            Some(pos) => {
                self.ops[pos] = *op;
            }
            None => self.ops.push(*op),
        }
    }
    fn remove(&mut self, op: &Operation<'a>) {
        match self.ops.iter().position(|inop| inop.label == op.label) {
            Some(pos) => {
                self.ops.remove(pos);
            }
            None => (),
        }
    }
}
fn parse1(input: &str) -> Vec<&str> {
    input.trim().split(',').collect()
}

fn parse2(input: &str) -> Vec<Operation> {
    input
        .trim()
        .split(',')
        .map(|op_str| {
            let chars = op_str.chars();
            match chars.last().unwrap() {
                '-' => Operation::new(op_str.split('-').next().unwrap(), None),
                _ => {
                    let mut bits = op_str.split('=');
                    Operation::new(
                        bits.next().unwrap(),
                        Some(bits.next().unwrap().parse::<usize>().unwrap()),
                    )
                }
            }
        })
        .collect()
}

fn checksum(steps: &str) -> usize {
    steps
        .chars()
        .fold(0, |cs, x| ((cs + x as usize) * 17) % 256)
}

fn solve_part1(input: &str) -> usize {
    parse1(input).iter().map(|x| checksum(x)).sum()
}

fn solve_part2(input: &str) -> usize {
    let ops = parse2(input);
    let mut boxes: Vec<LensBox<'_>> = Default::default();
    (0..256).for_each(|_| boxes.push(LensBox::new()));

    ops.iter().enumerate().for_each(|op| {
        let hash = checksum(op.1.label);
        let lens_box = &mut boxes[hash];
        match op.1.focal_length {
            Some(_) => {
                lens_box.push(op.1);
            }
            None => {
                lens_box.remove(op.1);
            }
        }
    });

    boxes
        .iter()
        .enumerate()
        .map(|(i, lens_box)| {
            lens_box
                .ops
                .iter()
                .enumerate()
                .map(|(j, op)| (i + 1) * (j + 1) * op.focal_length.unwrap() as usize)
                .sum::<usize>()
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
        assert_eq!(solve_part1(TEST_INPUT), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 145);
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse1(TEST_INPUT),
            vec![
                "rn=1", "cm-", "qp=3", "cm=2", "qp-", "pc=4", "ot=9", "ab=5", "pc-", "pc=6", "ot=7"
            ]
        );
    }

    #[test]
    fn test_parse2() {
        let p = parse2(TEST_INPUT);
        println!("{:?}", p);
    }

    #[test]
    fn test_sanity() {
        println!("hash rn = {}", checksum("rn"));
        println!("hash cm = {}", checksum("cm"));
        println!("hash qp = {}", checksum("qp"));
        println!("hash pc = {}", checksum("pc"));
    }
}
