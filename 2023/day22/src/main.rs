use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
enum ParseError {
    Block,
    Point,
    Int(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError::Int(err)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().ok_or(ParseError::Point)?.parse::<u32>()?;
        let y = parts.next().ok_or(ParseError::Point)?.parse::<u32>()?;
        let z = parts.next().ok_or(ParseError::Point)?.parse::<u32>()?;
        Ok(Point { x, y, z })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Block {
    a: Point,
    b: Point,
}

impl FromStr for Block {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('~');
        let a_str = parts.next().ok_or(ParseError::Block)?;
        let a = Point::from_str(a_str)?;
        let b_str = parts.next().ok_or(ParseError::Block)?;
        let b = Point::from_str(b_str)?;
        Ok(Block { a, b })
    }
}

struct Column {
    blocks: Vec<Block>,
}

impl FromStr for Column {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = s
            .lines()
            .map(|block| Block::from_str(block))
            .collect::<Result<Vec<Block>, _>>()?;

        blocks.sort_by(|a, b| a.a.z.min(a.b.z).cmp(&b.a.z.min(b.b.z)));

        Ok(Column { blocks })
    }
}

fn solve_part1(input: &str) -> usize {
    let column = Column::from_str(input).unwrap();
    0
}

fn solve_part2(input: &str) -> u64 {
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
        assert_eq!(solve_part1(TEST_INPUT), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 467835);
    }

    #[test]
    fn test_parse_block() {
        assert_eq!(
            Block::from_str("1,2,3~4,5,6").unwrap(),
            Block {
                a: Point { x: 1, y: 2, z: 3 },
                b: Point { x: 4, y: 5, z: 6 }
            }
        );
    }

    #[test]
    fn test_parse_column() {
        assert_eq!(
            Column::from_str("1,2,7~4,5,7\n1,2,6~4,5,6\n1,2,8~1,2,5")
                .unwrap()
                .blocks,
            vec![
                Block {
                    a: Point { x: 1, y: 2, z: 8 },
                    b: Point { x: 1, y: 2, z: 5 }
                },
                Block {
                    a: Point { x: 1, y: 2, z: 6 },
                    b: Point { x: 4, y: 5, z: 6 }
                },
                Block {
                    a: Point { x: 1, y: 2, z: 7 },
                    b: Point { x: 4, y: 5, z: 7 }
                }
            ]
        );
    }
}
