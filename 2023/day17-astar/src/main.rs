use std::ops::{Index, IndexMut};

use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[inline]
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    #[inline]
    #[must_use]
    pub fn clockwise(self) -> Self {
        Point::new(-self.y, self.x)
    }

    #[inline]
    #[must_use]
    pub fn counter_clockwise(self) -> Self {
        Point::new(self.y, -self.x)
    }

    #[inline]
    #[must_use]
    pub fn manhattan(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    #[inline]
    #[must_use]
    pub fn signum(self, other: Self) -> Self {
        Point::new((self.x - other.x).signum(), (self.y - other.y).signum())
    }
}

impl Hash for Point {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_u32(self.x as u32);
        hasher.write_u32(self.y as u32);
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn mul(self, rhs: i32) -> Self {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub bytes: Vec<T>,
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
        let width = raw[0].len() as i32;
        let height = raw.len() as i32;
        let mut bytes = Vec::with_capacity((width * height) as usize);
        raw.iter().for_each(|slice| bytes.extend_from_slice(slice));
        Grid {
            width,
            height,
            bytes,
        }
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    pub fn default_copy<U: Default + Copy>(&self) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            bytes: vec![U::default(); (self.width * self.height) as usize],
        }
    }

    pub fn find(&self, needle: T) -> Option<Point> {
        let to_point = |index| {
            let x = (index as i32) % self.width;
            let y = (index as i32) / self.width;
            Point::new(x, y)
        };
        self.bytes.iter().position(|&h| h == needle).map(to_point)
    }

    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, point: Point) -> &Self::Output {
        &self.bytes[(self.width * point.y + point.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.bytes[(self.width * point.y + point.x) as usize]
    }
}

const UP: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const RIGHT: usize = 3;

pub fn parse(input: &str) -> Grid<i32> {
    let Grid {
        width,
        height,
        bytes,
    } = Grid::parse(input);
    let bytes = bytes.iter().map(|b| (b - b'0') as i32).collect();
    Grid {
        width,
        height,
        bytes,
    }
}

pub const ORIGIN: Point = Point::new(0, 0);
pub const PUP: Point = Point::new(0, -1);
pub const PDOWN: Point = Point::new(0, 1);
pub const PLEFT: Point = Point::new(-1, 0);
pub const PRIGHT: Point = Point::new(1, 0);
pub const ORTHOGONAL: [Point; 4] = [PUP, PDOWN, PLEFT, PRIGHT];

fn astar(grid: &Grid<i32>, lower: i32, upper: i32) -> i32 {
    let size = grid.width - 1;
    let end = Point::new(size, size);

    let mut index = 0;
    let mut todo = vec![Vec::new(); 256];
    let mut seen: Grid<[i32; 4]> = grid.default_copy();

    todo[0].push((ORIGIN, RIGHT));
    todo[0].push((ORIGIN, DOWN));

    loop {
        while let Some((position, direction)) = todo[index % 256].pop() {
            let cost = seen[position][direction];

            let mut push = |direction: usize| {
                let step = ORTHOGONAL[direction];
                let mut next = position;
                let mut next_cost = cost;

                for _ in 0..lower {
                    next += step;
                    if !grid.contains(next) {
                        return;
                    }
                    next_cost += grid[next];
                }
                for _ in lower..upper {
                    next += step;
                    if !grid.contains(next) {
                        return;
                    }
                    next_cost += grid[next];

                    if seen[next][direction] == 0 || next_cost < seen[next][direction] {
                        let heuristic = (next_cost + next.manhattan(end)) as usize;
                        todo[heuristic % 256].push((next, direction));
                        seen[next][direction] = next_cost;
                    }
                }
            };

            if position == end {
                return cost;
            }

            if direction < 2 {
                push(LEFT);
                push(RIGHT);
            } else {
                push(UP);
                push(DOWN);
            }
        }

        index += 1;
    }
}

fn solve_part1(input: &str) -> i32 {
    let grid: Grid<i32> = parse(input);
    astar(&grid, 0, 3)
}

fn solve_part2(input: &str) -> i32 {
    let grid: Grid<i32> = parse(input);

    astar(&grid, 3, 10)
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
        assert_eq!(solve_part1(TEST_INPUT), 102);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 94);
    }
}
