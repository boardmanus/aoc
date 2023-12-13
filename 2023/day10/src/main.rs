use std::{
    collections::{HashMap, HashSet, LinkedList},
    ops::Add,
};

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug, PartialOrd, Ord)]
enum Dir {
    N,
    E,
    S,
    W,
}
impl Dir {
    fn dir(&self) -> (i64, i64) {
        match self {
            Dir::N => (0, -1),
            Dir::E => (1, 0),
            Dir::S => (0, 1),
            Dir::W => (-1, 0),
        }
    }
    fn opp(&self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::E => Dir::W,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
        }
    }
    fn to_char(&self) -> char {
        match self {
            Dir::N => 'N',
            Dir::E => 'E',
            Dir::S => 'S',
            Dir::W => 'W',
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Tile {
    Start,
    PipeNS,
    PipeEW,
    PipeNE,
    PipeNW,
    PipeSE,
    PipeSW,
}

impl Tile {
    fn from_char(s: char) -> Option<Tile> {
        match s {
            '|' => Some(Tile::PipeNS),
            '-' => Some(Tile::PipeEW),
            'L' => Some(Tile::PipeNE),
            'J' => Some(Tile::PipeNW),
            '7' => Some(Tile::PipeSW),
            'F' => Some(Tile::PipeSE),
            'S' => Some(Tile::Start),
            _ => None,
        }
    }
    fn to_char(&self) -> char {
        match self {
            Tile::Start => 'S',
            Tile::PipeNS => '|',
            Tile::PipeEW => '-',
            Tile::PipeNE => 'L',
            Tile::PipeNW => 'J',
            Tile::PipeSE => 'F',
            Tile::PipeSW => '7',
        }
    }
    fn dirs(&self) -> &[Dir] {
        match self {
            Tile::Start => &[Dir::N, Dir::S, Dir::E, Dir::W],
            Tile::PipeNS => &[Dir::N, Dir::S],
            Tile::PipeEW => &[Dir::E, Dir::W],
            Tile::PipeNE => &[Dir::N, Dir::E],
            Tile::PipeNW => &[Dir::N, Dir::W],
            Tile::PipeSE => &[Dir::S, Dir::E],
            Tile::PipeSW => &[Dir::S, Dir::W],
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}
impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
}
impl Add<(i64, i64)> for &Point {
    type Output = Point;
    fn add(self, rhs: (i64, i64)) -> Self::Output {
        Point::new(self.x + rhs.0, self.y + rhs.1)
    }
}
impl Add<(i64, i64)> for Point {
    type Output = Point;
    fn add(self, rhs: (i64, i64)) -> Self::Output {
        Point::new(self.x + rhs.0, self.y + rhs.1)
    }
}

type Grid = HashMap<Point, Tile>;

fn parse(input: &str) -> (Point, Grid, (usize, usize)) {
    let mut start = Point { x: 0, y: 0 };
    let h = input.lines().count();
    let w = input.lines().next().unwrap().len();
    let grid = input
        .lines()
        .enumerate()
        .fold(Grid::new(), |grid, (y, line)| {
            line.chars().enumerate().fold(grid, |mut grid, (x, c)| {
                if let Some(tile) = Tile::from_char(c) {
                    if tile == Tile::Start {
                        start = Point {
                            x: x as i64,
                            y: y as i64,
                        };
                    }
                    grid.insert(
                        Point {
                            x: x as i64,
                            y: y as i64,
                        },
                        tile,
                    );
                }
                grid
            })
        });
    (start, grid, (w, h))
}

fn add_paths(q: &mut LinkedList<(Dir, Point)>, tile: Tile, pt: &Point) {
    tile.dirs().iter().for_each(|dir| {
        q.push_back((*dir, *pt));
    });
}

fn start_dir(grid: &Grid, start: &Point) -> (Tile, Dir) {
    let tile = grid.get(start).unwrap();
    let mut dirs = tile
        .dirs()
        .iter()
        .filter(|dir| {
            if let Some(new_tile) = grid.get(&(start + dir.dir())) {
                new_tile.dirs().contains(&dir.opp())
            } else {
                false
            }
        })
        .collect::<Vec<_>>();
    dirs.sort();

    assert!(dirs.len() == 2);
    match dirs.as_slice() {
        &[Dir::N, Dir::E] => (Tile::PipeNE, Dir::N),
        &[Dir::N, Dir::S] => (Tile::PipeNS, Dir::N),
        &[Dir::N, Dir::W] => (Tile::PipeNW, Dir::N),
        &[Dir::E, Dir::W] => (Tile::PipeEW, Dir::E),
        &[Dir::E, Dir::S] => (Tile::PipeSE, Dir::S),
        &[Dir::W, Dir::S] => (Tile::PipeSW, Dir::S),
        _ => panic!("Invalid start tile"),
    }
}

fn dfs(grid: &Grid, start: &Point) -> (usize, HashMap<Point, (Tile, Dir, usize)>) {
    let tile = grid.get(start).unwrap();
    let mut visited: HashMap<Point, (Tile, Dir, usize)> = Default::default();
    let start_dir = start_dir(grid, start);
    visited
        .entry(*start)
        .or_insert((start_dir.0, start_dir.1, 0));
    let mut q = LinkedList::new();
    add_paths(&mut q, *tile, start);
    let mut max_path = 0;

    while !q.is_empty() {
        let (dir, pt) = q.pop_front().unwrap();
        let new_pt = pt + dir.dir();
        if let Some(tile) = grid.get(&new_pt) {
            if tile.dirs().contains(&dir.opp()) {
                if !visited.contains_key(&new_pt) {
                    let count = visited.get(&pt).unwrap().2 + 1;
                    visited.insert(new_pt, (*tile, dir, count));
                    max_path = count.max(max_path);
                    add_paths(&mut q, *tile, &new_pt);
                }
            }
        }
    }

    (max_path, visited)
}

fn winding_number(visited: &HashMap<Point, (Tile, Dir, usize)>, pt: &Point, width: usize) -> i64 {
    let mut winding_number = 0;
    for x in (pt.x + 1)..(width as i64) {
        let pt = Point::new(x, pt.y);
        if let Some(visit) = visited.get(&pt) {
            if visit.1 == Dir::N {
                winding_number += 1;
            } else if visit.1 == Dir::S {
                winding_number -= 1;
            } else {
                if visit.0.dirs().contains(&Dir::N) {
                    winding_number += 1;
                } else if visit.0.dirs().contains(&Dir::S) {
                    winding_number -= 1;
                }
            }
        }
    }
    winding_number
}

fn print_windings(windings: &HashMap<Point, Dir>, size: &(usize, usize)) {
    for y in 0..size.1 {
        for x in 0..size.0 {
            let pt = Point::new(x as i64, y as i64);
            if let Some(dir) = windings.get(&pt) {
                print!("{}", dir.to_char());
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn print_visited(visited: &HashMap<Point, (Tile, Dir, usize)>, size: &(usize, usize)) {
    for y in 0..size.1 {
        for x in 0..size.0 {
            let pt = Point::new(x as i64, y as i64);
            if let Some(visit) = visited.get(&pt) {
                print!("[{}{}]", visit.0.to_char(), visit.1.to_char());
            } else {
                print!("[..]");
            }
        }
        println!();
    }
}

fn w(start: &Point, grid: &HashMap<Point, Tile>) -> HashMap<Point, Dir> {
    let mut pos = *start;
    let mut last_dir: Option<Dir> = None;
    let mut winding = HashMap::new();
    while !winding.contains_key(&pos) {
        let mut tile = *grid.get(&pos).unwrap();
        if tile == Tile::Start {
            tile = start_dir(grid, start).0;
        }

        let a = tile.dirs()[0];
        let b = tile.dirs()[1];
        let (pt_dir, dir) = match last_dir {
            None => {
                if a == Dir::N || a == Dir::S {
                    (a, a)
                } else {
                    (b, b)
                }
            }
            Some(Dir::N) => {
                if a == Dir::S {
                    (b, Dir::N)
                } else {
                    assert!(b == Dir::S);
                    (a, Dir::N)
                }
            }
            Some(Dir::S) => {
                assert!(a == Dir::N);
                (b, Dir::S)
            }
            Some(Dir::E) => {
                if a == Dir::N {
                    assert!(b == Dir::W);
                    (Dir::S, Dir::N)
                } else if a == Dir::S {
                    assert!(b == Dir::W);
                    (Dir::S, Dir::S)
                } else {
                    assert!(b == Dir::W);
                    (Dir::E, Dir::E)
                }
            }
            Some(Dir::W) => {
                if a == Dir::N {
                    assert!(b == Dir::E);
                    (b, Dir::N)
                } else if a == Dir::S {
                    assert!(b == Dir::E);
                    (b, Dir::S)
                } else {
                    assert!(b == Dir::E);
                    (b, Dir::W)
                }
            }
            _ => panic!("Invalid last dir"),
        };
        println!(
            "{:?} => {}:{:?} {:?}",
            pos,
            tile.to_char(),
            tile.dirs(),
            dir
        );
        winding.insert(pos, dir);
        pos = pos + pt_dir.dir();
        last_dir = Some(pt_dir);
    }

    winding
}

fn winding(start: &Point, visited: &HashMap<Point, (Tile, Dir, usize)>) -> HashMap<Point, Dir> {
    let mut pos = *start;
    let mut winding = HashMap::new();
    while !winding.contains_key(&pos) {
        let visit = *visited.get(&pos).unwrap();
        let count = visit.2 as i64;
        let maybe_visit = visit
            .0
            .dirs()
            .iter()
            .filter_map(|dir| {
                let new_pos = pos + dir.dir();
                if let Some(visit) = visited.get(&new_pos) {
                    Some((new_pos, visit))
                } else {
                    None
                }
            })
            .find(|visit| {
                if (visit.1 .2 as i64 == count + 1 || visit.1 .2 as i64 == count - 1)
                    && !winding.contains_key(&visit.0)
                {
                    return true;
                }
                false
            });
        if let Some(visit) = maybe_visit {
            println!("{:?}: {:?}", pos, visit);
            match visit.1 {
                (_, Dir::N, _) => winding.insert(pos, Dir::N),
                (_, Dir::S, _) => winding.insert(pos, Dir::S),
                (Tile::PipeNW, _, _) => winding.insert(pos, Dir::N),
                (Tile::PipeNE, _, _) => winding.insert(pos, Dir::N),
                (Tile::PipeSW, _, _) => winding.insert(pos, Dir::S),
                (Tile::PipeSE, _, _) => winding.insert(pos, Dir::S),
                _ => winding.insert(pos, visit.1 .1),
            };
            pos = visit.0;
        } else {
            break;
        }
    }

    winding
}

fn solve_part1(input: &str) -> usize {
    let (start, grid, _) = parse(input);
    dfs(&grid, &start).0
}

fn solve_part2(input: &str) -> usize {
    let (start, grid, size) = parse(input);
    let visited = dfs(&grid, &start).1;
    print_visited(&visited, &size);
    //    let windings: HashMap<Point, Dir> = winding(&start, &visited);
    let windings: HashMap<Point, Dir> = w(&start, &grid);
    print_windings(&windings, &size);
    let mut inside_count = 0;
    let mut inside = Vec::<Point>::new();
    for w in 0..size.0 {
        for h in 0..size.1 {
            let pt = Point::new(w as i64, h as i64);
            if visited.get(&pt).is_none() {
                if winding_number(&visited, &pt, size.0) != 0 {
                    inside_count += 1;
                    inside.push(pt);
                }
            }
        }
    }
    println!("Inside: {:?}", inside);
    inside_count
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
    const TEST_INPUT_1_2: &str = include_str!("test_input1_2.txt");
    const TEST_INPUT_2: &str = include_str!("test_input2.txt");
    const TEST_INPUT_2_2: &str = include_str!("test_input2_2.txt");
    const TEST_INPUT_2_3: &str = include_str!("test_input2_3.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 4);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(solve_part1(TEST_INPUT_1_2), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 4);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(solve_part2(TEST_INPUT_2_2), 8);
    }

    #[test]
    fn test_part2_3() {
        assert_eq!(solve_part2(TEST_INPUT_2_3), 10);
    }

    #[test]
    fn test_parse() {
        let (start, grid, size) = parse(TEST_INPUT);
        assert_eq!(start, Point::new(1, 1));
        assert_eq!(size, (5, 5));
        assert_eq!(grid[&Point::new(1, 1)], Tile::Start);
        assert_eq!(grid[&Point::new(2, 1)], Tile::PipeEW);
        assert_eq!(grid[&Point::new(3, 1)], Tile::PipeSW);
        assert_eq!(grid[&Point::new(1, 2)], Tile::PipeNS);
        assert_eq!(grid[&Point::new(3, 2)], Tile::PipeNS);
        assert_eq!(grid[&Point::new(1, 3)], Tile::PipeNE);
        assert_eq!(grid[&Point::new(2, 3)], Tile::PipeEW);
        assert_eq!(grid[&Point::new(3, 3)], Tile::PipeNW);
    }
}
