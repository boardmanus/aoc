fn solve_part1(input: &str) -> String {
    0.to_string()
}

fn solve_part2(input: &str) -> String {
    0.to_string()
}

fn main() {
    let res = solve_part1(include_str!("input.txt"));
    println!("Part1: {res}");
    let res = solve_part2(include_str!("input.txt"));
    println!("Part2: {res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), "2=-1=0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 54.to_string());
    }
}
