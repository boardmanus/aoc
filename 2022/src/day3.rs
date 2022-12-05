use crate::aoc;

#[derive(Debug)]
enum Error {
    BadNumItems,
}

#[derive(Debug)]
struct RuckSack<'a> {
    a: &'a str,
    b: &'a str,
}

fn rucksack_from_str<'a>(item_str: &'a str) -> Result<RuckSack<'a>, Error> {
    if item_str.len() & 1 != 0 {
        return Err(Error::BadNumItems);
    }
    let num_items = item_str.len() / 2;
    let a: &str = &item_str[0..num_items];
    let b: &str = &item_str[num_items..item_str.len()];
    Ok(RuckSack::<'a> { a, b })
}

fn dup_item_str(a: &str, b: &str) -> Option<char> {
    for c in a.chars() {
        if b.find(c).is_some() {
            return Some(c);
        }
    }
    None
}

fn dup_item(rucksack: &RuckSack) -> Option<char> {
    dup_item_str(rucksack.a, rucksack.b)
}

fn item_priority(item: char) -> u32 {
    const LC_A_VALUE: u32 = 'a' as u32;
    const UC_A_VALUE: u32 = 'A' as u32;
    let val = item as u32;
    if val >= LC_A_VALUE {
        val - LC_A_VALUE + 1
    } else {
        val - UC_A_VALUE + 27
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
    fn solve(&self, lines: &Vec<String>) -> String {
        lines
            .iter()
            .flat_map(|line| rucksack_from_str(line))
            .flat_map(|rucksack| dup_item(&rucksack))
            .map(|item| item_priority(item))
            .sum::<u32>()
            .to_string()
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

fn common_item(group: &ElfGroup) -> Option<char> {
    let a = &group.rucksacks[0];
    let b = &group.rucksacks[1];
    let c = &group.rucksacks[2];
    for ci in a.chars() {
        if b.find(ci).is_some() && c.find(ci).is_some() {
            return Some(ci);
        }
    }
    None
}
pub struct Day3_2;
impl aoc::Aoc<u32> for Day3_2 {
    fn day(&self) -> u32 {
        3
    }
    fn puzzle_name(&self) -> &str {
        "Rucksack Card Sum"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        lines
            .chunks(3)
            .map(|chunk| ElfGroup::new(chunk))
            .flat_map(|group| common_item(&group))
            .map(|item| item_priority(item))
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rucksack_from_str() {
        let r = rucksack_from_str("abcdEFGH").unwrap();
        assert_eq!(r.a, "abcd");
        assert_eq!(r.b, "EFGH");
        assert!(rucksack_from_str("abcdABC").is_err());
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
            a: "abcd",
            b: "ABdC",
        };
        assert_eq!(dup_item(&r), Some('d'));
    }
}
