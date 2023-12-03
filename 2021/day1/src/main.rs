fn depth_increases(depths: &[i64]) -> usize {
    (1..depths.len())
        .into_iter()
        .filter(|i| depths[*i] > depths[i - 1])
        .count()
}
fn solve_part1(depths: &[i64]) -> usize {
    depth_increases(depths)
}

fn solve_part2(depths: &[i64]) -> usize {
    let windowed_depths = (0..depths.len() - 2)
        .into_iter()
        .map(|i| depths[i] + depths[i + 1] + depths[i + 2])
        .collect::<Vec<i64>>();
    depth_increases(&windowed_depths)
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .split('\n')
        .into_iter()
        .flat_map(|s| s.parse::<i64>())
        .collect()
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let depths = parse_input(INPUT);
    let part1 = solve_part1(&depths);
    println!("Part1: {part1}");
    let part2 = solve_part2(&depths);
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&parse_input(TEST_INPUT)), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&parse_input(TEST_INPUT)), 5);
    }
}
