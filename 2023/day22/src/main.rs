use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{Display, Formatter},
    num::ParseIntError,
    str::FromStr,
};

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

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
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

    fn xy_overlaps(&self, other: &Block) -> bool {
        other.a.x.min(other.b.x) <= self.a.x.max(self.b.x)
            && other.a.x.max(other.b.x) >= self.a.x.min(self.b.x)
            && other.a.y.min(other.b.y) <= self.a.y.max(self.b.y)
            && other.a.y.max(other.b.y) >= self.a.y.min(self.b.y)
    }

    fn overlaps(&self, other: &Block) -> bool {
        self.xy_overlaps(other) && other.a.z <= self.b.z && other.b.z >= self.a.z
    }

    fn sits_over(&self, other: &Block) -> bool {
        self.xy_overlaps(other) && other.b.z < self.a.z
    }

    fn sits_on(&self, other: &Block) -> bool {
        self.xy_overlaps(other) && other.b.z + 1 == self.a.z
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:4}: {}~{}", self.id, self.a, self.b)
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
        match self.b.z.cmp(&other.b.z) {
            Ordering::Equal => Some(self.id.cmp(&other.id)),
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
        let fall_dist = self.blocks[0].a.z - 1;
        self.blocks[0].a.z = 1;
        self.blocks[0].b.z -= fall_dist;

        (1..self.blocks.len()).for_each(|i| {
            if let Some(lo) = (0..i)
                .filter(|j| self.blocks[i].sits_over(&self.blocks[*j]))
                .map(|j| self.blocks[j].b.z + 1)
                .max()
            {
                let dz = self.blocks[i].b.z - self.blocks[i].a.z;
                self.blocks[i].a.z = lo;
                self.blocks[i].b.z = self.blocks[i].a.z + dz;
            } else {
                // No block below - land on bottom
                let dz = self.blocks[i].a.z - 1;
                self.blocks[i].a.z = 1;
                self.blocks[i].b.z -= dz;
            }
        });

        self.blocks.sort();
    }

    fn disintegratable(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .filter(|&block| {
                self.blocks[block.0 + 1..]
                    .iter()
                    .enumerate()
                    .filter(|&rest_block| rest_block.1.sits_on(block.1))
                    .all(|rest_block| {
                        self.blocks[0..(block.0 + 1 + rest_block.0)]
                            .iter()
                            .rev()
                            .filter(|support_block| rest_block.1.sits_on(*support_block))
                            .count()
                            > 1
                    })
            })
            .count()
    }

    fn chain_reaction(&self) -> usize {
        let chain_map = self.blocks.iter().enumerate().rev().fold(
            HashMap::<u64, usize>::new(),
            |mut map, block| {
                let supported = self.blocks[block.0 + 1..]
                    .iter()
                    .enumerate()
                    .fold(vec![block.1.id], |mut moved, rest_block| {
                        let support_blocks = self.blocks[0..(block.0 + 1 + rest_block.0)]
                            .iter()
                            .rev()
                            .filter(|support_block| rest_block.1.sits_on(*support_block))
                            .collect::<Vec<_>>();

                        if support_blocks.len() > 0
                            && support_blocks
                                .iter()
                                .all(|support_block| moved.contains(&support_block.id))
                        {
                            moved.push(rest_block.1.id);
                        }
                        moved
                    })
                    .len()
                    - 1;
                println!("Supported for {}: {}", block.1, supported);
                map.insert(block.1.id, supported);
                map
            },
        );

        chain_map.values().sum()
    }

    fn chain_reaction_at(&self, id: u64) -> usize {
        let block = self
            .blocks
            .iter()
            .enumerate()
            .find(|b| b.1.id == id)
            .unwrap();

        let supported = self.blocks[block.0 + 1..]
            .iter()
            .enumerate()
            .fold(vec![block.1.id], |mut moved, rest_block| {
                let support_blocks = self.blocks[0..(block.0 + 1 + rest_block.0)]
                    .iter()
                    .rev()
                    .filter(|support_block| rest_block.1.sits_on(*support_block))
                    .collect::<Vec<_>>();

                if support_blocks.len() > 0
                    && support_blocks
                        .iter()
                        .all(|support_block| moved.contains(&support_block.id))
                {
                    moved.push(rest_block.1.id);
                }
                moved
            })
            .len()
            - 1;
        println!("Supported for {}: {}", block.1, supported);

        supported
    }
}

