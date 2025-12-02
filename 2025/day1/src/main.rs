fn parse_input(input: &str) -> Vec<(char, i64)> {
    input
        .lines()
        .map(|line| {
            let (dir_str, num_str) = line.split_at(1);
            let dir = dir_str.chars().next().expect("No dir found");
            let clicks = num_str.parse::<i64>().expect("No clicks found");
            (dir, clicks)
        })
        .collect()
}

fn rotate(dir: char, clicks: i64) -> i64 {
    match dir {
        'L' => -clicks,
        'R' => clicks,
        _ => panic!("Unknown direction"),
    }
}

fn zero_clicked(start_pos: i64, rotation: i64) -> (i64, i64) {
    let end_pos = (start_pos + rotation).rem_euclid(100);
    let d_pos = end_pos - start_pos;
    let past_zero = match (start_pos, end_pos, d_pos) {
        (0, _, _) | (_, _, 0) => 0,
        (_, 0, _) => 1,
        _ => (d_pos.signum() != rotation.signum()) as i64,
    };

    (end_pos, (rotation / 100).abs() + past_zero)
}

pub fn part1(input: &str) -> usize {
    let moves = parse_input(input);
    let mut pos = 50;
    moves
        .iter()
        .map(|&(dir, clicks)| {
            pos = (pos + rotate(dir, clicks)).rem_euclid(100);
            pos
        })
        .filter(|&p| p == 0)
        .count()
}

pub fn part2(input: &str) -> usize {
    let moves = parse_input(input);
    let mut last_pos = 50;
    moves
        .iter()
        .map(|&(dir, clicks)| {
            let (curr_pos, z) = zero_clicked(last_pos, rotate(dir, clicks));
            last_pos = curr_pos;
            z
        })
        .sum::<i64>() as usize
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
    fn test_zero_clicked() {
        assert_eq!((1, 0), zero_clicked(0, 1));
        assert_eq!((99, 0), zero_clicked(0, -1));
        assert_eq!((99, 0), zero_clicked(0, 99));
        assert_eq!((1, 1), zero_clicked(0, 101));
        assert_eq!((0, 1), zero_clicked(0, 100));
        assert_eq!((0, 1), zero_clicked(0, -100));
        assert_eq!((0, 1), zero_clicked(1, 99));
        assert_eq!((0, 2), zero_clicked(1, 199));
        assert_eq!((0, 1), zero_clicked(1, -1));
        assert_eq!((0, 2), zero_clicked(1, -101));
        assert_eq!((1, 1), zero_clicked(1, -100));
        assert_eq!((1, 1), zero_clicked(1, 100));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
}
