#[derive(Debug, Copy, Clone, PartialEq)]
enum StoneChange {
    Replace(u64),
    Split(u64, u64),
}

impl StoneChange {
    fn change(stone: u64) -> StoneChange {
        if stone == 0 {
            StoneChange::Replace(1)
        } else {
            let num_digits = num_digits(stone);
            if num_digits % 2 == 0 {
                let pow = 10u64.pow(num_digits / 2);
                let left = stone / pow;
                let right = stone - left * pow;
                StoneChange::Split(left, right)
            } else {
                StoneChange::Replace(stone * 2024)
            }
        }
    }
}

fn num_digits(stone: u64) -> u32 {
    stone.ilog10() + 1
}

fn blink(stones: &[u64]) -> Vec<u64> {
    stones
        .iter()
        .fold(Vec::with_capacity(stones.len()), |mut coll, &stone| {
            match StoneChange::change(stone) {
                StoneChange::Replace(x) => coll.push(x),
                StoneChange::Split(l, r) => {
                    coll.push(l);
                    coll.push(r);
                }
            };
            coll
        })
}

fn blink_heaps(stones: &[u64], num_blinks: usize) -> usize {
    (0..num_blinks)
        .fold(Vec::from(stones), |coll, _| blink(&coll))
        .len()
}

fn blink_lots(stone: u64, num_blinks: usize) -> usize {
    if num_blinks == 0 {
        1
    } else {
        match StoneChange::change(stone) {
            StoneChange::Replace(x) => blink_lots(x, num_blinks - 1),
            StoneChange::Split(l, r) => {
                blink_lots(l, num_blinks - 1) + blink_lots(r, num_blinks - 1)
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|stone_str| stone_str.parse::<u64>().unwrap())
        .collect()
}

pub fn part1(input: &str) -> usize {
    let stones = parse_input(input);
    blink_heaps(&stones, 25)
}

pub fn part2(input: &str) -> usize {
    let stones = parse_input(input);
    blink_heaps(&stones, 75)
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 55312;
    pub const TEST_INPUT_2: &str = TEST_INPUT;

    #[test]
    fn test_num_digits() {
        assert_eq!(num_digits(7), 1);
        assert_eq!(num_digits(23), 2);
        assert_eq!(num_digits(541), 3);
    }

    #[test]
    fn test_stone_change() {
        assert_eq!(StoneChange::change(0), StoneChange::Replace(1));
        assert_eq!(StoneChange::change(1), StoneChange::Replace(2024));
        assert_eq!(StoneChange::change(1234), StoneChange::Split(12, 34));
    }

    #[test]
    fn test_blink_lots() {
        assert_eq!(blink_lots(125, 25) + blink_lots(17, 25), TEST_ANSWER);
    }

    #[test]
    fn test_part1_6_iters() {
        let mut stones = blink(&[125, 17]);
        assert_eq!(stones, &[253000, 1, 7]);
        stones = blink(&stones);
        assert_eq!(stones, &[253, 0, 2024, 14168]);
        stones = blink(&stones);
        assert_eq!(stones, &[512072, 1, 20, 24, 28676032]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }
}
