use std::collections::{hash_map::Entry, HashMap};

use aoc_utils::{
    dir::Dir4,
    grif::Graph,
    grud::{Grid, GridPos},
};

type TrailMap = Grid<u32, Dir4>;

fn traversable(g: &Grid<u32, Dir4>, from: &GridPos, to: &GridPos) -> bool {
    // A path is traversable if the to position is exactly 1 higher than the from.
    let to_height = g.at(to);
    let from_height = g.at(from);
    to_height.is_some() && from_height.is_some() && to_height.unwrap() == from_height.unwrap() + 1
}

fn trail_score(g: &TrailMap, location: GridPos) -> usize {
    // Perform a depth-first search, and count all nodes reached with a height of 9
    g.dfs(location, |_| true)
        .map(|n| if g.at(&n) == Some(9) { 1 } else { 0 })
        .sum()
}

fn trail_map_score(g: &TrailMap) -> usize {
    // Map score is the sum of all the trail scores from the start locations
    g.filter_items(0).map(|start| trail_score(g, start)).sum()
}

fn trail_map_rating_r(g: &TrailMap, from: GridPos, cache: &mut HashMap<GridPos, usize>) -> usize {
    if let Entry::Occupied(e) = cache.entry(from) {
        // Just return the cached value
        *e.get()
    } else {
        let rating = if g.at(&from) == Some(9) {
            // At max height, it's always one route
            1
        } else {
            // Recursively calculate the trail rating for each neighbour of this location.
            // The sum of them will be the trail rating at this location.
            g.neighbours(from)
                .map(|n| trail_map_rating_r(g, n, cache))
                .sum()
        };
        // Cache the trail rating determined at this location
        cache.insert(from, rating);
        rating
    }
}

fn trail_map_rating(g: &TrailMap) -> usize {
    // Trail map rating is the sum of all the ratings from each start position
    let mut cache: HashMap<GridPos, usize> = HashMap::new();
    g.filter_items(0)
        .map(|start| trail_map_rating_r(g, start, &mut cache))
        .sum()
}

pub fn part1(input: &str) -> usize {
    let trail_map = TrailMap::parse_items(input, |c| c.to_digit(10).unwrap(), traversable);
    trail_map_score(&trail_map)
}

pub fn part2(input: &str) -> usize {
    let trail_map = TrailMap::parse_items(input, |c| c.to_digit(10).unwrap(), traversable);
    trail_map_rating(&trail_map)
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
