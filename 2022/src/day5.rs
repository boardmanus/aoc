use crate::aoc;

#[derive(Debug)]
enum Error {
    BadNumItems,
}

pub struct Day5_1;
impl aoc::Aoc<u32> for Day5_1 {
    fn day(&self) -> u32 {
        5
    }
    fn puzzle_name(&self) -> &str {
        "???"
    }
    fn solve(&self, lines: &Vec<String>) -> u32 {
        lines.iter();
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stuff() {}
}
