pub fn part1(input: &str) -> String {
    input.to_string()
}

pub fn part2(input: &str) -> String {
    input.to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: &str = "part1";
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: &str = "part2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
}
