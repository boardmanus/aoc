fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().expect("wasn't a digit"))
                .collect()
        })
        .collect()
}

fn joltage_pos(bank: &[usize], num_batteries: usize) -> Option<(usize, usize)> {
    let joltage = *bank[..bank.len() - num_batteries + 1].iter().max()?;
    let (battery_pos, _) = bank.iter().enumerate().find(|&(_, &j)| j == joltage)?;
    Some((joltage, battery_pos))
}

fn max_joltage(bank: &[usize], current_joltage: usize, num_batteries: usize) -> usize {
    if num_batteries == 0 {
        current_joltage
    } else {
        let (b_joltage, b_pos) = joltage_pos(bank, num_batteries).expect("invalid");
        let joltage = current_joltage * 10 + b_joltage;
        max_joltage(&bank[b_pos + 1..], joltage, num_batteries - 1)
    }
}

pub fn part1(input: &str) -> usize {
    let banks = parse_input(input);
    banks.iter().map(|bank| max_joltage(bank, 0, 2)).sum()
}

pub fn part2(input: &str) -> usize {
    let banks = parse_input(input);
    banks.iter().map(|bank| max_joltage(bank, 0, 12)).sum()
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
