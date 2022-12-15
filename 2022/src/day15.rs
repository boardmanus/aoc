use std::{fmt, fmt::Display, str::FromStr};

use crate::aoc::Aoc;
use itertools::Itertools;
use nom::{
    self, bytes::complete::tag, character::complete::digit1, combinator::map_res,
    multi::separated_list0, sequence::separated_pair, IResult,
};

pub struct Day15_1;
impl Aoc for Day15_1 {
    fn day(&self) -> u32 {
        15
    }
    fn puzzle_name(&self) -> &str {
        "Regolith Reservoir"
    }
    fn solve(&self, lines: &[String]) -> String {
        0.to_string()
    }
}

pub struct Day15_2;
impl Aoc for Day15_2 {
    fn day(&self) -> u32 {
        15
    }
    fn puzzle_name(&self) -> &str {
        "Regolith Reservoir 2"
    }
    fn solve(&self, lines: &[String]) -> String {
        0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::as_vstrings;

    use super::*;

    const INPUT: [&str; 2] = [
        "498,4 -> 498,6 -> 496,6",
        "503,4 -> 502,4 -> 502,9 -> 494,9",
    ];

    #[test]
    fn test_soln() {
        let input_strs = as_vstrings(&INPUT[0..]);
        assert_eq!(Day15_1.solve(&input_strs), 24.to_string());
    }

    #[test]
    fn test_soln2() {
        let input_strs = as_vstrings(&INPUT[0..]);
        assert_eq!(Day15_2.solve(&input_strs), 93.to_string());
    }
}
