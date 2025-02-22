use std::collections::{HashMap, HashSet, VecDeque};

use aoc_utils::{
    dir::{Dir, Dir4},
    grud::{Grid, GridPos},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    loc: GridPos,
    dir: Dir4,
}

impl Pos {
    fn new(loc: GridPos, dir: Dir4) -> Pos {
        Pos { loc, dir }
    }
}

struct Paths {
    score: usize,
    paths: Vec<HashSet<GridPos>>,
}

impl Paths {
    fn init() -> Paths {
        Paths {
            score: 0,
            paths: vec![],
        }
    }

    fn new(score: usize, path: &HashSet<GridPos>) -> Paths {
        Paths {
            score,
            paths: vec![path.clone()],
        }
    }

    fn add(&mut self, path: &HashSet<GridPos>) {
        self.paths.push(path.clone());
    }
}

fn find_min_path(grid: &Grid<char, Dir4>) -> (usize, HashSet<GridPos>) {
    let start = grid.find('S').unwrap();
    let end = grid.find('E').unwrap();
    let mut paths = Paths::init();
    let mut visited: HashMap<Pos, usize> = HashMap::new();
    let mut options: VecDeque<(Pos, usize, HashSet<GridPos>)> =
        VecDeque::from([(Pos::new(start, Dir4::E), 0, HashSet::from([start]))]);

    while let Some((pos, score, path)) = options.pop_front() {
        if let Some(old_score) = visited.get(&pos) {
            if score > *old_score {
                continue;
            }
        }

        visited.insert(pos, score);
        if pos.loc == end {
            if score < paths.score || paths.score == 0 {
                paths = Paths::new(score, &path);
            } else if score == paths.score {
                paths.add(&path);
            }
            continue;
        }
        let new_loc = pos.loc + pos.dir.into();
        if let Some(c) = grid.at(&new_loc) {
            if c != '#' {
                let mut p = path.clone();
                p.insert(new_loc);
                options.push_back((Pos::new(new_loc, pos.dir), score + 1, p));
            }
            options.push_back((
                Pos::new(pos.loc, pos.dir.rotate_ccw()),
                score + 1000,
                path.clone(),
            ));
            options.push_back((Pos::new(pos.loc, pos.dir.rotate_cw()), score + 1000, path));
        }
    }

    let locs = paths
        .paths
        .into_iter()
        .fold(HashSet::new(), |acc, x| acc.union(&x).map(|&h| h).collect());

    (paths.score, locs)
}

pub fn part1(input: &str) -> usize {
    let grid = Grid::<char, Dir4>::parse(input);
    let res = find_min_path(&grid);
    println!("{:?}", res.1);
    res.0
}

pub fn part2(input: &str) -> usize {
    let mut grid = Grid::<char, Dir4>::parse(input);
    let res = find_min_path(&grid);
    res.1.iter().for_each(|i| _ = grid.set(i, 'O'));
    res.1.len()
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 7036;
    pub const TEST_INPUT_P1_2: &str = include_str!("data/input_example_2");
    pub const TEST_ANSWER_P1_2: usize = 11048;
    pub const TEST_INPUT_P2_1: &str = include_str!("data/input_example");
    pub const TEST_OUTPUT_P2_1: &str = include_str!("data/output_part2_1");
    pub const TEST_INPUT_P2_2: &str = include_str!("data/input_example_2");
    pub const TEST_OUTPUT_P2_2: &str = include_str!("data/output_part2_2");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(TEST_INPUT_P1_2), TEST_ANSWER_P1_2);
    }

    #[test]
    fn test_part2_1() {
        let mut grid = Grid::<char, Dir4>::parse(TEST_INPUT_P2_1);
        let res = find_min_path(&grid);
        res.1.iter().for_each(|i| _ = grid.set(i, 'O'));
        println!("{grid}");
        assert_eq!(grid.to_string(), TEST_OUTPUT_P2_1);
    }

    #[test]
    fn test_part2_2() {
        let mut grid = Grid::<char, Dir4>::parse(TEST_INPUT_P2_2);
        let res = find_min_path(&grid);
        res.1.iter().for_each(|i| _ = grid.set(i, 'O'));
        println!("{grid}");
        assert_eq!(grid.to_string(), TEST_OUTPUT_P2_2);
    }
}
