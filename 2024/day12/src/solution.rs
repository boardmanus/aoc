use std::collections::{HashMap, HashSet};

use aoc_utils::{
    dir::{Dir, Dir4},
    grid::{Grid, Index},
};

struct Plot {
    vege: char,
    locations: Vec<Index>,
}

impl Plot {
    fn new(garden: &Garden, index: Index) -> Plot {
        let vege = garden.0.at(index).unwrap();
        let mut locations = garden.filter_connected(index);
        locations.sort();
        Plot { vege, locations }
    }

    fn perimeter(&self, garden: &Garden) -> usize {
        self.locations
            .iter()
            .map(|&i| 4 - garden.count_matches(i, self.vege))
            .sum()
    }

    fn sides(&self, garden: &Garden) -> usize {
        let mut all_edges: HashSet<(Index, Dir4)> =
            self.locations
                .iter()
                .fold(HashSet::new(), |mut col, &index| {
                    garden.edge_dirs(index, self.vege, &mut col);
                    col
                });
        let mut num_sides = 0usize;

        while let Some(&(start_pos, start_dir)) = all_edges.iter().next() {
            let mut check_dir = start_dir;
            let mut travel_dir = start_dir.rotate_cw();
            let mut next_pos = start_pos + travel_dir;
            all_edges.remove(&(start_pos, start_dir));
            while start_pos != next_pos || start_dir != check_dir {
                if garden.0.at(next_pos) != Some(self.vege) {
                    next_pos = next_pos - travel_dir;
                    travel_dir = travel_dir.rotate_cw();
                    check_dir = check_dir.rotate_cw();
                    num_sides += 1;
                } else if garden.0.at(next_pos + check_dir) == Some(self.vege) {
                    next_pos = next_pos + check_dir;
                    travel_dir = travel_dir.rotate_ccw();
                    check_dir = check_dir.rotate_ccw();
                    num_sides += 1;
                } else {
                    all_edges.remove(&(next_pos, check_dir));
                    next_pos = next_pos + travel_dir;
                }
            }
        }
        num_sides
    }

    fn fence_cost(&self, garden: &Garden, discount: bool) -> usize {
        let value = if discount {
            self.sides(garden)
        } else {
            self.perimeter(garden)
        };

        value * self.locations.len()
    }
}

struct Garden(Grid<char>);

impl Garden {
    fn parse(input: &str) -> Garden {
        Garden(Grid::<char>::parse(input))
    }

    fn count_matches(&self, index: Index, vege: char) -> usize {
        Dir4::cw()
            .filter(|&d| self.0.matches(index + d, vege))
            .count()
    }

    fn edge_dirs(&self, index: Index, vege: char, edges: &mut HashSet<(Index, Dir4)>) {
        Dir4::cw().for_each(|dir| {
            if self.0.at(index + dir) != Some(vege) {
                edges.insert((index, dir));
            }
        });
    }

    pub fn filter_connected(&self, index: Index) -> Vec<Index> {
        let mut searched: HashSet<Index> = HashSet::new();
        let mut to_search = vec![index];
        let mut connected = Vec::<Index>::new();
        let vege = self.0.at(index);

        while let Some(i) = to_search.pop() {
            assert!(!searched.contains(&i));
            connected.push(i);
            Dir4::cw()
                .map(|d| i + d)
                .filter(|&i2| self.0.at(i2) == vege && !searched.contains(&i2))
                .for_each(|i3| {
                    if !to_search.contains(&i3) {
                        to_search.push(i3);
                    }
                });
            searched.insert(i);
        }

        connected
    }

    fn find_plot(&self, index: Index) -> Plot {
        Plot::new(self, index)
    }

    fn fence_cost(&self, discount: bool) -> usize {
        let mut used_locations: HashSet<Index> = HashSet::new();

        let plots = self
            .0
            .row_index_iter()
            .fold(Vec::<Plot>::new(), |mut acc, v| {
                if !used_locations.contains(&v.0) {
                    let plot = self.find_plot(v.0);
                    plot.locations.iter().for_each(|&l| {
                        used_locations.insert(l);
                    });
                    acc.push(plot);
                }
                acc
            });
        plots
            .iter()
            .map(|plot| plot.fence_cost(self, discount))
            .sum()
    }
}

pub fn part1(input: &str) -> usize {
    let garden = Garden::parse(input);
    garden.fence_cost(false)
}

pub fn part2(input: &str) -> usize {
    let garden = Garden::parse(input);
    garden.fence_cost(true)
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 1930;
    pub const TEST_INPUT_P1_2: &str = include_str!("data/input_example_2");
    pub const TEST_ANSWER_P1_2: usize = 772;
    pub const TEST_INPUT_P1_3: &str = include_str!("data/input_example_3");
    pub const TEST_ANSWER_P1_3: usize = 140;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 1206;
    pub const TEST_INPUT_P2_1: &str = include_str!("data/input_example_p2_1");
    pub const TEST_ANSWER_P2_1: usize = 80;
    pub const TEST_INPUT_P2_2: &str = include_str!("data/input_example_p2_2");
    pub const TEST_ANSWER_P2_2: usize = 236;
    pub const TEST_INPUT_P2_3: &str = include_str!("data/input_example_p2_3");
    pub const TEST_ANSWER_P2_3: usize = 368;

    #[test]
    fn test_plot_new() {
        let garden = Garden::parse(TEST_INPUT);
        let plot = Plot::new(&garden, Index(0, 0));
        let v = plot.locations.clone();
        let v2 = vec![
            Index(0, 0),
            Index(1, 0),
            Index(2, 0),
            Index(3, 0),
            Index(0, 1),
            Index(1, 1),
            Index(2, 1),
            Index(3, 1),
            Index(2, 2),
            Index(3, 2),
            Index(4, 2),
            Index(2, 3),
        ];
        assert_eq!(v, v2);
        assert_eq!(garden.0.at(Index(0, 0)).unwrap(), plot.vege);
    }

    #[test]
    fn test_plot_sides() {
        let garden = Garden::parse(TEST_INPUT_P2_1);
        let plot = Plot::new(&garden, Index(2, 1));
        assert_eq!(plot.sides(&garden), 8);
    }
    #[test]
    fn test_part1_2() {
        assert_eq!(part1(TEST_INPUT_P1_2), TEST_ANSWER_P1_2);
    }

    #[test]
    fn test_part1_3() {
        assert_eq!(part1(TEST_INPUT_P1_3), TEST_ANSWER_P1_3);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
    #[test]
    fn test_part2_1() {
        assert_eq!(part2(TEST_INPUT_P2_1), TEST_ANSWER_P2_1);
    }
    #[test]
    fn test_part2_2() {
        assert_eq!(part2(TEST_INPUT_P2_2), TEST_ANSWER_P2_2);
    }
    #[test]
    fn test_part2_3() {
        assert_eq!(part2(TEST_INPUT_P2_3), TEST_ANSWER_P2_3);
    }
}