impl Display for Column {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for block in self.blocks.iter().rev() {
            writeln!(f, "{}", block.to_string())?;
        }
        Ok(())
    }
}

impl FromStr for Column {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = s
            .lines()
            .map(|block| Block::from_str(block))
            .collect::<Result<Vec<Block>, _>>()?;

        blocks.sort();

        Ok(Column { blocks })
    }
}

fn solve_part1(input: &str) -> usize {
    let mut column = Column::from_str(input).unwrap();
    column.fall();
    column.disintegratable()
}

fn solve_part2(input: &str) -> usize {
    let mut column = Column::from_str(input).unwrap();
    column.fall();
    column.chain_reaction()
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //let part1 = solve_part1(INPUT);
    //println!("Part1: {part1}");
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
        assert_eq!(solve_part2(TEST_INPUT_2), 7);
    }

    #[test]
    fn test_block_sits_on() {
        let a = Block::new(1, Point::new(1, 2, 7), Point::new(4, 5, 7));
        assert!(!a.sits_on(&a));

        let b = Block::new(1, Point::new(1, 2, 6), Point::new(4, 5, 6));
        assert!(a.sits_on(&b));
        assert!(!b.sits_on(&a));

        let c = Block::new(1, Point::new(1, 2, 5), Point::new(4, 5, 5));
        assert!(!a.sits_on(&c));
        assert!(!c.sits_on(&a));

        let a = Block::new(1, Point::new(1, 1, 1), Point::new(1, 3, 1));
        let b = Block::new(2, Point::new(1, 1, 0), Point::new(3, 1, 0));
        assert!(a.sits_on(&b));
        assert!(!b.sits_on(&a));

        let a = Block::new(1, Point::new(1, 2, 7), Point::new(4, 2, 7));
        let b = Block::new(2, Point::new(1, 2, 6), Point::new(1, 5, 6));
        assert!(a.sits_on(&b));
        assert!(!b.sits_on(&a));
    }

    #[test]
    fn test_block_sits_over() {
        let a = Block::new(1, Point::new(1, 2, 7), Point::new(4, 5, 7));
        assert!(!a.sits_over(&a));
        let b = Block::new(1, Point::new(1, 2, 6), Point::new(4, 5, 6));
        assert!(a.sits_over(&b));
        assert!(!b.sits_over(&a));
        let c = Block::new(1, Point::new(1, 2, 5), Point::new(4, 5, 5));
        assert!(a.sits_over(&c));
        assert!(!c.sits_over(&a));

        let a = Block::new(1, Point::new(1, 1, 10), Point::new(1, 3, 10));
        let b = Block::new(2, Point::new(1, 1, 0), Point::new(3, 1, 0));
        assert!(a.sits_over(&b));
        assert!(!b.sits_over(&a));

        let a = Block::new(1, Point::new(1, 2, 7), Point::new(4, 2, 7));
        let b = Block::new(2, Point::new(1, 2, 6), Point::new(1, 5, 6));
        assert!(a.sits_over(&b));
        assert!(!b.sits_over(&a));

        let a = Block::new(2, Point::new(1, 2, 6), Point::new(1, 5, 6));
        let b = Block::new(1, Point::new(1, 2, 0), Point::new(1, 2, 3));
        assert!(a.sits_over(&b));
        assert!(!b.sits_over(&a));
    }
    #[test]
    fn test_overlaps() {
        let a = Block::new(1, Point::new(10, 10, 7), Point::new(10, 9, 7));
        let b = Block::new(2, Point::new(0, 10, 7), Point::new(10, 10, 7));
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));

        let b = Block::new(2, Point::new(0, 10, 6), Point::new(10, 10, 6));
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));

        let b = Block::new(2, Point::new(0, 10, 8), Point::new(10, 10, 8));
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));

        let a = Block::new(1, Point::new(1, 1, 1), Point::new(1, 1, 10));
        let b = Block::new(2, Point::new(2, 1, 15), Point::new(0, 1, 5));
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));

        let b = Block::new(2, Point::new(2, 1, 11), Point::new(0, 1, 11));
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
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
            Column::from_str("1,2,15~4,2,15\n3,3,21~3,3,21\n1,2,10~1,5,10\n1,2,8~1,2,5").unwrap();
        println!("Prefall:\n{col}");
        col.fall();
        println!("Postfall:\n{col}");
        assert_eq!(
            col.blocks,
            vec![
                Block::new(2, Point::new(3, 3, 1), Point::new(3, 3, 1)),
                Block::new(4, Point::new(1, 2, 1), Point::new(1, 2, 4)),
                Block::new(3, Point::new(1, 2, 5), Point::new(1, 5, 5)),
                Block::new(1, Point::new(1, 2, 6), Point::new(4, 2, 6)),
            ]
        );
    }
    #[test]
    fn test_fall2() {
        let mut col = Column::from_str("2,4,5~2,6,5\n2,6,1~2,6,3").unwrap();
        println!("Prefall:\n{col}");
        col.fall();
        println!("Postfall:\n{col}");
        assert_eq!(
            col.blocks,
            vec![
                Block::new(2, Point::new(2, 6, 1), Point::new(2, 6, 3)),
                Block::new(1, Point::new(2, 4, 4), Point::new(2, 6, 4)),
            ]
        );
    }
    #[test]
    fn test_all_fall() {
        let mut col = Column::from_str(include_str!("input.txt")).unwrap();
        println!("Prefall:");
        col.blocks[0..20].iter().for_each(|b| println!("{}", *b));
        col.fall();
        println!("Postfall:");
        col.blocks[0..20].iter().for_each(|b| println!("{}", *b));

        assert!((1..200).all(|z| {
            let mut blocks = col.blocks.iter().filter(|b| b.a.z <= z && b.b.z >= z);
            while let Some(test_block) = blocks.next() {
                if !blocks.clone().all(|b| {
                    let res = !b.overlaps(test_block);
                    if !res {
                        println!("Overlap: {} <=> {}", test_block, b)
                    }
                    res
                }) {
                    return false;
                }
            }
            true
        }));

        assert!(col.blocks.iter().enumerate().all(|block| {
            block.1.a.z == 1
                || col.blocks[..block.0]
                    .iter()
                    .any(|s_block| block.1.sits_on(s_block))
        }));
    }

    #[test]
    fn test_chain_reaction_at() {
        let mut col = Column::from_str(include_str!("input.txt")).unwrap();
        col.fall();
        col.chain_reaction_at(147);
    }

    #[test]
    fn test_parse() {
        let col = Column::from_str("1,2,7~4,2,7\n3,3,21~3,3,21\n1,2,6~1,5,6\n1,2,8~1,2,5").unwrap();
        assert_eq!(
            col.blocks,
            vec![
                Block {
                    id: 4,
                    a: Point { x: 1, y: 2, z: 5 },
                    b: Point { x: 1, y: 2, z: 8 }
                },
                Block {
                    id: 3,
                    a: Point { x: 1, y: 2, z: 6 },
                    b: Point { x: 1, y: 5, z: 6 },
                },
                Block {
                    id: 1,
                    a: Point { x: 1, y: 2, z: 7 },
                    b: Point { x: 4, y: 2, z: 7 }
                },
                Block {
                    id: 2,
                    a: Point { x: 3, y: 3, z: 21 },
                    b: Point { x: 3, y: 3, z: 21 }
                },
            ]
        );
    }
}
