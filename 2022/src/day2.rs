use std::{num::Wrapping, str::FromStr};

use crate::aoc;

const ROCK: u32 = 0;
const PAPER: u32 = 1;
const SCISSORS: u32 = 2;

const LOSS_SCORE: u32 = 0;
const DRAW_SCORE: u32 = 3;
const WIN_SCORE: u32 = 6;

enum Error {
    ParseRPS,
    ParseRes,
    ParseLine,
}

fn parse_rps(rps_str: &str) -> Result<u32, Error> {
    match rps_str {
        "A" | "X" => Ok(ROCK),
        "B" | "Y" => Ok(PAPER),
        "C" | "Z" => Ok(SCISSORS),
        _ => Err(Error::ParseRPS),
    }
}

fn parse_res(res_str: &str) -> Result<u32, Error> {
    match res_str {
        "X" => Ok(LOSS_SCORE),
        "Y" => Ok(DRAW_SCORE),
        "Z" => Ok(WIN_SCORE),
        _ => Err(Error::ParseRes),
    }
}

type RpsChallenge = (u32, u32);
fn parse_rps_line(line_str: &str) -> Result<RpsChallenge, Error> {
    let fields = line_str.split(' ').collect::<Vec<&str>>();
    if fields.len() == 2 {
        let rps_a = parse_rps(fields[0])?;
        let rps_b = parse_rps(fields[1])?;
        Ok((rps_a, rps_b))
    } else {
        Err(Error::ParseLine)
    }
}

type RpsResult = (u32, u32);
fn parse_res_line(line_str: &str) -> Result<RpsResult, Error> {
    let fields = line_str.split(' ').collect::<Vec<&str>>();
    if fields.len() == 2 {
        let rps = parse_rps(fields[0])?;
        let res = parse_res(fields[1])?;
        Ok((rps, res))
    } else {
        Err(Error::ParseLine)
    }
}

fn rps_value(rps: u32) -> u32 {
    rps + 1
}

const WRAPPED: u32 = std::u32::MAX - 1;
fn rps_score(challenge: RpsChallenge) -> u32 {
    match (Wrapping(challenge.1) - Wrapping(challenge.0)).0 {
        0 => DRAW_SCORE,
        1 | WRAPPED => WIN_SCORE,
        _ => LOSS_SCORE,
    }
}

fn rps_for_res(res: RpsResult) -> u32 {
    match res.1 {
        LOSS_SCORE => (res.0 + 2) % 3,
        DRAW_SCORE => res.0,
        WIN_SCORE => (res.0 + 1) % 3,
        _ => panic!(),
    }
}

fn challenge_score2(challenge: RpsChallenge) -> u32 {
    rps_score(challenge) + rps_value(challenge.1)
}

fn result_score(rps_res: RpsResult) -> u32 {
    let rps = rps_for_res(rps_res);
    rps_score((rps_res.0, rps)) + rps_value(rps)
}

pub struct Day2_1_2;
impl aoc::Aoc<u32> for Day2_1_2 {
    fn day(&self) -> u32 {
        2
    }
    fn puzzle_name(&self) -> &str {
        "Paper, Scissors, Rock 1.1"
    }
    fn solve(&self, lines: &Vec<String>) -> u32 {
        lines
            .iter()
            .flat_map(|line| parse_rps_line(line))
            .map(|challenge| challenge_score2(challenge))
            .sum::<u32>()
    }
}

pub struct Day2_1;
impl aoc::Aoc<u32> for Day2_1 {
    fn day(&self) -> u32 {
        2
    }
    fn puzzle_name(&self) -> &str {
        "Paper, Scissors, Rock"
    }
    fn solve(&self, lines: &Vec<String>) -> u32 {
        lines
            .iter()
            .flat_map(|line| parse_line(line))
            .map(|challenge| challenge_score(challenge))
            .sum::<u32>()
    }
}

