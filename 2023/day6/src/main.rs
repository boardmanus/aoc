fn parse_line_1(line: &str) -> Option<Vec<u64>> {
    line.split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().ok())
        .collect()
}

fn parse_line_2(line: &str) -> Option<u64> {
    line.split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .ok()
}

fn parse_input_1(input: &str) -> Option<Vec<(u64, u64)>> {
    let mut lines = input.lines();
    let time = parse_line_1(lines.nth(0)?)?;
    let dist = parse_line_1(lines.nth(0)?)?;
    Some(time.into_iter().zip(dist).collect())
}

fn parse_input_2(input: &str) -> Option<(u64, u64)> {
    let mut lines = input.lines();
    let time = parse_line_2(lines.nth(0)?)?;
    let dist = parse_line_2(lines.nth(0)?)?;
    Some((time, dist))
}

fn num_ways(time: u64, dist: u64) -> usize {
    let b = time as f64;
    let c = (dist + 1) as f64;
    let d = ((b * b - 4.0 * c) as f64).sqrt();
    let e = ((b - d) / 2.0).ceil() as usize;
    let s = ((b + d) / 2.0).trunc() as usize;
    s - e + 1
}

fn solve_part1(input: &str) -> usize {
    let races = parse_input_1(input).unwrap();
    races.iter().map(|(t, d)| num_ways(*t, *d)).product()
}

fn solve_part2(input: &str) -> usize {
    let (t, d) = parse_input_2(input).unwrap();
    num_ways(t, d)
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let part1 = solve_part1(INPUT);
    println!("Part1: {part1}");
    let part2 = solve_part2(INPUT);
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
    const TEST_INPUT_2: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 71503);
    }

    #[test]
    fn test_parse_line_1() {
        assert_eq!(parse_line_1("Time:      7  15   30"), Some(vec![7, 15, 30]));
    }

    #[test]
    fn test_parse_line_2() {
        assert_eq!(parse_line_2("Time:      7  15   30"), Some(71530));
    }

    #[test]
    fn test_parse_input_1() {
        assert_eq!(
            parse_input_1(TEST_INPUT),
            Some(vec![(7, 9), (15, 40), (30, 200),])
        );
    }
}
