use std::collections::{HashMap, HashSet};

use aoc_utils::grid::{Grid, Index};

type AntennaGrid = Grid<char>;
type AntennaMap = HashMap<char, Vec<Index>>;

fn is_valid_antinode_pos(grid: &AntennaGrid, index: &Index) -> bool {
    grid.is_valid(*index)
}

fn antenna_map(grid: &AntennaGrid) -> AntennaMap {
    grid.iter().enumerate().filter(|(_i, &c)| c != '.').fold(
        AntennaMap::new(),
        |mut acc, (i, &c)| {
            let index = grid.index_from(i);
            acc.entry(c).or_default().push(index);
            acc
        },
    )
}

fn antinode_harmonics(grid: &AntennaGrid, antenna: &Index, offset: Index) -> Vec<Index> {
    let mut v = Vec::<Index>::new();
    let mut antinode: Index = *antenna;
    while is_valid_antinode_pos(grid, &antinode) {
        v.push(antinode);
        antinode = antinode + offset;
    }
    v
}

fn antinodes_for_antennas(
    grid: &AntennaGrid,
    antenna_a: &Index,
    antenna_b: &Index,
    with_harmonics: bool,
) -> Vec<Index> {
    let offset = *antenna_b - *antenna_a;
    if with_harmonics {
        let mut v = antinode_harmonics(grid, antenna_a, *antenna_a - *antenna_b);
        v.append(&mut antinode_harmonics(
            grid,
            antenna_b,
            *antenna_b - *antenna_a,
        ));
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
    antenna_locations: &Vec<Index>,
    with_harmonics: bool,
) -> HashSet<Index> {
    let a = (0..antenna_locations.len() - 1).fold(HashSet::<Index>::new(), |mut acc, i| {
        let loc = antenna_locations[i];
        let others = &antenna_locations[i + 1..];
        others
            .iter()
            .map(|other_loc| antinodes_for_antennas(grid, &loc, other_loc, with_harmonics))
            .flatten()
            .for_each(|antinode| {
                acc.insert(antinode);
            });
        acc
    });
    a
}

pub fn part1(input: &str) -> usize {
    let mut grid = AntennaGrid::parse(input);
    let antennas = antenna_map(&grid);
    let antinodes = antennas
        .iter()
        .fold(HashSet::<Index>::default(), |acc, (_c, locations)| {
            acc.union(&antinode_locations(&grid, locations, false))
                .map(|i| *i)
                .collect()
        });

    antinodes.iter().for_each(|an| grid.set(*an, '#'));
    println!("{grid}");

    antinodes.len()
}

pub fn part2(input: &str) -> usize {
    let mut grid = AntennaGrid::parse(input);
    let antennas = antenna_map(&grid);
    let antinodes = antennas
        .iter()
        .fold(HashSet::<Index>::default(), |acc, (_c, locations)| {
            acc.union(&antinode_locations(&grid, locations, true))
                .map(|i| *i)
                .collect()
        });

    antinodes.iter().for_each(|an| grid.set(*an, '#'));
    println!("{grid}");

    antinodes.len()
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