pub struct Day2_2;
impl aoc::Aoc<u32> for Day2_2 {
    fn day(&self) -> u32 {
        2
    }
    fn puzzle_name(&self) -> &str {
        "Paper, Scissors, Rock 2"
    }
    fn solve(&self, lines: &Vec<String>) -> u32 {
        lines
            .iter()
            .flat_map(|line| parse_line_res(line))
            .map(|challenge| challenge_score_res(challenge))
            .sum()
    }
}
pub struct Day2_2_2;
impl aoc::Aoc<u32> for Day2_2_2 {
    fn day(&self) -> u32 {
        2
    }
    fn puzzle_name(&self) -> &str {
        "Paper, Scissors, Rock 2.1"
    }
    fn solve(&self, lines: &Vec<String>) -> u32 {
        lines
            .iter()
            .flat_map(|line| parse_res_line(line))
            .map(|rps_res| result_score(rps_res))
            .sum()
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

enum RPSResult {
    LOSS,
    DRAW,
    WIN,
}
impl RPSResult {
    fn value(&self) -> u32 {
        match self {
            RPSResult::LOSS => 0,
            RPSResult::DRAW => 3,
            RPSResult::WIN => 6,
        }
    }
    fn from_str(rps_str: &str) -> Result<Self, u32> {
        match rps_str {
            "X" => Ok(RPSResult::LOSS),
            "Y" => Ok(RPSResult::DRAW),
            "Z" => Ok(RPSResult::WIN),
            _ => Err(1),
        }
    }
}

impl FromStr for RPS {
    type Err = u32;
    fn from_str(rps_str: &str) -> Result<Self, Self::Err> {
        match rps_str {
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Scissors),
            _ => Err(1),
        }
    }
}

type Challenge = (RPS, RPS);
type ChallengeRes = (RPS, RPSResult);

fn challenge_score(challenge: Challenge) -> u32 {
    let cscore = match challenge.0 {
        RPS::Rock => match challenge.1 {
            RPS::Rock => RPSResult::DRAW.value(),
            RPS::Paper => RPSResult::WIN.value(),
            RPS::Scissors => RPSResult::LOSS.value(),
        },
        RPS::Paper => match challenge.1 {
            RPS::Rock => RPSResult::LOSS.value(),
            RPS::Paper => RPSResult::DRAW.value(),
            RPS::Scissors => RPSResult::WIN.value(),
        },
        RPS::Scissors => match challenge.1 {
            RPS::Rock => RPSResult::WIN.value(),
            RPS::Paper => RPSResult::LOSS.value(),
            RPS::Scissors => RPSResult::DRAW.value(),
        },
    };

    cscore + score(challenge.1)
}

fn challenge_score_res(challenge: ChallengeRes) -> u32 {
    let cscore = match challenge.0 {
        RPS::Rock => score(match challenge.1 {
            RPSResult::LOSS => RPS::Scissors,
            RPSResult::DRAW => RPS::Rock,
            RPSResult::WIN => RPS::Paper,
        }),
        RPS::Paper => score(match challenge.1 {
            RPSResult::LOSS => RPS::Rock,
            RPSResult::DRAW => RPS::Paper,
            RPSResult::WIN => RPS::Scissors,
        }),
        RPS::Scissors => score(match challenge.1 {
            RPSResult::LOSS => RPS::Paper,
            RPSResult::DRAW => RPS::Scissors,
            RPSResult::WIN => RPS::Rock,
        }),
    };

    cscore + challenge.1.value()
}

fn score(rps: RPS) -> u32 {
    match rps {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    }
}
fn parse_line(line: &String) -> Result<Challenge, u32> {
    let rps = line
        .split(' ')
        .map(|rps_str| RPS::from_str(rps_str).unwrap())
        .collect::<Vec<RPS>>();

    Ok((rps[0], rps[1]))
}

fn parse_line_res(line: &String) -> Result<ChallengeRes, u32> {
    let rps_res_str = line.split(' ').collect::<Vec<&str>>();

    let rps = RPS::from_str(rps_res_str[0])?;
    let res = RPSResult::from_str(rps_res_str[1])?;

    Ok((rps, res))
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROCK_SCORE: u32 = 1;
    const PAPER_SCORE: u32 = 2;
    const SCISSORS_SCORE: u32 = 3;

    #[test]
    fn test_parse_rps() {
        assert_eq!(
            parse_line(&String::from("A X")).unwrap(),
            (RPS::Rock, RPS::Rock)
        );
        assert_eq!(
            parse_line(&String::from("B Y")).unwrap(),
            (RPS::Paper, RPS::Paper)
        );
        assert_eq!(
            parse_line(&String::from("C Z")).unwrap(),
            (RPS::Scissors, RPS::Scissors)
        );
    }

    #[test]
    fn test_challenge_score() {
        assert_eq!(
            challenge_score((RPS::Rock, RPS::Scissors)),
            RPSResult::LOSS.value() + score(RPS::Scissors)
        );
        assert_eq!(
            challenge_score2((ROCK, SCISSORS)),
            LOSS_SCORE + SCISSORS_SCORE
        );
        assert_eq!(
            challenge_score((RPS::Scissors, RPS::Rock)),
            RPSResult::WIN.value() + score(RPS::Rock)
        );
        assert_eq!(challenge_score2((SCISSORS, ROCK)), WIN_SCORE + ROCK_SCORE);
        assert_eq!(
            challenge_score((RPS::Paper, RPS::Scissors)),
            RPSResult::WIN.value() + score(RPS::Scissors)
        );
        assert_eq!(
            challenge_score2((PAPER, SCISSORS)),
            WIN_SCORE + SCISSORS_SCORE
        );
        assert_eq!(
            challenge_score((RPS::Paper, RPS::Paper)),
            RPSResult::DRAW.value() + score(RPS::Paper)
        );
        assert_eq!(challenge_score2((PAPER, PAPER)), DRAW_SCORE + PAPER_SCORE);
    }

    #[test]
    fn test_rps_for_res() {
        assert_eq!(rps_for_res((ROCK, LOSS_SCORE)), SCISSORS);
        assert_eq!(rps_for_res((PAPER, LOSS_SCORE)), ROCK);
        assert_eq!(rps_for_res((SCISSORS, LOSS_SCORE)), PAPER);
        assert_eq!(rps_for_res((ROCK, DRAW_SCORE)), ROCK);
        assert_eq!(rps_for_res((PAPER, DRAW_SCORE)), PAPER);
        assert_eq!(rps_for_res((SCISSORS, DRAW_SCORE)), SCISSORS);
        assert_eq!(rps_for_res((ROCK, WIN_SCORE)), PAPER);
        assert_eq!(rps_for_res((PAPER, WIN_SCORE)), SCISSORS);
        assert_eq!(rps_for_res((SCISSORS, WIN_SCORE)), ROCK);
    }

    #[test]
    fn test_challenge_score_res() {
        assert_eq!(
            challenge_score_res((RPS::Rock, RPSResult::LOSS)),
            RPSResult::LOSS.value() + score(RPS::Scissors)
        );
        assert_eq!(
            result_score((ROCK, LOSS_SCORE)),
            LOSS_SCORE + SCISSORS_SCORE
        );
        assert_eq!(
            challenge_score_res((RPS::Scissors, RPSResult::WIN)),
            RPSResult::WIN.value() + score(RPS::Rock)
        );
        assert_eq!(result_score((SCISSORS, WIN_SCORE)), WIN_SCORE + ROCK_SCORE);
        assert_eq!(
            challenge_score_res((RPS::Paper, RPSResult::WIN)),
            RPSResult::WIN.value() + score(RPS::Scissors)
        );
        assert_eq!(result_score((PAPER, WIN_SCORE)), WIN_SCORE + SCISSORS_SCORE);

        assert_eq!(
            challenge_score_res((RPS::Paper, RPSResult::DRAW)),
            RPSResult::DRAW.value() + score(RPS::Paper)
        );
        assert_eq!(result_score((PAPER, DRAW_SCORE)), DRAW_SCORE + PAPER_SCORE);
    }
}
