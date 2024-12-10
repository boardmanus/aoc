use std::collections::{HashMap, HashSet};

use aoc_utils::{
    dir::{Dir, Dir4},
    grid::{Grid, Index},
};

type Trail = Vec<Index>;
type TrailTopo = Grid<u32>;
type TrailHeads = HashMap<Index, HashMap<Index, usize>>;
type Trails = HashSet<Trail>;

fn find_trails(grid: &TrailTopo) -> TrailHeads {
    let trail_starts = grid.pos_with_item(0);
    assert!(trail_starts.len() > 0);
    let mut trail_heads = TrailHeads::default();
    let mut complete_trails = Trails::default();
    let mut trails = trail_starts
        .iter()
        .map(|&s| vec![s])
        .collect::<Vec<Trail>>();

    while let Some(trail) = trails.pop() {
        let last = *trail.last().unwrap();
        let last_height = grid.at(last).unwrap();
        if last_height == 9 {
            let first = *trail.first().unwrap();
            *trail_heads
                .entry(first)
                .or_default()
                .entry(last)
                .or_default() += 1;
            complete_trails.insert(trail);
        } else {
            Dir4::cw()
                .map(|dir| last + dir)
                .filter(|&i| {
                    if grid.is_valid(i) {
                        let height = grid.at(i).unwrap();
                        height == last_height + 1
                    } else {
                        false
                    }
                })
                .map(|i| {
                    let mut t = trail.clone();
                    t.push(i);
                    t
                })
                .for_each(|t| trails.push(t));
        }
    }
    trail_heads
}

pub fn part1(input: &str) -> usize {
    let grid = Grid::parse_items(input, |c| c.to_digit(10).unwrap());
    let trails = find_trails(&grid);
    trails.iter().map(|(_, s)| s.len()).sum()
}

pub fn part2(input: &str) -> usize {
    let grid = Grid::parse_items(input, |c| c.to_digit(10).unwrap());
    let trails = find_trails(&grid);
    trails
        .iter()
        .map(|(_, s)| s.iter().map(|(_, &r)| r).sum::<usize>())
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 36;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 81;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
}
