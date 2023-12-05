use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, space1, u64},
    combinator::eof,
    multi::{many1, separated_list1},
    sequence::{pair, separated_pair, terminated},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Data {
    a: u64,
    b: u64,
    size: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    data: Data,
    l: Option<Box<Node>>,
    r: Option<Box<Node>>,
}

impl Node {
    pub fn new(data: Data) -> Self {
        Node {
            data,
            l: None,
            r: None,
        }
    }

    pub fn find_data(&self, a: u64) -> Option<&Data> {
        if a >= self.data.a && a < self.data.a + self.data.size {
            Some(&self.data)
        } else if a < self.data.a && self.l.is_some() {
            self.l.as_ref().unwrap().find_data(a)
        } else if a >= self.data.a + self.data.size && self.r.is_some() {
            self.r.as_ref().unwrap().find_data(a)
        } else {
            None
        }
    }

    pub fn find(&self, a: u64) -> u64 {
        if let Some(data) = self.find_data(a) {
            data.b + a - data.a
        } else {
            a
        }
    }
    /*
        pub fn find_range(&self, range: (u64, u64)) -> Vec<(u64, u64)> {
            let mut a = range.0;
            let end = a + range.1;
            let mut res = vec![];
            while a < end {
                let data = self.find_data(a);
            }
        }
    */
    pub fn insert(&mut self, data: &Data) {
        let target_node = if self.data.a < data.a && self.data.a + self.data.size <= data.a {
            &mut self.r
        } else if self.data.a > data.a && self.data.a >= data.a + data.size {
            &mut self.l
        } else {
            panic!("Overlapping nodes: {:?} <=> {:?}", self.data, data);
        };

        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(data),
            &mut None => *target_node = Some(Box::new(Node::new(*data))),
        }
    }
}

fn data_line(input: &str) -> IResult<&str, Data> {
    // 2702707184 1771488746 32408643
    let (input, values) = terminated(separated_list1(space1, u64), line_ending)(input)?;
    Ok((
        input,
        Data {
            a: values[1],
            b: values[0],
            size: values[2],
        },
    ))
}

fn data_tree(input: &str) -> IResult<&str, Option<Node>> {
    let mut i = input;
    let mut tree: Option<Node> = None;
    while let Ok((input, data)) = data_line(i) {
        if let Some(ref mut t) = tree {
            t.insert(&data);
        } else {
            tree = Some(Node::new(data));
        }
        i = input;
    }
    Ok((i, tree))
}

fn seeds(input: &str) -> IResult<&str, Vec<u64>> {
    println!("Seeds input: {:?}", &input[..10]);
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(tag(" "), u64)(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, seeds))
}

fn seed_ranges(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(tag(" "), separated_pair(u64, tag(" "), u64))(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, seeds))
}

fn map_name(input: &str) -> IResult<&str, &str> {
    let (input, name) = take_until(" ")(input)?;
    let (input, _) = tag(" map:\n")(input)?;
    Ok((input, name))
}

fn parse_maps(input: &str) -> IResult<&str, Vec<(&str, Option<Node>)>> {
    //let (input, _) = line_ending(input)?;
    let (input, trees) = many1(terminated(
        pair(map_name, data_tree),
        alt((line_ending, eof)),
    ))(input)?;
    Ok((input, trees))
}

fn parse_seed_maps(input: &str) -> IResult<&str, (Vec<u64>, Vec<(&str, Option<Node>)>)> {
    let (input, seeds) = seeds(input)?;
    let (input, _) = line_ending(input)?;
    let (input, res) = parse_maps(input)?;
    Ok((input, (seeds, res)))
}

fn parse_seed_ranges_maps(
    input: &str,
) -> IResult<&str, (Vec<(u64, u64)>, Vec<(&str, Option<Node>)>)> {
    let (input, seeds) = seed_ranges(input)?;
    let (input, _) = line_ending(input)?;
    let (input, res) = parse_maps(input)?;
    Ok((input, (seeds, res)))
}

fn solve_part1(input: &str) -> u64 {
    let (_, (seeds, trees)) = parse_seed_maps(input).expect("Failed to parse input");
    seeds
        .iter()
        .map(|seed| {
            trees
                .iter()
                .fold(*seed, |acc, (_name, tree)| tree.as_ref().unwrap().find(acc))
        })
        .min()
        .unwrap()
}

fn solve_part2(input: &str) -> u64 {
    let (_, (seeds, trees)) = parse_seed_ranges_maps(input).expect("Failed to parse input");
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
        let (input, (seeds, trees)) = parse_seed_maps(TEST_INPUT).expect("Failed to parse input");
        println!("Remaining input: {:?}", input);
        assert_eq!(seeds, vec![79, 14, 55, 13]);
        assert_eq!(trees.len(), 7);
    }

    #[test]
    fn test_data_line() {
        assert_eq!(
            data_line("2702707184 1771488746 32408643\n"),
            Ok((
                "",
                Data {
                    a: 2702707184,
                    b: 1771488746,
                    size: 32408643
                }
            ))
        );
    }
}
