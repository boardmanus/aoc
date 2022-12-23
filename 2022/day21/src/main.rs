use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{complete::alpha0, complete::digit0},
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op<'a> {
    Yell(i64),
    Plus(&'a str, &'a str),
    Minus(&'a str, &'a str),
    Mult(&'a str, &'a str),
    Div(&'a str, &'a str),
    Eq(&'a str, &'a str),
    Solve,
}

impl<'a> Op<'a> {
    fn apply(&self, a: i64, b: i64) -> Option<i64> {
        match self {
            Op::Yell(x) => Some(*x),
            Op::Plus(_, _) => Some(a + b),
            Op::Minus(_, _) => Some(a - b),
            Op::Mult(_, _) => Some(a * b),
            Op::Div(_, _) => Some(a / b),
            Op::Eq(_, _) => Some((a == b) as i64),
            Op::Solve => None,
        }
    }

    fn solve(&self, lhs: i64, rhs_a: Option<i64>, rhs_b: Option<i64>) -> Option<i64> {
        if let Some(a) = rhs_a {
            match self {
                Op::Plus(_, _) => Some(lhs - a),
                Op::Minus(_, _) => Some(a - lhs),
                Op::Mult(_, _) => Some(lhs / a),
                Op::Div(_, _) => Some(a / lhs),
                Op::Eq(_, _) => rhs_a,
                Op::Yell(_) => Some(lhs),
                Op::Solve => None,
            }
        } else if let Some(b) = rhs_b {
            match self {
                Op::Plus(_, _) => Some(lhs - b),
                Op::Minus(_, _) => Some(lhs + b),
                Op::Mult(_, _) => Some(lhs / b),
                Op::Div(_, _) => Some(lhs * b),
                Op::Eq(_, _) => rhs_b,
                Op::Yell(_) => Some(lhs),
                Op::Solve => None,
            }
        } else {
            None
        }
    }

    fn ab(&self) -> Option<(&'a str, &'a str)> {
        match self {
            Op::Yell(_) | Op::Solve => None,
            Op::Plus(a, b) | Op::Minus(a, b) | Op::Div(a, b) | Op::Mult(a, b) | Op::Eq(a, b) => {
                Some((a, b))
            }
        }
    }
}

fn parse_monkey_name(input: &str) -> IResult<&str, &str> {
    alpha0(input)
}

fn parse_monkey_yell(input: &str) -> IResult<&str, Op> {
    let res = digit0(input)?;
    let num = res.1.parse().unwrap();
    Ok((res.0, Op::Yell(num)))
}

fn parse_monkey_op(input: &str) -> IResult<&str, Op> {
    let res = tuple((
        parse_monkey_name,
        alt((tag(" + "), tag(" - "), tag(" * "), tag(" / "))),
        parse_monkey_name,
    ))(input)?;
    let op = match res.1 .1 {
        " + " => Op::Plus(res.1 .0, res.1 .2),
        " - " => Op::Minus(res.1 .0, res.1 .2),
        " * " => Op::Mult(res.1 .0, res.1 .2),
        " / " => Op::Div(res.1 .0, res.1 .2),
        _ => panic!(),
    };
    Ok((res.0, op))
}

fn parse(input: &str) -> HashMap<&str, Op> {
    let mut monkeys: HashMap<&str, Op> = Default::default();
    input
        .split('\n')
        .flat_map(|i| {
            separated_pair(
                parse_monkey_name,
                tag(": "),
                alt((parse_monkey_op, parse_monkey_yell)),
            )(i)
        })
        .for_each(|r| {
            monkeys.insert(r.1 .0, r.1 .1);
        });
    monkeys
}

fn value<'a>(
    monkey: &'a str,
    all_ops: &'a HashMap<&'a str, Op>,
    cache: &mut HashMap<&'a str, i64>,
) -> Option<i64> {
    let op = all_ops[monkey];
    let v = if let Some(n) = cache.get(monkey) {
        *n
    } else if let Op::Yell(n) = op {
        n
    } else if let Some((a, b)) = op.ab() {
        op.apply(value(a, all_ops, cache)?, value(b, all_ops, cache)?)?
    } else {
        return None;
    };
    cache.insert(monkey, v);
    Some(v)
}

fn solve_x<'a>(
    res: i64,
    monkey: &'a str,
    all_ops: &'a HashMap<&'a str, Op>,
    cache: &mut HashMap<&'a str, i64>,
) -> Option<i64> {
    let op = all_ops[monkey];
    if op == Op::Solve {
        Some(res)
    } else if let Some((a, b)) = op.ab() {
        let lhs = value(a, all_ops, cache);
        let rhs = value(b, all_ops, cache);
        let unsolved_monkey = if lhs.is_some() { b } else { a };
        solve_x(op.solve(res, lhs, rhs)?, unsolved_monkey, all_ops, cache)
    } else {
        None
    }
}

fn as_eq(op: Op) -> Op {
    match op {
        Op::Plus(a, b) | Op::Minus(a, b) | Op::Div(a, b) | Op::Mult(a, b) => Op::Eq(a, b),
        _ => panic!(),
    }
}

fn solve_part1(input: &str) -> String {
    let all_ops = parse(input);
    let mut all_res: HashMap<&str, i64> = Default::default();
    value("root", &all_ops, &mut all_res).unwrap().to_string()
}

fn solve_part2(input: &str) -> String {
    let root = "root";
    let humn = "humn";
    let mut all_ops = parse(input);
    let mut cache: HashMap<&str, i64> = Default::default();
    *all_ops.get_mut(humn).unwrap() = Op::Solve;
    *all_ops.get_mut(root).unwrap() = as_eq(all_ops[root]);
    solve_x(0, root, &all_ops, &mut cache).unwrap().to_string()
}

fn main() {
    let part1 = solve_part1(include_str!("input.txt"));
    println!("Part1: {part1}");
    let part2 = solve_part2(include_str!("input.txt"));
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {
    use std::collections::{vec_deque, HashSet};

    use super::*;

    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT), 152.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT), 301.to_string());
    }
}
