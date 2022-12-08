use itertools::Itertools;
use std::collections::HashMap;

use crate::aoc::Aoc;

pub struct Day8_1;
impl Aoc for Day8_1 {
    fn day(&self) -> u32 {
        8
    }
    fn puzzle_name(&self) -> &str {
        "???"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        Default::default()
    }
}

pub struct Day8_2;
impl Aoc for Day8_2 {
    fn day(&self) -> u32 {
        8
    }
    fn puzzle_name(&self) -> &str {
        "??? 2"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use lazy_static::lazy_static;

    const INPUT: [&str; 23] = [
        "$ cd /",
        "$ ls",
        "dir a",
        "14848514 b.txt",
        "8504156 c.dat",
        "dir d",
        "$ cd a",
        "$ ls",
        "dir e",
        "29116 f",
        "2557 g",
        "62596 h.lst",
        "$ cd e",
        "$ ls",
        "584 i",
        "$ cd ..",
        "$ cd ..",
        "$ cd d",
        "$ ls",
        "4060174 j",
        "8033020 d.log",
        "5626152 d.ext",
        "7214296 k",
    ];

    lazy_static! {
        static ref CMD_LINES: Vec<String> = vec![
            String::from("$ cd /"),
            String::from("$ ls"),
            String::from("dir a"),
            String::from("$ cd a"),
            String::from("$ ls"),
            String::from("1 b"),
            String::from("2 c"),
        ];
    }
    #[test]
    fn test_soln() {
        let input_strs = INPUT
            .map(|s| String::from(s))
            .into_iter()
            .collect::<Vec<String>>();

        assert_eq!(Day8_1.solve(&input_strs), 95437.to_string());
    }

    #[test]
    fn test_soln2() {
        let input_strs = INPUT
            .map(|s| String::from(s))
            .into_iter()
            .collect::<Vec<String>>();

        assert_eq!(Day8_2.solve(&input_strs), 24933642.to_string());
    }
}
