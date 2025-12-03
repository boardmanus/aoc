fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn max_joltage(bank: &[usize], num_batteries: usize) -> usize {
    if num_batteries == 0 {
        return 0;
    }
    let max_j = bank[..bank.len() - num_batteries + 1].iter().max().unwrap();
    let (b_pos, _) = bank.iter().enumerate().find(|(_, j)| *j == max_j).unwrap();
    max_j * 10_usize.pow(num_batteries as u32 - 1)
        + max_joltage(&bank[b_pos + 1..], num_batteries - 1)
}

pub fn part1(input: &str) -> usize {
    let banks = parse_input(input);
    banks
        .iter()
        .map(|bank| {
            let f = bank[..bank.len() - 1].iter().max().unwrap();
            let (pos, _) = bank[..].iter().enumerate().find(|(i, b)| *b == f).unwrap();
            let l = bank[pos + 1..].iter().max().unwrap();
            10 * f + l
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let banks = parse_input(input);
    banks.iter().map(|bank| max_joltage(bank, 12)).sum()
}

const INPUT: &str = include_str!("data/input");
fn main() {
    aoc_utils::run::main(INPUT, part1, part2);
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 357;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 3121910778619;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
}
