use std::ops::RangeInclusive;

pub fn part1(input: &str) -> usize {
    let pairs = parse_input(input);
    pairs.iter().map(|range| double_range(range.clone())).sum()
}

pub fn part2(input: &str) -> usize {
    let pairs = parse_input(input);
    pairs.iter().map(|range| dups_in_range(range.clone())).sum()
}

fn dups_in_range(range: RangeInclusive<usize>) -> usize {
    range.map(|v| if is_dup(v) { v } else { 0 }).sum()
}

fn double_range(range: RangeInclusive<usize>) -> usize {
    range.map(|v| if is_double(v) { v } else { 0 }).sum()
}

#[allow(dead_code)]
fn is_dup_s(v: usize) -> bool {
    let s = v.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();
    (1..=len / 2)
        .filter(|&dup_len| len.is_multiple_of(dup_len))
        .any(|dup_len| {
            let seg = &bytes[..dup_len];
            bytes.chunks(dup_len).all(|chunk| chunk == seg)
        })
}

#[allow(dead_code)]
fn is_double_s(v: usize) -> bool {
    let s = v.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();
    bytes[..len / 2] == bytes[len / 2..]
}

fn is_dup(v: usize) -> bool {
    let num_digits = (v as f64).log10() as usize + 1;
    (1..=num_digits / 2)
        .filter(|&dup_len| num_digits.is_multiple_of(dup_len))
        .any(|dup_len| {
            let d = 10_usize.pow(dup_len as u32);
            let seg = v % d;
            (0..num_digits / dup_len - 1).all(|i| (v / d.pow((i + 1) as u32)) % d == seg)
        })
}

fn is_double(v: usize) -> bool {
    let num_digits = (v as f64).log10() as u32 + 1;
    if num_digits % 2 == 1 {
        return false;
    }
    let d = (10usize).pow(num_digits / 2);
    (v / d) == (v % d)
}

fn parse_input(input: &str) -> Vec<RangeInclusive<usize>> {
    input
        .trim()
        .split(',')
        .filter_map(|pair| {
            let (a, b) = pair.split_once('-')?;
            Some(a.parse::<usize>().ok()?..=b.parse::<usize>().ok()?)
        })
        .collect()
}

const INPUT: &str = include_str!("data/input");
fn main() {
    aoc_utils::run::main(INPUT, part1, part2);
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 1227775554;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 4174379265;

    #[test]
    fn test_dups_in_range() {
        (11usize..=22)
            .filter(|v| is_dup(*v))
            .for_each(|x| println!("{x}"));
    }

    #[test]
    fn test_is_dup() {
        assert!(!is_dup(12));
        assert!(is_dup(11));
        assert!(is_dup(1212));
        assert!(is_dup(121212));
        assert!(is_dup(111111));
        assert!(is_dup(1234512345));
        assert!(is_dup(123123123));
        assert!(!is_double(1212121));
        assert!(!is_double(121213));
    }

    #[test]
    fn test_is_double() {
        assert!(is_double(11));
        assert!(is_double(1212));
        assert!(!is_double(4567456))
    }

    #[test]
    fn test_double_range() {
        assert_eq!(double_range(10..=12), 11);
        assert_eq!(double_range(2220..=2222), 2222);
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
