#[macro_use]
extern crate lazy_static;

use std::{
    collections::{HashMap, HashSet},
    fmt,
    ops::Add,
};

#[derive(Debug, PartialEq, Hash, Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

lazy_static! {
    static ref MASKS: Vec<u8> =
        [[Dir::NorthWest, Dir::North, Dir::NorthEast],
        [Dir::NorthEast, Dir::East, Dir::SouthEast],
        [Dir::SouthWest, Dir::West, Dir::NorthWest],
        [Dir::SouthEast, Dir::South, Dir::SouthWest],].iter().map(|a| a.iter().fold(0u8, |m, d| m | 1 << (*d as u8))).collect();
}

impl Dir {

    fn dir(&self) -> Elf {
        match self {
            Dir::North => Elf(0, -1),
            Dir::South => Elf(0, 1),
            Dir::West => Elf(-1, 0),
            Dir::East => Elf(1, 0),
            Dir::NorthWest => Elf(-1, -1),
            Dir::NorthEast => Elf(1, -1),
            Dir::SouthWest => Elf(-1, 1),
            Dir::SouthEast => Elf(1, 1),
        }
    }
}

impl From<u8> for Dir {
    fn from(value: u8) -> Self {
        match value % 8 {
            0 => Dir::North,
            1 => Dir::South,
            2 => Dir::West,
            3 => Dir::East,
            4 => Dir::NorthWest,
            5 => Dir::NorthEast,
            6 => Dir::SouthWest,
            7 => Dir::SouthEast,
            _ => panic!(),
        }
    }
}

impl Iterator for Dir {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        let val = *self as u8;
        if val < 7 {
            Some(Dir::from(val + 1))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Elf(i64, i64);

impl Elf {
    fn move_dir(&self, dir: Dir) -> Elf {
        *self + dir.dir()
    }
    fn min(&self, rhs: &Elf) -> Elf {
        Elf(self.0.min(rhs.0), self.1.min(rhs.1))
    }
    fn max(&self, rhs: &Elf) -> Elf {
        Elf(self.0.max(rhs.0), self.1.max(rhs.1))
    }
}

impl Add for Elf {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Elf(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, PartialEq)]
struct Grid {
    elves: HashSet<Elf>,
    start: u8,
}

impl Grid {
    fn parse_row(line: &str) -> Option<Vec<bool>> {
        let row = line
            .chars()
            .map(|c| match c {
                '#' => true,
                _ => false,
            })
            .collect::<Vec<_>>();
        Some(row)
    }

    fn parse_input(input: &str) -> Grid {
        let rows = input.trim()
            .split('\n')
            .flat_map(|line: &str| Grid::parse_row(line))
            .collect::<Vec<_>>();

        let elves = rows
            .iter()
            .enumerate()
            .flat_map(|row| {
                row.1
                    .iter()
                    .enumerate()
                    .filter(|col| *col.1)
                    .map(move |col| Elf(col.0 as i64, row.0 as i64))
            })
            .collect::<HashSet<_>>();

        Grid { elves, start: 0 }
    }

    fn has_elf(&self, elf: &Elf) -> bool {
        self.elves.contains(elf)
    }

    fn adjacent_elves(&self, elf: &Elf) -> u8 {
        Dir::North.filter(|i| self.has_elf(&(*elf + i.dir())))
            .map(|i| 1 << i as u8)
            .sum()
    }

    fn propose_move(&self, elf: &Elf) -> Option<Elf> {
        let adjacent = self.adjacent_elves(elf);
        if adjacent == 0 {
            return None;
        }
        for i in 0..4 {
            let dir = Dir::from((self.start + i) % 4);
            if (adjacent & MASKS[dir as usize]) == 0 {
                return Some(elf.move_dir(dir));
            }
        }
        None
    }

    fn all_proposals(&self) -> HashMap<Elf, Elf> {
        let mut moves: HashSet<Elf> = Default::default();
        self.elves
            .iter()
            .map(|elf| (elf, self.propose_move(elf)))
            .fold(Default::default(), |mut acc, elf| {
                if let Some(m) = elf.1 {
                    if moves.contains(&m) {
                        acc.remove(&m);
                    } else {
                        moves.insert(m);
                        acc.insert(m, *elf.0);
                    }
                }
                acc
            })
    }

    fn iterate(&mut self) -> i64 {
        let proposals = self.all_proposals();
        let mut count = 0;
        proposals.iter().for_each(|mv| {
            self.elves.remove(mv.1);
            self.elves.insert(*mv.0);
            count += 1;
        });
        self.start = (self.start + 1) % 4;
        count
    }

    fn bounds(&self) -> (Elf, Elf) {
        self.elves
            .iter()
            .fold((Elf(i64::MAX, i64::MAX), Elf(0, 0)), |mm, elf| {
                (elf.min(&mm.0), elf.max(&mm.1))
            })
    }

    fn blank_spots(&self) -> i64 {
        let b = self.bounds();
        (b.1 .0 - b.0 .0 + 1) * (b.1 .1 - b.0 .1 + 1) - self.elves.len() as i64
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = self.bounds();
        for y in b.0 .1..=b.1 .1 {
            for x in b.0 .0..=b.1 .0 {
                let c = match self.elves.contains(&Elf(x, y)) {
                    true => '#',
                    false => '.',
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

fn solve_part1(input: &str) -> String {
    let mut grid = Grid::parse_input(input);
    println!("{grid}");
    for _ in 0..10 {
        grid.iterate();
        println!("{grid}");
    }
    grid.blank_spots().to_string()
}

fn solve_part2(input: &str) -> String {
    let mut grid = Grid::parse_input(input);
    let mut count = 1;
    while grid.iterate() != 0 {
        count += 1;
    }
    println!("{grid}");
    count.to_string()
}

fn main() {
    let part1 = solve_part1(include_str!("input.txt"));
    println!("Part1: {part1}");
    let part2 = solve_part2(include_str!("input.txt"));
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 110.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 20.to_string());
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            Grid::parse_row("...#.##."),
            Some(vec![false, false, false, true, false, true, true, false])
        );

        assert_eq!(
            Grid::parse_input(".#.\n#.#\n#.."),
            Grid {
                elves: HashSet::from_iter(vec![Elf(1, 0), Elf(0, 1), Elf(2, 1), Elf(0, 2)]),
                start: 0
            }
        );
    }

    #[test]
    fn test_masks() {
        assert_eq!(MASKS[Dir::North as usize], 1 <<Dir::North as u8| 1 << Dir::NorthEast as u8 | 1 << Dir::NorthWest as u8);
        assert_eq!(MASKS[Dir::West as usize], 1 <<Dir::West as u8| 1 << Dir::SouthWest as u8 | 1 << Dir::NorthWest as u8);
    }
}
