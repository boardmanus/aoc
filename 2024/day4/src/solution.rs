use std::ops::{Add, Sub};

struct Dir(i64, i64);
const DIRS: [Dir; 8] = [
    Dir(-1, -1),
    Dir(0, -1),
    Dir(1, -1),
    Dir(-1, 0),
    Dir(1, 0),
    Dir(-1, 1),
    Dir(0, 1),
    Dir(1, 1),
];

const SEARCH: &str = "XMAS";

fn first_char(s: &str) -> Option<char> {
    s.chars().nth(0)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Pos(i64, i64);

impl Pos {
    fn new(x: i64, y: i64) -> Pos {
        Pos { 0: x, 1: y }
    }

    fn next(&self, dir: &Dir) -> Pos {
        *self + dir
    }
}
impl Add<&Dir> for Pos {
    type Output = Pos;

    fn add(self, rhs: &Dir) -> Self::Output {
        Pos {
            0: self.0 + rhs.0,
            1: self.1 + rhs.1,
        }
    }
}

impl Sub for Pos {
    type Output = Dir;

    fn sub(self, rhs: Self) -> Self::Output {
        Dir(self.0 - rhs.0, self.1 - rhs.1)
    }
}

struct Grid {
    width: usize,
    height: usize,
    g: Vec<char>,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let rows_cols: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
        let width = rows_cols[0].len();
        let height = rows_cols.len();
        let g = rows_cols.iter().flatten().map(|x| *x).collect::<Vec<_>>();
        Grid { width, height, g }
    }

    fn at(&self, pos: &Pos) -> Option<char> {
        if (pos.0 as usize) < self.width && (pos.1 as usize) < self.height {
            Some(self.g[(pos.0 as usize) + (pos.1 as usize) * self.width])
        } else {
            None
        }
    }

    fn around(&self, pos: &Pos) -> Vec<Pos> {
        DIRS.iter().map(|d| *pos + d).collect()
    }

    fn pos_with_char(&self, c: char) -> Vec<Pos> {
        self.g
            .iter()
            .enumerate()
            .filter(|&(_i, &c2)| c == c2)
            .map(|(i, _c)| {
                Pos::new(
                    i.rem_euclid(self.width) as i64,
                    i.div_euclid(self.width) as i64,
                )
            })
            .collect()
    }

    fn count_matches_in_dir(&self, pos: &Pos, s: &str, dir: &Dir) -> usize {
        let mut next_pos = *pos + dir;
        if s.chars().all(|c| {
            let good = self.at(&next_pos) == Some(c);
            println!("--> {:?} '{c}'", next_pos);
            next_pos = next_pos + dir;
            good
        }) {
            1
        } else {
            0
        }
    }

    fn count_matches(&self, pos: &Pos, s: &str) -> usize {
        if let Some(c) = first_char(s) {
            self.around(pos)
                .into_iter()
                .filter(|pos2| self.at(pos2) == Some(c))
                .map(|pos2| {
                    println!(" -> '{c}' @ {:?}", pos2);
                    let count = self.count_matches_in_dir(&pos2, &s[1..], &(pos2 - *pos));
                    count
                })
                .sum()
        } else {
            println!("Found xmaS @ {:?}", pos);
            1
        }
    }

    fn at_match(&self, pos: &Pos, c: char) -> bool {
        self.at(pos) == Some(c)
    }
}

pub fn part1(input: &str) -> usize {
    let grid = Grid::parse(input);
    let start = grid.pos_with_char(first_char(SEARCH).unwrap());
    start
        .iter()
        .map(|pos| {
            let c = grid.count_matches(pos, &SEARCH[1..]);
            println!("{:?} = {c}", pos);
            c
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let grid = Grid::parse(input);
    let start = grid.pos_with_char('A');
    start
        .iter()
        .filter(|&pos| {
            ((grid.at_match(&pos.next(&Dir(-1, -1)), 'M')
                && grid.at_match(&pos.next(&Dir(1, 1)), 'S'))
                || (grid.at(&(*pos + &Dir(-1, -1))) == Some('S')
                    && grid.at(&(*pos + &Dir(1, 1))) == Some('M')))
                && ((grid.at(&(*pos + &Dir(-1, 1))) == Some('M')
                    && grid.at(&(*pos + &Dir(1, -1))) == Some('S'))
                    || (grid.at(&(*pos + &Dir(-1, 1))) == Some('S')
                        && grid.at(&(*pos + &Dir(1, -1))) == Some('M')))
        })
        .count()
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 18;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 9;

    #[test]
    fn test_grid_parse() {
        let g: Grid = Grid::parse("1234\n1234\n5678\n");
        assert_eq!(
            g.g,
            vec!['1', '2', '3', '4', '1', '2', '3', '4', '5', '6', '7', '8']
        );
    }

    #[test]
    fn test_grid_at() {
        let g: Grid = Grid::parse("1234\n1234\n5678\n");
        assert_eq!(g.at(&Pos::new(2, 1)), Some('3'));
    }

    #[test]
    fn test_grid_pos_with_char() {
        let g: Grid = Grid::parse("1234\n1234\n5678\n");
        assert_eq!(g.pos_with_char('4'), vec![Pos::new(3, 0), Pos::new(3, 1)]);
    }

    #[test]
    fn test_grid_count_matches() {
        let g: Grid = Grid::parse("1234\n1234\n5678\n");
        assert_eq!(g.count_matches(&Pos::new(2, 0), &"4"), 2);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
}
