use std::vec;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Stuff {
    Ash,
    Rock,
}

struct Grid {
    rows: Vec<u64>,
    cols: Vec<u64>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            rows: vec![0; height],
            cols: vec![0; width],
        }
    }
    fn set(&mut self, x: usize, y: usize, stuff: Stuff) {
        if stuff == Stuff::Ash {
            self.rows[y] &= !(1 << x);
            self.cols[x] &= !(1 << y);
        } else {
            self.rows[y] |= 1 << x;
            self.cols[x] |= 1 << y;
        }
    }
    fn get(&self, x: usize, y: usize) -> Stuff {
        if self.rows[y] & (1 << x) == 0 {
            Stuff::Ash
        } else {
            Stuff::Rock
        }
    }
    fn row(&self, y: usize) -> u64 {
        self.rows[y]
    }

    fn col(&self, x: usize) -> u64 {
        self.cols[x]
    }
}

fn parse(input: &str) -> Vec<Grid> {
    input
        .split("\n\n")
        .map(|grid_lines| {
            let width = grid_lines.lines().next().unwrap().len();
            let height = grid_lines.lines().count();
            let mut grid = Grid::new(width, height);
            for (y, line) in grid_lines.lines().enumerate() {
                for (x, c) in line.chars().enumerate() {
                    grid.set(
                        x,
                        y,
                        match c {
                            '.' => Stuff::Ash,
                            '#' => Stuff::Rock,
                            _ => panic!("Unknown char: {}", c),
                        },
                    );
                }
            }
            grid
        })
        .collect()
}

fn is_reflection(values: &Vec<u64>, start: usize) -> bool {
    for i in 0..=start {
        let j = start + 1 + i;
        if j < values.len() && values[start - i] != values[j] {
            return false;
        }
    }
    true
}

fn find_reflection(values: &Vec<u64>) -> Option<usize> {
    for i in 0..(values.len() - 1) {
        if values[i] == values[i + 1] {
            if is_reflection(&values, i) {
                return Some(i);
            }
        }
    }
    None
}

fn reflection_value(grid: &Grid) -> usize {
    if let Some(reflection) = find_reflection(&grid.rows) {
        (reflection + 1) * 100
    } else if let Some(reflection) = find_reflection(&grid.cols) {
        reflection + 1
    } else {
        panic!("No reflection found!");
    }
}

fn solve_part1(input: &str) -> usize {
    let grids = parse(input);
    grids.iter().map(|grid| reflection_value(grid)).sum()
}

fn solve_part2(input: &str) -> u64 {
    0
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let part1 = solve_part1(INPUT);
    println!("Part1: {part1}");
    let part2 = solve_part2(INPUT);
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
    const TEST_INPUT_2: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 405);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 467835);
    }

    #[test]
    fn test_parse() {
        let grids = parse(TEST_INPUT);
        assert_eq!(grids.len(), 2);

        let grid = &grids[0];
        assert_eq!(grid.rows.len(), 7);
        assert_eq!(grid.cols.len(), 9);
        assert_eq!(grid.get(4, 4), Stuff::Rock);
        assert_eq!(grid.get(8, 6), Stuff::Ash);
        assert_eq!(grid.get(7, 6), Stuff::Rock);
    }
}
