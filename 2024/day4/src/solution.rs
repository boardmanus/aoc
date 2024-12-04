use aoc_utils::{grid::{Grid, Index}, str::AocStr};

const SEARCH: &str = "XMAS";


trait Day4Grid {
    fn count_matches_in_dir(&self, pos: Index, s: &str, dir: Index) -> usize;
    fn count_matches(&self, pos: Index, s: &str) -> usize;
}

impl Day4Grid for Grid<char> {

    fn count_matches_in_dir(&self, pos: Index, s: &str, dir: Index) -> usize {
        let mut next_pos = pos + dir;
        if s.chars().all(|c| {
            let good = self.at(next_pos) == Some(c);
            next_pos = next_pos + dir;
            good
        }) {
            1
        } else {
            0
        }
    }

    fn count_matches(&self, pos: Index, s: &str) -> usize {
        if let Some(c) = s.first() {
            self.around(pos)
                .into_iter()
                .filter(|&pos2| self.at(pos2) == Some(c))
                .map(|pos2| {
                    let count = self.count_matches_in_dir(pos2, &s[1..], pos2 - pos);
                    count
                })
                .sum()
        } else {
            1
        }
    }
}

pub fn part1(input: &str) -> usize {
    let grid = Grid::<char>::parse(input);
    let start = grid.pos_with_item(SEARCH.first().unwrap());
    start
        .iter()
        .map(|&pos| grid.count_matches(pos, &SEARCH[1..]))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let grid = Grid::<char>::parse(input);
    let start = grid.pos_with_item('A');
    start
        .iter()
        .filter(|&pos| {
            ((grid.at_match(*pos + Index(-1, -1), 'M')
                && grid.at_match(*pos + Index(1, 1), 'S'))
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
        assert_eq!(g.count_matches(Index(2, 0), &"4"), 2);
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
