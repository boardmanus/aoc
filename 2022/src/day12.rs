use crate::aoc::Aoc;
use itertools::Itertools;

type Matrix = Vec<Vec<usize>>;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn offset(&self, dp: &(i32, i32)) -> Pos {
        let x = ((self.0 as i32) + dp.0) as usize;
        let y = ((self.1 as i32) + dp.1) as usize;
        Pos(x, y)
    }
}

struct Graph {
    m: Matrix,
    s: Pos,
    e: Pos,
}

impl Graph {
    fn get(&self, p: &Pos) -> usize {
        if p.1 >= self.m.len() || p.0 >= self.m[0].len() {
            100000
        } else {
            self.m[p.1][p.0]
        }
    }
    /*
        fn to_lines(&self) -> Vec<String> {
            self.m
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|v| char::from_u32(*v as u32).unwrap())
                        .collect::<String>()
                })
                .collect_vec()
        }
    */
    fn find_lows(&self) -> Vec<Pos> {
        let mut lows: Vec<Pos> = Default::default();
        self.m.iter().enumerate().fold(&mut lows, |vp, row| {
            let row_sp = row
                .1
                .iter()
                .enumerate()
                .filter(|col| self.m[row.0][col.0] == 'a' as usize)
                .fold(vp, |vp2, v| {
                    vp2.push(Pos(v.0, row.0));
                    vp2
                });
            row_sp
        });
        lows
    }

    fn neighbors(&self, pos: &Pos) -> Vec<(Pos, usize)> {
        let a = self.get(pos);
        let n = [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .filter(|dp| {
                let b = self.get(&pos.offset(dp));
                let diff = (b as i32) - (a as i32);
                diff < 2
            })
            .map(|dp| (pos.offset(dp), 1))
            .collect_vec();
        n
    }
}

fn lines_to_graph(lines: &[String]) -> Graph {
    let mut s = Pos(0, 0);
    let mut e = Pos(0, 0);
    let m = lines
        .iter()
        .enumerate()
        .map(|line| {
            line.1
                .chars()
                .enumerate()
                .map(|c| match c.1 {
                    'S' => {
                        s = Pos(c.0, line.0);
                        'a' as usize
                    }
                    'E' => {
                        e = Pos(c.0, line.0);
                        'z' as usize
                    }
                    _ => c.1 as usize,
                })
                .collect_vec()
        })
        .collect_vec();

    Graph { m, s, e }
}

pub struct Day12_1;
impl Aoc for Day12_1 {
    fn day(&self) -> u32 {
        12
    }
    fn puzzle_name(&self) -> &str {
        "Hill Climbing"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        let g = lines_to_graph(lines);
        let path = pathfinding::prelude::dijkstra(&g.s, |p| g.neighbors(p), |p| *p == g.e);
        (path.unwrap().0.len() - 1).to_string()
    }
}

pub struct Day12_2;
impl Aoc for Day12_2 {
    fn day(&self) -> u32 {
        12
    }
    fn puzzle_name(&self) -> &str {
        "Hill Climb 2"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        let mut g = lines_to_graph(lines);
        let starting_pos = g.find_lows();
        starting_pos
            .iter()
            .map(|sp| {
                g.s = sp.clone();
                let path = pathfinding::prelude::dijkstra(&g.s, |p| g.neighbors(p), |p| *p == g.e);
                let l = if let Some(p) = path {
                    p.0.len() - 1
                } else {
                    1000000
                };
                l
            })
            .min()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::as_vstrings;

    use super::*;

    const INPUT: [&str; 5] = ["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"];

    #[test]
    fn test_soln() {
        let input_strs = as_vstrings(&INPUT[0..]);
        assert_eq!(Day12_1.solve(&input_strs), 31.to_string());
    }

    #[test]
    fn test_soln2() {
        let input_strs = as_vstrings(&INPUT[0..]);

        assert_eq!(Day12_2.solve(&input_strs), 29.to_string());
    }

    #[test]
    fn test_line_rev() {
        let mut lines = as_vstrings(&INPUT[0..]);
        let g = lines_to_graph(&lines);
        let lines2 = g.to_lines();
        assert_eq!(lines2[g.s.1].chars().nth(g.s.0), Some('a'));
        assert_eq!(lines2[g.e.1].chars().nth(g.e.0), Some('z'));
        lines[g.s.1] = lines[g.s.1]
            .chars()
            .map(|c| match c {
                'S' => 'a',
                'E' => 'z',
                _ => c,
            })
            .collect::<String>();

        lines[g.e.1] = lines[g.e.1]
            .chars()
            .map(|c| match c {
                'S' => 'a',
                'E' => 'z',
                _ => c,
            })
            .collect::<String>();

        assert_eq!(lines, lines2);
    }
    #[test]
    fn test_pos_offset() {
        assert_eq!(Pos(0, 0).offset(&(-1, -1)), Pos(0, usize::MAX));
    }

    #[test]
    fn test_abs_diff() {
        let a: usize = 100000;
        let b: usize = 99;
        assert_eq!(a.abs_diff(b), 99901);
        assert_eq!(b.abs_diff(a), 99901);
    }
}
