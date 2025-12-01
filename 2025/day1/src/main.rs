use std::io::Cursor;

fn parse_input(input: &str) -> Vec<(char, i64)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.chars();
            let dir = parts.next().expect("No dir found");
            let clicks = parts.as_str().parse::<i64>().expect("No clicks found");
            (dir, clicks)
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let moves = parse_input(input);
    let mut pos = 50;
    moves
        .iter()
        .map(|(dir, clicks)| {
            println!("pos {pos} + ({dir}, {clicks})");
            pos = match dir {
                'L' => pos - clicks,
                'R' => pos + clicks,
                _ => panic!("Unknown direction"),
            }
            .rem_euclid(100);
            pos
        })
        .filter(|&p| p == 0)
        .count()
}

pub fn part2(input: &str) -> usize {
    let moves = parse_input(input);
    let mut last_pos = 50;
    moves.iter().fold(0, |zeros, (dir, clicks)| {
        print!("pos {last_pos} + ({dir}, {clicks}) ");
        let mut linear_pos = match dir {
            'L' => last_pos - clicks,
            'R' => last_pos + clicks,
            _ => panic!("Unknown direction"),
        };

        let div = (clicks / 100).abs();
        let curr_pos = linear_pos.rem_euclid(100);
        let so = last_pos.signum();
        let no = curr_pos.signum();
        let d = if so != no { 1 } else { 0 };
        println!(
            "{linear_pos}/{curr_pos} => {so}/{no}, d={d}, div={div} => {}",
            div + d
        );

        if last_pos == curr_pos || last_pos == 0 {
            println!("last_pos={last_pos}, curr_pos={curr_pos}, clicks={clicks}, d={d}, div={div}");
        }

        last_pos = curr_pos.rem_euclid(100);
        zeros + div.abs() + d
    }) as usize
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
    pub const TEST_ANSWER_2: usize = 6;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
}
