use aoc_utils::{
    dir::{Dir, Dir8},
    grud, pos2d,
};

const SEARCH: &str = "XMAS";
type Grid = grud::Grid<char, Dir8>;
type Pos2d = pos2d::Pos2d<i64>;

trait Day4Grid {
    fn matches_in_dir(&self, pos: Pos2d, s: &str, dir: Dir8) -> bool;
    fn count_matches(&self, pos: Pos2d, s: &str) -> usize;
}

impl Day4Grid for Grid {
    fn matches_in_dir(&self, pos: Pos2d, s: &str, dir: Dir8) -> bool {
        let mut next_pos = pos;
        s.chars().all(|c| {
            let good = self.at(&next_pos) == Some(c);
            next_pos = next_pos + dir;
            good
        })
    }

    fn count_matches(&self, pos: Pos2d, s: &str) -> usize {
        Dir8::cw()
            .filter(|&dir| self.matches_in_dir(pos, s, dir))
            .count()
    }
}

pub fn part1(input: &str) -> usize {
    let grid = Grid::parse(input);
    grid.filter_items('X')
        .map(|pos| grid.count_matches(pos, SEARCH))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let corners = [Dir8::NW, Dir8::NE, Dir8::SE, Dir8::SW];
    let grid = Grid::parse(input);
    grid.filter_items('A')
        .filter(|&pos| {
            let num_matches = corners
                .iter()
                .filter(|&&dir| grid.matches_in_dir(pos - dir, "MAS", dir))
                .count();
            num_matches == 2
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
        let g = Grid::parse("1234\n1234\n5678\n");
        assert_eq!(g.count_matches(Pos2d::new(2, 0), &"34"), 2);
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
