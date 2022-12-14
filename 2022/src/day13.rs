use std::{cmp::Ordering, str::FromStr};

use crate::aoc::Aoc;
use itertools::Itertools;
use nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map_res,
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

fn lines_to_packet_pairs(lines: &[String]) -> Vec<(Packet, Packet)> {
    lines
        .chunks(3)
        .map(|chunk| {
            (
                parse_packet(&chunk[0]).unwrap().1,
                parse_packet(&chunk[1]).unwrap().1,
            )
        })
        .collect_vec()
}

fn lines_to_packets(lines: &[String]) -> Vec<Packet> {
    lines
        .iter()
        .flat_map(|line| parse_packet(line))
        .map(|pkt| pkt.1)
        .collect_vec()
}

fn compare_pkts(p1: &Packet, p2: &Packet) -> Ordering {
    match (p1, p2) {
        (Arr(a), Num(n)) => compare_pkts(p1, &Arr(vec![Num(*n)])),
        (Num(n), Arr(a)) => compare_pkts(&Arr(vec![Num(*n)]), p2),
        (Num(a), Num(b)) => a.cmp(b),
        (Arr(a), Arr(b)) => {
            for i in 0..a.len() {
                if i >= b.len() {
                    return Ordering::Greater;
                }
                let res = compare_pkts(&a[i], &b[i]);
                if res != Ordering::Equal {
                    return res;
                }
            }
            a.len().cmp(&b.len())
        }
    }
}

pub struct Day13_1;
impl Aoc for Day13_1 {
    fn day(&self) -> u32 {
        13
    }
    fn puzzle_name(&self) -> &str {
        "Distress Signal"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        lines_to_packet_pairs(&lines)
            .iter()
            .enumerate()
            .filter(|pair| pair.1 .0.cmp(&pair.1 .1) == Ordering::Less)
            .fold(0, |s, pair| s + pair.0 + 1)
            .to_string()
    }
}

pub struct Day13_2;
impl Aoc for Day13_2 {
    fn day(&self) -> u32 {
        13
    }
    fn puzzle_name(&self) -> &str {
        "Distress Signal 2"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        let two = parse_packet("[[2]]").unwrap().1;
        let six = parse_packet("[[6]]").unwrap().1;
        let mut pkts = lines_to_packets(lines);
        pkts.push(two.clone());
        pkts.push(six.clone());

        pkts.iter()
            .sorted_by(|a, b| compare_pkts(*a, *b))
            .enumerate()
            .filter(|p| *p.1 == two || *p.1 == six)
            .map(|p| p.0 + 1)
            .product::<usize>()
            .to_string()
    }
}

use crate::day13::Packet::{Arr, Num};

#[derive(PartialOrd, PartialEq, Eq, Debug, Clone)]
enum Packet {
    Num(usize),
    Arr(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, p2: &Packet) -> Ordering {
        compare_pkts(self, p2)
    }
}

fn num_packet(i: &str) -> Result<Packet, <usize as FromStr>::Err> {
    let num = i.parse::<usize>()?;
    Ok(Packet::Num(num))
}

fn parse_packet(s: &str) -> IResult<&str, Packet> {
    let p = delimited(
        char('['),
        separated_list0(tag(","), alt((map_res(digit1, num_packet), parse_packet))),
        char(']'),
    )(s)?;

    Ok((p.0, Arr(p.1)))
}

#[cfg(test)]
mod tests {
    use crate::aoc::as_vstrings;

    use super::*;

    const INPUT: [&str; 24] = [
        "[1,1,3,1,1]",
        "[1,1,5,1,1]",
        "",
        "[[1],[2,3,4]]",
        "[[1],4]",
        "",
        "[9]",
        "[[8,7,6]]",
        "",
        "[[4,4],4,4]",
        "[[4,4],4,4,4]",
        "",
        "[7,7,7,7]",
        "[7,7,7]",
        "",
        "[]",
        "[3]",
        "",
        "[[[]]]",
        "[[]]",
        "",
        "[1,[2,[3,[4,[5,6,7]]]],8,9]",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]",
        "",
    ];

    #[test]
    fn test_soln() {
        let input_strs = as_vstrings(&INPUT[0..]);
        assert_eq!(Day13_1.solve(&input_strs), 13.to_string());
    }

    #[test]
    fn test_soln2() {
        let input_strs = as_vstrings(&INPUT[0..]);
        assert_eq!(Day13_2.solve(&input_strs), 140.to_string());
    }

    #[test]
    fn test_packet_ordering() {
        assert_eq!(
            Arr(vec![Arr(Default::default())]).cmp(&Arr(vec![Arr(vec![Arr(Default::default())])])),
            Ordering::Less
        );
        assert_eq!(
            Arr(vec![Arr(Default::default())]).cmp(&Arr(vec![Num(1)])),
            Ordering::Less
        );
    }

    #[test]
    fn test_packet_equality() {
        let a = Arr(vec![Num(1), Num(2), Arr(vec![Num(3)])]);
        let b = Arr(vec![Num(1), Num(2), Arr(vec![Num(3)])]);
        let c = Arr(vec![Num(1), Num(2), Arr(vec![Num(3), Num(4)])]);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_small_parse() {
        let i = as_vstrings(&["[[1],[2,3,4]]", "[[1],4]", ""]);
        let i = as_vstrings(&["[[4,4],4,4]", "[[4,4],4,4,4]", ""]);
        assert_eq!(Day13_1.solve(&i), 1.to_string());
    }
    #[test]
    fn test_parse_packet() {
        assert_eq!(
            parse_packet("[12]"),
            Ok(("", Packet::Arr(vec![Packet::Num(12)])))
        );

        let res = Arr(vec![
            Num(12),
            Arr(vec![Num(1), Num(2)]),
            Arr(vec![Arr(Default::default())]),
        ]);
        assert_eq!(parse_packet("[12,[1,2],[[]]]"), Ok(("", res)));
        let res = Packet::Arr(vec![Num(12), Arr(vec![Num(1), Num(2)])]);
        assert_eq!(parse_packet("[12,[1,2]]"), Ok(("", res)));
    }
}
