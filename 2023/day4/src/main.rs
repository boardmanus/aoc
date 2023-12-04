use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{space1, u64, u8},
    multi::separated_list1,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

fn card_line(input: &str) -> IResult<&str, (usize, HashSet<u8>, HashSet<u8>)> {
    let (input, card_num) =
        delimited(pair(tag("Card"), space1), u64, pair(tag(":"), space1))(input)?;
    let (input, winning_nums) = separated_list1(space1, u8)(input)?;
    let (input, card_nums) = preceded(
        tuple((space1, tag("|"), space1)),
        separated_list1(space1, u8),
    )(input)?;

    Ok((
        input,
        (
            card_num as usize,
            HashSet::from_iter(winning_nums.into_iter()),
            HashSet::from_iter(card_nums.into_iter()),
        ),
    ))
}

fn scratch_card_sum(line: &str) -> usize {
    let (_, ticket) = card_line(line).unwrap();
    let num_matches = ticket.1.intersection(&ticket.2).count();
    if num_matches == 0 {
        0
    } else {
        1 << (num_matches - 1)
    }
}

fn scratch_card_copy_sum(
    mut acc: HashMap<usize, usize>,
    line: &str,
    max_card_num: usize,
) -> HashMap<usize, usize> {
    let (_, ticket) = card_line(line).unwrap();
    let card = ticket.0;
    let num_matches = ticket.1.intersection(&ticket.2).count();
    *acc.entry(card).or_insert(0) += 1;

    let num_copies = *acc.get(&card).unwrap();
    for copy in (card + 1)..max_card_num.min(card + num_matches + 1) {
        *acc.entry(copy).or_insert(0) += num_copies;
    }

    acc
}

fn solve_part1(input: &str) -> usize {
    input.lines().map(scratch_card_sum).sum()
}

fn solve_part2(input: &str) -> usize {
    let max_card_num = input.lines().count() + 1;
    let card_freqs = input
        .lines()
        .fold(HashMap::<usize, usize>::default(), |acc, line| {
            scratch_card_copy_sum(acc, line, max_card_num)
        });
    card_freqs.iter().fold(0, |acc, (_, count)| acc + count)
}

fn main() {
    let part1 = solve_part1(include_str!("input.txt"));
    println!("Part1: {part1}");
    let part2 = solve_part2(include_str!("input.txt"));
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
    const TEST_INPUT_2: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 30);
    }

    #[test]
    fn test_parseline() {
        assert_eq!(
            parse_line("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            (
                3,
                HashSet::from([1, 21, 53, 59, 44]),
                HashSet::from([69, 82, 63, 72, 16, 21, 14, 1])
            )
        );
    }
}
