use std::collections::{HashMap, HashSet, VecDeque};

use aoc_utils::{
    dir::{Dir, Dir4},
    grid::{Grid, Index},
};

enum Move {
    Turn(Dir4),
    Walk(Dir4),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    loc: Index,
    dir: Dir4,
}

impl Pos {
    fn new(loc: Index, dir: Dir4) -> Pos {
        Pos { loc, dir }
    }
}

fn find_min_path(grid: &Grid<char>) -> usize {
    let start = grid.find('S').unwrap();
    let end = grid.find('E').unwrap();
    let mut visited: HashMap<Pos, usize> = HashMap::new();
    let mut options: VecDeque<(Pos, usize, HashSet<Index>)> =
        VecDeque::from([(Pos::new(start, Dir4::E), 0, HashSet::new())]);

    while let Some((pos, score, mut path)) = options.pop_front() {
        if let Some(&old_score) = visited.get(&pos) {
            if score >= old_score {
                continue;
            }
        }
        visited.insert(pos, score);
        if pos.loc == end {
            continue;
        }
        let new_loc = pos.loc + pos.dir;
        if let Some(c) = grid.at(new_loc) {
            if c != '#' {
                path.insert(new_loc);
                options.push_back((Pos::new(new_loc, pos.dir), score + 1, path.clone()));
            }
            options.push_back((
                Pos::new(pos.loc, pos.dir.rotate_ccw()),
                score + 1000,
                path.clone(),
            ));
            options.push_back((Pos::new(pos.loc, pos.dir.rotate_cw()), score + 1000, path));
        }
    }

    *Dir4::cw()
        .filter_map(|dir| visited.get(&Pos::new(end, dir)))
        .min()
        .unwrap()
}

pub fn part1(input: &str) -> usize {
    let grid = Grid::<char>::parse(input);
    find_min_path(&grid)
}

pub fn part2(input: &str) -> String {
    input.to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 7036;
    pub const TEST_INPUT_P1_2: &str = include_str!("data/input_example_2");
    pub const TEST_ANSWER_P1_2: usize = 11048;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: &str = "part2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(TEST_INPUT_P1_2), TEST_ANSWER_P1_2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
}
