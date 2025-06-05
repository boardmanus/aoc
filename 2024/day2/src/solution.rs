use aoc_utils::str::AocStr;

pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.parse_lines(|line| line.parse_nums())
}

pub fn is_safe_diff(maybe_last_sign: Option<i32>, a: i32, b: i32) -> (bool, Option<i32>) {
    let diff = b - a;
    let sign = Some(diff.signum());
    let safe =
        (diff != 0) && (diff.abs() < 4) && (maybe_last_sign.is_none() || maybe_last_sign == sign);
    (safe, sign)
}

pub fn find_bad_index(levels: &[i32]) -> Option<usize> {
    let mut safe: (bool, Option<i32>) = (false, None);
    (0..levels.len() - 1).find(|&i| {
        safe = is_safe_diff(safe.1, levels[i], levels[i + 1]);
        !safe.0
    })
}

pub fn is_safe(levels: &[i32]) -> bool {
    find_bad_index(levels).is_none()
}

pub fn without_index(v: &[i32], remove_i: usize) -> Vec<i32> {
    (0..v.len())
        .filter(|&i| i != remove_i)
        .map(|i| v[i])
        .collect()
}

pub fn is_safe_with_removal(levels: &[i32]) -> bool {
    if let Some(bad_index) = find_bad_index(levels) {
        is_safe(&without_index(levels, bad_index))
            || is_safe(&without_index(levels, bad_index + 1))
            || (bad_index > 0) && is_safe(&without_index(levels, bad_index - 1))
    } else {
        true
    }
}

pub fn part1(input: &str) -> usize {
    let levels = parse_input(input);
    levels.iter().filter(|&level| is_safe(level)).count()
}

pub fn part2(input: &str) -> usize {
    let levels = parse_input(input);
    levels
        .iter()
        .filter(|&level| is_safe_with_removal(level))
        .count()
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 2;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 4;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2("10 11 9 8\n"), 1);
        assert_eq!(part2("59 55 52 49 48\n"), 1);
    }

    #[test]
    fn test_parse_input() {
        let i = parse_input("1 2 3 4\n6 7 8\n9\n");
        assert_eq!(i, vec![vec![1, 2, 3, 4], vec![6, 7, 8], vec![9]]);
    }
}
