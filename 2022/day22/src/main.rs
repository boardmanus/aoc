use std::fmt::Display;

use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
struct Row {
    start: usize,
    ground: Vec<bool>,
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.start {
            write!(f, " ")?;
        }
        for b in &self.ground {
            write!(f, "{}", if *b { '.' } else { '#' })?;
        }
        Ok(())
    }
}

impl Row {
    fn new(start: usize, ground: &[bool]) -> Row {
        Row {
            start,
            ground: ground.into(),
        }
    }

    fn parse(line: &str) -> Row {
        let start = line.find(&['.', '#']).unwrap();
        let ground_str = &line[start..line.len()];
        let mut ground = Vec::<bool>::with_capacity(ground_str.len());
        for (i, c) in ground_str.chars().enumerate() {
            ground.push(c == '.');
        }
        Row { start, ground }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cmd {
    Left,
    Right,
    Fwd(usize),
}

impl Cmd {
    fn parse(line: &str) -> Vec<Cmd> {
        let re = Regex::new(r"L|R|\d+").unwrap();
        line.split_inclusive(&re)
            .flat_map(|x| match x {
                "L" => Some(Cmd::Left),
                "R" => Some(Cmd::Right),
                "" => None,
                _ => Some(Cmd::Fwd(x.parse().unwrap())),
            })
            .collect()
    }
}

type Col = Row;
struct Map {
    rows: Vec<Row>,
    cols: Vec<Col>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            writeln!(f, "{row}")?;
        }
        writeln!(f)?;
        for col in &self.cols {
            writeln!(f, "{col}")?;
        }
        Ok(())
    }
}
impl Map {
    fn start_pos(&self) -> Pos {
        let row = &self.rows[0];
        let i = row.ground.iter().enumerate().find(|x| *x.1).unwrap();
        Pos {
            x: (i.0 + row.start) as i64,
            y: 0,
            dir: Dir::Right,
        }
    }

    fn next(&self, pos: &Pos) -> Option<Pos> {
        match pos.dir {
            Dir::Right => {
                let row = &self.rows[pos.y as usize];
                let mut x = pos.x + 1;
                if x as usize >= row.start + row.ground.len() {
                    x = row.start as i64;
                }
                if row.ground[x as usize - row.start] {
                    Some(Pos {
                        x,
                        y: pos.y,
                        dir: pos.dir,
                    })
                } else {
                    None
                }
            }
            Dir::Down => {
                let col = &self.cols[pos.x as usize];
                let mut y = pos.y + 1;
                if y as usize >= col.start + col.ground.len() {
                    y = col.start as i64;
                }
                if col.ground[y as usize - col.start] {
                    Some(Pos {
                        x: pos.x,
                        y,
                        dir: pos.dir,
                    })
                } else {
                    None
                }
            }
            Dir::Left => {
                let row = &self.rows[pos.y as usize];
                let mut x = pos.x - 1;
                if x < row.start as i64 {
                    x = (row.start + row.ground.len() - 1) as i64;
                }
                if row.ground[x as usize - row.start] {
                    Some(Pos {
                        x,
                        y: pos.y,
                        dir: pos.dir,
                    })
                } else {
                    None
                }
            }
            Dir::Up => {
                let col = &self.cols[pos.x as usize];
                let mut y = pos.y - 1;
                if y < col.start as i64 {
                    y = (col.start + col.ground.len() - 1) as i64;
                }
                if col.ground[y as usize - col.start] {
                    Some(Pos {
                        x: pos.x,
                        y,
                        dir: pos.dir,
                    })
                } else {
                    None
                }
            }
        }
    }

    fn cols_from_rows(rows: &Vec<Row>) -> Vec<Col> {
        let mut cols = Vec::<Col>::default();

        for row in rows.iter().enumerate() {
            for col_idx in 0..(row.1.start + row.1.ground.len()) {
                if let Some(col) = cols.get_mut(col_idx) {
                    if col_idx < row.1.start {
                        col.start += 1;
                    } else {
                        assert_eq!(col.start + col.ground.len(), row.0);
                        col.ground.push(row.1.ground[col_idx - row.1.start]);
                    }
                } else {
                    let mut col = Col::new(row.0, Default::default());
                    if col_idx < row.1.start {
                        col.start += 1;
                    } else {
                        assert_eq!(col.start + col.ground.len(), row.0);
                        col.ground.push(row.1.ground[col_idx - row.1.start]);
                    }
                    cols.push(col);
                }
            }
        }
        cols
    }

    fn parse(input: &str) -> Map {
        let rows = input.split('\n').map(|s| Row::parse(s)).collect();
        let cols = Map::cols_from_rows(&rows);
        Map { rows, cols }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}
struct Pos {
    x: i64,
    y: i64,
    dir: Dir,
}

impl Pos {
    fn password(&self) -> usize {
        (self.y as usize + 1) * 1000 + (self.x as usize + 1) * 4 + self.dir as usize
    }
}

fn parse(input: &str) -> (Map, Vec<Cmd>) {
    let partition = input.split("\n\n").collect::<Vec<_>>();
    assert_eq!(partition.len(), 2);
    (Map::parse(partition[0]), Cmd::parse(partition[1].trim()))
}

fn new_dir(curr_dir: Dir, cmd: Cmd) -> Dir {
    match curr_dir {
        Dir::Right => {
            if cmd == Cmd::Left {
                Dir::Up
            } else {
                Dir::Down
            }
        }
        Dir::Left => {
            if cmd == Cmd::Left {
                Dir::Down
            } else {
                Dir::Up
            }
        }
        Dir::Up => {
            if cmd == Cmd::Left {
                Dir::Left
            } else {
                Dir::Right
            }
        }
        Dir::Down => {
            if cmd == Cmd::Left {
                Dir::Right
            } else {
                Dir::Left
            }
        }
    }
}
fn solve_part1(input: &str) -> String {
    let (map, cmds) = parse(input);
    let start = map.start_pos();
    let pos = cmds.iter().fold(start, |pos, cmd| {
        let next_pos = match *cmd {
            Cmd::Left | Cmd::Right => Pos {
                x: pos.x,
                y: pos.y,
                dir: new_dir(pos.dir, *cmd),
            },
            Cmd::Fwd(dist) => {
                let mut new_pos = pos;
                for _ in 0..dist {
                    if let Some(p) = map.next(&new_pos) {
                        new_pos = p;
                    } else {
                        break;
                    }
                }
                new_pos
            }
        };
        next_pos
    });
    pos.password().to_string()
}

fn solve_part2(input: &str) -> String {
    0.to_string()
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
        assert_eq!(solve_part2(TEST_INPUT), 0.to_string());
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            Row::parse("     ...#..."),
            Row::new(5, &[true, true, true, false, true, true, true])
        );
    }

    #[test]
    fn test_parse_cmds() {
        assert_eq!(
            Cmd::parse("10R5L5R10L4R5L5"),
            vec![
                Cmd::Fwd(10),
                Cmd::Right,
                Cmd::Fwd(5),
                Cmd::Left,
                Cmd::Fwd(5),
                Cmd::Right,
                Cmd::Fwd(10),
                Cmd::Left,
                Cmd::Fwd(4),
                Cmd::Right,
                Cmd::Fwd(5),
                Cmd::Left,
                Cmd::Fwd(5)
            ]
        )
    }

    #[test]
    fn test_parse_input() {
        let res = parse(TEST_INPUT);
        println!("{}", res.0);
        assert_eq!(res.0.rows.len(), 12);
        assert_eq!(res.0.cols.len(), 12);
        assert_eq!(res.1.len(), 13);
    }
}
