use std::collections::{HashMap, HashSet};

#[derive(Hash, Eq, PartialEq, Debug)]
enum Dir {
    N,
    E,
    S,
    W,
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum Tile {
    Start,
    Pipe(Dir, Dir),
}

impl Tile {
    fn from_char(s: char) -> Option<Tile> {
        match s {
            '|' => Some(Tile::Pipe(Dir::N, Dir::S)),
            '-' => Some(Tile::Pipe(Dir::E, Dir::W)),
            'L' => Some(Tile::Pipe(Dir::N, Dir::E)),
            'J' => Some(Tile::Pipe(Dir::N, Dir::W)),
            '7' => Some(Tile::Pipe(Dir::S, Dir::E)),
            'F' => Some(Tile::Pipe(Dir::S, Dir::W)),
            'S' => Some(Tile::Start),
            _ => None,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i64,
    y: i64,
}

type Grid = HashMap<Point, Tile>;

fn parse(input: &str) -> (Point, Grid) {
    let mut start = Point { x: 0, y: 0 };
    let grid = input
        .lines()
        .enumerate()
        .fold(Grid::new(), |mut grid, (y, line)| {
            line.chars().enumerate().fold(grid, |mut grid, (x, c)| {
                if let Some(tile) = Tile::from_char(c) {
                    if tile == Tile::Start {
                        start = Point {
                            x: x as i64,
                            y: y as i64,
                        };
                    }
                    grid.insert(
                        Point {
                            x: x as i64,
                            y: y as i64,
                        },
                        tile,
                    );
                }
                grid
            })
        });
    (start, grid)
}

fn solve_part1(input: &str) -> usize {
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
    const TEST_INPUT_1_2: &str = include_str!("test_input1_2.txt");
    const TEST_INPUT_2: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 4);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(solve_part1(TEST_INPUT_1_2), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 467835);
    }
}
