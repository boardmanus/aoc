use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn new(x: i64, y: i64) -> Self {
        Coord { x, y }
    }

    fn adj(&self) -> CoordIter {
        CoordIter { i: 0, coord: self }
    }
}

struct CoordIter<'a> {
    i: usize,
    coord: &'a Coord,
}

impl<'a> Iterator for CoordIter<'a> {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        const ADJ: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        if self.i < 4 {
            let c = ADJ[self.i];
            self.i += 1;
            Some(Coord::new(self.coord.x + c.0, self.coord.y + c.1))
        } else {
            None
        }
    }
}

fn next_coord(to_check: &mut HashMap<Coord, usize>) -> Option<(Coord, usize)> {
    if let Some(kp) = to_check.iter().next() {
        let c = *kp.0;
        let risk = *kp.1;
        to_check.remove(&c);
        Some((c, risk))
    } else {
        None
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Node {
    coord: Coord,
    risk: usize,
}

impl Node {
    fn new(coord: Coord, risk: usize) -> Self {
        Node { coord, risk }
    }
}
// Order so that lower risk levels are greater since BinaryHeap is a max-heap
impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ord = self.risk.cmp(&other.risk).reverse();
        match ord {
            Ordering::Equal => None,
            _ => Some(ord),
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(order) = self.partial_cmp(other) {
            order
        } else if self.coord.y != other.coord.y {
            self.coord.y.cmp(&other.coord.y).reverse()
        } else {
            self.coord.x.cmp(&other.coord.x).reverse()
        }
    }
}

#[derive(Debug)]
struct Grid {
    width: i64,
    height: i64,
    times: i64,
    g: Vec<u8>,
}

impl Grid {
    fn parse(input: &str, times: i64) -> Self {
        let lines = input
            .split('\n')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        let width = lines[0].len();
        let height = lines.len();
        let mut g = vec![0u8; width * height];
        lines.into_iter().enumerate().for_each(|(y, line)| {
            line.chars()
                .enumerate()
                .for_each(|(x, c)| g[y * width + x] = c as u8 - '0' as u8)
        });
        Grid {
            width: width as i64,
            height: height as i64,
            times,
            g,
        }
    }

    fn start(&self) -> Coord {
        Coord::new(0, 0)
    }

    fn end(&self) -> Coord {
        Coord::new(
            (self.width * self.times).saturating_sub(1),
            (self.height * self.times).saturating_sub(1),
        )
    }

    fn contains(&self, coord: &Coord) -> bool {
        coord.x >= 0
            && coord.x < self.width * self.times
            && coord.y >= 0
            && coord.y < self.height * self.times
    }

    fn index(&self, coord: &Coord) -> usize {
        (coord.x + coord.y * self.width * self.times) as usize
    }

    fn small_index(&self, x: i64, y: i64) -> usize {
        (x + y * self.width) as usize
    }

    fn get(&self, coord: &Coord) -> Option<u8> {
        if self.contains(coord) {
            let ygrid = coord.y % self.height;
            let xgrid = coord.x % self.width;
            let inc = (coord.y / self.height + coord.x / self.width) as u8;
            Some(((self.g[self.small_index(xgrid, ygrid)] + inc - 1) % 9) + 1)
        } else {
            None
        }
    }

    fn search(&self) -> usize {
        let mut to_check = BinaryHeap::<Node>::new();
        to_check.push(Node::new(self.start(), 0));

        let mut visited =
            vec![false; (self.width * self.height * self.times * self.times) as usize];

        let end_index = self.index(&self.end());

        while let Some(Node { coord, risk }) = to_check.pop() {
            let index = self.index(&coord);
            if visited[index] {
                continue;
            }

            if index == end_index {
                return risk;
            }

            visited[index] = true;

            coord.adj().filter(|c| self.contains(c)).for_each(|c| {
                let jump = self.get(&c).expect("val") as usize;
                let new_risk = risk + jump;
                if !visited[self.index(&c)] {
                    to_check.push(Node {
                        coord: c,
                        risk: new_risk,
                    });
                }
            });
        }

        0
    }
}

fn solve_part1(input: &str) -> usize {
    let grid = Grid::parse(input, 1);
    grid.search()
}

fn solve_part2(input: &str) -> usize {
    let grid = Grid::parse(input, 5);
    grid.search()
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let part1 = solve_part1(INPUT);
    println!("Part1: {part1}");
    let part2 = solve_part2(INPUT);
    println!("Part2: {part2}");
    solve_part2(INPUT);
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 315);
    }

    #[test]
    fn test_parse() {
        let grid = Grid::parse(TEST_INPUT, 1);
        assert_eq!(grid.get(&Coord::new(0, 0)), Some(1));
        assert_eq!(grid.get(&Coord::new(9, 0)), Some(2));
        assert_eq!(grid.get(&Coord::new(1, 9)), Some(3));
        assert_eq!(grid.get(&Coord::new(-1, 8)), None);
        assert_eq!(grid.get(&Coord::new(1, -1)), None);
        assert_eq!(grid.get(&Coord::new(50, 2)), None);
        assert_eq!(grid.get(&Coord::new(1, 50)), None);
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
    }

    #[test]
    fn test_grid_get() {
        let grid = Grid::parse(TEST_INPUT, 5);
        assert_eq!(grid.get(&Coord::new(0, 0)), Some(1));
        assert_eq!(grid.get(&Coord::new(10, 0)), Some(2));
        assert_eq!(grid.get(&Coord::new(20, 0)), Some(3));
        assert_eq!(grid.get(&Coord::new(20, 10)), Some(4));
        assert_eq!(grid.get(&Coord::new(20, 20)), Some(5));
        assert_eq!(grid.get(&Coord::new(0, 40)), Some(5));
        assert_eq!(grid.get(&Coord::new(27, 0)), Some(9));
        assert_eq!(grid.get(&Coord::new(37, 0)), Some(1));
        assert_eq!(grid.get(&Coord::new(47, 0)), Some(2));
    }

    #[test]
    fn test_adj() {
        let test = [
            Coord::new(4, 5),
            Coord::new(5, 6),
            Coord::new(6, 5),
            Coord::new(5, 4),
        ];
        Coord::new(5, 5).adj().for_each(|c| {
            test.contains(&c);
        })
    }
}
