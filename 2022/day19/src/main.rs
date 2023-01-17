mod rts;
use rts::BluePrint;

fn solve_part1(input: &str) -> String {
    if let Ok(blueprints) = BluePrint::parse(input) {
        blueprints
            .iter()
            .map(|bp| bp.quality_level(24))
            .sum::<usize>()
            .to_string()
    } else {
        "failed".to_string()
    }
}

fn solve_part2(input: &str) -> String {
    if let Ok(blueprints) = BluePrint::parse(input) {
        blueprints
            .iter()
            .take(3)
            .map(|bp| bp.max_geodes(32))
            .product::<usize>()
            .to_string()
    } else {
        "failed".to_string()
    }
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
        assert_eq!(solve_part1(TEST_INPUT), 33.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), "3472");
    }
}
