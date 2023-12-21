use std::{
    collections::{HashMap, HashSet},
    ops::Add,
    str::FromStr,
};

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

    fn normalize(&self, width: usize, height: usize) -> (usize, usize) {
        (
            self.x.rem_euclid(width as i64) as usize,
            self.y.rem_euclid(height as i64) as usize,
        )
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Row {
    rocks: [u64; 3],
}

impl Row {
    const EMPTY: Self = Row { rocks: [0; 3] };

    fn has_rock(&self, x: usize) -> bool {
        let (i, j) = (x / 64, x % 64);
        self.rocks[i] & (1 << j) != 0
    }

    fn add_rock(&mut self, x: usize) {
        let (i, j) = (x / 64, x % 64);
        self.rocks[i] |= 1 << j;
    }

    fn clr_rock(&mut self, x: usize) {
        let (i, j) = (x / 64, x % 64);
        self.rocks[i] &= !(1 << j);
    }
}

struct Rocks {
    width: usize,
    height: usize,
    rows: Vec<Row>,
}

impl Rocks {
    fn new(width: usize, height: usize) -> Self {
        Rocks {
            width,
            height,
            rows: vec![Row::EMPTY; height],
        }
    }

    fn has_rock(&self, p: &Point) -> bool {
        let (x, y) = p.normalize(self.width, self.height);
        self.rows[y].has_rock(x)
    }

    fn add_rock(&mut self, p: &Point) {
        let (x, y) = p.normalize(self.width, self.height);
        self.rows[y].add_rock(x)
    }

    fn clr_rock(&mut self, p: &Point) {
        let (x, y) = p.normalize(self.width, self.height);
        self.rows[y].clr_rock(x)
    }
}

struct Walk {
    rows: Vec<Row>,
}

struct Grid {
    start: Point,
    rocks: Rocks,
}

impl Grid {
    fn has_rock(&self, p: &Point) -> bool {
        self.rocks.has_rock(p)
    }

    fn walk(&self, from: &HashSet<Point>) -> HashSet<Point> {
        let mut to = HashSet::new();
        for p in from {
            for d in &Point::DIRS {
                let p = p + d;
                if !self.has_rock(&p) {
                    to.insert(p);
                }
            }
        }
        to
    }

    fn stroll(&self, steps: usize) -> HashSet<Point> {
        (0..steps).fold(HashSet::from([self.start]), |locs, _| self.walk(&locs))
    }

    fn fill_count(&self) -> (usize, usize) {
        let mut even = 0;
        let mut odd = 0;
        for y in 0..self.rocks.height {
            for x in 0..self.rocks.width {
                let p = Point::new(x as i64, y as i64);
                if self.has_rock(&p) {
                    continue;
                }
                match (x % 2, y % 2) {
                    (0, 0) => even += 1,
                    (1, 1) => even += 1,
                    (0, 1) => odd += 1,
                    (1, 0) => odd += 1,
                    _ => unreachable!(),
                }
            }
        }
        (even, odd)
    }

    fn print(&self, walk: &HashSet<Point>) {
        for y in 0..self.rocks.height {
            for x in 0..self.rocks.width {
                let p = Point::new(x as i64, y as i64);
                if self.has_rock(&p) {
                    print!("#");
                } else if walk.contains(&p) {
                    print!("O");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}
impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        let mut rocks = Rocks::new(width, height);
        let mut start = None;
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => rocks.add_rock(&Point::new(x as i64, y as i64)),
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

    #[test]
    fn test_part1() {
        let grid: Grid = Grid::from_str(TEST_INPUT).unwrap();
        assert_eq!(grid.stroll(6).len(), 16);
    }

    #[test]
    fn test_print() {
        let grid: Grid = Grid::from_str(TEST_INPUT).unwrap();
        for i in 0..1 {
            println!("i={}", i);
            let walk = grid.stroll(i);
            grid.print(&walk);
        }

        println!("fill_count = {:?}", grid.fill_count());

        let walk = grid.stroll(grid.rocks.width);
        println!(
            "start=({:?}), walk_len={}, stroll length={}, min-max-y=({}, {}), min-max-x=({}, {})",
            grid.start,
            walk.len(),
            grid.rocks.width,
            walk.iter().map(|p| p.y).min().unwrap(),
            walk.iter().map(|p| p.y).max().unwrap(),
            walk.iter().map(|p| p.y).min().unwrap(),
            walk.iter().map(|p| p.x).max().unwrap()
        );
    }

    #[test]
    fn test_part2() {
        let grid: Grid = Grid::from_str(TEST_INPUT).unwrap();
        assert_eq!(grid.stroll(6).len(), 16);
        assert_eq!(grid.stroll(10).len(), 50);
        assert_eq!(grid.stroll(50).len(), 1594);
        assert_eq!(grid.stroll(100).len(), 6536);
        //assert_eq!(grid.stroll(500).len(), 167004);
        //assert_eq!(grid.stroll(1000).len(), 668697);
        //assert_eq!(grid.stroll(5000).len(), 16733044);
    }
}
