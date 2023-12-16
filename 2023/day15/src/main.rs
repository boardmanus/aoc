fn parse(input: &str) -> Vec<&str> {
    input.trim().split(',').collect()
}

fn checksum(steps: &str) -> usize {
    steps
        .chars()
        .fold(0, |cs, x| ((cs + x as usize) * 17) % 256)
}

fn solve_part1(input: &str) -> usize {
    parse(input).iter().map(|x| checksum(x)).sum()
}

fn solve_part2(input: &str) -> u64 {
    0
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
        assert_eq!(solve_part1(TEST_INPUT), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 467835);
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(TEST_INPUT),
            vec![
                "rn=1", "cm-", "qp=3", "cm=2", "qp-", "pc=4", "ot=9", "ab=5", "pc-", "pc=6", "ot=7"
            ]
        );
    }
}
