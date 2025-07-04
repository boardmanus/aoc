#![allow(dead_code)]

use std::{collections::HashSet, rc::Rc};

use aoc_utils::{
    dir::Dir4,
    grif::Graph,
    grud::{Grid, GridPos},
};

type PlotOp = fn(plot: &Plot) -> usize;

struct Plot {
    grid: Rc<Grid<char, Dir4>>,
    locations: Vec<GridPos>,
}

impl Plot {
    fn new(grid: Rc<Grid<char, Dir4>>, index: GridPos) -> Plot {
        let mut locations = grid.dfs(index, |_| true).collect::<Vec<_>>();
        locations.sort();
        Plot { grid, locations }
    }

    fn perimeter(&self) -> usize {
        self.locations
            .iter()
            .map(|&i| 4 - self.grid.neighbours(i).count())
            .sum()
    }

    fn corners(&self) -> usize {
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
                .map(|v| [pos, pos + v.0, pos + v.1, pos + v.0 + v.1].map(|p| self.grid.at(&p)))
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

    fn sides(&self) -> usize {
        // number of sides == number of corners
        self.corners()
    }

    fn fence_cost(&self) -> usize {
        self.perimeter() * self.locations.len()
    }

    fn discounted_fence_cost(&self) -> usize {
        self.sides() * self.locations.len()
    }
}

struct Garden {
    grid: Rc<Grid<char, Dir4>>,
    plots: Vec<Plot>,
}

impl Garden {
    fn parse(input: &str) -> Garden {
        let grid = Rc::new(Grid::<char, Dir4>::parse_walkable(input, |g, from, to| {
            g.at(from) == g.at(to)
        }));
        let mut used_locations: HashSet<GridPos> = HashSet::new();
        let plots = grid.iter_pos().fold(Vec::<Plot>::new(), |mut acc, v| {
            if !used_locations.contains(&v) {
                let plot = Plot::new(grid.clone(), v);
                plot.locations.iter().for_each(|&l| {
                    used_locations.insert(l);
                });
                acc.push(plot);
            }
            acc
        });
        Garden { grid, plots }
    }

    fn plot_sum(&self, op: PlotOp) -> usize {
        self.plots.iter().map(op).sum()
    }

    fn fence_cost(&self) -> usize {
        self.plot_sum(|plot| plot.fence_cost())
    }

    fn discounted_fence_cost(&self) -> usize {
        self.plot_sum(|plot| plot.discounted_fence_cost())
    }
}

pub fn part1(input: &str) -> usize {
    let garden: Garden = Garden::parse(input);
    garden.fence_cost()
}

pub fn part2(input: &str) -> usize {
    let garden = Garden::parse(input);
    garden.discounted_fence_cost()
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
        let plot = Plot::new(garden.grid, GridPos::new(0, 0));
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
    }

    #[test]
    fn test_plot_sides() {
        let garden = Garden::parse(TEST_INPUT_P2_1);
        let plot = Plot::new(garden.grid, GridPos::new(2, 1));
        assert_eq!(plot.sides(), 8);
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
