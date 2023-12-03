use std::{collections::HashSet, fmt::Display, ops::Add, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, PartialEq, Clone)]
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

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.rows {
            for c in 0..self.cols {
                let c = self.data[self.index(&Point::new(c, r))];
                match c {
                    0 => write!(f, "*")?,
                    _ => write!(f, "{}", c)?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn index(&self, p: &Point) -> usize {
        (p.y * self.cols + p.x) as usize
    }

    fn in_bounds(&self, p: &Point) -> bool {
        p.x >= 0 && p.x < self.cols && p.y >= 0 && p.y < self.rows
    }

    fn get(&self, p: &Point) -> Option<u8> {
        if self.in_bounds(p) {
            Some(self.data[self.index(p)])
        } else {
            None
        }
    }

    fn update_point<'a>(&mut self, p: &'a Point) -> Option<&'a Point> {
        if self.in_bounds(p) {
            let i = self.index(p);
            self.data[i] += 1;
            if self.data[i] > 9 {
                self.data[i] = 0;
                Some(p)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn update(&mut self) -> usize {
        let mut flashed = HashSet::<Point>::new();
        let mut flashers = Vec::<Point>::new();
        for y in 0..self.rows {
            for x in 0..self.cols {
                let pt = Point::new(x, y);
                if let Some(p) = self.update_point(&pt) {
                    flashers.push(*p);
                    flashed.insert(*p);
                }
            }
        }

        while let Some(flasher) = flashers.pop() {
            ADJ.iter().for_each(|dp| {
                let p = flasher + *dp;
                if !flashed.contains(&p) {
                    if let Some(p2) = self.update_point(&p) {
                        flashers.push(*p2);
                        flashed.insert(*p2);
                    }
                }
            });
        }

        flashed.len()
    }

    fn update_n(&mut self, n: usize) -> usize {
        (0..n).map(|i| self.update()).sum()
    }
}

const ADJ: [Point; 8] = [
    Point { x: 1, y: 1 },
    Point { x: 1, y: 0 },
    Point { x: 1, y: -1 },
    Point { x: -1, y: -1 },
    Point { x: -1, y: 0 },
    Point { x: -1, y: 1 },
    Point { x: 0, y: 1 },
    Point { x: 0, y: -1 },
];

fn parse(input: &str) -> Grid {
    Grid::from_str(input).expect("Valid grid")
}

fn solve_part1(g: &Grid) -> usize {
    let mut grid = g.clone();
    grid.update_n(100)
}

fn solve_part2(g: &Grid) -> usize {
    let mut grid = g.clone();
    let mut step = 1;
    let all_flash = (grid.rows * grid.cols) as usize;
    while grid.update() != all_flash {
        step += 1;
    }
    step
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let grid = parse(INPUT);
    let part1 = solve_part1(&grid);
    println!("Part1: {part1}");
    let part2 = solve_part2(&grid);
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
    const TEST_INPUT_2: &str = include_str!("test_input_2.txt");

    #[test]
    fn test_part1() {
        let grid = parse(TEST_INPUT);
        assert_eq!(solve_part1(&grid), 1656);
    }

    #[test]
    fn test_part2() {
        let grid = parse(TEST_INPUT);
        assert_eq!(solve_part2(&grid), 195);
    }

    #[test]
    fn test_update_n() {
        let mut grid = parse(TEST_INPUT);
        assert_eq!(grid.update_n(10), 204);
        assert_eq!(grid.update_n(90), 1656 - 204);
    }

    #[test]
    fn test_grid_update() {
        let mut grid = parse("11111\n19991\n19191\n19991\n11111");
        let grid2 = parse("34543\n40004\n50005\n40004\n34543\n");
        let grid3 = parse("45654\n51115\n61116\n51115\n45654");
        grid.update();
        assert_eq!(grid, grid2);
        grid.update();
        assert_eq!(grid, grid3);
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(TEST_INPUT_2),
            Grid {
                rows: 5,
                cols: 5,
                data: vec![
                    1, 1, 1, 1, 1, 1, 9, 9, 9, 1, 1, 9, 1, 9, 1, 1, 9, 9, 9, 1, 1, 1, 1, 1, 1
                ]
            }
        );
    }
}
