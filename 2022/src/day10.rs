use std::cmp::Ordering;

use crate::aoc::Aoc;

enum Op {
    Noop,
    Addx(i32),
}

fn line_to_op(line: &str) -> Op {
    let mut op_strs = line.split(' ');
    match op_strs.next().unwrap() {
        "noop" => Op::Noop,
        "addx" => Op::Addx(op_strs.next().unwrap().parse::<i32>().unwrap()),
        _ => panic!(),
    }
}

pub struct Day10_1;
impl Aoc for Day10_1 {
    fn day(&self) -> u32 {
        10
    }
    fn puzzle_name(&self) -> &str {
        "CRT"
    }
    fn solve(&self, lines: &[String]) -> String {
        //const SAMPLE_CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220];
        let mut sample_num = 20;
        let mut x = 1;
        let mut cycle: i32 = 0;
        lines
            .iter()
            .map(|line| line_to_op(line))
            .fold(0, |sum, op| -> i32 {
                let old_x = x;
                x += match op {
                    Op::Noop => {
                        cycle += 1;
                        0
                    }
                    Op::Addx(dx) => {
                        cycle += 2;
                        dx
                    }
                };
                let old_sample_num = sample_num;
                match cycle.cmp(&sample_num) {
                    Ordering::Greater => {
                        sample_num += 40;
                        sum + old_sample_num * old_x
                    }
                    Ordering::Equal => {
                        sample_num += 40;
                        sum + cycle * old_x
                    }
                    _ => sum,
                }
            })
            .to_string()
    }
}

pub struct Day10_2;
impl Aoc for Day10_2 {
    fn day(&self) -> u32 {
        10
    }
    fn puzzle_name(&self) -> &str {
        "CRT 2"
    }
    fn solve(&self, lines: &[String]) -> String {
        let mut x: i32 = 1;
        let display: String = lines
            .iter()
            .map(|line| line_to_op(line))
            .fold(Vec::<i32>::default(), |moves, op| -> Vec<i32> {
                let mut new_moves = moves;
                match op {
                    Op::Noop => new_moves.push(0),
                    Op::Addx(dx) => {
                        new_moves.push(0);
                        new_moves.push(dx);
                    }
                };
                new_moves
            })
            .iter()
            .enumerate()
            .map(|e| -> char {
                let ds = x.abs_diff((e.0 % 40) as i32);
                let c = if ds < 2 { '#' } else { '.' };
                x += e.1;
                c
            })
            .enumerate()
            .flat_map(|(i, c)| {
                {
                    if i != 0 && i % 40 == 0 {
                        Some('\n')
                    } else {
                        None
                    }
                }
                .into_iter()
                .chain(std::iter::once(c))
            })
            .collect();
        format!("\n{display}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 146] = [
        "addx 15", "addx -11", "addx 6", "addx -3", "addx 5", "addx -1", "addx -8", "addx 13",
        "addx 4", "noop", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1",
        "addx 5", "addx -1", "addx -35", "addx 1", "addx 24", "addx -19", "addx 1", "addx 16",
        "addx -11", "noop", "noop", "addx 21", "addx -15", "noop", "noop", "addx -3", "addx 9",
        "addx 1", "addx -3", "addx 8", "addx 1", "addx 5", "noop", "noop", "noop", "noop", "noop",
        "addx -36", "noop", "addx 1", "addx 7", "noop", "noop", "noop", "addx 2", "addx 6", "noop",
        "noop", "noop", "noop", "noop", "addx 1", "noop", "noop", "addx 7", "addx 1", "noop",
        "addx -13", "addx 13", "addx 7", "noop", "addx 1", "addx -33", "noop", "noop", "noop",
        "addx 2", "noop", "noop", "noop", "addx 8", "noop", "addx -1", "addx 2", "addx 1", "noop",
        "addx 17", "addx -9", "addx 1", "addx 1", "addx -3", "addx 11", "noop", "noop", "addx 1",
        "noop", "addx 1", "noop", "noop", "addx -13", "addx -19", "addx 1", "addx 3", "addx 26",
        "addx -30", "addx 12", "addx -1", "addx 3", "addx 1", "noop", "noop", "noop", "addx -9",
        "addx 18", "addx 1", "addx 2", "noop", "noop", "addx 9", "noop", "noop", "noop", "addx -1",
        "addx 2", "addx -37", "addx 1", "addx 3", "noop", "addx 15", "addx -21", "addx 22",
        "addx -6", "addx 1", "noop", "addx 2", "addx 1", "noop", "addx -10", "noop", "noop",
        "addx 20", "addx 1", "addx 2", "addx 2", "addx -6", "addx -11", "noop", "noop", "noop",
    ];

    #[test]
    fn test_soln() {
        let input_strs = INPUT
            .map(|s| String::from(s))
            .into_iter()
            .collect::<Vec<String>>();

        assert_eq!(Day10_1.solve(&input_strs), 13140.to_string());
    }

    #[test]
    fn test_soln2() {
        let input_strs = INPUT
            .map(|s| String::from(s))
            .into_iter()
            .collect::<Vec<String>>();

        assert_eq!(
            Day10_2.solve(&input_strs),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                .to_string()
        );
    }
}
