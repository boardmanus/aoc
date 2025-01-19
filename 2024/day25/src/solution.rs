use aoc_utils::str::AocStr;

#[derive(Debug, Clone, PartialEq)]
struct Shape(Vec<usize>);

impl Shape {
    fn parse(input: &str) -> (Shape, bool) {
        let lines = input.lines().collect::<Vec<_>>();
        let is_lock = lines[0] == "#####";

        let shape = (0..5)
            .map(|i| {
                lines
                    .iter()
                    .map(|&line| line.nth(i))
                    .filter(|c| *c == '#')
                    .count()
                    - 1
            })
            .collect::<Vec<_>>();

        (Shape(shape), is_lock)
    }

    fn fits(&self, shape: &Shape) -> bool {
        (0..self.0.len()).all(|i| self.0[i] + shape.0[i] <= 5)
    }
}

fn parse_input(input: &str) -> (Vec<Shape>, Vec<Shape>) {
    let mut locks_n_keys = (vec![], vec![]);
    input.split("\n\n").for_each(|lk_str| {
        let (shape, is_lock) = Shape::parse(lk_str);
        if is_lock {
            locks_n_keys.0.push(shape);
        } else {
            locks_n_keys.1.push(shape);
        }
    });
    locks_n_keys
}

pub fn part1(input: &str) -> usize {
    let (locks, keys) = parse_input(input);
    println!("Num locks={}, Num keys={}", locks.len(), keys.len());
    println!("Locks: {:?}", locks);
    println!("Keys: {:?}", keys);
    locks
        .iter()
        .map(|lock| keys.iter().filter(|key| key.fits(lock)).count())
        .sum()
}

pub fn part2(input: &str) -> String {
    input.to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 3;
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
