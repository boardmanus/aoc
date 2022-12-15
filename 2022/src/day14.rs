use std::{fmt, fmt::Display, str::FromStr};

use crate::aoc::Aoc;
use itertools::Itertools;
use nom::{
    self, bytes::complete::tag, character::complete::digit1, combinator::map_res,
    multi::separated_list0, sequence::separated_pair, IResult,
};

type Pos = (i32, i32);
type Formation = Vec<Pos>;

fn num_val(i: &str) -> Result<i32, <i32 as FromStr>::Err> {
    let num = i.parse::<i32>()?;
    Ok(num)
}

fn parse_pos(s: &str) -> IResult<&str, Pos> {
    let vals = separated_pair(map_res(digit1, num_val), tag(","), map_res(digit1, num_val))(s)?;
    Ok(vals)
}
fn parse_formations(s: &str) -> IResult<&str, Formation> {
    let p = separated_list0(tag(" -> "), parse_pos)(s)?;
    Ok(p)
}

fn lines_to_formations(lines: &[String]) -> Vec<Formation> {
    lines
        .iter()
        .map(|line| parse_formations(line).unwrap().1)
        .collect_vec()
}

fn analyse_formations(formations: &[Formation]) {
    let min = min_pos(formations);
    let max = max_pos(formations);
    println!("Formation: min={min:?}, max={max:?}");
}

fn max_pos(formations: &[Formation]) -> Pos {
    formations
        .iter()
        .map(|f| f.iter().fold((0, 0), |m, p| (m.0.max(p.0), m.1.max(p.1))))
        .fold((0, 0), |m, p| (m.0.max(p.0), m.1.max(p.1)))
}

fn min_pos(formations: &[Formation]) -> Pos {
    formations
        .iter()
        .map(|f| {
            f.iter()
                .fold((i32::MAX, i32::MAX), |m, p| (m.0.min(p.0), m.1.min(p.1)))
        })
        .fold((i32::MAX, i32::MAX), |m, p| (m.0.min(p.0), m.1.min(p.1)))
}

struct Map {
    minx: i32,
    max: Pos,
    stride: i32,
    m: Vec<char>,
}

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.m
            .chunks(self.stride.try_into().unwrap())
            .fold(Ok(()), |_, row| {
                let (s, e) = if row.len() > 80 {
                    (row.len() / 2 - 40, row.len() / 2 + 40)
                } else {
                    (0, row.len())
                };
                writeln!(f, "{}", row[s..e].iter().collect::<String>())
            })
    }
}
impl Map {
    fn new(formations: &[Formation]) -> Self {
        let min = min_pos(formations);
        let max = max_pos(formations);
        let stride = max.0 - min.0 + 1;
        let len = stride * (max.1 + 1);
        let mut map = Map {
            minx: min.0,
            max,
            stride,
            m: vec!['.'; len.try_into().unwrap()],
        };
        formations.iter().for_each(|f| map.add_formations(f));
        map
    }

    fn lower(&self, pos: Pos) -> Option<Pos> {
        if pos.1 >= self.max.1 || pos.0 < 0 || pos.0 >= self.max.0 {
            None
        } else if let Some(new_pos) = [0, -1, 1]
            .iter()
            .map(|dx| (pos.0 + dx, pos.1 + 1))
            .find(|p| !self.is_solid(p))
        {
            Some(new_pos)
        } else {
            Some(pos)
        }
    }

    fn drop(&self, sand_pos: Pos) -> Option<Pos> {
        let mut curr_pos = sand_pos;
        while let Some(new_pos) = self.lower(curr_pos) {
            if new_pos == curr_pos {
                return Some(curr_pos);
            }
            curr_pos = new_pos
        }
        None
    }

    fn fill(&mut self, sand_pos: Pos) -> usize {
        let mut amount = 0;
        while let Some(spos) = self.drop(sand_pos) {
            amount += 1;
            self.add(&spos, 'o');
            if spos == (500, 0) {
                break;
            }
        }
        amount
    }

    fn add(&mut self, p: &Pos, c: char) {
        let i = self.index(p);
        self.m[i] = c;
    }

    fn add_line(&mut self, p1: &Pos, p2: &Pos) {
        let minx = p1.0.min(p2.0);
        let miny = p1.1.min(p2.1);
        for x in 0..=(p2.0 - p1.0).abs() {
            for y in 0..=(p2.1 - p1.1).abs() {
                let i = self.index(&(minx + x, miny + y));
                self.m[i] = '#';
            }
        }
    }

    fn add_formations(&mut self, f: &Formation) {
        let mut last_pos = &f[0];
        f.iter().for_each(|p| {
            self.add_line(last_pos, p);
            last_pos = p;
        });
    }

    fn index(&self, pos: &Pos) -> usize {
        let i = pos.0 - self.minx + self.stride * pos.1;
        i.try_into().unwrap()
    }

    fn is_solid(&self, pos: &Pos) -> bool {
        self.m[self.index(pos)] != '.'
    }
}
pub struct Day14_1;
impl Aoc for Day14_1 {
    fn day(&self) -> u32 {
        14
    }
    fn puzzle_name(&self) -> &str {
        "Regolith Reservoir"
    }
    fn solve(&self, lines: &[String]) -> String {
        let formations = lines_to_formations(lines);
        let mut map = Map::new(&formations);
        let amount = map.fill((500, 0));
        //println!("{map}");
        amount.to_string()
    }
}

pub struct Day14_2;
impl Aoc for Day14_2 {
    fn day(&self) -> u32 {
        14
    }
    fn puzzle_name(&self) -> &str {
        "Regolith Reservoir 2"
    }
    fn solve(&self, lines: &[String]) -> String {
        let mut formations = lines_to_formations(lines);
        let max = max_pos(&formations);
        formations.push(vec![(0, max.1 + 2), (1000, max.1 + 2)]);
        let mut map = Map::new(&formations);
        let amount = map.fill((500, 0));
        //println!("{map}");
        amount.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::as_vstrings;

    use super::*;

    const INPUT: [&str; 2] = [
        "498,4 -> 498,6 -> 496,6",
        "503,4 -> 502,4 -> 502,9 -> 494,9",
    ];

    #[test]
    fn test_soln() {
        let input_strs = as_vstrings(&INPUT[0..]);
        assert_eq!(Day14_1.solve(&input_strs), 24.to_string());
    }

    #[test]
    fn test_soln2() {
        let input_strs = as_vstrings(&INPUT[0..]);
        assert_eq!(Day14_2.solve(&input_strs), 93.to_string());
    }

    #[test]
    fn test_parse_formations() {
        assert_eq!(
            parse_formations("1,2 -> 10,29 -> 120,345").unwrap().1,
            vec![(1, 2), (10, 29), (120, 345)]
        );
    }
}
