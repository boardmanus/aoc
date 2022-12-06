use itertools::Itertools;

use crate::aoc;

fn find_start(input_str: &str, num_unique: usize) -> (usize, &str) {
    let val = input_str[num_unique - 1..]
        .chars()
        .enumerate()
        .find(|(i, _c)| {
            input_str[*i..*i + num_unique]
                .chars()
                .into_iter()
                .duplicates()
                .next()
                .is_none()
        })
        .unwrap();
    (val.0 + num_unique, &input_str[val.0..val.0 + 4])
}

pub struct Day6_1;
impl aoc::Aoc<u32> for Day6_1 {
    fn day(&self) -> u32 {
        6
    }
    fn puzzle_name(&self) -> &str {
        "Tuning Trouble"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        find_start(&lines[0].as_str(), 4).0.to_string()
    }
}

pub struct Day6_2;
impl aoc::Aoc<u32> for Day6_2 {
    fn day(&self) -> u32 {
        6
    }
    fn puzzle_name(&self) -> &str {
        "Tuning Trouble 2"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        find_start(&lines[0].as_str(), 14).0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soln() {
        assert_eq!(find_start("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), (5, "vwbj"));
        assert_eq!(find_start("nppdvjthqldpwncqszvftbrmjlhg", 4), (6, "pdvj"));
        assert_eq!(
            find_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            (10, "rfnt")
        );
        assert_eq!(
            find_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            (11, "zqfr")
        );
    }

    #[test]
    fn test_soln2() {
        assert_eq!(find_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).0, 19);
        assert_eq!(find_start("bvwbjplbgvbhsrlpgdmjqwftvncz", 14).0, 23);
        assert_eq!(find_start("nppdvjthqldpwncqszvftbrmjlhg", 14).0, 23);
        assert_eq!(find_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14).0, 29);
        assert_eq!(find_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14).0, 26);
    }
}
