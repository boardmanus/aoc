use std::{cmp::Ordering, num::ParseIntError, str::FromStr};

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

impl Point {
    fn new(x: u32, y: u32, z: u32) -> Self {
        Point { x, y, z }
    }
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

fn next_id() -> u64 {
    unsafe {
        static mut ID: u64 = 0;
        ID += 1;
        ID
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Block {
    id: u64,
    a: Point,
    b: Point,
}

impl Block {
    fn new(id: u64, a: Point, b: Point) -> Self {
        let id = id;
        if a.z <= b.z {
            Block { id, a, b }
        } else {
            Block { id, a: b, b: a }
        }
    }

    fn sits_over(&self, other: &Block) -> bool {
        other.a.x.min(other.b.x) <= self.a.x.max(self.b.x)
            && other.a.x.max(other.b.x) >= self.a.x.min(self.b.x)
            && other.a.y.min(other.b.y) <= self.a.y.max(self.b.y)
            && other.a.y.max(other.b.y) >= self.a.y.min(self.b.y)
            && other.a.z > self.a.z
    }

    fn sits_on(&self, other: &Block) -> bool {
        other.a.x.min(other.b.x) <= self.a.x.max(self.b.x)
            && other.a.x.max(other.b.x) >= self.a.x.min(self.b.x)
            && other.a.y.min(other.b.y) <= self.a.y.max(self.b.y)
            && other.a.y.max(other.b.y) >= self.a.y.min(self.b.y)
            && other.a.z + 1 == self.a.z
    }
}
impl FromStr for Block {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('~');
        let a_str = parts.next().ok_or(ParseError::Block)?;
        let a = Point::from_str(a_str)?;
        let b_str = parts.next().ok_or(ParseError::Block)?;
        let b = Point::from_str(b_str)?;
        let id = next_id();
        Ok(Block::new(id, a, b))
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.a.z.cmp(&other.a.z) {
            Ordering::Equal => match self.a.x.cmp(&other.a.x) {
                Ordering::Equal => Some(self.a.y.cmp(&other.a.y)),
                x => Some(x),
            },
            z => Some(z),
        }
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(x) => x,
            None => Ordering::Equal,
        }
    }
}

struct Column {
    blocks: Vec<Block>,
}

impl Column {
    fn fall(&mut self) {
        let fall_dist = self.blocks[0].a.z;
        self.blocks[0].a.z = 0;
        self.blocks[0].b.z -= fall_dist;

        (0..self.blocks.len()).for_each(|i| {
            if let Some(lo) = (0..i)
                .rev()
                .find(|j| self.blocks[i].sits_over(&self.blocks[*j]))
            {
                self.blocks[i].a.z = self.blocks[lo].a.z + 1;
                self.blocks[i].b.z -= self.blocks[lo].b.z - self.blocks[lo].a.z;
            }
        });
    }

    fn disintegrate(&mut self) -> usize {
        self.fall();

        self.blocks
            .iter()
            .enumerate()
            .filter(|test| {
                let (i, block) = test;
                let lo = if let Some(lo) = &self.blocks[i + 1..]
                    .iter()
                    .enumerate()
                    .find(|(_, lo_block)| lo_block.a.z == block.a.z + 1)
                {
                    *lo
                } else {
                    // nothing above
                    return true;
                };
                let hi = if let Some(hi) = &self.blocks[lo.0..]
                    .iter()
                    .enumerate()
                    .find(|(_, hi_block)| hi_block.a.z > block.b.z + 1)
                {
                    *hi
                } else {
                    lo
                };
                let lo_supp = if let Some(lo) = &self.blocks[0..*i]
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, lo_block)| lo_block.b.z < block.a.z)
                {
                    *lo
                } else {
                    *test
                };
                self.blocks[lo.0..=hi.0]
                    .iter()
                    .enumerate()
                    .all(|(_, block)| {
                        let num = &self.blocks[lo_supp.0..*i]
                            .iter()
                            .filter(|lo_block| block.sits_on(*lo_block))
                            .count();
                        *num > 1
                    })
            })
            .count()
    }
}
impl FromStr for Column {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = s
            .lines()
            .map(|block| Block::from_str(block))
            .collect::<Result<Vec<Block>, _>>()?;

        blocks.sort_by(|a, b| a.cmp(b));

        Ok(Column { blocks })
    }
}

fn solve_part1(input: &str) -> usize {
    let mut column = Column::from_str(input).unwrap();
    column.disintegrate()
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
    fn test_block_sits_on() {
        let a = Block::new(1, Point::new(1, 2, 7), Point::new(4, 5, 7));
        assert!(!a.sits_on(&a));
        let b = Block::new(1, Point::new(1, 2, 6), Point::new(4, 5, 6));
        assert!(a.sits_on(&b));
        let c = Block::new(1, Point::new(1, 2, 5), Point::new(4, 5, 5));
        assert!(!a.sits_on(&c));

        assert!(
            Block::new(1, Point::new(1, 1, 1), Point::new(1, 3, 1)).sits_on(&Block::new(
                2,
                Point::new(1, 1, 0),
                Point::new(3, 1, 0)
            ))
        );
    }

    #[test]
    fn test_parse_block() {
        assert_eq!(
            Block::from_str("1,2,6~4,5,3").unwrap(),
            Block {
                id: 1,
                a: Point { x: 4, y: 5, z: 3 },
                b: Point { x: 1, y: 2, z: 6 }
            }
        );
    }

    #[test]
    fn test_fall() {
        let mut col =
            Column::from_str("1,2,7~4,2,7\n3,3,21~3,3,21\n1,2,6~1,5,6\n1,2,8~1,2,5").unwrap();
        col.fall();
        assert_eq!(
            col.blocks,
            vec![
                Block {
                    id: 2,
                    a: Point { x: 3, y: 3, z: 0 },
                    b: Point { x: 3, y: 3, z: 0 }
                },
                Block {
                    id: 4,
                    a: Point { x: 1, y: 2, z: 0 },
                    b: Point { x: 1, y: 2, z: 3 },
                },
                Block {
                    id: 3,
                    a: Point { x: 1, y: 2, z: 6 },
                    b: Point { x: 1, y: 5, z: 6 }
                },
                Block {
                    id: 1,
                    a: Point { x: 1, y: 2, z: 7 },
                    b: Point { x: 4, y: 2, z: 7 }
                },
            ]
        );
    }
}
