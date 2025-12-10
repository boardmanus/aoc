use itertools::Itertools;

use aoc_utils::pos2d::Pos2d;

pub fn part1(input: &str) -> usize {
    let pts = parse_input(input);
    (0..pts.len()).fold(0_i64, |max, i| {
        (i + 1..pts.len()).fold(max, |max, j| {
            let a = pts[i];
            let b = pts[j];
            let v = b - a;
            ((v.x.abs() + 1) * (v.y.abs() + 1)).max(max)
        })
    }) as usize
}

pub fn part2(input: &str) -> usize {
    let pts = parse_input(input);

    let mut max = 0;
    for pair in pts.iter().combinations(2) {
        let [p1, p2] = pair[0..2] else { unreachable!() };
        let x1 = p1.x.min(p2.x);
        let x2 = p1.x.max(p2.x);
        let y1 = p1.y.min(p2.y);
        let y2 = p1.y.max(p2.y);
        // Logic inspired by the Sutherland–Hodgman algorithm
        if pts.iter().enumerate().all(|(k, &p)| {
            match (((x1 + 1)..x2).contains(&p.x), ((y1 + 1)..y2).contains(&p.y)) {
                (true, true) => false, // Point inside
                (true, false) => {
                    let np = pts[(k + 1) % pts.len()];
                    if p.x == np.x {
                        let range = p.y.min(np.y)..=p.y.max(np.y);
                        // Check if line crosses horizontally
                        return !(y1 != y2 && range.contains(&y1) && range.contains(&y2));
                    }
                    true
                }
                (false, true) => {
                    let np = pts[(k + 1) % pts.len()];
                    if p.y == np.y {
                        let range = p.x.min(np.x)..=p.x.max(np.x);
                        // Check if line crosses vertically
                        return !(x1 != x2 && range.contains(&x1) && range.contains(&x2));
                    }
                    true
                }
                (false, false) => true, // Completely outside
            }
        }) {
            max = max.max((y2 - y1 + 1) * (x2 - x1 + 1));
        }
    }
    max as usize
}

fn parse_input(input: &str) -> Vec<Pos2d<i64>> {
    input
        .lines()
        .map(|line| {
            let (x_str, y_str) = line.split_once(',').unwrap();
            Pos2d::new(x_str.parse::<i64>().unwrap(), y_str.parse::<i64>().unwrap())
        })
        .collect()
}
const INPUT: &str = include_str!("data/input");
fn main() {
    aoc_utils::run::main(INPUT, part1, part2);
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 50;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 24;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
}
