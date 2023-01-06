#[macro_use]
extern crate lazy_static;
use std::{collections::HashSet, fmt::Display, ops::Add};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos(i64, i64);

impl Add for Pos {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

lazy_static! {
    static ref COORDS: [Pos; 5] = [
        Pos(0, 0),
        Dir::Right.coord(),
        Dir::Down.coord(),
        Dir::Left.coord(),
        Dir::Up.coord(),
    ];
}

impl Dir {
    fn parse(input: char) -> Option<Dir> {
        match input {
            '#' => None,
            '.' => None,
            '>' => Some(Dir::Right),
            'v' => Some(Dir::Down),
            '<' => Some(Dir::Left),
            '^' => Some(Dir::Up),
            _ => panic!(),
        }
    }

    fn char(&self) -> char {
        match self {
            Dir::Right => '>',
            Dir::Down => 'v',
            Dir::Left => '<',
            Dir::Up => '^',
        }
    }

    fn coord(&self) -> Pos {
        match self {
            Dir::Right => Pos(1, 0),
            Dir::Down => Pos(0, 1),
            Dir::Left => Pos(-1, 0),
            Dir::Up => Pos(0, -1),
        }
    }
}

impl From<i64> for Dir {
    fn from(value: i64) -> Self {
        match value.rem_euclid(4) {
            0 => Dir::Right,
            1 => Dir::Down,
            2 => Dir::Left,
            3 => Dir::Up,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Map {
    rows: Vec<(u128, u128)>,
    cols: Vec<(u128, u128)>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#.{}", "#".repeat(self.cols.len()))?;
        for row in self.rows.iter().enumerate() {
            write!(f, "#")?;
            for col in self.cols.iter().enumerate() {
                let c = match self.count_bliz(&Pos(col.0 as i64, row.0 as i64)) {
                    None => '.',
                    Some(BlizCell::Single(dir)) => dir.char(),
                    Some(BlizCell::Multi(num)) => ((num % 4) + '0' as u8) as char,
                };
                write!(f, "{c}")?;
            }
            writeln!(f, "#")?;
        }
        writeln!(f, "{}.#", "#".repeat(self.cols.len()))
    }
}
type Todo = HashSet<Pos>;

#[derive(PartialEq)]
enum BlizCell {
    Single(Dir),
    Multi(u8),
}

impl Map {
    fn parse(input: &str) -> Map {
        let cells = input
            .split('\n')
            .filter(|s| !s.is_empty() && s[2..3] != *"#")
            .map(|line| {
                line.chars()
                    .filter(|c| *c != '#')
                    .map(Dir::parse)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let rows = cells
            .iter()
            .map(|row| {
                row.iter().enumerate().fold((0u128, 0u128), |r, v| -> _ {
                    if let Some(d) = v.1 {
                        let rbit = if *d == Dir::Right { 1 << v.0 } else { 0u128 };
                        let lbit = if *d == Dir::Left { 1 << v.0 } else { 0u128 };
                        (r.0 | rbit, r.1 | lbit)
                    } else {
                        r
                    }
                })
            })
            .collect::<Vec<_>>();
        let cols = (0..cells[0].len())
            .map(|col| {
                (0..cells.len()).fold((0u128, 0u128), |r, row| -> _ {
                    if let Some(d) = cells[row][col] {
                        let dbit = if d == Dir::Down { 1 << row } else { 0u128 };
                        let ubit = if d == Dir::Up { 1 << row } else { 0u128 };
                        (r.0 | dbit, r.1 | ubit)
                    } else {
                        r
                    }
                })
            })
            .collect::<Vec<_>>();
        Map { rows, cols }
    }

    fn start(&self) -> Pos {
        Pos(0, -1)
    }

    fn end(&self) -> Pos {
        Pos(self.cols.len() as i64 - 1, self.rows.len() as i64)
    }

    fn update(&self) -> Map {
        let row_end_bit = 1u128 << self.cols.len();
        let rows = self
            .rows
            .iter()
            .map(|row| {
                let mut r = row.0 << 1;
                if r & row_end_bit != 0 {
                    r = r & (row_end_bit - 1) | 1;
                }
                let mut l = if row.1 & 1 == 0 {
                    row.1
                } else {
                    row.1 | row_end_bit
                };
                l >>= 1;
                (r, l)
            })
            .collect::<Vec<_>>();

        let col_end_bit = 1u128 << self.rows.len();
        let cols = self
            .cols
            .iter()
            .map(|col| {
                let mut d = col.0 << 1;
                if d & col_end_bit != 0 {
                    d = d & (col_end_bit - 1) | 1;
                }
                let mut u = if col.1 & 1 == 0 {
                    col.1
                } else {
                    col.1 | col_end_bit
                };
                u >>= 1;
                (d, u)
            })
            .collect::<Vec<_>>();
        Map { rows, cols }
    }

    fn in_bounds(&self, pos: &Pos) -> bool {
        if pos.0 < 0
            || pos.1 < 0
            || pos.0 >= self.cols.len() as i64
            || pos.1 >= self.rows.len() as i64
        {
            false
        } else {
            true
        }
    }
    fn cell_empty(&self, pos: &Pos) -> bool {
        if *pos == self.start() || *pos == self.end() {
            true
        } else if !self.in_bounds(pos) {
            false
        } else {
            self.count_bliz(pos) == None
        }
    }

    fn count_bliz(&self, pos: &Pos) -> Option<BlizCell> {
        let mut count = 0;
        let mut dir = Dir::Right;
        let h = &self.rows[pos.1 as usize];
        let v = &self.cols[pos.0 as usize];
        if h.0 & (1 << pos.0) != 0 {
            count += 1;
            dir = Dir::Right;
        }
        if h.1 & (1 << pos.0) != 0 {
            count += 1;
            dir = Dir::Left;
        }
        if v.0 & (1 << pos.1) != 0 {
            count += 1;
            dir = Dir::Down;
        }
        if v.1 & (1 << pos.1) != 0 {
            count += 1;
            dir = Dir::Up;
        }
        match count {
            0 => None,
            1 => Some(BlizCell::Single(dir)),
            _ => Some(BlizCell::Multi(count)),
        }
    }

    fn iterate(&mut self, todo: &Todo) -> (Map, Todo) {
        let new_map = self.update();
        let new_todo = todo
            .iter()
            .map(|p| {
                COORDS
                    .iter()
                    .map(|dp| *p + *dp)
                    .filter(|p| new_map.cell_empty(p))
            })
            .flatten()
            .fold(Todo::default(), |mut new_todo, p| {
                new_todo.insert(p);
                new_todo
            });
        (new_map, new_todo)
    }

    fn find(&mut self, start: &Pos, end: &Pos) -> u32 {
        let mut todo: HashSet<Pos> = [*start].into();
        let mut count = 0u32;
        while !todo.contains(end) {
            count += 1;
            (*self, todo) = self.iterate(&todo);
        }
        count
    }
}

fn solve_part1(input: &str) -> String {
    let mut map = Map::parse(input);
    println!("{map}");
    map.find(&map.start(), &map.end()).to_string()
}

fn solve_part2(input: &str) -> String {
    let mut map = Map::parse(input);
    let mut total = 0u32;
    println!("{map}");
    total += map.find(&map.start(), &map.end());
    println!("{map}");
    total += map.find(&map.end(), &map.start());
    println!("{map}");
    total += map.find(&map.start(), &map.end());
    println!("{map}");
    total.to_string()
}

fn main() {
    let res = solve_part1(include_str!("input.txt"));
    println!("Part1: {res}");
    let res = solve_part2(include_str!("input.txt"));
    println!("Part2: {res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 18.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 54.to_string());
    }

    #[test]
    fn test_add_pos() {
        assert_eq!(Pos(1, 1) + Pos(2, 2), Pos(3, 3));
    }

    #[test]
    fn test_iterate() {
        let mut map = Map::parse(TEST_INPUT);
        let mut todo: HashSet<Pos> = [map.start()].into();
        (map, todo) = map.iterate(&todo);
        assert_eq!(todo, [Pos(0, 0), Pos(0, -1)].into());
        let mut todo: HashSet<Pos> = [Pos(0, 0)].into();
        (_, todo) = map.iterate(&todo);
        assert_eq!(todo, [Pos(0, 0), Pos(0, -1), Pos(0, 1)].into());
    }
    #[test]
    fn test_cell_empty() {
        let map = Map::parse(TEST_INPUT);

        // Entry points
        assert!(map.cell_empty(&Pos(0, -1)));
        assert!(map.cell_empty(&Pos(5, 4)));

        // Out of bounds
        assert!(!map.cell_empty(&Pos(6, 2)));
        assert!(!map.cell_empty(&Pos(-1, 2)));
        assert!(!map.cell_empty(&Pos(2, 4)));
        assert!(!map.cell_empty(&Pos(2, -1)));

        assert!(map.cell_empty(&Pos(0, 1)));
        assert!(!map.cell_empty(&Pos(0, 2)));
    }
    #[test]
    fn test_update() {
        let map = Map::parse(TEST_INPUT);
        let update = map.update();
        assert_eq!(
            update.rows,
            vec![
                (0b000110, 0b010100),
                (0b000000, 0b011001),
                (0b010011, 0b001000),
                (0b000001, 0b100000)
            ]
        );
        assert_eq!(
            update.cols,
            vec![
                (0b0000, 0b0000),
                (0b1000, 0b0100),
                (0b0001, 0b0000),
                (0b0000, 0b0100),
                (0b0000, 0b1100),
                (0b0000, 0b0000)
            ]
        );
    }

    #[test]
    fn test_parse() {
        let map = Map::parse(TEST_INPUT);
        assert_eq!(
            map.rows,
            vec![
                (0b000011, 0b101000),
                (0b000000, 0b110010),
                (0b101001, 0b010000),
                (0b100000, 0b000001)
            ]
        );
        assert_eq!(
            map.cols,
            vec![
                (0b0000, 0b0000),
                (0b0100, 0b1000),
                (0b1000, 0b0000),
                (0b0000, 0b1000),
                (0b0000, 0b1001),
                (0b0000, 0b0000)
            ]
        );
    }
}
