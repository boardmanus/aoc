use aoc_utils::{
    dir::{Dir, Dir8},
    grid::{Grid, Index},
    str::AocStr,
};

const SEARCH: &str = "XMAS";

trait Day4Grid {
    fn matches_in_dir(&self, pos: Index, s: &str, dir: Dir8) -> bool;
    fn count_matches(&self, pos: Index, s: &str) -> usize;
}

impl Day4Grid for Grid<char> {
    fn matches_in_dir(&self, pos: Index, s: &str, dir: Dir8) -> bool {
        let mut next_pos = pos;
        s.chars().all(|c| {
            let good = self.at(next_pos) == Some(c);
            next_pos = next_pos + dir;
            good
        })
    }

    fn count_matches(&self, pos: Index, s: &str) -> usize {
        Dir8::cw()
            .into_iter()
            .filter(|&dir| self.matches_in_dir(pos, s, dir))
            .count()
    }
}

pub fn part1(input: &str) -> usize {
    let grid = Grid::<char>::parse(input);
    let start = grid.filter_pos(SEARCH.first().unwrap());
    start
        .iter()
        .map(|&pos| grid.count_matches(pos, SEARCH))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let grid = Grid::<char>::parse(input);
    let start = grid.filter_pos('A');
    start
        .iter()
        .filter(|&pos| {
            ((grid.matches(*pos + Index(-1, -1), 'M') && grid.matches(*pos + Index(1, 1), 'S'))
                || (grid.at(*pos + Index(-1, -1)) == Some('S')
                    && grid.at(*pos + Index(1, 1)) == Some('M')))
                && ((grid.at(*pos + Index(-1, 1)) == Some('M')
                    && grid.at(*pos + Index(1, -1)) == Some('S'))
                    || (grid.at(*pos + Index(-1, 1)) == Some('S')
                        && grid.at(*pos + Index(1, -1)) == Some('M')))
        })
        .count()
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 18;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 9;

    #[test]
    fn test_grid_count_matches() {
        let g = Grid::<char>::parse("1234\n1234\n5678\n");
        assert_eq!(g.count_matches(Index(2, 0), &"34"), 2);
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
