use std::{
    collections::{HashMap, HashSet},
    fmt,
};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Elf(i64, i64);

impl Elf {
    fn move_dir(&self, dir: u8) -> Elf {
        match dir {
            0 => Elf(self.0, self.1 - 1),
            1 => Elf(self.0, self.1 + 1),
            2 => Elf(self.0 - 1, self.1),
            3 => Elf(self.0 + 1, self.1),
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Grid {
    elves: HashSet<Elf>,
    start: u8,
}

impl Grid {
    fn parse_row(line: &str) -> Option<Vec<bool>> {
        if line.is_empty() {
            return None;
        }
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

    const ADJACENT: [(i64, i64); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];

    const NORTH_MASK: u8 = 0b00000111;
    const EAST_MASK: u8 = 0b00011100;
    const SOUTH_MASK: u8 = 0b01110000;
    const WEST_MASK: u8 = 0b11000001;

    const MASKS: [u8; 4] = [
        Grid::NORTH_MASK,
        Grid::SOUTH_MASK,
        Grid::WEST_MASK,
        Grid::EAST_MASK,
    ];

    fn has_elf(&self, elf: &Elf) -> bool {
        self.elves.contains(elf)
    }

    fn adjacent_elves(&self, elf: &Elf) -> u8 {
        Grid::ADJACENT
            .iter()
            .enumerate()
            .filter(|i| self.has_elf(&Elf(elf.0 + i.1 .0, elf.1 + i.1 .1)))
            .map(|i| 1 << i.0)
            .sum()
    }

    fn propose_move(&self, elf: &Elf) -> Option<Elf> {
        let adjacent = self.adjacent_elves(elf);
        if adjacent == 0 {
            return None;
        }
        for i in 0..4 {
            let dir = (self.start + i) % 4;
            if (adjacent & Grid::MASKS[dir as usize]) == 0 {
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
        let mut min = Elf(i64::MAX, i64::MAX);
        let mut max = Elf(0, 0);
        self.elves.iter().for_each(|elf| {
            min.0 = min.0.min(elf.0);
            min.1 = min.1.min(elf.1);
            max.0 = max.0.max(elf.0);
            max.1 = max.1.max(elf.1);
        });
        (min, max)
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
                let c = if self.elves.contains(&Elf(x, y)) {
                    '#'
                } else {
                    '.'
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
    for i in 0..10 {
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

    const INPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT), 110.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT), 20.to_string());
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
}
