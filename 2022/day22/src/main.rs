use std::{collections::HashMap, fmt::Display};

use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
struct Row(Vec<Option<bool>>);

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in &self.0 {
            write!(
                f,
                "{}",
                if let Some(c) = *b {
                    if c {
                        '.'
                    } else {
                        '#'
                    }
                } else {
                    ' '
                }
            )?;
        }
        Ok(())
    }
}

impl Row {
    fn parse(line: &str, width: usize) -> Row {
        let mut row = Vec::<Option<bool>>::with_capacity(width);
        for c in line.chars() {
            row.push(match c {
                '.' => Some(true),
                '#' => Some(false),
                _ => None,
            })
        }
        for _ in row.len()..width {
            row.push(None);
        }
        Row(row)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Rotate {
    Left,
    Right,
}

impl Rotate {
    fn val(&self) -> i64 {
        match self {
            Rotate::Left => -1,
            Rotate::Right => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cmd {
    Rot(Rotate),
    Fwd(usize),
}

impl Cmd {
    fn parse(line: &str) -> Vec<Cmd> {
        let re = Regex::new(r"L|R|\d+").unwrap();
        line.split_inclusive(&re)
            .flat_map(|x| match x {
                "L" => Some(Cmd::Rot(Rotate::Left)),
                "R" => Some(Cmd::Rot(Rotate::Right)),
                "" => None,
                _ => Some(Cmd::Fwd(x.parse().unwrap())),
            })
            .collect()
    }
}

type Path = HashMap<(i64, i64), Dir>;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Face(i64, i64, Dir);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Teleporter(Face, Face);

impl Teleporter {
    fn transform(&self, pos: &Pos, len: usize) -> Pos {
        let l = len as i64;
        let op = Face(pos.x % l, pos.y % l, pos.dir);
        let tl = Face(self.1 .0 * l, self.1 .1 * l, self.1 .2.opposite());
        let br = Face(tl.0 + l - 1, tl.1 + l - 1, tl.2);
        match (self.0 .2, self.1 .2) {
            (Dir::Right, Dir::Left) => Pos::new(tl.0, tl.1 + op.1, tl.2),
            (Dir::Right, Dir::Right) => Pos::new(br.0, br.1 - op.1, tl.2),
            (Dir::Right, Dir::Up) => Pos::new(br.0 - op.1, tl.1, tl.2),
            (Dir::Right, Dir::Down) => Pos::new(tl.0 + op.1, br.1, tl.2),

            (Dir::Down, Dir::Left) => Pos::new(tl.0, br.1 - op.0, tl.2),
            (Dir::Down, Dir::Right) => Pos::new(br.0, tl.1 + op.0, tl.2),
            (Dir::Down, Dir::Up) => Pos::new(tl.0 + op.0, tl.1, tl.2),
            (Dir::Down, Dir::Down) => Pos::new(br.0 - op.0, br.1, tl.2),

            (Dir::Left, Dir::Left) => Pos::new(tl.0, br.1 - op.1, tl.2),
            (Dir::Left, Dir::Right) => Pos::new(br.0, tl.1 + op.1, tl.2),
            (Dir::Left, Dir::Up) => Pos::new(tl.0 + op.1, tl.1, tl.2),
            (Dir::Left, Dir::Down) => Pos::new(br.0 - op.1, br.1, tl.2),

            (Dir::Up, Dir::Left) => Pos::new(tl.0, tl.1 + op.0, tl.2),
            (Dir::Up, Dir::Right) => Pos::new(br.0, br.1 - op.0, tl.2),
            (Dir::Up, Dir::Up) => Pos::new(br.0 - op.0, tl.1, tl.2),
            (Dir::Up, Dir::Down) => Pos::new(tl.0 + op.0, br.1, tl.2),
        }
    }
}

struct Map {
    cubic: bool,
    sqr_size: usize,
    rows: Vec<Row>,
    teleporters: HashMap<Face, Teleporter>,
}

const TELEPORTERS_4: [Teleporter; 14] = [
    Teleporter(Face(2, 0, Dir::Left), Face(1, 1, Dir::Up)), //], +1 //2
    Teleporter(Face(2, 0, Dir::Up), Face(0, 1, Dir::Up)),   //], -1),  //4
    Teleporter(Face(2, 0, Dir::Right), Face(3, 2, Dir::Right)), //], -1),
    Teleporter(Face(2, 1, Dir::Right), Face(3, 2, Dir::Up)), //], -1), //2
    Teleporter(Face(3, 2, Dir::Up), Face(2, 1, Dir::Right)), //], -1), //2
    Teleporter(Face(3, 2, Dir::Right), Face(2, 0, Dir::Right)), //], -1), //4
    Teleporter(Face(3, 2, Dir::Down), Face(0, 1, Dir::Left)), //], -1), //6
    Teleporter(Face(2, 2, Dir::Down), Face(0, 1, Dir::Down)), //], -1), //4
    Teleporter(Face(2, 2, Dir::Left), Face(1, 1, Dir::Down)), //], -1), //2
    Teleporter(Face(1, 1, Dir::Down), Face(2, 2, Dir::Up)), //], -1),  //2
    Teleporter(Face(0, 1, Dir::Down), Face(2, 2, Dir::Down)), //], -1), //4
    Teleporter(Face(0, 1, Dir::Left), Face(2, 2, Dir::Down)), //], -1), //6
    Teleporter(Face(0, 1, Dir::Up), Face(2, 0, Dir::Up)),   //], -1),    //4
    Teleporter(Face(1, 1, Dir::Up), Face(2, 0, Dir::Left)), //], 1),   //2
];

const TELEPORTERS_50: [Teleporter; 14] = [
    Teleporter(Face(1, 0, Dir::Left), Face(0, 2, Dir::Left)),
    Teleporter(Face(1, 0, Dir::Up), Face(0, 3, Dir::Left)),
    Teleporter(Face(2, 0, Dir::Up), Face(0, 3, Dir::Down)), //], -1),
    Teleporter(Face(2, 0, Dir::Right), Face(1, 2, Dir::Right)), //], -1), //2
    Teleporter(Face(2, 0, Dir::Down), Face(1, 1, Dir::Right)), //], -1), //2
    Teleporter(Face(1, 1, Dir::Right), Face(2, 0, Dir::Down)), //], -1), //4
    Teleporter(Face(1, 2, Dir::Right), Face(2, 0, Dir::Right)), //], -1), //6
    Teleporter(Face(1, 2, Dir::Down), Face(0, 3, Dir::Right)), //], -1), //4
    Teleporter(Face(0, 3, Dir::Right), Face(1, 2, Dir::Down)), //], -1), //2
    Teleporter(Face(0, 3, Dir::Down), Face(2, 0, Dir::Up)), //], -1),  //2
    Teleporter(Face(0, 3, Dir::Left), Face(1, 0, Dir::Up)), //], -1), //4
    Teleporter(Face(0, 2, Dir::Left), Face(1, 0, Dir::Left)), //], -1), //6
    Teleporter(Face(0, 2, Dir::Up), Face(1, 1, Dir::Left)), //], -1),    //4
    Teleporter(Face(1, 1, Dir::Left), Face(0, 2, Dir::Up)), //], 1),   //2
];

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            writeln!(f, "{row}")?;
        }
        writeln!(f)?;
        Ok(())
    }
}
impl Map {
    fn print_path(&self, path: &Path) {
        for y in 0..self.rows.len() {
            for x in 0..self.rows[0].0.len() {
                match self.rows[y].0[x] {
                    None => print!(" "),
                    Some(false) => print!("#"),
                    Some(true) => {
                        if let Some(step) = path.get(&(x as i64, y as i64)) {
                            print!("{step}")
                        } else {
                            print!(".");
                        }
                    }
                }
            }
            println!();
        }
        println!();
    }

    fn start_pos(&self) -> Pos {
        let row = &self.rows[0];
        let i = row
            .0
            .iter()
            .enumerate()
            .find(|x| x.1.unwrap_or(false))
            .unwrap();
        Pos {
            x: i.0 as i64,
            y: 0,
            dir: Dir::Right,
        }
    }

    fn next(&self, pos: &Pos) -> Option<Pos> {
        let dir = pos.dir.dir();
        let x = pos.x + dir.0;
        let y = pos.y + dir.1;
        if x < 0 || x >= (self.rows[0].0.len() as i64) || y < 0 || y >= (self.rows.len() as i64) {
            return self.teleport(pos);
        }
        if let Some(g) = self.rows[y as usize].0[x as usize] {
            if g {
                Some(Pos { x, y, dir: pos.dir })
            } else {
                None
            }
        } else {
            self.teleport(pos)
        }
    }

    fn cell_pos(&self, old_pos: &Pos, new_pos: &Pos) -> Option<Pos> {
        let cell = self.rows[new_pos.y as usize].0[new_pos.x as usize];
        if let Some(c) = cell {
            if c {
                Some(*new_pos)
            } else {
                Some(*old_pos)
            }
        } else {
            None
        }
    }

    fn teleport_cubic(&self, pos: &Pos) -> Option<Pos> {
        let teleporter = self.teleporters[&pos.into(self.sqr_size)];
        let new_pos = teleporter.transform(pos, self.sqr_size);
        println!(
            "teleport from {:?}|{:?} <-> {:?}|{:?}",
            pos, teleporter.0, teleporter.1, new_pos
        );
        self.cell_pos(pos, &new_pos)
    }

    fn teleport_flat(&self, pos: &Pos) -> Option<Pos> {
        match pos.dir {
            Dir::Right => (0..(self.rows[0].0.len() / self.sqr_size)).find_map(|i| {
                self.cell_pos(pos, &Pos::new((i * self.sqr_size) as i64, pos.y, pos.dir))
            }),
            Dir::Down => (0..(self.rows.len() / self.sqr_size)).find_map(|i| {
                self.cell_pos(pos, &Pos::new(pos.x, (i * self.sqr_size) as i64, pos.dir))
            }),
            Dir::Left => (1..(self.rows[0].0.len() / self.sqr_size + 1))
                .rev()
                .find_map(|i| {
                    self.cell_pos(
                        pos,
                        &Pos::new((i * self.sqr_size - 1) as i64, pos.y, pos.dir),
                    )
                }),
            Dir::Up => (1..(self.rows.len() / self.sqr_size + 1))
                .rev()
                .find_map(|i| {
                    self.cell_pos(
                        pos,
                        &Pos::new(pos.x, (i * self.sqr_size - 1) as i64, pos.dir),
                    )
                }),
        }
    }
    fn teleport(&self, pos: &Pos) -> Option<Pos> {
        if self.cubic {
            self.teleport_cubic(pos)
        } else {
            self.teleport_flat(pos)
        }
    }

    fn parse(input: &str, cubic: bool) -> Map {
        let lines = input.split('\n');
        let width = lines.clone().map(|l| l.len()).max().unwrap();
        let rows = lines.map(|s| Row::parse(s, width)).collect::<Vec<_>>();
        let sqr_size = if rows.len() / 50 > 0 && rows[0].0.len() > 50 {
            50
        } else {
            4
        };
        let mut teleporters = HashMap::<Face, Teleporter>::default();
        if sqr_size == 4 {
            TELEPORTERS_4
        } else {
            TELEPORTERS_50
        }
        .iter()
        .for_each(|t| {
            teleporters.insert(t.0, *t);
        });
        Map {
            cubic,
            sqr_size,
            rows,
            teleporters,
        }
    }

    fn operate(&self, pos: &Pos, cmd: Cmd, steps: &mut Path) -> Pos {
        let next_pos = match cmd {
            Cmd::Rot(dir) => Pos {
                x: pos.x,
                y: pos.y,
                dir: pos.dir.rotate(dir),
            },
            Cmd::Fwd(dist) => {
                let mut new_pos = *pos;
                for _ in 0..dist {
                    if let Some(p) = self.next(&new_pos) {
                        steps.insert((p.x, p.y), p.dir);
                        new_pos = p;
                    } else {
                        break;
                    }
                }
                new_pos
            }
        };
        steps.insert((next_pos.x, next_pos.y), next_pos.dir);
        next_pos
    }

    fn walk(&self, cmds: &[Cmd]) -> (Pos, Path) {
        let mut path = Path::default();
        let start_pos = self.start_pos();
        path.insert((start_pos.x, start_pos.y), start_pos.dir);
        let pos = cmds.iter().fold(start_pos, |pos, cmd| match *cmd {
            Cmd::Rot(dir) => {
                let p = Pos {
                    x: pos.x,
                    y: pos.y,
                    dir: pos.dir.rotate(dir),
                };
                path.insert((p.x, p.y), p.dir);
                p
            }
            Cmd::Fwd(_) => self.operate(&pos, *cmd, &mut path),
        });
        (pos, path)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl From<i64> for Dir {
    fn from(value: i64) -> Self {
        match value {
            0 => Dir::Right,
            1 => Dir::Down,
            2 => Dir::Left,
            3 => Dir::Up,
            _ => panic!(),
        }
    }
}
impl Dir {
    fn dir(&self) -> (i64, i64) {
        match self {
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Up => (0, -1),
        }
    }

    fn opposite(&self) -> Dir {
        (*self as i64 + 2).rem_euclid(4).into()
    }

    fn rotate(&self, dir: Rotate) -> Dir {
        (dir.val() + *self as i64).rem_euclid(4).into()
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Dir::Right => '>',
            Dir::Down => 'v',
            Dir::Left => '<',
            Dir::Up => '^',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos {
    x: i64,
    y: i64,
    dir: Dir,
}

impl Pos {
    fn new(x: i64, y: i64, dir: Dir) -> Pos {
        Pos { x, y, dir }
    }

    fn into(&self, len: usize) -> Face {
        Face(self.x / len as i64, self.y / len as i64, self.dir)
    }

    fn password(&self) -> usize {
        (self.y as usize + 1) * 1000 + (self.x as usize + 1) * 4 + self.dir as usize
    }
}

fn parse(input: &str, cubic: bool) -> (Map, Vec<Cmd>) {
    let partition = input.split("\n\n").collect::<Vec<_>>();
    assert_eq!(partition.len(), 2);
    (
        Map::parse(partition[0], cubic),
        Cmd::parse(partition[1].trim()),
    )
}

fn solve_part1(input: &str) -> String {
    let (map, cmds) = parse(input, false);
    let (final_pos, path) = map.walk(&cmds);
    map.print_path(&path);
    println!("Final Pos: {:?}", final_pos);
    final_pos.password().to_string()
}

fn solve_part2(input: &str) -> String {
    let (map, cmds) = parse(input, true);
    let (final_pos, path) = map.walk(&cmds);
    map.print_path(&path);
    println!("Final Pos: {:?}", final_pos);
    final_pos.password().to_string()
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

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 6032.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 5031.to_string());
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            Row::parse("     ...#...", 12),
            Row(vec![
                None,
                None,
                None,
                None,
                None,
                Some(true),
                Some(true),
                Some(true),
                Some(false),
                Some(true),
                Some(true),
                Some(true)
            ])
        );
    }

    #[test]
    fn test_parse_cmds() {
        assert_eq!(
            Cmd::parse("10R5L5R10L4R5L5"),
            vec![
                Cmd::Fwd(10),
                Cmd::Rot(Rotate::Right),
                Cmd::Fwd(5),
                Cmd::Rot(Rotate::Left),
                Cmd::Fwd(5),
                Cmd::Rot(Rotate::Right),
                Cmd::Fwd(10),
                Cmd::Rot(Rotate::Left),
                Cmd::Fwd(4),
                Cmd::Rot(Rotate::Right),
                Cmd::Fwd(5),
                Cmd::Rot(Rotate::Left),
                Cmd::Fwd(5)
            ]
        )
    }

    #[test]
    fn test_parse_input() {
        let res = parse(TEST_INPUT, false);
        println!("{}", res.0);
        assert_eq!(res.0.rows.len(), 12);
        assert_eq!(res.1.len(), 13);
    }
}
