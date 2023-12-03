use std::{
    collections::{HashSet, VecDeque},
    ops::Add,
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Add<(i64, i64)> for Point {
    type Output = Self;
    fn add(self, rhs: (i64, i64)) -> Self::Output {
        Point::new(self.x + rhs.0, self.y + rhs.1)
    }
}
impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Cell {
    point: Point,
    depth: u8,
}

impl Cell {
    fn new(x: i64, y: i64, depth: u8) -> Self {
        Cell {
            point: Point::new(x, y),
            depth,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Grid {
    rows: i64,
    cols: i64,
    data: Vec<u8>,
}

impl FromStr for Grid {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.lines().count() as i64;
        let cols = s.lines().next().expect("A value").len() as i64;
        let data = s
            .lines()
            .flat_map(|s| s.chars().map(|c| c as u8 - '0' as u8))
            .collect::<Vec<u8>>();
        Ok(Grid { rows, cols, data })
    }
}

impl Grid {
    fn get(&self, x: i64, y: i64) -> Option<Cell> {
        if x >= 0 && x < self.cols && y >= 0 && y < self.rows {
            Some(Cell::new(x, y, self.data[(y * self.cols + x) as usize]))
        } else {
            None
        }
    }
    fn low_value(&self, x: i64, y: i64) -> Option<Cell> {
        let v = self.get(x, y)?;
        if [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .iter()
            .flat_map(|(dx, dy)| self.get(x + dx, y + dy))
            .all(|a| (a.depth as i64 - v.depth as i64) > 0)
        {
            Some(v)
        } else {
            None
        }
    }

    fn low_values(&self) -> Vec<Cell> {
        (0..self.rows)
            .flat_map(|y| (0..self.cols).flat_map(move |x| self.low_value(x, y)))
            .collect()
    }
    fn get_interior(&self, point: &Point) -> Option<Cell> {
        let cell = self.get(point.x, point.y)?;
        if cell.depth < 9 {
            Some(cell)
        } else {
            None
        }
    }
    fn basin(&self, point: Point) -> usize {
        let mut set: HashSet<Cell> = HashSet::new();
        let mut q: VecDeque<Cell> = VecDeque::new();
        if let Some(cell) = self.get_interior(&point) {
            q.push_back(cell);
        }
        while let Some(next_cell) = q.pop_back() {
            [(-1, 0), (0, 1), (1, 0), (0, -1)]
                .iter()
                .flat_map(|pt| self.get_interior(&(next_cell.point + *pt)))
                .fold(&mut q, |acc, c| {
                    if set.insert(c) {
                        acc.push_back(c);
                    }
                    acc
                });
        }
        set.len()
    }
}

fn solve_part1(grid: &Grid) -> usize {
    grid.low_values().iter().map(|x| x.depth as usize + 1).sum()
}

fn solve_part2(grid: &Grid) -> usize {
    let mut basin_sizes = grid
        .low_values()
        .iter()
        .map(|x| grid.basin(x.point))
        .collect::<Vec<_>>();
    basin_sizes.sort();
    basin_sizes.iter().rev().take(3).product()
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let grid = Grid::from_str(INPUT).expect("Valid parse data");
    let part1 = solve_part1(&grid);
    println!("Part1: {part1}");
    let part2 = solve_part2(&grid);
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let grid = Grid::from_str(TEST_INPUT).expect("Valid data");
        println!("{:?}", grid.low_values());
        assert_eq!(solve_part1(&grid), 15);
    }

    #[test]
    fn test_part2() {
        let grid = Grid::from_str(TEST_INPUT).expect("Valid data");
        assert_eq!(solve_part2(&grid), 1134);
    }

    #[test]
    fn test_basin() {
        let grid = Grid::from_str(TEST_INPUT).expect("Valid data");
        assert_eq!(grid.basin(Point::new(0, 0)), 3);
        assert_eq!(grid.basin(Point::new(1, 1)), 0);
        assert_eq!(grid.basin(Point::new(0, 3)), 14);
    }
    #[test]
    fn test_low_value() {
        let grid = Grid::from_str("123\n454\n231\n").expect("valid");
        assert_eq!(grid.low_value(0, 0), Some(Cell::new(0, 0, 1)));
        assert_eq!(grid.low_value(1, 0), None);
        assert_eq!(grid.low_value(2, 0), None);
        assert_eq!(grid.low_value(0, 1), None);
        assert_eq!(grid.low_value(1, 1), None);
        assert_eq!(grid.low_value(2, 1), None);
        assert_eq!(grid.low_value(0, 2), Some(Cell::new(0, 2, 2)));
        assert_eq!(grid.low_value(1, 2), None);
        assert_eq!(grid.low_value(2, 2), Some(Cell::new(2, 2, 1)));

        assert_eq!(
            grid.low_values(),
            vec![Cell::new(0, 0, 1), Cell::new(0, 2, 2), Cell::new(2, 2, 1)]
        );
    }

    #[test]
    fn test_parse() {
        let grid = Grid::from_str("123\n456\n789\n").expect("valid");
        assert_eq!(
            grid,
            Grid {
                rows: 3,
                cols: 3,
                data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
            }
        );
    }
}
