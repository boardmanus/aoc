use regex::Regex;

#[derive(Debug, PartialEq)]
enum Op {
    Mul(i64, i64),
    Do,
    Dont,
}

impl Op {
    fn from(input: &str) -> Option<(Op, &str)> {
        let re = Regex::new(r"^(mul\((\d+),(\d+)\))|^(don't)|^(do)").ok()?;
        let c = re.captures(input)?;
        if let Some(_mul) = c.get(1) {
            let a = c.get(2)?.as_str().parse::<i64>().ok()?;
            let b = c.get(3)?.as_str().parse::<i64>().ok()?;
            Some((Op::Mul(a, b), c.get(0)?.as_str()))
        } else if let Some(_dont_mul) = c.get(4) {
            Some((Op::Dont, c.get(0)?.as_str()))
        } else if let Some(_do_mul) = c.get(5) {
            Some((Op::Do, c.get(0)?.as_str()))
        } else {
            None
        }
    }

    fn value(&self) -> i64 {
        match self {
            Op::Do => 1,
            Op::Dont => 0,
            Op::Mul(a, b) => a * b,
        }
    }
}

fn parse_mul(input: &str, all: bool) -> Vec<Op> {
    let mut i = 0;
    let mut do_mul = true;
    let mut muls: Vec<Op> = Vec::default();
    while i < input.len() {
        if let Some((op, match_str)) = Op::from(&input[i..]) {
            match op {
                Op::Mul(a, b) => {
                    if all || do_mul {
                        muls.push(Op::Mul(a, b));
                    }
                }
                Op::Do => do_mul = true,
                Op::Dont => do_mul = false,
            }
            i += match_str.len();
        } else {
            i += 1
        }
    }
    muls
}

pub fn part1(input: &str) -> i64 {
    let mults = parse_mul(input, true);
    mults.iter().fold(0, |coll, mul| coll + mul.value())
}

pub fn part2(input: &str) -> i64 {
    let mults = parse_mul(input, false);
    mults.iter().fold(0, |coll, mul| coll + mul.value())
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: i64 = 161;
    pub const TEST_INPUT_2: &str = include_str!("data/input_example2");
    pub const TEST_ANSWER_2: i64 = 48;

    #[test]
    fn test_mul_from() {
        assert_eq!(Op::from("mul(2,3)"), Some((Op::Mul(2, 3), "mul(2,3)")));
        assert_eq!(Op::from("mul(2,3)xxxxx"), Some((Op::Mul(2, 3), "mul(2,3)")));
        assert_eq!(Op::from(" mul(2,3)"), None);
        assert_eq!(Op::from("mul(2, 3)"), None);
        assert_eq!(Op::from("don'"), Some((Op::Do, "do")));
        assert_eq!(Op::from("don't"), Some((Op::Dont, "don't")));
    }

    #[test]
    fn test_parse_mul() {
        assert_eq!(
            parse_mul("yyymul(1,2)xxxxmul(3,4)zzz", true),
            vec![Op::Mul(1, 2), Op::Mul(3, 4)]
        );
        assert_eq!(
            parse_mul("yyymul(1,2)xdon'txmul(3,4)zzdozmul(5,6)", true),
            vec![Op::Mul(1, 2), Op::Mul(3, 4), Op::Mul(5, 6)]
        );
        assert_eq!(
            parse_mul("yyymul(1,2)xdon'txmul(3,4)zzdozmul(5,6)", false),
            vec![Op::Mul(1, 2), Op::Mul(5, 6)]
        );
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
