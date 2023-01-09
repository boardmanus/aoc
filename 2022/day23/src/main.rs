#[macro_use]
extern crate lazy_static;

use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display},
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
    static ref MASKS: Vec<u8> = [
        [Dir::North, Dir::NorthWest, Dir::NorthEast],
        [Dir::South, Dir::SouthEast, Dir::SouthWest],
        [Dir::West, Dir::SouthWest, Dir::NorthWest],
        [Dir::East, Dir::NorthEast, Dir::SouthEast],
    ]
    .iter()
    .map(|a| a.iter().fold(0u8, |m, d| m | d.bit()))
    .collect();
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

    fn bit(&self) -> u8 {
        1 << *self as u8
    }

    fn iter(&self, num: usize) -> impl Iterator<Item = Dir> {
        DirIter(Some(*self), num).take(num)
    }

    fn next(&self, num: usize) -> Dir {
        Dir::from((*self as u8 + 1) % num as u8)
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

struct DirIter(Option<Dir>, usize);

impl Iterator for DirIter {
    type Item = Dir;
    fn next(&mut self) -> Option<Dir> {
        match self.0 {
            Some(dir) => {
                *self = DirIter(Some(dir.next(self.1)), self.1);
                Some(dir)
            }
            None => None,
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

#[derive(Debug, PartialEq, Clone)]
struct Grid {
    elves: HashSet<Elf>,
    start: Dir,
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
        let rows = input
            .trim()
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

        Grid {
            elves,
            start: Dir::North,
        }
    }

    fn has_elf(&self, elf: &Elf) -> bool {
        self.elves.contains(elf)
    }

    fn adjacent_elves(&self, elf: &Elf) -> u8 {
        Dir::North
            .iter(8)
            .filter(|i| self.has_elf(&(*elf + i.dir())))
            .map(|dir| dir.bit())
            .sum()
    }

    fn propose_move(&self, elf: &Elf) -> Option<Elf> {
        match self.adjacent_elves(elf) {
            0 => None,
            adj => self
                .start
                .iter(4)
                .find_map(|o| match adj & MASKS[o as usize] {
                    0 => Some(elf.move_dir(o)),
                    _ => None,
                }),
        }
    }

    fn all_proposals(&self) -> (HashSet<Elf>, usize) {
        let mut num_changes = 0usize;
        let proposals = self
            .elves
            .iter()
            .map(|elf| (elf, self.propose_move(elf)))
            .fold(HashMap::<Elf, Elf>::default(), |mut acc, elf| {
                if let Some(new_elf) = elf.1 {
                    if let Some(existing_elf) = acc.remove(&new_elf) {
                        acc.insert(existing_elf, existing_elf);
                        acc.insert(*elf.0, *elf.0);
                        num_changes -= 1;
                    } else {
                        acc.insert(new_elf, *elf.0);
                        num_changes += 1;
                    }
                } else {
                    acc.insert(*elf.0, *elf.0);
                }
                acc
            })
            .into_keys()
            .collect::<HashSet<_>>();
        (proposals, num_changes)
    }

    fn bounds(&self) -> (Elf, Elf) {
        self.elves
            .iter()
            .fold((Elf(i64::MAX, i64::MAX), Elf(0, 0)), |mm, elf| {
                (elf.min(&mm.0), elf.max(&mm.1))
            })
    }

    fn blank_spots(&self, extend: i64) -> i64 {
        let b = self.bounds();
        (b.1 .0 - b.0 .0 + extend) * (b.1 .1 - b.0 .1 + extend) - self.elves.len() as i64
    }
}

impl Iterator for Grid {
    type Item = Grid;
    fn next(&mut self) -> Option<Self::Item> {
        let (proposals, num_changes) = self.all_proposals();
        match num_changes {
            0 => None,
            _ => {
                let curr_grid = Grid {
                    elves: std::mem::replace(&mut self.elves, proposals),
                    start: self.start,
                };
                self.start = self.start.next(4);
                Some(curr_grid)
            }
        }
    }
}

impl<'a> Display for Grid {
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
    grid.nth(10).unwrap().blank_spots(1).to_string()
}

fn solve_part2(input: &str) -> String {
    let grid = Grid::parse_input(input);
    (grid.count() + 1).to_string()
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
                start: Dir::North,
            }
        );
    }

    #[test]
    fn test_adj() {
        let grid = Grid::parse_input(".#.\n.##\n#..");
        assert_eq!(
            grid.adjacent_elves(&Elf(1, 1)),
            Dir::North.bit() | Dir::East.bit() | Dir::SouthWest.bit()
        );
    }
    #[test]
    fn test_iterator() {
        assert_eq!(
            Dir::North.iter(8).collect::<Vec<_>>(),
            vec![
                Dir::North,
                Dir::South,
                Dir::West,
                Dir::East,
                Dir::NorthWest,
                Dir::NorthEast,
                Dir::SouthWest,
                Dir::SouthEast
            ]
        );
    }

    #[test]
    fn test_masks() {
        assert_eq!(
            MASKS[Dir::North as usize],
            Dir::North.bit() | Dir::NorthEast.bit() | Dir::NorthWest.bit()
        );
        assert_eq!(
            MASKS[Dir::West as usize],
            Dir::West.bit() | Dir::SouthWest.bit() | Dir::NorthWest.bit()
        );
    }
}
