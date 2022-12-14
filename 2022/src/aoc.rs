use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

pub trait Aoc {
    fn day(&self) -> u32;
    fn puzzle_name(&self) -> &str;
    fn solve(&self, lines: &[String]) -> String;
    fn input_name(&self) -> String {
        format!("input_day{:}.txt", self.day())
    }
}

pub fn read_lines(fname: &str) -> io::Result<Vec<String>> {
    let input_file = File::open(fname)?;
    let reader = io::BufReader::new(input_file);
    Ok(reader
        .lines()
        .into_iter()
        .map(|line| line.unwrap_or(String::default()))
        .collect())
}

pub fn as_vstrings(strs: &[&str]) -> Vec<String> {
    strs.iter()
        .map(|s| String::from(*s))
        .into_iter()
        .collect::<Vec<String>>()
}

pub fn to_rows(lines: &[String]) -> Vec<Vec<usize>> {
    lines
        .iter()
        .map(|line| line.chars().map(|c| c as usize).collect_vec())
        .collect_vec()
}
