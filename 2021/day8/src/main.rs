use std::collections::HashMap;

use lazy_static::lazy_static;

const DIGIT_STRS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

trait ToHashMap: Iterator {
    fn to_hashmap<K, V>(
        self,
        fkey: impl Fn(&Self::Item) -> K,
        fval: impl Fn(&Self::Item) -> V,
    ) -> HashMap<K, V>
    where
        K: std::hash::Hash + Eq + Clone,
        Self: Sized,
    {
        self.fold(HashMap::default(), |mut acc, t| {
            acc.insert(fkey(&t), fval(&t));
            acc
        })
    }
}

impl<I: Iterator> ToHashMap for I {}

lazy_static! {
    static ref BITS_TO_DIGIT: HashMap<u8, u8> = DIGIT_STRS.iter().enumerate().to_hashmap(|x| str_to_bits(x.1), |x| x.0 as u8);
    static ref NUM_SEGS: HashMap<u8, usize> = DIGIT_STRS.iter().enumerate().to_hashmap(|x| x.0 as u8, |x| x.1.len());
    static ref SEG_BIT: HashMap<char, u8> = "abcdefg".chars().to_hashmap(|c| *c, |c| 1 << (*c as u8 - 'a' as u8));
    static ref BIT_SEG: HashMap<u8, char> = (0..8).to_hashmap(|b| *b, |b| ('a' as u8 + *b) as char);

    //static ref DIGIT_BITS: HashMap<usize, u8> = strs_to_bits(&DIGIT);

    static ref SELECT: [usize; 4] = [
        DIGIT_STRS[1].len(),
        DIGIT_STRS[4].len(),
        DIGIT_STRS[7].len(),
        DIGIT_STRS[8].len(),
    ];
}

fn char_to_bit(c: char) -> u8 {
    1 << (c as u8 - 'a' as u8)
}
/*
fn bit_to_char(b: u8, b2c) -> char {
    let mut c = 0u8;
    let mut bit = b;
    while bit != 0 {
        c += 1;
        bit >>= 1;
    }
    (c as u8 + 'a' as u8 - 1) as char
}
*/
fn str_to_bits(s: &str) -> u8 {
    s.chars().fold(0, |bits, c| bits | char_to_bit(c))
}
/*
fn bits_to_str(b: u8) -> String {
    (0..8).fold(String::default(), |mut s, i| {
        let bit = 1 << i;
        if b & bit != 0 {
            s.push(bit_to_char(bit));
        }
        s
    })
}
*/

fn strs_to_bits(strs: &[&str]) -> HashMap<usize, Vec<u8>> {
    strs.iter().fold(HashMap::default(), |mut acc, s| {
        let bits = str_to_bits(s);
        if let Some(v) = acc.get_mut(&s.len()) {
            v.push(bits);
        } else {
            acc.insert(s.len(), vec![bits]);
        }
        acc
    })
}

