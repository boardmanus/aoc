use std::ops::Range;

use iter_tools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Data {
    seeds: Range<u64>,
    adjust: i64,
}

impl Data {
    fn new(seeds: Range<u64>, adjust: i64) -> Self {
        Self { seeds, adjust }
    }

    fn from_tup(t: (u64, u64, u64)) -> Self {
        Self {
            seeds: t.1..(t.1 + t.2),
            adjust: (t.0 as i64) - (t.1 as i64),
        }
    }

    fn from_vec(v: Vec<u64>) -> Self {
        Self {
            seeds: v[1]..(v[1] + v[2]),
            adjust: (v[0] as i64) - (v[1] as i64),
        }
    }
}

fn parse_map_range(input: &str) -> Data {
    // 2702707184 1771488746 32408643
    Data::from_vec(
        input
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect(),
    )
}

fn parse_map(input: &str) -> (&str, Vec<Data>) {
    let mut lines = input.lines();
    let name = lines.next().unwrap().split_whitespace().next().unwrap();
    let mut v = lines.map(parse_map_range).collect::<Vec<_>>();
    v.sort_by(|a, b| a.seeds.start.cmp(&b.seeds.start));
    (name, v)
}

fn parse_seeds(line: &str) -> Vec<u64> {
    line.split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn parse_seed_ranges(line: &str) -> Vec<Range<u64>> {
    let mut v = line
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .chunks(2)
        .into_iter()
        .map(|mut a| {
            let val = a.next().unwrap();
            let size = a.next().unwrap();
            val..(val + size)
        })
        .collect::<Vec<_>>();
    v.sort_by(|a, b| a.start.cmp(&b.start));
    v
}

fn parse_ranges(input: &str) -> (Vec<Range<u64>>, Vec<(&str, Vec<Data>)>) {
    let mut groups = input.split("\n\n");
    let seeds = parse_seed_ranges(groups.next().unwrap());
    (seeds, groups.map(parse_map).collect())
}

fn parse(input: &str) -> (Vec<u64>, Vec<(&str, Vec<Data>)>) {
    let mut groups = input.split("\n\n");
    let seeds = parse_seeds(groups.next().unwrap());
    (seeds, groups.map(parse_map).collect())
}

fn solve_part1(input: &str) -> u64 {
    let (seeds, maps) = parse(input);
    seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed, |acc, (name, map)| {
                for data in map.iter() {
                    if acc < data.seeds.start {
                        println!("{name}: {acc} < {:?} => no mapping", data.seeds);
                        return acc;
                    } else if data.seeds.contains(&acc) {
                        println!("{name}: {acc} in {:?} => {}", data.seeds, data.adjust);
                        return (acc as i64 + data.adjust) as u64;
                    }
                }
                println!(
                    "{name}: {acc} > {:?} => no mapping",
                    map.last().unwrap().seeds
                );
                acc
            })
        })
        .min()
        .unwrap()
}

fn solve_part2(input: &str) -> u64 {
    /*
    let (seed_ranges, maps) = parse_ranges(input);
    seeds
        .iter()
        .map(|seed_range| {
            (seed_range.0..seed_range.0 + seed_range.1)
                .map(|seed| {
                    trees
                        .iter()
                        .fold(seed, |acc, (_name, tree)| tree.as_ref().unwrap().find(acc))
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
        */
    0
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let part1 = solve_part1(INPUT);
    println!("Part1: {part1}");
    let part2 = solve_part2(INPUT);
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
    const TEST_INPUT_2: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 46);
    }

    #[test]
    fn test_parser() {
        let (seeds, maps) = parse(TEST_INPUT);
        assert_eq!(seeds, vec![79, 14, 55, 13]);
        assert_eq!(maps.len(), 7);
    }

    #[test]
    fn test_parse_map_range() {
        assert_eq!(
            parse_map_range("2702707184 1771488746 32408643"),
            Data::from_tup((2702707184, 1771488746, 32408643))
        );
    }
}
