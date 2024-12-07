use std::u64;

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
enum Op {
    Add,
    Multiply,
    Concat,
}

impl Op {
    fn concat(lhs: u64, rhs: u64) -> u64 {
        let c_str = [lhs.to_string(), rhs.to_string()].join("");
        c_str.parse::<u64>().unwrap_or(u64::MAX)
    }
    fn apply(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Op::Add => lhs + rhs,
            Op::Multiply => lhs * rhs,
            Op::Concat => Op::concat(lhs, rhs),
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
}
pub fn part1(input: &str) -> u64 {
    let equations = Equation::parse_all(input);
    equations
        .iter()
        .filter(|&equation| equation.tests_out(&[Op::Add, Op::Multiply]))
        .map(|equation| equation.test)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let equations = Equation::parse_all(input);
    equations
        .iter()
        .filter(|&equation| equation.tests_out(&[Op::Add, Op::Multiply, Op::Concat]))
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
