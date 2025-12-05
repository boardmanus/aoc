use std::ops::RangeInclusive;

fn parse_input(input: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    input
        .split_once("\n\n")
        .map(|(range_str, ingredient_str)| {
            let mut x: Vec<_> = range_str
                .lines()
                .map(|line| {
                    line.split_once('-')
                        .map(|(a, b)| a.parse::<usize>().unwrap()..=b.parse::<usize>().unwrap())
                        .unwrap()
                })
                .collect();
            x.sort_by(|x, y| x.start().cmp(y.start()));

            let y = ingredient_str
                .lines()
                .map(|line| line.parse::<usize>().unwrap())
                .collect();
            (x, y)
        })
        .unwrap()
}

pub fn part1(input: &str) -> usize {
    let (ranges, ingredients) = parse_input(input);
    ranges
        .iter()
        .fold(Vec::<RangeInclusive<usize>>::new(), |mut acc, range| {
            if let Some(last) = acc.last_mut() {
                if last.end() >= range.start() {
                    *last = (*last.start())..=(*range.end());
                } else {
                    acc.push(range.clone());
                }
            } else {
                acc.push(range.clone());
            }
            acc
        });

    ingredients
        .iter()
        .filter_map(|i| ranges.iter().find(|r| r.contains(i)))
        .count()
}

pub fn part2(input: &str) -> usize {
    0
}

const INPUT: &str = include_str!("data/input");
fn main() {
    aoc_utils::run::main(INPUT, part1, part2);
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 3;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 14;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
}
