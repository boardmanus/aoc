use std::collections::HashMap;

type DigitMap = HashMap<&'static str, i64>;
fn digit_map() -> DigitMap {
    HashMap::from([
        ("0", 0),
        ("zero", 0),
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
    ])
}

fn solve_part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter(|c| c.is_numeric());
            let f = digits
                .nth(0)
                .expect(format!("Expected a first digit '{line}'").as_str());
            let l = digits.last().unwrap_or(f);
            let fl = [f, l]
                .iter()
                .collect::<String>()
                .parse::<i64>()
                .expect("Expected a number");
            fl
        })
        .sum()
}

fn starts_with_digit(line: &str, digit_map: &DigitMap) -> Option<i64> {
    for (digit, value) in digit_map {
        if line.starts_with(digit) {
            return Some(*value);
        }
    }
    None
}

fn line_to_digits(line: &str, digit_map: &DigitMap) -> Vec<i64> {
    (0..line.len()).fold(Vec::new(), |mut acc, i| {
        let d = starts_with_digit(&line[i..], digit_map);
        if let Some(digit) = d {
            acc.push(digit);
        }
        acc
    })
}

fn solve_part2(input: &str) -> i64 {
    let dm = digit_map();
    input
        .lines()
        .map(|line| {
            let digits = line_to_digits(line, &dm);
            let f = digits.first().expect("Expected a first digit");
            let l = digits.last().unwrap_or(f);
            f * 10 + l
        })
        .sum()
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
    const TEST_INPUT_2: &str = include_str!("test_input2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 142);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 281);
    }

    #[test]
    fn test_starts_with_digit() {
        let dm = digit_map();
        assert_eq!(starts_with_digit("onexxx", &dm), Some(1));
        assert_eq!(starts_with_digit("7xxx", &dm), Some(7));
        assert_eq!(starts_with_digit("xxx7", &dm), None);
    }
}
