use std::collections::{HashMap, HashSet};

use aoc_utils::{
    dir::Dir8,
    grud::{Grid, GridPos, GridVec},
    uterators::PairsIterator,
};

type AntennaGrid = Grid<char, Dir8>;
type AntennaMap = HashMap<char, Vec<GridPos>>;

fn is_valid_antinode_pos(grid: &AntennaGrid, pos: &GridPos) -> bool {
    grid.is_valid(pos)
}

fn antenna_map(grid: &AntennaGrid) -> AntennaMap {
    grid.iter_pair()
        .filter(|(_, c)| *c != '.')
        .fold(AntennaMap::new(), |mut acc, (pos, c)| {
            acc.entry(c).or_default().push(pos);
            acc
        })
}

fn antinode_harmonics(grid: &AntennaGrid, antenna: &GridPos, offset: GridVec) -> Vec<GridPos> {
    let mut v = Vec::<GridPos>::new();
    let mut antinode: GridPos = *antenna;
    while grid.is_valid(&antinode) {
        v.push(antinode);
        antinode = antinode + offset;
    }
    v
}

fn antinodes_for_antennas(
    grid: &AntennaGrid,
    antenna_a: &GridPos,
    antenna_b: &GridPos,
    with_harmonics: bool,
) -> Vec<GridPos> {
    let offset = *antenna_b - *antenna_a;
    if with_harmonics {
        let mut v = antinode_harmonics(grid, antenna_a, offset);
        v.append(&mut antinode_harmonics(grid, antenna_b, -offset));
        v
    } else {
        [*antenna_a - offset, *antenna_b + offset]
            .iter()
            .filter(|other_loc| is_valid_antinode_pos(grid, other_loc))
            .map(|p| *p)
            .collect()
    }
}

fn antinode_locations(
    grid: &AntennaGrid,
    antenna_locations: &Vec<GridPos>,
    with_harmonics: bool,
) -> HashSet<GridPos> {
    antenna_locations
        .iter()
        .pairs()
        .map(|(a, b)| antinodes_for_antennas(grid, a, b, with_harmonics))
        .flatten()
        .fold(HashSet::default(), |mut acc, a| {
            acc.insert(a);
            acc
        })
}

pub fn part1(input: &str) -> usize {
    let grid = AntennaGrid::parse(input);
    let antennas = antenna_map(&grid);
    antennas
        .iter()
        .fold(HashSet::<GridPos>::default(), |acc, (_c, locations)| {
            acc.union(&antinode_locations(&grid, locations, false))
                .map(|i| *i)
                .collect()
        })
        .len()
}

pub fn part2(input: &str) -> usize {
    let mut grid = AntennaGrid::parse(input);
    let antennas = antenna_map(&grid);
    antennas
        .iter()
        .fold(HashSet::<GridPos>::default(), |acc, (_c, locations)| {
            acc.union(&antinode_locations(&grid, locations, true))
                .map(|i| *i)
                .collect()
        })
        .len()
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 14;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 34;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
}
