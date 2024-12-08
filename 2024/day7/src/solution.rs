use std::u64;

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
enum Op {
    Add,
    Multiply,
    Concat,
    Subtract,
    Divide,
    Uncat,
}

impl Op {
    fn concat(lhs: u64, rhs: u64) -> u64 {
        let c_str = [lhs.to_string(), rhs.to_string()].join("");
        c_str.parse::<u64>().unwrap_or(u64::MAX)
    }
    fn concat2(lhs: u64, rhs: u64) -> u64 {
        //lhs * 10u64.pow((rhs as f64 + 0.5).log10().ceil() as u32) + rhs
        lhs * 10u64.pow(rhs.checked_ilog10().unwrap_or(0) + 1) + rhs
    }

    fn uncat(lhs: u64, rhs: u64) -> u64 {
        let lhs_str = lhs.to_string();
        let uncat_str = lhs_str.strip_suffix(&rhs.to_string());
        if let Some(uncat_str) = uncat_str {
            if uncat_str.is_empty() {
                0
            } else {
                uncat_str.parse::<u64>().unwrap()
            }
        } else {
            lhs
        }
    }

    fn uncat2(lhs: u64, rhs: u64) -> u64 {
        lhs / (rhs.checked_ilog10().unwrap_or(0) as u64 + 1)
    }

    fn apply(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Op::Add => lhs + rhs,
            Op::Multiply => lhs * rhs,
            Op::Concat => Op::concat(lhs, rhs),
            Op::Subtract => lhs - rhs,
            Op::Divide => lhs / rhs,
            Op::Uncat => Op::uncat(lhs, rhs),
        }
    }

    fn possible(&self, lhs: u64, rhs: u64) -> bool {
        match self {
            Op::Add | Op::Multiply | Op::Concat => true,
            Op::Subtract => lhs >= rhs,
            Op::Divide => lhs % rhs == 0,
            Op::Uncat => lhs.to_string().ends_with(&rhs.to_string()),
        }
    }

    fn reverse(&self) -> Op {
        match self {
            Op::Add => Op::Subtract,
            Op::Multiply => Op::Divide,
            Op::Concat => Op::Uncat,
            Op::Uncat => Op::Concat,
            Op::Divide => Op::Multiply,
            Op::Subtract => Op::Subtract,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Equation {
    test: u64,
    values: Vec<u64>,
}

impl Equation {
    fn parse(e_str: &str) -> Equation {
        let mut s = e_str.split(":");
        let test = s.next().unwrap().parse::<u64>().unwrap();
        let value_str = s.next().unwrap();
        let values = value_str
            .split_whitespace()
            .map(|v_str| v_str.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        Equation { test, values }
    }

    fn parse_all(input: &str) -> Vec<Equation> {
        input.lines().map(|e_str| Equation::parse(e_str)).collect()
    }

    fn tests_out(&self, ops: &[Op]) -> bool {
        let mut possibles: Vec<(u64, &[u64])> = vec![(self.values[0], &self.values[1..])];
        while let Some(p) = possibles.pop() {
            if p.1.len() == 0 {
                let result = p.0 == self.test;
                if result {
                    return true;
                }
            } else {
                ops.iter().for_each(|op| {
                    let rhs = *p.1.first().unwrap();
                    let res = op.apply(p.0, rhs);
                    if res <= self.test {
                        possibles.push((res, &p.1[1..]));
                    }
                })
            };
        }
        false
    }

    fn rtests_out(&self, ops: &[Op]) -> bool {
        let rops: Vec<Op> = ops.iter().map(|op| op.reverse()).collect();
        let rvalues: Vec<u64> = self.values.iter().rev().map(|x| *x).collect();
        let mut possibles: Vec<(u64, &[u64])> = vec![(self.test, &rvalues[0..])];

        while let Some(p) = possibles.pop() {
            if p.1.len() == 0 {
                let result = p.0 == 0;
                if result {
                    return true;
                }
            } else {
                rops.iter().for_each(|op| {
                    let rhs = *p.1.first().unwrap();
                    if op.possible(p.0, rhs) {
                        let res = op.apply(p.0, rhs);
                        possibles.push((res, &p.1[1..]));
                    }
                })
            };
        }
        false
    }
}
pub fn part1(input: &str) -> u64 {
    let equations = Equation::parse_all(input);
    equations
        .iter()
        .filter(|&equation| equation.rtests_out(&[Op::Add, Op::Multiply]))
        .map(|equation| equation.test)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let equations = Equation::parse_all(input);
    equations
        .iter()
        .filter(|&equation| equation.rtests_out(&[Op::Add, Op::Multiply, Op::Concat]))
        .map(|equation| equation.test)
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: u64 = 3749;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: u64 = 11387;

    #[test]
    fn test_parse_equation() {
        let equation = Equation::parse("12: 1 2 3 4 5");
        assert_eq!(
            equation,
            Equation {
                test: 12,
                values: vec![1, 2, 3, 4, 5]
            }
        );
    }

    #[test]
    fn test_concat() {
        assert_eq!(Op::Concat.apply(1, 2), 12);
        assert_eq!(Op::Concat.apply(435, 123), 435123);
        assert_eq!(Op::concat(1, 2), Op::concat2(1, 2));
        assert_eq!(Op::concat(435, 123), Op::concat2(435, 123));
        assert_eq!(Op::concat(111111, 33333), Op::concat2(111111, 33333));
        assert_eq!(Op::concat(9, 9), Op::concat2(9, 9));
        assert_eq!(Op::concat(10, 10), Op::concat2(10, 10));
    }

    #[test]
    fn test_uncat() {
        assert_eq!(Op::uncat(21, 1), 2);
        assert_eq!(Op::uncat(1234567, 234), 1234567);
        assert_eq!(Op::uncat(111, 1), 11);
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
