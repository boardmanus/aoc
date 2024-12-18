use std::collections::{HashMap, VecDeque};

use aoc_utils::{
    dir::{Dir, Dir4},
    grid::{Grid, Index},
};

fn parse_input(input: &str) -> Vec<Index> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(",").map(|s| s.parse::<i64>().unwrap());
            Index(it.next().unwrap(), it.next().unwrap())
        })
        .collect()
}

fn create_grid(blocks: &[Index], width: usize, height: usize) -> Grid<char> {
    let mut grid = Grid::new('.', width, height);
    for &b in blocks {
        grid.set(b, '#');
    }
    grid
}

fn shortest_path(grid: &Grid<char>, start: Index, end: Index) -> Option<usize> {
    let mut visited: HashMap<Index, usize> = HashMap::new();
    let mut nexties: VecDeque<(Index, usize)> = VecDeque::from([(start, 0)]);
    while let Some(next) = nexties.pop_front() {
        let shortest = *visited.entry(next.0).or_insert(usize::MAX);
        if next.1 < shortest {
            visited.insert(next.0, next.1);
            if next.0 != end {
                Dir4::cw()
                    .map(|dir| next.0 + dir)
                    .filter(|&pos| grid.is_valid(pos))
                    .filter(|&pos| grid.at(pos).unwrap() == '.')
                    .for_each(|pos| nexties.push_back((pos, next.1 + 1)));
            }
        }
    }

    Some(*visited.get(&end)?)
}

fn first_blocker(grid: &mut Grid<char>, blocks: &[Index]) -> Option<Index> {
    let start = Index(0, 0);
    let end = Index(grid.width() as i64 - 1, grid.height() as i64 - 1);
    for &block in blocks {
        grid.set(block, '#');
        if shortest_path(grid, start, end).is_none() {
            return Some(block);
        }
    }
    None
}

pub fn part1(input: &str) -> usize {
    let blocks = parse_input(input);
    let grid = create_grid(&blocks[0..1024], 71, 71);
    shortest_path(&grid, Index(0, 0), Index(70, 70)).unwrap()
}

pub fn part2(input: &str) -> String {
    let blocks = parse_input(input);
    let mut grid = create_grid(&blocks[0..1024], 71, 71);
    let blocker = first_blocker(&mut grid, &blocks[1024..]).unwrap();
    format!("{},{}", blocker.0, blocker.1)
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 22;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: Index = Index(6, 1);

    #[test]
    fn test_create_grid() {
        let blocks = parse_input(TEST_INPUT);
        let grid = create_grid(&blocks[0..12], 7, 7);
        assert_eq!(grid.to_string(), include_str!("data/grid_example"));
    }

    #[test]
    fn test_part1() {
        let blocks = parse_input(TEST_INPUT);
        let grid = create_grid(&blocks[0..12], 7, 7);
        assert_eq!(
            shortest_path(&grid, Index(0, 0), Index(6, 6)).unwrap(),
            TEST_ANSWER
        );
    }

    #[test]
    fn test_part2() {
        let blocks = parse_input(TEST_INPUT_2);
        let mut grid = create_grid(&blocks[0..12], 7, 7);
        let blocker = first_blocker(&mut grid, &blocks[12..]).unwrap();
        assert_eq!(blocker, TEST_ANSWER_2)
    }
}
