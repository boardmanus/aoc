use crate::utils::sorted;

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let v = input
        .lines()
        .map(|line| {
            let mut v = line
                .split_whitespace()
                .map(|v_str| v_str.parse::<u32>().unwrap());
            (v.next().unwrap(), v.next().unwrap())
        })
        .collect::<Vec<_>>();

    (
        v.iter().map(|p| p.0).collect::<Vec<_>>(),
        v.iter().map(|p| p.1).collect::<Vec<_>>(),
    )
}

fn sum_differences(left: &[u32], right: &[u32]) -> u32 {
    assert_eq!(left.len(), right.len());
    let left_s = sorted(left);
    let right_s = sorted(right);

    (0..left_s.len())
        .map(|i| (left_s[i] as i32 - right_s[i] as i32).abs())
        .sum::<i32>() as u32
}

pub fn part1(input: &str) -> u32 {
    let (left, right) = parse_input(input);
    sum_differences(&left, &right)
}

fn similarity_score(left: &[u32], right: &[u32]) -> u32 {
    left.iter()
        .map(|v| v * (right.iter().filter(|&ov| v == ov).count() as u32))
        .sum::<u32>()
}

pub fn part2(input: &str) -> u32 {
    let (left, right) = parse_input(input);
    similarity_score(&left, &right)
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: u32 = 11;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: u32 = 31;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input("12 11\n10 8"), (vec![12, 10], vec![11, 8]));
    }

    #[test]
    fn test_sum_differences() {
        assert_eq!(sum_differences(&[1, 2, 3], &[6, 5, 4]), 9);
    }

    #[test]
    fn test_similarity_score() {
        assert_eq!(similarity_score(&[1, 2, 3], &[1, 3, 3]), 1 + 3 * 2);
    }
}
