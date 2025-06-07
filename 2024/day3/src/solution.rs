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

    fn do_mul(&self) -> Option<bool> {
        match self {
            Op::Do => Some(true),
            Op::Dont => Some(false),
            Op::Mul(_, _) => None,
        }
    }

    fn value(&self) -> i64 {
        match self {
            Op::Mul(a, b) => a * b,
            _ => 0,
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

struct ParseOps {
    re: Regex,
}

impl ParseOps {
    fn new() -> Self {
        let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
        ParseOps { re }
    }

    fn parse<'a>(&'a self, input: &'a str) -> impl Iterator<Item = Op> + 'a {
        self.re
            .captures_iter(input)
            .filter_map(|cap| match cap.get(0)?.as_str() {
                "do()" => Some(Op::Do),
                "don't()" => Some(Op::Dont),
                _ => Some(Op::Mul(
                    cap.get(1)?.as_str().parse().ok()?,
                    cap.get(2)?.as_str().parse().ok()?,
                )),
            })
    }
}

pub fn part1(input: &str) -> i64 {
    ParseOps::new().parse(input).map(|op| op.value()).sum()
}

pub fn part2(input: &str) -> i64 {
    let mut do_mul = true;
    ParseOps::new()
        .parse(input)
        .map(|op| match op.do_mul() {
            Some(do_mul_op) => {
                do_mul = do_mul_op;
                0
            }
            None => match do_mul {
                true => op.value(),
                false => 0,
            },
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: i64 = 161;
    pub const TEST_INPUT_2: &str = include_str!("data/input_example2");
    pub const TEST_ANSWER_2: i64 = 48;

    #[test]
    fn test_re_captures_iter() {
        let re = Regex::new(r"(1|2|3)").unwrap();
        let captures: Vec<_> = re
            .captures_iter("1234321")
            .filter_map(|x| x.get(1))
            .map(|x| {
                println!("{}", x.as_str());
                x.as_str()
            })
            .collect();
        assert_eq!(captures.len(), 6);
        assert_eq!(captures[0], "1");
        assert_eq!(captures[1], "2");
        assert_eq!(captures[2], "3");
        assert_eq!(captures[3], "3");
        assert_eq!(captures[4], "2");
        assert_eq!(captures[5], "1");
    }

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
