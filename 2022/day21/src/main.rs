use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{complete::alpha0, complete::digit0},
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy)]
enum Op<'a> {
    Yell(i64),
    Plus(&'a str, &'a str),
    Minus(&'a str, &'a str),
    Mult(&'a str, &'a str),
    Div(&'a str, &'a str),
}

fn monkey_name(input: &str) -> IResult<&str, &str> {
    alpha0(input)
}

fn monkey_yell(input: &str) -> IResult<&str, Op> {
    let res = digit0(input)?;
    let num = res.1.parse().unwrap();
    Ok((res.0, Op::Yell(num)))
}

fn monkey_op(input: &str) -> IResult<&str, Op> {
    let res = tuple((
        monkey_name,
        alt((tag(" + "), tag(" - "), tag(" * "), tag(" / "))),
        monkey_name,
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

fn value<'a>(
    monkey: &'a str,
    all_ops: &'a HashMap<&'a str, Op>,
    cache: &mut HashMap<&'a str, i64>,
) -> i64 {
    if let Some(n) = cache.get(monkey) {
        *n
    } else {
        match all_ops[monkey] {
            Op::Yell(n) => {
                cache.insert(monkey, n);
                n
            }
            Op::Plus(m1, m2) => value(m1, all_ops, cache) + value(m2, all_ops, cache),
            Op::Minus(m1, m2) => value(m1, all_ops, cache) - value(m2, all_ops, cache),
            Op::Mult(m1, m2) => value(m1, all_ops, cache) * value(m2, all_ops, cache),
            Op::Div(m1, m2) => value(m1, all_ops, cache) / value(m2, all_ops, cache),
        }
    }
}

fn parse<'a>(input: &'a str) -> HashMap<&'a str, Op> {
    let mut monkeys: HashMap<&str, Op> = Default::default();
    input
        .split('\n')
        .flat_map(|i| separated_pair(monkey_name, tag(": "), alt((monkey_op, monkey_yell)))(i))
        .for_each(|r| {
            monkeys.insert(r.1 .0, r.1 .1);
        });
    monkeys
}

fn solve_part1(input: &str) -> String {
    let all_ops = parse(input);
    let mut all_res: HashMap<&str, i64> = Default::default();
    value("root", &all_ops, &mut all_res).to_string()
}

fn maybe_value<'a>(
    monkey: &'a str,
    all_ops: &'a HashMap<&'a str, Op>,
    cache: &mut HashMap<&'a str, i64>,
) -> Option<i64> {
    if monkey == "humn" {
        None
    } else {
        if let Some(n) = cache.get(monkey) {
            Some(*n)
        } else {
            let n = match all_ops[monkey] {
                Op::Yell(n) => {
                    cache.insert(monkey, n);
                    n
                }
                Op::Plus(m1, m2) => {
                    maybe_value(m1, all_ops, cache)? + maybe_value(m2, all_ops, cache)?
                }
                Op::Minus(m1, m2) => {
                    maybe_value(m1, all_ops, cache)? - maybe_value(m2, all_ops, cache)?
                }
                Op::Mult(m1, m2) => {
                    maybe_value(m1, all_ops, cache)? * maybe_value(m2, all_ops, cache)?
                }
                Op::Div(m1, m2) => {
                    maybe_value(m1, all_ops, cache)? / maybe_value(m2, all_ops, cache)?
                }
            };
            Some(n)
        }
    }
}

fn apply_lhs_op(op: Op, a: i64, b: i64) -> i64 {
    match op {
        Op::Yell(_) => a,
        Op::Plus(_, _) => a - b,
        Op::Minus(_, _) => b - a,
        Op::Mult(_, _) => a / b,
        Op::Div(_, _) => b / a,
    }
}

fn apply_rhs_op(op: Op, a: i64, b: i64) -> i64 {
    match op {
        Op::Yell(_) => a,
        Op::Plus(_, _) => a - b,
        Op::Minus(_, _) => a + b,
        Op::Mult(_, _) => a / b,
        Op::Div(_, _) => a * b,
    }
}

fn solve_x<'a>(
    res: i64,
    monkey: &'a str,
    all_ops: &'a HashMap<&'a str, Op>,
    cache: &mut HashMap<&'a str, i64>,
) -> i64 {
    if monkey == "humn" {
        return res;
    }
    let op = all_ops[monkey];
    match op {
        Op::Yell(x) => x,
        Op::Plus(a, b) | Op::Minus(a, b) | Op::Div(a, b) | Op::Mult(a, b) => {
            let lhs = maybe_value(a, all_ops, cache);
            let rhs = maybe_value(b, all_ops, cache);
            if let Some(l) = lhs {
                solve_x(apply_lhs_op(op, res, l), b, all_ops, cache)
            } else if let Some(r) = rhs {
                solve_x(apply_rhs_op(op, res, r), a, all_ops, cache)
            } else {
                panic!()
            }
        }
    }
}

fn solve_part2(input: &str) -> String {
    let all_ops = parse(input);
    let mut cache: HashMap<&str, i64> = Default::default();
    let val = match all_ops["root"] {
        Op::Yell(x) => x,
        Op::Plus(a, b) | Op::Minus(a, b) | Op::Div(a, b) | Op::Mult(a, b) => {
            if let Some(v) = maybe_value(a, &all_ops, &mut cache) {
                solve_x(v, b, &all_ops, &mut cache)
            } else if let Some(v) = maybe_value(b, &all_ops, &mut cache) {
                solve_x(v, a, &all_ops, &mut cache)
            } else {
                panic!()
            }
        }
    };
    val.to_string()
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
