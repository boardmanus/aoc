use std::{ops::Add, slice::Chunks, str::FromStr};

use crate::aoc;

#[derive(Debug)]
enum Error {
    BadNumItems,
}

#[derive(Debug)]
struct RuckSack {
    a: String,
    b: String,
}

impl FromStr for RuckSack {
    type Err = Error;
    fn from_str(item_str: &str) -> Result<Self, Error> {
        if item_str.len() & 1 != 0 {
            return Err(Error::BadNumItems);
        }
        let num_items = item_str.len() / 2;
        let a = String::from(&item_str[0..num_items]);
        let b = String::from(&item_str[num_items..item_str.len()]);
        Ok(RuckSack { a, b })
    }
}

fn dup_item_str(a: &String, b: &String) -> char {
    for c in a.chars() {
        if b.find(c).is_some() {
            return c;
        }
    }
    panic!()
}

fn dup_item(rucksack: &RuckSack) -> char {
    dup_item_str(&rucksack.a, &rucksack.b)
}

fn item_priority(item: char) -> u32 {
    const a_value: u32 = 'a' as u32;
    const A_value: u32 = 'A' as u32;
    let val = item as u32;
    if val >= a_value {
        val - a_value + 1
    } else {
        val - A_value + 27
    }
}

pub struct Day3_1;
impl aoc::Aoc<u32> for Day3_1 {
    fn day(&self) -> u32 {
        3
    }
    fn puzzle_name(&self) -> &str {
        "Rucksack Priority Sum"
    }
    fn solve(&self, lines: &Vec<String>) -> u32 {
        lines
            .iter()
            .flat_map(|line| RuckSack::from_str(line))
            .map(|rucksack| dup_item(&rucksack))
            .map(|item| item_priority(item))
            .sum()
    }
}

struct ElfGroup {
    rucksacks: Vec<String>,
}

impl ElfGroup {
    fn new(chunks: &[String]) -> Self {
        ElfGroup {
            rucksacks: chunks.to_vec(),
        }
    }
}

fn common_item(group: &ElfGroup) -> char {
    let a = &group.rucksacks[0];
    let b = &group.rucksacks[1];
    let c = &group.rucksacks[2];
    for ci in a.chars() {
        if b.find(ci).is_some() && c.find(ci).is_some() {
            return ci;
        }
    }
    panic!()
}
pub struct Day3_2;
impl aoc::Aoc<u32> for Day3_2 {
    fn day(&self) -> u32 {
        3
    }
    fn puzzle_name(&self) -> &str {
        "Rucksack Priority Sum"
    }
    fn solve(&self, lines: &Vec<String>) -> u32 {
        lines
            .chunks(3)
            .map(|chunk| ElfGroup::new(chunk))
            .map(|group| common_item(&group))
            .map(|item| item_priority(item))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rucksack_from_str() {
        let r = RuckSack::from_str("abcdEFGH").unwrap();
        assert_eq!(r.a, "abcd");
        assert_eq!(r.b, "EFGH");
        assert!(RuckSack::from_str("abcdABC").is_err());
    }

    #[test]
    fn test_item_priotiry() {
        assert_eq!(item_priority('a'), 1);
        assert_eq!(item_priority('z'), 26);
        assert_eq!(item_priority('A'), 27);
        assert_eq!(item_priority('Z'), 52);
    }

    #[test]
    fn test_dup_item() {
        let r = RuckSack {
            a: String::from("abcd"),
            b: String::from("ABdC"),
        };
        assert_eq!(dup_item(&r), 'd');
    }
}
