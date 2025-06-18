use std::collections::HashSet;

use aoc_utils::dir::{Dir, Dir4};
use aoc_utils::grud;
use aoc_utils::pos2d;

type Grid = grud::Grid<char, Dir4>;
type Pos2d = pos2d::Pos2d<i64>;

fn parse_input(input: &str) -> (Grid, Pos2d) {
    let grid = Grid::parse(input);
    let start = grid.find('^').unwrap();
    (grid, start)
}

fn next_move(grid: &Grid, start: Pos2d, dir: Dir4) -> Option<(Pos2d, Dir4)> {
    let new_pos = start + dir;
    match grid.at(&new_pos)? {
        '^' | '.' => Some((new_pos, dir)),
        '#' => Some((start, dir.rotate_cw())),
        _ => None,
    }
}

fn march(grid: &Grid, start: Pos2d) -> HashSet<Pos2d> {
    let mut visited = HashSet::from([start]);
    let mut dir = Dir4::N;
    let mut pos = start;

    while let Some(item) = next_move(grid, pos, dir) {
        (pos, dir) = item;
        visited.insert(pos);
    }
    visited
}

fn is_endless(grid: &Grid, start: Pos2d, barrier: Pos2d) -> bool {
    let mut updated_grid = grid.clone();
    updated_grid.set(&barrier, '#');
    let mut dir: Dir4 = Dir4::N;
    let mut visited = HashSet::<_>::from([(start, dir)]);
    let mut pos = start;

    while let Some(item) = next_move(&updated_grid, pos, dir) {
        if !visited.insert(item) {
            return true;
        }
        (pos, dir) = item;
    }

    false
}

pub fn part1(input: &str) -> usize {
    let (grid, start) = parse_input(input);
    march(&grid, start).len()
}

pub fn part2(input: &str) -> usize {
    let (grid, start) = parse_input(input);
    let visited = march(&grid, start);

    let v = visited
        .iter()
        .filter(|&&pos| is_endless(&grid, start, pos))
        .collect::<Vec<_>>();

    v.len()
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 41;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 6;

    #[test]
    fn test_permutations() {
        let obstacles = vec![
            Pos2d::new(0, 0),
            Pos2d::new(1, 1),
            Pos2d::new(2, 2),
            Pos2d::new(3, 3),
            Pos2d::new(4, 4),
            Pos2d::new(5, 5),
            Pos2d::new(6, 6),
            Pos2d::new(7, 7),
            Pos2d::new(8, 8),
            Pos2d::new(9, 9),
        ];
        let perms = permutations(&obstacles);
        assert_eq!(perms.len(), n_choose_k(obstacles.len(), 3)); // 10 choose 3
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
