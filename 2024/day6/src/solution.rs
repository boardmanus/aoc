use std::collections::HashSet;

use aoc_utils::dir::Dir;
use aoc_utils::grid::{Grid, Index};

fn parse_input(input: &str) -> (Grid<char>, Index) {
    let grid = Grid::<char>::parse(input);
    let start = grid.find('^').unwrap();
    (grid, start)
}

fn next_move(grid: &Grid<char>, start: Index, dir: Dir) -> Option<(Index, Dir)> {
    match grid.at(start + dir)? {
        '^' | '.' => Some((start + dir, dir)),
        '#' => next_move(grid, start, dir.rotate_cw()),
        _ => None,
    }
}

fn march(grid: &Grid<char>, start: Index) -> HashSet<Index> {
    let mut visited = HashSet::<Index>::from([start]);
    let mut dir = Dir::N;
    let mut pos = start;

    while let Some(item) = next_move(grid, pos, dir) {
        pos = item.0;
        dir = item.1;
        visited.insert(pos);
    }
    visited
}

fn is_endless(grid: &Grid<char>, start: Index, barrier: Index) -> bool {
    let mut dir: Dir = Dir::N;
    let mut visited = HashSet::<_>::from([(start, dir)]);
    let mut pos = start;
    let mut grid2 = grid.clone();
    grid2.set(barrier, '#');

    while let Some(item) = next_move(&grid2, pos, dir) {
        pos = item.0;
        dir = item.1;
        if !visited.insert((pos, dir)) {
            return true;
        }
    }

    false
}

pub fn part1(input: &str) -> usize {
    let (grid, start) = parse_input(input);
    let locations = march(&grid, start);
    locations.len()
}

pub fn part2(input: &str) -> usize {
    let (grid, start) = parse_input(input);
    let visited = march(&grid, start);

    visited
        .iter()
        .filter(|&index| is_endless(&grid, start, *index))
        .count()
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 41;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 6;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
}
