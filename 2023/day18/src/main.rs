use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    num::ParseIntError,
    str::FromStr,
};

#[derive(Debug)]
enum ParseError {
    BadDir,
    BadColour,
    // We will defer to the parse error implementation for their error.
    // Supplying extra info requires adding more data to the type.
    BadHex(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> ParseError {
        ParseError::BadHex(err)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    R,
    D,
    L,
    U,
}

impl Dir {
    fn from_val(val: u64) -> Self {
        match val {
            0 => Dir::R,
            1 => Dir::D,
            2 => Dir::L,
            3 => Dir::U,
            _ => panic!("Bad dir value"),
        }
    }
}

impl FromStr for Dir {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Dir::R),
            "L" => Ok(Dir::L),
            "U" => Ok(Dir::U),
            "D" => Ok(Dir::D),
            _ => Err(ParseError::BadDir),
        }
    }
}

fn colour_from_str(s: &str) -> Result<u64, ParseError> {
    if s.len() == 9 {
        let c = u64::from_str_radix(&s[2..s.len() - 1], 16)?;
        Ok(c)
    } else {
        Err(ParseError::BadColour)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Dig {
    dir: Dir,
    steps: i64,
    color: u64,
}

impl Dig {
    fn new(dir: Dir, steps: i64, color: u64) -> Self {
        Dig { dir, steps, color }
    }

    fn hack(&self) -> Self {
        let instruction = self.color;
        Dig {
            dir: Dir::from_val(instruction & 0x3),
            steps: instruction as i64 >> 4,
            color: 0,
        }
    }
}

impl FromStr for Dig {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let dir = Dir::from_str(parts.next().unwrap_or(""))?;
        let steps = parts.next().unwrap_or("").parse::<i64>()?;
        let color = colour_from_str(parts.next().unwrap_or(""))?;
        Ok(Dig { dir, steps, color })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    const ORIGIN: Pos = Pos { x: 0, y: 0 };

    fn new(x: i64, y: i64) -> Self {
        Pos { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Pos,
    end: Pos,
}

impl Line {
    fn new(start: Pos, end: Pos) -> Self {
        Line { start, end }
    }

    fn rev(&self) -> Line {
        Line::new(self.end, self.start)
    }
}

#[derive(Debug)]
struct Lines {
    border_count: i64,
    points: Vec<Pos>,
}

impl Lines {
    fn from(old_moves: Vec<Dig>) -> Self {
        let mut horz: Vec<Line> = Vec::new();
        let moves = old_moves.iter().map(|dig| dig.hack()).collect::<Vec<_>>();
        let mut cur = Pos::ORIGIN;
        let mut end = Pos::ORIGIN;
        let mut border_count = 0;
        let mut points = Vec::<Pos>::new();
        for dig in moves {
            end = match dig.dir {
                Dir::R => Pos::new(cur.x + dig.steps, cur.y),
                Dir::L => Pos::new(cur.x - dig.steps, cur.y),
                Dir::U => Pos::new(cur.x, cur.y - dig.steps),
                Dir::D => Pos::new(cur.x, cur.y + dig.steps),
            };
            border_count += dig.steps;
            let line = Line::new(cur, end);
            match dig.dir {
                Dir::R => horz.push(line),
                Dir::L => horz.push(line.rev()),
                _ => (),
            }
            points.push(cur);
            cur = end;
        }
        assert_eq!(end, Pos::ORIGIN);
        points.push(Pos::ORIGIN);

        Lines {
            border_count,
            points,
        }
    }

    fn area(&self) -> i64 {
        let mut area = 0;
        for i in 0..self.points.len() - 1 {
            let p1 = self.points[i];
            let p2 = self.points[i + 1];
            area += p1.x * p2.y - p2.x * p1.y;
        }
        (area + self.border_count) / 2 + 1
    }
}

struct Map {
    holes: HashSet<Pos>,
    cur: Pos,
    min: Pos,
    max: Pos,
}

impl Map {
    fn new() -> Self {
        Map {
            holes: HashSet::from([]),
            cur: Pos::ORIGIN,
            min: Pos::ORIGIN,
            max: Pos::ORIGIN,
        }
    }
    fn from_hole_str(s: &str) -> Self {
        let mut map = Map::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    let pos = Pos::new(x as i64, y as i64);
                    map.dig_at(&pos);
                }
            }
        }
        map
    }

    fn from(moves: Vec<Dig>) -> Self {
        let mut map = Map::new();
        map.dig_at(&Pos::ORIGIN);
        for dig in moves {
            map.dig(dig);
        }
        map
    }

    fn num_holes(&self) -> usize {
        self.holes.len()
    }

    fn update_min_max(&mut self, pos: &Pos) {
        self.min.x = self.min.x.min(pos.x);
        self.min.y = self.min.y.min(pos.y);
        self.max.x = self.max.x.max(pos.x);
        self.max.y = self.max.y.max(pos.y);
    }

    fn dig_at(&mut self, pos: &Pos) {
        self.holes.insert(*pos);
        self.update_min_max(pos);
    }

    fn dig(&mut self, dig: Dig) -> Pos {
        let pos = &mut self.cur;
        for _ in 0..dig.steps {
            match dig.dir {
                Dir::R => pos.x += 1,
                Dir::L => pos.x -= 1,
                Dir::U => pos.y -= 1,
                Dir::D => pos.y += 1,
            }
            self.holes.insert(*pos);
        }
        self.min.x = self.min.x.min(pos.x);
        self.min.y = self.min.y.min(pos.y);
        self.max.x = self.max.x.max(pos.x);
        self.max.y = self.max.y.max(pos.y);
        *pos
    }

    fn dig_interior(&mut self) -> usize {
        let interior = self.interior();
        for pos in interior {
            self.dig_at(&pos);
        }
        self.num_holes()
    }

    fn interior(&self) -> HashSet<Pos> {
        let mut fill: HashSet<Pos> = Default::default();

        for y in self.min.y..=self.max.y {
            let mut inside = false;
            let mut start_v: Option<i32> = None;
            for x in self.min.x..=self.max.x {
                if self.holes.contains(&Pos::new(x, y)) {
                    if start_v.is_none() {
                        let mut start = 0;
                        if self.holes.contains(&Pos::new(x, y - 1)) {
                            start -= 1
                        }
                        if self.holes.contains(&Pos::new(x, y + 1)) {
                            start += 1;
                        }
                        start_v = Some(start);
                    }
                } else {
                    if let Some(start) = start_v {
                        let mut end = 0;
                        if self.holes.contains(&Pos::new(x - 1, y - 1)) {
                            end -= 1
                        }
                        if self.holes.contains(&Pos::new(x - 1, y + 1)) {
                            end += 1;
                        }
                        if start + end == 0 {
                            inside = !inside;
                        }
                        start_v = None;
                    }

                    if inside {
                        fill.insert(Pos::new(x, y));
                    }
                }
            }
        }
        fill
    }
}

impl ToString for Map {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for y in self.min.y..=self.max.y {
            for x in self.min.x..=self.max.x {
                if self.holes.contains(&Pos::new(x, y)) {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }
        s
    }
}

fn parse(input: &str) -> Vec<Dig> {
    input
        .lines()
        .map(|line| Dig::from_str(line))
        .flatten()
        .collect::<Vec<_>>()
}

fn solve_part1(input: &str) -> usize {
    let moves = parse(input);
    let mut map = Map::from(moves);
    println!("{}", map.to_string());
    let holes = map.dig_interior();
    println!("{}", map.to_string());
    holes
}

fn solve_part2(input: &str) -> usize {
    let moves = parse(input);
    let lines = Lines::from(moves);
    println!("{:?}", lines.points);
    lines.area() as usize
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
        assert_eq!(solve_part1(TEST_INPUT), 62);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 952408144115);
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("R 6 (#70c710)"), vec![Dig::new(Dir::R, 6, 0x70c710)]);
    }

    #[test]
    fn test_boundary() {
        let moves = parse(TEST_INPUT);
        let map = Map::from(moves);
        assert_eq!(map.to_string(), include_str!("test_input_boundary.txt"));
        println!("min={:?}, max={:?}", map.min, map.max);
    }

    #[test]
    fn test_dir_hack() {
        let dig = Dig::new(Dir::R, 6, 0x70c710);
        assert_eq!(dig.hack(), Dig::new(Dir::R, 0x70c71, 0));
    }

    #[test]
    fn test_lines_from() {
        let digs = parse("R 6 (#000040)\nR 6 (#000041)\nR 6 (#000042)\nR 6 (#000043)\n");
        let lines = Lines::from(digs);
        println!("{:?}", lines.points);
        println!("count={}", lines.area());
    }

    #[test]
    fn test_boundary_edge() {
        let hole_str = "..##...
#######
#..####
####...
..##...
";
        let mut map = Map::from_hole_str(hole_str);
        println!("{}", map.to_string());
        map.dig_interior();
        println!("{}", map.to_string());
        assert_eq!(
            map.to_string(),
            "..##...
#######
#######
####...
..##...
"
        );
        println!("min={:?}, max={:?}", map.min, map.max);
    }
}
