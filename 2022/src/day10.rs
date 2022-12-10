use crate::aoc::Aoc;

pub struct Day10_1;
impl Aoc for Day9_1 {
    fn day(&self) -> u32 {
        10
    }
    fn puzzle_name(&self) -> &str {
        "???"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        lines[0]
    }
}

pub struct Day10_2;
impl Aoc for Day10_2 {
    fn day(&self) -> u32 {
        10
    }
    fn puzzle_name(&self) -> &str {
        "??? 2"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        lines[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 8] = ["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];
    const INPUT2: [&str; 8] = ["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"];

    #[test]
    fn test_soln() {
        let input_strs = INPUT
            .map(|s| String::from(s))
            .into_iter()
            .collect::<Vec<String>>();

        assert_eq!(Day9_1.solve(&input_strs), 13.to_string());
    }

    #[test]
    fn test_soln2() {
        let input_strs = INPUT2
            .map(|s| String::from(s))
            .into_iter()
            .collect::<Vec<String>>();

        assert_eq!(Day9_2.solve(&input_strs), 36.to_string());
    }
}
