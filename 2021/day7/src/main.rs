fn parse(input: &str) -> Vec<u16> {
    input
        .trim()
        .split(',')
        .flat_map(|s| u16::from_str_radix(s, 10))
        .collect()
}

fn solve_part1(crabs: &[u16]) -> usize {
    let mut sorted = Vec::from(crabs);
    sorted.sort_unstable();
    let v = sorted[crabs.len() / 2];
    sorted.iter().map(|crab| crab.abs_diff(v) as usize).sum()
}

fn solve_part2(crabs: &[u16]) -> usize {
    let v = (crabs.iter().map(|v| *v as usize).sum::<usize>() + 1) / crabs.len();
    crabs
        .iter()
        .map(|crab| {
            let d = v.abs_diff(*crab as usize);
            let res = d * (d + 1) / 2;
            println!("{} - {v} = {d} => {}", *crab, res);
            res
        })
        .sum()
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let crabs = parse(INPUT);
    let part1 = solve_part1(&crabs);
    println!("Part1: {part1}");
    let part2 = solve_part2(&crabs);
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let crabs = parse(TEST_INPUT);
        assert_eq!(solve_part1(&crabs), 37);
    }

    #[test]
    fn test_part2() {
        let crabs = parse(TEST_INPUT);
        assert_eq!(solve_part2(&crabs), 168);
    }

    #[test]
    fn test_parse() {
        let crabs = parse(TEST_INPUT);
        assert_eq!(crabs, [16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
    }
}
