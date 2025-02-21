#![allow(dead_code)]

use std::collections::HashSet;

use aoc_utils::{
    dir::{Dir, Dir4},
    grof,
    grud::{Grid, GridPos},
};

struct Plot {
    vege: char,
    locations: Vec<GridPos>,
}

impl Plot {
    fn new(garden: &Garden, index: GridPos) -> Plot {
        let vege = garden.0.at(&index).unwrap();
        let mut locations = garden.filter_connected_dfs(index);
        locations.sort();
        Plot { vege, locations }
    }

    fn perimeter(&self, garden: &Garden) -> usize {
        self.locations
            .iter()
            .map(|&i| 4 - garden.0.neighbours(i).count())
            .sum()
    }

    fn corners(&self, garden: &Garden) -> usize {
        self.locations
            .iter()
            .map(|&pos| {
                // Take all the corners for a position in the plot
                [
                    (Dir4::N, Dir4::E),
                    (Dir4::S, Dir4::E),
                    (Dir4::S, Dir4::W),
                    (Dir4::N, Dir4::W),
                ]
                .into_iter()
                // Get the vege in each corner pos
                // NE => 1 3 SW => 2 0
                //       0 2       3 1
                .map(|v| [pos, pos + v.0, pos + v.1, pos + v.0 + v.1].map(|p| garden.0.at(&p)))
                .filter(|p| {
                    // keep if both side positions don't match, or bot side positions do match, and diag doesn't
                    // y .  ||  x y  => corner
                    // x y  ||  x x
                    (p[0] != p[1] && p[0] != p[2]) || (p[0] == p[1] && p[0] == p[2] && p[0] != p[3])
                })
                .count()
            })
            .sum()
    }

    fn sides2(&self, garden: &Garden) -> usize {
        // number of sides == number of corners
        self.corners(garden)
    }

    fn sides(&self, garden: &Garden) -> usize {
        let mut all_edges: HashSet<(GridPos, Dir4)> =
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
                if garden.0.at(&next_pos) != Some(self.vege) {
                    next_pos = next_pos - travel_dir.to_vec2d();
                    travel_dir = travel_dir.rotate_cw();
                    check_dir = check_dir.rotate_cw();
                    num_sides += 1;
                } else if garden.0.at(&(next_pos + check_dir)) == Some(self.vege) {
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
            self.sides2(garden)
        } else {
            self.perimeter(garden)
        };

        value * self.locations.len()
    }
}

struct Garden(Grid<char, Dir4>);

impl Garden {
    fn parse(input: &str) -> Garden {
        Garden(Grid::<char, Dir4>::parse_walkable(input, |g, from, to| {
            g.at(from) == g.at(to)
        }))
    }

    fn edge_dirs(&self, index: GridPos, vege: char, edges: &mut HashSet<(GridPos, Dir4)>) {
        Dir4::cw().for_each(|dir| {
            if self.0.at(&(index + dir)) != Some(vege) {
                edges.insert((index, dir));
            }
        });
    }

    fn filter_connected_dfs(&self, index: GridPos) -> Vec<GridPos> {
        let mut locations: Vec<GridPos> = Vec::new();
        grof::algorithms::dfs(&self.0, index, |n| locations.push(*n));
        locations
    }

    fn create_plots(&self) -> Vec<Plot> {
        let mut used_locations: HashSet<GridPos> = HashSet::new();

        self.0.iter_pos().fold(Vec::<Plot>::new(), |mut acc, v| {
            if !used_locations.contains(&v) {
                let plot = Plot::new(self, v);
                plot.locations.iter().for_each(|&l| {
                    used_locations.insert(l);
                });
                acc.push(plot);
            }
            acc
        })
    }

    fn fence_cost(&self, discount: bool) -> usize {
        let plots = self.create_plots();
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
        let plot = Plot::new(&garden, GridPos::new(0, 0));
        let v = plot.locations.clone();
        let v2 = vec![
            GridPos::new(0, 0),
            GridPos::new(1, 0),
            GridPos::new(2, 0),
            GridPos::new(3, 0),
            GridPos::new(0, 1),
            GridPos::new(1, 1),
            GridPos::new(2, 1),
            GridPos::new(3, 1),
            GridPos::new(2, 2),
            GridPos::new(3, 2),
            GridPos::new(4, 2),
            GridPos::new(2, 3),
        ];
        assert_eq!(v, v2);
        assert_eq!(garden.0.at(&GridPos::new(0, 0)).unwrap(), plot.vege);
    }

    #[test]
    fn test_plot_sides() {
        let garden = Garden::parse(TEST_INPUT_P2_1);
        let plot = Plot::new(&garden, GridPos::new(2, 1));
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
