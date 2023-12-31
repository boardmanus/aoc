use std::{
    collections::{HashMap, HashSet},
    ops::Add,
    str::FromStr,
};

use pathfinding::matrix::Matrix;

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

struct Rocks2 {
    width: usize,
    height: usize,
    rows: Vec<Row>,
}

impl Rocks2 {
    fn new(width: usize, height: usize) -> Self {
        Rocks2 {
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

struct Rocks {
    width: usize,
    height: usize,
    locs: HashSet<Point>,
}

impl Rocks {
    fn new(width: usize, height: usize) -> Self {
        Rocks {
            width,
            height,
            locs: Default::default(),
        }
    }

    fn has_rock(&self, p: &Point) -> bool {
        let (x, y) = p.normalize(self.width, self.height);
        self.locs.contains(&Point::new(x as i64, y as i64))
    }

    fn add_rock(&mut self, p: &Point) {
        let (x, y) = p.normalize(self.width, self.height);
        self.locs.insert(Point::new(x as i64, y as i64));
    }

    fn clr_rock(&mut self, p: &Point) {
        let (x, y) = p.normalize(self.width, self.height);
        self.locs.remove(&Point::new(x as i64, y as i64));
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

    fn stroll(&self, steps: usize) -> usize {
        let mut ys = vec![];
        let width = self.rocks.width;
        /*
                (0..steps)
                    .fold(HashSet::from([self.start]), |locs, i| self.walk(&locs))
                    .len()
        */
        let mut reachable = HashSet::from([self.start]);

        /*
        let mut checked: HashSet<Point> = HashSet::from([self.start]);
        let mut from = vec![self.start];
        let mut to = vec![];
        let parity = steps & 1;
        */
        for i in 0..steps {
            /*
            for p in from.iter() {
                if (i & 1) != parity {
                    reachable.insert(*p);
                }
                for d in &Point::DIRS {
                    let p = p + d;
                    if !self.has_rock(&p) {
                        if checked.insert(p) {
                            to.push(p);
                        }
                    }
                }
            }
            from.clear();
            std::mem::swap(&mut from, &mut to);
            */

            for p in reachable.drain().collect::<Vec<_>>() {
                Point::DIRS
                    .iter()
                    .map(|d| p + *d)
                    .filter(|p| !self.has_rock(p))
                    .for_each(|p| _ = reachable.insert(p));
            }

            if (i + 1) % width == width / 2 {
                ys.push(reachable.len());
                if let &[y0, y1, y2] = &ys[..] {
                    println!("ys={:?}", ys);
                    let x = (steps - width / 2) / width;
                    return (x * x * (y0 + y2 - 2 * y1) + x * (4 * y1 - 3 * y0 - y2) + 2 * y0) / 2;
                }
            }
        }
        //reachable.extend(from.iter());
        reachable.len()
    }

    // Find the number of walkable points for and odd or even fill.
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

    fn min_max_walk(&self, start: &Point, steps: usize) -> (usize, usize) {
        // Assumptions about the start location, and size of grid.
        assert_eq!(start.x as usize, self.rocks.width / 2);
        assert_eq!(start.y as usize, self.rocks.height / 2);
        assert_eq!(self.rocks.width, self.rocks.height);
        assert!(self.rocks.width % 2 == 1);

        // Determine the number of locations in a filled grid for odd and even cases.
        let (odd, even) = self.fill_count();

        // Find the number of grids that can be filled.
        let max_grid_width = (((steps + self.rocks.width / 2) / self.rocks.width) * 2 + 1) as i64;
        assert!(max_grid_width % 2 == 1);

        let radius = max_grid_width / 2;
        let (mut even_grids, mut odd_grids) = if radius % 2 == 0 {
            ((radius + 1) * (radius + 1), radius * radius)
        } else {
            (radius * radius, (radius + 1) * (radius + 1))
        };

        // We're on an odd step, so swap around the even and odd grids.
        if steps % 1 == 1 {
            let grid = even_grids;
            even_grids = odd_grids;
            odd_grids = grid;
        }

        let offset = steps % self.rocks.width;
        println!("Steps={steps}, Radius={radius}, Offset={offset}, EvenGrids={even_grids}, OddGrids={odd_grids}");

        (0, (even_grids as usize * even + odd_grids as usize * odd))
    }

    fn guess_walk(&self, steps: usize) -> usize {
        let small_stroll = 3 * self.rocks.width + steps % self.rocks.width;
        if steps <= small_stroll {
            return self.stroll(steps);
        }

        // Create a template of strolling that contains all the possible grid geometries
        // for this step cycle.
        let locs = self.stroll(small_stroll);

        // Find the number of grids that can be filled.
        let max_grid_width = (((steps + self.rocks.width / 2) / self.rocks.width) * 2 + 1) as i64;
        assert!(max_grid_width % 2 == 1);
        let radius = max_grid_width / 2;

        // Determine the number of locations in a filled grid for odd and even cases.
        let (odd, even) = self.fill_count();
        let (mut even_grids, mut odd_grids) = if radius % 2 == 0 {
            ((radius + 1) * (radius + 1), radius * radius)
        } else {
            (radius * radius, (radius + 1) * (radius + 1))
        };

        // We're on an odd step, so swap around the even and odd grids.
        if steps % 1 == 1 {
            let grid = even_grids;
            even_grids = odd_grids;
            odd_grids = grid;
        }

        let filled_count = odd * odd_grids as usize + even * even_grids as usize;

        // Starting from the top, rotate around the template to fill in the partial grids.
        //let mut north_count =

        0
    }

    fn print(&self, walk: &HashSet<Point>, steps: usize) {
        let steps = ((steps / self.rocks.width) * self.rocks.width + self.rocks.width / 2) as i64;
        for y in (self.start.y - steps)..(self.start.y + steps + 1) {
            if y % self.rocks.height as i64 == 0 {
                ((self.start.x - steps)..(self.start.x + steps + 1)).for_each(|x| {
                    if x % self.rocks.width as i64 == 0 {
                        print!("+");
                    }
                    print!("-")
                });
                println!();
            }
            for x in (self.start.x - steps)..(self.start.x + steps + 1) {
                if x % self.rocks.width as i64 == 0 {
                    print!("|");
                }
                let p = Point::new(x as i64, y as i64);
                if p == self.start {
                    print!("S");
                } else if self.has_rock(&p) {
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
    grid.stroll(64)
}

fn solve_part2(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    grid.stroll(26501365)
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
        assert_eq!(grid.stroll(6), 16);
    }

    #[test]
    fn test_part2_input() {
        const INPUT: &str = include_str!("input.txt");
        let grid: Grid = Grid::from_str(INPUT).unwrap();
        let res = grid.stroll(grid.rocks.width / 2);
        let steps = 26501365;
        println!("Reachable={}", res);
        println!(
            "Steps={steps}, steps-width/2={}, steps-width/2 % width={}",
            steps - grid.rocks.width / 2,
            (steps - grid.rocks.width / 2) % grid.rocks.width
        );
    }

    #[test]
    fn test_fill_count() {
        let grid: Grid = Grid::from_str(TEST_INPUT).unwrap();
        let counts = grid.fill_count();
        println!("Fill Count = {:?}", counts);
        assert_eq!(
            counts.0 + counts.1,
            grid.rocks.width * grid.rocks.height - grid.rocks.locs.len()
        )
    }

    #[test]
    fn test_min_max_walk() {
        let grid: Grid = Grid::from_str(TEST_INPUT).unwrap();
        let steps = 6;
        let min_max = grid.min_max_walk(&grid.start, steps);
        println!("MinMax({steps}) = {:?}", min_max);
        let steps = 10;
        let min_max = grid.min_max_walk(&grid.start, steps);
        println!("MinMax({steps}) = {:?}", min_max);
        let steps = 50;
        let min_max = grid.min_max_walk(&grid.start, steps);
        println!("MinMax({steps}) = {:?}", min_max);
        let steps = 100;
        let min_max = grid.min_max_walk(&grid.start, steps);
        println!("MinMax({steps}) = {:?}", min_max);

        let steps = 16733044;
        let min_max = grid.min_max_walk(&grid.start, steps);
        println!("MinMax({steps}) = {:?}", min_max);
    }

    #[test]
    fn test_print() {
        let grid: Grid = Grid::from_str(TEST_INPUT).unwrap();
        for i in 0..10 {
            println!("i={}", i);
            let walk = grid.stroll(i);
            //grid.print(&walk, i);
        }

        println!("fill_count = {:?}", grid.fill_count());

        let walk = grid.stroll(grid.rocks.width);
        println!(
            "start=({:?}), walk_len={}, stroll length={}",
            grid.start, walk, grid.rocks.width,
        );
    }

    #[test]
    fn test_part2() {
        let grid: Grid = Grid::from_str(TEST_INPUT).unwrap();
        assert_eq!(grid.stroll(6), 16);

        let dist = 10;
        let walk = grid.stroll(dist);
        assert_eq!(walk, 50);
        //grid.print(&walk, dist);

        let dist = 50;
        let walk = grid.stroll(dist);
        assert_eq!(walk, 1594);
        //grid.print(&walk, dist);

        let dist = 100;
        let walk = grid.stroll(dist);
        assert_eq!(walk, 6536);
        //grid.print(&walk, dist);

        //assert_eq!(grid.stroll(500), 167004);
        //assert_eq!(grid.stroll(1000), 668697);
        //assert_eq!(grid.stroll(5000), 16733044);
    }
}
