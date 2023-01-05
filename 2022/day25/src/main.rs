use std::{fmt::Debug, fmt::Display, str::FromStr};

use regex::Regex;

#[derive(PartialEq)]
struct Snafu(String);

impl Snafu {
    fn digit(val: u64) -> char {
        match val % 5 {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => panic!(),
        }
    }
    fn value(c: char) -> i64 {
        match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '=' => -2,
            '-' => -1,
            _ => panic!(),
        }
    }
}
impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for Snafu {
    fn from(value: u64) -> Self {
        let mut rem = value;
        let mut snafu_str = String::new();
        while rem > 0 {
            snafu_str.push(Snafu::digit(rem));
            rem = (rem + 2) / 5;
        }
        Snafu(snafu_str.chars().rev().collect())
    }
}

impl From<&Snafu> for u64 {
    fn from(value: &Snafu) -> Self {
        value
            .0
            .chars()
            .map(|c| Snafu::value(c))
            .fold(0, |dec, v| ((dec as i64) * 5 + v) as u64)
    }
}

impl FromStr for Snafu {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\s*([012\-=]+)\s*").unwrap();
        let cap = re.captures(s).ok_or(())?;
        let value = cap.get(1).ok_or(())?.as_str().to_string();
        Ok(Snafu(value))
    }
}

fn parse_fuel_requirements(input: &str) -> Vec<Snafu> {
    input
        .split('\n')
        .flat_map(|line| line.parse::<Snafu>())
        .collect::<Vec<_>>()
}

fn solve_part1(input: &str) -> String {
    let snafus = parse_fuel_requirements(input);
    let sum: u64 = snafus.iter().map(|snafu| u64::from(snafu)).sum();
    Snafu::from(sum).to_string()
}

fn solve_part2(input: &str) -> String {
    0.to_string()
}

fn main() {
    let res = solve_part1(include_str!("input.txt"));
    println!("Part1: {res}");
    let res = solve_part2(include_str!("input.txt"));
    println!("Part2: {res}");
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    const TEST_SNAFU: &str = include_str!("test_snafu.txt");
    const TEST_INPUT: &str = include_str!("test_input.txt");

    fn parse_test_snafu() -> Result<Vec<(u64, Snafu)>, ()> {
        let re = Regex::new(r"\s+(\d+)\s+([012\-=]+)").or(Err(()))?;
        let list = TEST_SNAFU
            .split('\n')
            .flat_map(|line| {
                let cap = re.captures(line)?;
                let base10 = cap.get(1)?.as_str().parse::<u64>().ok()?;
                let snafu = cap.get(2)?.as_str().parse::<Snafu>().ok()?;
                Some((base10, snafu))
            })
            .collect();
        Ok(list)
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), "2=-1=0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 54.to_string());
    }

    #[test]
    fn test_dec_to_snafu() {
        let brochure = parse_test_snafu().unwrap();
        brochure.iter().for_each(|b| {
            assert_eq!(Snafu::from(b.0), b.1);
        });
    }

    #[test]
    fn test_snafu_to_dec() {
        let brochure = parse_test_snafu().unwrap();
        brochure
            .iter()
            .for_each(|b| assert_eq!(b.0, u64::from(&b.1)));
    }
}
