use aoc_utils::{dir::Dir4, grud::Grid};

#[derive(Debug)]
struct Block {
    g: Grid<char, Dir4>,
}

impl Block {
    fn parse(input: &str) -> Block {
        Block {
            g: Grid::<char, Dir4>::parse(input),
        }
    }

    fn area(&self) -> usize {
        self.g.filter_items('#').count()
    }
}

#[derive(Debug)]
struct Fit {
    width: usize,
    height: usize,
    num_blocks: Vec<usize>,
}

#[derive(Debug)]
struct Puzzle {
    blocks: Vec<Block>,
    fits: Vec<Fit>,
}

impl Puzzle {
    fn definite_or_possible_fits(&self) -> Vec<&Fit> {
        self.fits
            .iter()
            .map(|f| {
                let min_area = f
                    .num_blocks
                    .iter()
                    .enumerate()
                    .map(|(i, n)| n * self.blocks[i].area())
                    .sum::<usize>();
                let max_area = f.num_blocks.iter().sum::<usize>() * 9;
                let area = f.width * f.height;
                (f, min_area, area, max_area)
            })
            .filter(|a| a.1 <= a.2 && a.2 >= a.3)
            .map(|a| {
                println!(
                    "Definit: Fit={:?}, min={},area={},max={}",
                    a.0, a.1, a.2, a.3
                );
                a.0
            })
            .collect()
    }

    fn possible_fits(&self) -> Vec<&Fit> {
        self.fits
            .iter()
            .map(|f| {
                let min_area = f
                    .num_blocks
                    .iter()
                    .enumerate()
                    .map(|(i, n)| n * self.blocks[i].area())
                    .sum::<usize>();
                let max_area = f.num_blocks.iter().sum::<usize>() * 9;
                let area = f.width * f.height;
                (f, min_area, area, max_area)
            })
            .filter(|a| a.1 <= a.2 && a.2 < a.3)
            .map(|a| {
                println!(
                    "Possible: Fit={:?}, min={},area={},max={}",
                    a.0, a.1, a.2, a.3
                );
                a.0
            })
            .collect()
    }
}

fn parse_input(input: &str) -> Puzzle {
    let sections = input.split("\n\n");
    let blocks = sections
        .clone()
        .take(6)
        .map(|g_str| {
            let (_, grid_str) = g_str.split_once(":\n").unwrap();
            Block::parse(grid_str)
        })
        .collect::<Vec<_>>();
    let fits = sections
        .skip(6)
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (dim_str, count_str) = line.split_once(": ").unwrap();
            let (width_str, height_str) = dim_str.split_once('x').unwrap();
            let width = width_str.parse::<usize>().unwrap();
            let height = height_str.parse::<usize>().unwrap();
            let num_blocks = count_str
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            Fit {
                width,
                height,
                num_blocks,
            }
        })
        .collect::<Vec<_>>();
    Puzzle { blocks, fits }
}

pub fn part1(input: &str) -> usize {
    let puzzle = parse_input(input);
    puzzle.definite_or_possible_fits().len()
}

pub fn part2(input: &str) -> usize {
    0
}

const INPUT: &str = include_str!("data/input");
fn main() {
    aoc_utils::run::main(INPUT, part1, part2);
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 2;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 0;

    #[test]
    fn test_general_area() {
        let puzzle = parse_input(INPUT);
        puzzle.definite_or_possible_fits();
        puzzle.possible_fits();
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
