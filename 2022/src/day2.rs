use std::{ops::Add, str::FromStr};

use crate::aoc;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Rps(u32);
const ROCK: Rps = Rps(0);
const PAPER: Rps = Rps(1);
const SCISSORS: Rps = Rps(2);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Score(u32);
const LOSS_SCORE: Score = Score(0);
const DRAW_SCORE: Score = Score(3);
const WIN_SCORE: Score = Score(6);

enum Error {
    ParseRPS,
    ParseRes,
    ParseLine,
}

impl FromStr for Rps {
    type Err = Error;
    fn from_str(rps_str: &str) -> Result<Rps, Error> {
        match rps_str {
            "A" | "X" => Ok(ROCK),
            "B" | "Y" => Ok(PAPER),
            "C" | "Z" => Ok(SCISSORS),
            _ => Err(Error::ParseRPS),
        }
    }
}

impl FromStr for Score {
    type Err = Error;
    fn from_str(res_str: &str) -> Result<Score, Error> {
        match res_str {
            "X" => Ok(LOSS_SCORE),
            "Y" => Ok(DRAW_SCORE),
            "Z" => Ok(WIN_SCORE),
            _ => Err(Error::ParseRes),
        }
    }
}

impl Add for Score {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

struct RpsChallenge(Rps, Rps);
impl FromStr for RpsChallenge {
    type Err = Error;
    fn from_str(line_str: &str) -> Result<Self, Error> {
        let fields = line_str.split(' ').collect::<Vec<&str>>();
        if fields.len() == 2 {
            let left = Rps::from_str(fields[0])?;
            let right = Rps::from_str(fields[1])?;
            Ok(RpsChallenge(left, right))
        } else {
            Err(Error::ParseLine)
        }
    }
}

struct RpsResult(Rps, Score);
impl FromStr for RpsResult {
    type Err = Error;
    fn from_str(line_str: &str) -> Result<RpsResult, Error> {
        let fields = line_str.split(' ').collect::<Vec<&str>>();
        if fields.len() == 2 {
            let rps = Rps::from_str(fields[0])?;
            let score = Score::from_str(fields[1])?;
            Ok(RpsResult(rps, score))
        } else {
            Err(Error::ParseLine)
        }
    }
}
fn rps_value(rps: Rps) -> Score {
    Score(rps.0 + 1)
}

fn rps_score(challenge: &RpsChallenge) -> Score {
    match 3 + challenge.1 .0 - challenge.0 .0 {
        3 => DRAW_SCORE,
        4 | 1 => WIN_SCORE,
        _ => LOSS_SCORE,
    }
}

fn rps_for_res(res: &RpsResult) -> Rps {
    match res.1 {
        LOSS_SCORE => Rps((res.0 .0 + 2) % 3),
        DRAW_SCORE => Rps(res.0 .0),
        WIN_SCORE => Rps((res.0 .0 + 1) % 3),
        _ => panic!(),
    }
}

fn challenge_score2(challenge: &RpsChallenge) -> Score {
    rps_score(challenge) + rps_value(challenge.1)
}

fn result_score(rps_res: &RpsResult) -> Score {
    let rps = rps_for_res(rps_res);
    rps_score(&RpsChallenge(rps_res.0, rps)) + rps_value(rps)
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
            .flat_map(|line| RpsChallenge::from_str(line))
            .map(|challenge| challenge_score2(&challenge).0)
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
            .flat_map(|line| RpsResult::from_str(line))
            .map(|rps_res| result_score(&rps_res).0)
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

    const ROCK_SCORE: Score = Score(1);
    const PAPER_SCORE: Score = Score(2);
    const SCISSORS_SCORE: Score = Score(3);

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
            challenge_score2(&RpsChallenge(ROCK, SCISSORS)),
            LOSS_SCORE + SCISSORS_SCORE
        );
        assert_eq!(
            challenge_score((RPS::Scissors, RPS::Rock)),
            RPSResult::WIN.value() + score(RPS::Rock)
        );
        assert_eq!(
            challenge_score2(&RpsChallenge(SCISSORS, ROCK)),
            WIN_SCORE + ROCK_SCORE
        );
        assert_eq!(
            challenge_score((RPS::Paper, RPS::Scissors)),
            RPSResult::WIN.value() + score(RPS::Scissors)
        );
        assert_eq!(
            challenge_score2(&RpsChallenge(PAPER, SCISSORS)),
            WIN_SCORE + SCISSORS_SCORE
        );
        assert_eq!(
            challenge_score((RPS::Paper, RPS::Paper)),
            RPSResult::DRAW.value() + score(RPS::Paper)
        );
        assert_eq!(
            challenge_score2(&RpsChallenge(PAPER, PAPER)),
            DRAW_SCORE + PAPER_SCORE
        );
    }

    #[test]
    fn test_rps_for_res() {
        assert_eq!(rps_for_res(&RpsResult(ROCK, LOSS_SCORE)), SCISSORS);
        assert_eq!(rps_for_res(&RpsResult(PAPER, LOSS_SCORE)), ROCK);
        assert_eq!(rps_for_res(&RpsResult(SCISSORS, LOSS_SCORE)), PAPER);
        assert_eq!(rps_for_res(&RpsResult(ROCK, DRAW_SCORE)), ROCK);
        assert_eq!(rps_for_res(&RpsResult(PAPER, DRAW_SCORE)), PAPER);
        assert_eq!(rps_for_res(&RpsResult(SCISSORS, DRAW_SCORE)), SCISSORS);
        assert_eq!(rps_for_res(&RpsResult(ROCK, WIN_SCORE)), PAPER);
        assert_eq!(rps_for_res(&RpsResult(PAPER, WIN_SCORE)), SCISSORS);
        assert_eq!(rps_for_res(&RpsResult(SCISSORS, WIN_SCORE)), ROCK);
    }

    #[test]
    fn test_challenge_score_res() {
        assert_eq!(
            challenge_score_res((RPS::Rock, RPSResult::LOSS)),
            RPSResult::LOSS.value() + score(RPS::Scissors)
        );
        assert_eq!(
            result_score(&RpsResult(ROCK, LOSS_SCORE)),
            LOSS_SCORE + SCISSORS_SCORE
        );
        assert_eq!(
            challenge_score_res((RPS::Scissors, RPSResult::WIN)),
            RPSResult::WIN.value() + score(RPS::Rock)
        );
        assert_eq!(
            result_score(&RpsResult(SCISSORS, WIN_SCORE)),
            WIN_SCORE + ROCK_SCORE
        );
        assert_eq!(
            challenge_score_res((RPS::Paper, RPSResult::WIN)),
            RPSResult::WIN.value() + score(RPS::Scissors)
        );
        assert_eq!(
            result_score(&RpsResult(PAPER, WIN_SCORE)),
            WIN_SCORE + SCISSORS_SCORE
        );

        assert_eq!(
            challenge_score_res((RPS::Paper, RPSResult::DRAW)),
            RPSResult::DRAW.value() + score(RPS::Paper)
        );
        assert_eq!(
            result_score(&RpsResult(PAPER, DRAW_SCORE)),
            DRAW_SCORE + PAPER_SCORE
        );
    }
}
