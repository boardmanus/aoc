use lazy_regex::regex_captures;
use aoc_utils::vec2d::Vec2d;

type Coord = Vec2d<usize>;

const ADJUSTMENT: Coord = Coord::new(10000000000000, 10000000000000);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Claw {
    a: Coord,
    b: Coord,
    prize: Coord,
}

impl Claw {
    fn new(a: Coord, b: Coord, prize: Coord) -> Claw {
        Claw { a, b, prize }
    }

    fn adjust(c: Coord) -> Coord {
        c + ADJUSTMENT
    }

    fn parse(input: &str, adjust_prize_loc: bool) -> Vec<Claw> {
        input
            .split("\n\n")
            .filter_map(|cm| {
                let mut it = cm.lines().filter_map(|line| {
                    let (_, x_str, y_str) =
                        regex_captures!(r"^.*: X[+=](\d+), Y[+=](\d+)$", line)?;
                    Some(Coord::new(x_str.parse::<usize>().ok()?, y_str.parse::<usize>().ok()?))
                });
                let (a, b, mut prize) = (it.next()?, it.next()?, it.next()?);
                if adjust_prize_loc {
                    prize = Self::adjust(prize);
                }
                Some(Claw::new(a, b, prize))
            })
            .collect()
    }

    fn solve(&self) -> Option<(isize, isize)> {
        let b_n: isize =
            ((self.a.x * self.prize.y) as isize - (self.a.y * self.prize.x) as isize).abs();
        let b_d: isize = ((self.a.x * self.b.y) as isize - (self.a.y * self.b.x) as isize).abs();
        if b_n < b_d || b_n % b_d != 0 {
            None
        } else {
            let b = b_n / b_d;
            let a_n = ((self.prize.x as isize) - b * (self.b.x as isize)).abs();
            let a_d = self.a.x as isize;
            if a_n < a_d || a_n % a_d != 0 {
                None
            } else {
                let a = a_n / a_d;
                Some((a, b))
            }
        }
    }
}

fn tokens(a: isize, b: isize) -> isize {
    a * 3 + b
}

pub fn part1(input: &str) -> usize {
    let claws = Claw::parse(input, false);
    claws
        .iter()
        .filter_map(|claw| claw.solve())
        .map(|(a, b)| tokens(a, b))
        .sum::<isize>() as usize
}

pub fn part2(input: &str) -> usize {
    let claws = Claw::parse(input, true);
    claws
        .iter()
        .filter_map(|claw| claw.solve())
        .map(|(a, b)| tokens(a, b))
        .sum::<isize>() as usize
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 480;

    #[test]
    fn test_parse() {
        let claws = Claw::parse(TEST_INPUT, false);
        assert_eq!(
            claws,
            vec![
                Claw::new(
                    Coord::new(94, 34),
                    Coord::new(22, 67),
                    Coord::new(8400, 5400)
                ),
                Claw::new(
                    Coord::new(26, 66),
                    Coord::new(67, 21),
                    Coord::new(12748, 12176)
                ),
                Claw::new(
                    Coord::new(17, 86),
                    Coord::new(84, 37),
                    Coord::new(7870, 6450)
                ),
                Claw::new(
                    Coord::new(69, 23),
                    Coord::new(27, 71),
                    Coord::new(18641, 10279)
                )
            ]
        );
    }

    #[test]
    fn test_solve() {
        let claws = Claw::parse(TEST_INPUT, false);
        assert_eq!(claws[0].solve(), Some((80, 40)));
        assert_eq!(claws[2].solve(), Some((38, 86)));
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }
}
