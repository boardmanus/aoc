use std::{
    collections::HashMap,
    fmt,
    hash::{DefaultHasher, Hash, Hasher},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Rock {
    Circle,
    Square,
}

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Option<Rock>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            grid: vec![None; height * width],
        }
    }
    fn set(&mut self, x: usize, y: usize, rock: Option<Rock>) {
        self.grid[y * self.width + x] = rock;
    }
    fn get(&self, x: usize, y: usize) -> Option<Rock> {
        self.grid[y * self.width + x]
    }
    fn tilt_north(&mut self) {
        for x in 0..self.width {
            let mut empty: usize = 0;
            for y in 0..self.height {
                match self.get(x, y) {
                    Some(Rock::Square) => {
                        empty = 0;
                    }
                    Some(Rock::Circle) => {
                        self.set(x, y, None);
                        self.set(x, y - empty, Some(Rock::Circle));
                    }
                    None => empty += 1,
                }
            }
        }
    }
    fn tilt_west(&mut self) {
        for y in 0..self.height {
            let mut empty: usize = 0;
            for x in 0..self.width {
                match self.get(x, y) {
                    Some(Rock::Square) => {
                        empty = 0;
                    }
                    Some(Rock::Circle) => {
                        self.set(x, y, None);
                        self.set(x - empty, y, Some(Rock::Circle));
                    }
                    None => empty += 1,
                }
            }
        }
    }
    fn tilt_south(&mut self) {
        for x in 0..self.width {
            let mut empty: usize = 0;
            for y in (0..self.height).rev() {
                match self.get(x, y) {
                    Some(Rock::Square) => {
                        empty = 0;
                    }
                    Some(Rock::Circle) => {
                        self.set(x, y, None);
                        self.set(x, y + empty, Some(Rock::Circle));
                    }
                    None => empty += 1,
                }
            }
        }
    }
    fn tilt_east(&mut self) {
        for y in 0..self.height {
            let mut empty: usize = 0;
            for x in (0..self.width).rev() {
                match self.get(x, y) {
                    Some(Rock::Square) => {
                        empty = 0;
                    }
                    Some(Rock::Circle) => {
                        self.set(x, y, None);
                        self.set(x + empty, y, Some(Rock::Circle));
                    }
                    None => empty += 1,
                }
            }
        }
    }
    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }
    fn hash(&self) -> u64 {
        let mut hash = DefaultHasher::new();
        self.grid.hash(&mut hash);
        hash.finish()
    }
    fn sum_load(&self) -> usize {
        let mut load = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                load += match self.get(x, y) {
                    Some(Rock::Circle) => self.height - y,
                    _ => 0,
                }
            }
        }
        load
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x, y) {
                    Some(Rock::Circle) => write!(f, "O")?,
                    Some(Rock::Square) => write!(f, "#")?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> Grid {
    let mut grid = Grid::new(input.lines().next().unwrap().len(), input.lines().count());
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set(
                x,
                y,
                match c {
                    '.' => None,
                    'O' => Some(Rock::Circle),
                    '#' => Some(Rock::Square),
                    _ => panic!("Invalid input"),
                },
            );
        }
    }
    grid
}

fn solve_part1(input: &str) -> usize {
    let mut grid = parse(input);
    println!("{}", grid);
    grid.tilt_north();
    println!("{}", grid);
    grid.sum_load()
}

fn solve_part2(input: &str) -> usize {
    let mut grid = parse(input);
    println!("{}", grid);
    let mut seen: HashMap<u64, (u64, usize)> = HashMap::new();
    let mut lookup: HashMap<u64, (u64, usize)> = HashMap::new();
    for i in 0..1000000000 {
        let last_seen = seen.get(&grid.hash());
        match last_seen {
            Some(last) => {
                println!("Cycle detected at {} and {}. load={}", last.0, i, last.1);
                for j in last.0..i {
                    let rstate = lookup.get(&j).unwrap();
                    println!("State at {} is {} => load={}", j, rstate.0, rstate.1);
                }
                let state = (1000000000 - last.0) % (i - last.0) + last.0;
                let rstate = lookup.get(&state).unwrap();
                println!("State at {} is {} => load={}", 1000000000, state, rstate.1);
                return rstate.1;
            }
            None => {
                let h = grid.hash();
                let l = grid.sum_load();
                seen.insert(h, (i, grid.sum_load()));
                lookup.insert(i, (h, grid.sum_load()));
            }
        }
        grid.cycle();
    }
    grid.sum_load()
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
        assert_eq!(solve_part1(TEST_INPUT), 136);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 64);
    }

    #[test]
    fn test_cycle() {
        let mut grid = parse(TEST_INPUT);
        grid.cycle();
        assert_eq!(
            format!("{grid}"),
            ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
"
        );
        grid.cycle();
        assert_eq!(
            format!("{grid}"),
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O
"
        );
        grid.cycle();
        assert_eq!(
            format!("{grid}"),
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
"
        );
    }
}
