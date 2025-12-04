use aoc_utils::{
    dir::Dir8,
    grud::{Grid, GridPos},
};

fn is_removable(grid: &Grid<char, Dir8>, pos: &GridPos) -> bool {
    grid.at(pos) == Some('@')
        && grid
            .neighbours(*pos)
            .filter(|x| grid.at(x) == Some('@'))
            .count()
            < 4
}

fn remove_rolls(grid: &mut Grid<char, Dir8>) -> usize {
    let removable = grid
        .iter_pos()
        .filter(|x| is_removable(grid, x))
        .collect::<Vec<_>>();
    let num = removable.len();
    for pos in removable {
        grid.set(&pos, '.');
    }
    num
}
pub fn part1(input: &str) -> usize {
    let grid = Grid::<char, Dir8>::parse(input);
    grid.iter_pos()
        .filter(|pos| is_removable(&grid, pos))
        .count()
}

pub fn part2(input: &str) -> usize {
    let mut grid = Grid::<char, Dir8>::parse(input);
    let mut n = remove_rolls(&mut grid);
    let mut total = n;
    while n > 0 {
        n = remove_rolls(&mut grid);
        total += n;
    }
    total
}

const INPUT: &str = include_str!("data/input");
fn main() {
    aoc_utils::run::main(INPUT, part1, part2);
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 13;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 43;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
}
