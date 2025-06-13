use aoc_utils::str::AocStr;

trait SpecialU64Ops {
    fn checked_concat(self, rhs: u64) -> Option<u64>;
    fn checked_uncat(self, rhs: u64) -> Option<u64>;
    fn checked_perfect_div(self, rhs: u64) -> Option<u64>;
}

impl SpecialU64Ops for u64 {
    fn checked_concat(self, rhs: u64) -> Option<u64> {
        Some(self * 10u64.pow(rhs.checked_ilog10()? + 1) + rhs)
    }

    fn checked_uncat(self, rhs: u64) -> Option<u64> {
        let p = 10u64.pow(rhs.checked_ilog10()? + 1);
        if self % p == rhs {
            Some(self / p)
        } else {
            None
        }
    }

    fn checked_perfect_div(self, rhs: u64) -> Option<u64> {
        if self % rhs == 0 {
            self.checked_div(rhs)
        } else {
            None
        }
    }
}

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
    fn apply(&self, lhs: u64, rhs: u64) -> Option<u64> {
        match self {
            Op::Add => lhs.checked_add(rhs),
            Op::Multiply => lhs.checked_mul(rhs),
            Op::Concat => lhs.checked_concat(rhs),
            Op::Subtract => lhs.checked_sub(rhs),
            Op::Divide => lhs.checked_perfect_div(rhs),
            Op::Uncat => lhs.checked_uncat(rhs),
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
    fn parse(line: &str) -> Equation {
        let mut s = line.split(":");
        let test = s.next().unwrap().parse::<u64>().unwrap();
        let values = s.next().unwrap().parse_nums::<u64>();
        Equation { test, values }
    }

    fn parse_all(input: &str) -> Vec<Equation> {
        input.parse_lines(Equation::parse)
    }

    fn evaluate_r(ops: &[Op], x: u64, values: &[u64]) -> bool {
        match values.len() {
            0 => false,
            1 => x == values[0],
            _ => ops.iter().any(|op| match op.apply(x, values[0]) {
                None => false,
                Some(res) => Equation::evaluate_r(ops, res, &values[1..]),
            }),
        }
    }

    // Evaluate is slower than reverse evaluate, as more operations are valid compared to the
    // inverse operations used by revaluate.
    // eg; for a checked_perfect_div, the two numbers must be divisble, whereas, for a checked_mul,
    // any operation is valid.
    fn revaluate(&self, ops: &[Op]) -> bool {
        let rops = ops.iter().map(|op| op.reverse()).collect::<Vec<_>>();
        let values = self.values.iter().rev().copied().collect::<Vec<_>>();
        Equation::evaluate_r(&rops, self.test, &values)
    }

    #[allow(unused)]
    fn evaluate(&self, ops: &[Op]) -> bool {
        let mut values = self.values.clone();
        values.push(self.test);
        Equation::evaluate_r(ops, values[0], &values[1..])
    }
}

pub fn part1(input: &str) -> u64 {
    let equations = Equation::parse_all(input);
    equations
        .iter()
        .filter(|&equation| equation.revaluate(&[Op::Add, Op::Multiply]))
        .map(|equation| equation.test)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let equations = Equation::parse_all(input);
    equations
        .iter()
        .filter(|&equation| equation.revaluate(&[Op::Add, Op::Multiply, Op::Concat]))
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
        assert_eq!(Op::Concat.apply(1, 2), Some(12));
        assert_eq!(Op::Concat.apply(435, 123), Some(435123));
    }

    #[test]
    fn test_uncat() {
        assert_eq!(21.checked_uncat(1), Some(2));
        assert_eq!(1234567.checked_uncat(234), None);
        assert_eq!(111.checked_uncat(1), Some(11));
        assert_eq!(111222333.checked_uncat(2333), Some(11122));
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
