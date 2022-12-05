use std::fs::File;
use std::io::{self, BufRead};

pub trait Aoc<R> {
    fn day(&self) -> u32;
    fn puzzle_name(&self) -> &str;
    fn solve(&self, lines: &Vec<String>) -> String;
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
