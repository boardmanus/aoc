use std::collections::HashMap;

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
            let num_digits = stone.ilog10() + 1;
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

type StoneCache = HashMap<(u64, usize), usize>;

fn cached_blink_lots(stone: u64, num_blinks: usize, cache: &mut StoneCache) -> usize {
    let key = (stone, num_blinks);
    if let Some(&count) = cache.get(&key) {
        return count;
    }

    let count = if num_blinks == 0 {
        1
    } else {
        match StoneChange::change(stone) {
            StoneChange::Replace(x) => cached_blink_lots(x, num_blinks - 1, cache),
            StoneChange::Split(l, r) => {
                cached_blink_lots(l, num_blinks - 1, cache)
                    + cached_blink_lots(r, num_blinks - 1, cache)
            }
        }
    };

    cache.insert(key, count);

    count
}

fn blink_lots(stones: &[u64], num_blinks: usize) -> usize {
    let mut cache: HashMap<(u64, usize), usize> = StoneCache::default();

    stones
        .iter()
        .map(|stone| cached_blink_lots(*stone, num_blinks, &mut cache))
        .sum()
}
fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|stone_str| stone_str.parse::<u64>().unwrap())
        .collect()
}

pub fn part1(input: &str) -> usize {
    blink_lots(&parse_input(input), 25)
}

pub fn part2(input: &str) -> usize {
    blink_lots(&parse_input(input), 75)
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 55312;

    #[test]
    fn test_stone_change() {
        assert_eq!(StoneChange::change(0), StoneChange::Replace(1));
        assert_eq!(StoneChange::change(1), StoneChange::Replace(2024));
        assert_eq!(StoneChange::change(1234), StoneChange::Split(12, 34));
    }

    #[test]
    fn test_blink_lots() {
        assert_eq!(blink_lots(&[125, 17], 6), 22);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }
}