type InOutStr<'a> = ([&'a str; 10], [&'a str; 4]);

#[derive(Default)]
struct InOut([u8; 10], [u8; 4]);

impl<'a> From<&InOutStr<'a>> for InOut {
    fn from(inoutstr: &InOutStr<'a>) -> Self {
        let mut inout: InOut = Default::default();
        inoutstr
            .0
            .iter()
            .enumerate()
            .for_each(|a| inout.0[a.0] = str_to_bits(a.1));
        inoutstr
            .1
            .iter()
            .enumerate()
            .for_each(|a| inout.1[a.0] = str_to_bits(a.1));
        inout
    }
}

fn parse_line(line: &str) -> InOutStr {
    let mut inout: InOutStr = Default::default();
    line.split(' ').enumerate().for_each(|a| {
        if a.0 < 10 {
            inout.0[a.0] = a.1;
        } else if a.0 > 10 {
            inout.1[a.0 - 11] = a.1;
        }
    });
    inout
}

fn solve(io: &HashMap<usize, Vec<u8>>) -> HashMap<u8, u8> {
    let mut num_map: HashMap<u8, u8> = Default::default();
    let mut seg_map: HashMap<char, u8> = Default::default();

    io.iter().for_each(|x| println!("{} => {:?}", x.0, x.1));
    num_map.insert(1, io[&2][0]);
    num_map.insert(4, io[&4][0]);
    num_map.insert(7, io[&3][0]);
    num_map.insert(8, io[&7][0]);
    num_map.insert(
        6,
        *io[&6].iter().find(|x| (!*x & num_map[&1]) != 0).unwrap(),
    );
    num_map.insert(
        0,
        *io[&6]
            .iter()
            .find(|x| (!*x & num_map[&4] & !num_map[&1]) != 0)
            .unwrap(),
    );

    num_map.insert(
        9,
        *io[&6]
            .iter()
            .find(|x| (!*x & num_map[&4] == 0) && (!*x & num_map[&1] == 0))
            .unwrap(),
    );

    num_map.insert(2, *io[&5].iter().find(|x| *x & !num_map[&9] != 0).unwrap());

    seg_map.insert('a', !num_map[&1] & num_map[&7]);
    seg_map.insert('c', num_map[&1] & !num_map[&6]);
    seg_map.insert('d', num_map[&4] & !num_map[&0]);
    seg_map.insert('e', num_map[&8] & !num_map[&9]);
    seg_map.insert('f', num_map[&1] & !num_map[&2]);
    seg_map.insert('b', num_map[&4] & !num_map[&1] & num_map[&0]);
    seg_map.insert('g', num_map[&0] & !num_map[&4] & !num_map[&7] & num_map[&9]);

    println!("segmap: {:?}", seg_map);
    println!("nummap: {:?}", num_map);

    num_map.insert(
        5,
        *io[&5]
            .iter()
            .find(|x| (!*x & num_map[&9]) == seg_map[&'c'])
            .unwrap(),
    );

    num_map.insert(
        3,
        *io[&5]
            .iter()
            .find(|x| (!*x & num_map[&9]) == seg_map[&'b'])
            .unwrap(),
    );

    num_map.iter().fold(HashMap::default(), |mut acc, b| {
        acc.insert(*b.1, *b.0);
        acc
    })
}

fn parse(input: &str) -> Vec<InOutStr> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| parse_line(l))
        .collect()
}

fn solve_part1(inout: &[InOutStr]) -> usize {
    inout
        .iter()
        .flat_map(|io| io.1)
        .filter(|d| SELECT.iter().any(|x| *x == d.len()))
        .count()
}

fn solve_part2(inoutstr: &[InOutStr]) -> usize {
    inoutstr
        .iter()
        .map(|io| {
            let m = solve(&strs_to_bits(&io.0));
            io.1.iter()
                .map(|s| {
                    let bits = str_to_bits(s);
                    let val = m[&bits];
                    println!("{s} => {val}");
                    val
                })
                .fold(0usize, |acc, d| acc * 10 + (d as usize))
        })
        .sum()
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let inout = parse(INPUT);
    let part1 = solve_part1(&inout);
    println!("Part1: {part1}");
    let part2 = solve_part2(&inout);
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let segments = parse(TEST_INPUT);
        assert_eq!(solve_part1(&segments), 26);
    }

    #[test]
    fn test_part2() {
        let segments = parse(TEST_INPUT);
        assert_eq!(solve_part2(&segments), 61229);
    }

    #[test]
    fn test_parse() {
        let segments = parse_line(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        assert_eq!(
            segments,
            (
                [
                    "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb",
                    "cagedb", "ab"
                ],
                ["cdfeb", "fcadb", "cdfeb", "cdbaf"]
            )
        );
    }

    #[test]
    fn test_str_to_bits() {
        assert_eq!(str_to_bits("ab"), 3);
        assert_eq!(str_to_bits("abcdefg"), 0b1111111);
        assert_eq!(str_to_bits("ag"), 0b1000001);
    }

    #[test]
    fn test_solve() {}

    /*
    #[test]
    fn test_bits_to_str() {
        assert_eq!(bits_to_str(0b1), "a");
        assert_eq!(bits_to_str(0b1111111), "abcdefg");
        assert_eq!(bits_to_str(0b1000001), "ag");
    }
    */

    #[test]
    fn test_not() {
        assert_eq!(!0xfu8, 0xf0u8);
    }
}
