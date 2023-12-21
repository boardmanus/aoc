use std::{collections::HashSet, ops::Add, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    const DIRS: [Point; 4] = [
        Point { x: 0, y: 1 },
        Point { x: 0, y: -1 },
        Point { x: 1, y: 0 },
        Point { x: -1, y: 0 },
    ];

    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add for &Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

struct Grid {
    start: Point,
    rocks: HashSet<Point>,
}

impl Grid {
    fn walk(&self, from: &HashSet<Point>) -> HashSet<Point> {
        let mut to = HashSet::new();
        for p in from {
            for d in &Point::DIRS {
                let p = p + d;
                if !self.rocks.contains(&p) {
                    to.insert(p);
                }
            }
        }
        to
    }

    fn stroll(&self, steps: usize) -> HashSet<Point> {
        (0..steps).fold(HashSet::from([self.start]), |locs, _| self.walk(&locs))
    }
}
impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rocks = HashSet::new();
        let mut start = None;
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => _ = rocks.insert(Point::new(x as i64, y as i64)),
                    'S' => start = Some(Point::new(x as i64, y as i64)),
                    _ => {}
                }
            }
        }
        Ok(Grid {
            start: start.unwrap(),
            rocks,
        })
    }
}

fn solve_part1(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    grid.stroll(64).len()
}

fn solve_part2(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    grid.stroll(26501365).len()
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
        let grid: Grid = Grid::from_str(TEST_INPUT).unwrap();
        assert_eq!(grid.stroll(6).len(), 16);
    }

    #[test]
    fn test_part2() {
        let grid: Grid = Grid::from_str(TEST_INPUT).unwrap();
        assert_eq!(grid.stroll(6).len(), 16);
        assert_eq!(grid.stroll(10).len(), 50);
        assert_eq!(grid.stroll(50).len(), 1594);
        assert_eq!(grid.stroll(100).len(), 6536);
        assert_eq!(grid.stroll(500).len(), 167004);
        assert_eq!(grid.stroll(1000).len(), 668697);
        assert_eq!(grid.stroll(5000).len(), 16733044);
    }
}
