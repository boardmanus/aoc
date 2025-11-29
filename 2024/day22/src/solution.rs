use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Secret(usize);

impl Secret {
    fn parse(input: &str) -> Vec<Secret> {
        input
            .lines()
            .filter_map(|line| line.parse::<usize>().ok())
            .map(Secret)
            .collect()
    }

    fn mix(self, val: usize) -> Secret {
        Secret(val ^ self.0)
    }

    fn prune(self) -> Secret {
        Secret(self.0 % 16777216)
    }

    fn next_value(self) -> Secret {
        let secret = self.mix(self.0 * 64).prune();
        let secret = secret.mix(secret.0 / 32).prune();
        secret.mix(secret.0 * 2048).prune()
    }
}

impl Iterator for Secret {
    type Item = Secret;

    fn next(&mut self) -> Option<Self::Item> {
        let old_secret = *self;
        *self = old_secret.next_value();
        Some(old_secret)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Seq([i8; 4]);

impl Seq {
    fn from(slice: &[i8]) -> Seq {
        Seq([slice[0], slice[1], slice[2], slice[3]])
    }
}

type SeqPrice = HashMap<Seq, i8>;
type SeqCache = HashMap<Seq, Vec<i8>>;

fn price_seq_cache(secret: Secret, num: usize, cache: &mut SeqCache) {
    let prices = secret
        .map(|s| (s.0 % 10) as i8)
        .take(num)
        .collect::<Vec<_>>();

    let price_diffs = prices[1..]
        .iter()
        .enumerate()
        .map(|(i, &p)| p - prices[i])
        .collect::<Vec<_>>();

    let seq_prices = (0..price_diffs.len() - 3).fold(SeqPrice::new(), |mut cache, i| {
        let seq = Seq::from(&price_diffs[i..]);
        cache.entry(seq).or_insert(prices[i + 4]);
        cache
    });

    for (seq, price) in seq_prices {
        cache.entry(seq).or_default().push(price);
    }
}

pub fn part1(input: &str) -> usize {
    let initial_seeds = Secret::parse(input);
    initial_seeds
        .iter()
        .filter_map(|&seed| Some(seed.clone().nth(2000)?.0))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let secrets = Secret::parse(input);
    let mut cache = SeqCache::new();
    for secret in secrets {
        price_seq_cache(secret, 2000, &mut cache);
    }
    let max = cache
        .values()
        .map(|prices| prices.iter().map(|p| *p as isize).sum())
        .max();
    max.unwrap_or(0) as usize
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 37327623;
    pub const TEST_INPUT_2: &str = include_str!("data/input_example_2");
    pub const TEST_ANSWER_2: usize = 23;

    #[test]
    fn test_secret_to_prices() {
        let expected_prices: [i8; 10] = [3, 0, 6, 5, 4, 4, 6, 4, 4, 2];
        let secret = Secret(123);
        let prices = secret
            .map(|s| (s.0 % 10) as i8)
            .take(10)
            .collect::<Vec<_>>();
        assert_eq!(&prices, &expected_prices);
    }

    #[test]
    fn test_price_changes() {
        let expected_changes: [i8; 9] = [-3, 6, -1, -1, 0, 2, -2, 0, -2];
        let secret = Secret(123);
        let prices = secret
            .map(|s| (s.0 % 10) as i8)
            .take(10)
            .collect::<Vec<_>>();
        let changes = prices[1..]
            .iter()
            .enumerate()
            .map(|(i, &p)| p - prices[i])
            .collect::<Vec<_>>();

        assert_eq!(&changes, &expected_changes);
    }

    #[test]
    fn test_price_seq_cache() {
        let expected_seq = Seq([-1, -1, 0, 2]);
        let mut cache = &mut SeqCache::new();
        price_seq_cache(Secret(123), 10, &mut cache);
        let price_entry = cache.get(&expected_seq);
        assert!(price_entry.is_some());
        assert_eq!(*price_entry.unwrap(), vec![6]);
        for (_seq, prices) in cache.iter() {
            assert_eq!(prices.len(), 1);
        }
    }

    #[test]
    fn test_next_secret() {
        let secrets = [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ]
        .iter()
        .map(|x| Secret(*x))
        .collect::<Vec<_>>();
        let seed = Secret::seed(123);
        let test_secrets = seed.take(10).collect::<Vec<_>>();
        assert_eq!(test_secrets, secrets);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
}
