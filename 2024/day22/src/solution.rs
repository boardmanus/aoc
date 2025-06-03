use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

fn mix(val: usize, secret: usize) -> usize {
    val ^ secret
}

fn prune(secret: usize) -> usize {
    secret % 16777216
}

fn next_seed(secret: usize) -> usize {
    let res = secret * 64;
    let secret = prune(mix(res, secret));

    let res = secret / 32;
    let secret = prune(mix(res, secret));

    let res = secret * 2048;
    let secret = prune(mix(res, secret));

    secret
}

fn nth_seed(secret: usize, n: usize) -> usize {
    let mut secret = secret;
    for _ in 0..n {
        secret = next_seed(secret);
    }
    secret
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Seq([i8; 4]);
impl Seq {
    fn from(slice: &[i8]) -> Seq {
        let mut a = [0_i8; 4];
        a.copy_from_slice(slice);
        Seq(a)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PriceInfo {
    i: usize,
    price: i8,
    seq: Seq,
}
impl PriceInfo {
    fn new(i: usize, price: i8, seq: Seq) -> PriceInfo {
        PriceInfo { i, price, seq }
    }
}

type SeqCache<'a> = HashMap<Seq, (usize, i8)>;

// 1. Find index in sequence where numbers are the same at + 4
// 2. Cache index of the first instance of any sequence
fn find_sequences(secret: usize) -> Vec<PriceInfo> {
    const SEQ_LEN: usize = 2000;
    const NUM_CHANGES: usize = 4;

    let mut seed = secret;

    let prices = (0..SEQ_LEN)
        .map(|_| {
            let last = seed;
            seed = next_seed(seed);
            (last % 10) as i8
        })
        .collect::<Vec<_>>();

    let price_diffs = (0..SEQ_LEN - 1)
        .map(|i| (prices[i + 1] - prices[i]) as i8)
        .collect::<Vec<_>>();

    // Cache holding all the seen sequences.
    // Used to ensure only the first instance of the sequence is used
    let seq_cache: SeqCache =
        (0..SEQ_LEN - NUM_CHANGES - 1).fold(SeqCache::new(), |mut cache, i| {
            cache
                .entry(Seq::from(&price_diffs[i..i + 4]))
                .or_insert((i, prices[i]));
            cache
        });

    let match_prices: Vec<PriceInfo> = (0..SEQ_LEN - NUM_CHANGES)
        .filter(|&i| {
            prices[i] == prices[i + NUM_CHANGES]
                && prices[i] > 0
                && seq_cache.contains_key(&Seq::from(&price_diffs[i..i + 4]))
        })
        .map(|x| PriceInfo::new(x, prices[x], Seq::from(&price_diffs[x..x + 4])))
        .collect::<Vec<_>>();

    println!(
        "match-prices: secret={secret}, num-matches={} => {:?}",
        match_prices.len(),
        match_prices
    );

    match_prices
}

fn most_bananas(seeds: &Vec<usize>) -> usize {
    let mut cache: HashMap<Seq, Vec<usize>> = HashMap::new();

    seeds.iter().for_each(|&seed| {
        find_sequences(seed).into_iter().for_each(|info| {
            cache.entry(info.seq).or_default().push(info.price as usize);
        })
    });

    let mut values = cache.values().collect::<Vec<_>>();
    values.sort_by(|a, b| a.len().cmp(&b.len()).reverse());
    println!("sizes={:?}", values);

    let test = cache.get(&Seq::from(&[2, -2, -4, 4])); //[-2i8, 1, -1, 3]);
    println!("test values = {:?}", test);

    cache
        .into_iter()
        .map(|(_, prices)| prices.iter().sum())
        .fold(0, usize::max)
}

pub fn part1(input: &str) -> usize {
    let initial_seeds = parse_input(input);
    initial_seeds.iter().map(|&seed| nth_seed(seed, 2000)).sum()
}

fn prices(seed: usize) -> Vec<i64> {
    let mut seed = seed;
    (0..2000)
        .map(|_| {
            let last = seed;
            seed = next_seed(seed);
            (last % 10) as i64
        })
        .collect::<Vec<_>>()
}
pub fn part2(input: &str) -> usize {
    let initial_seeds = parse_input(input);
    most_bananas(&initial_seeds)
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 37327623;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 23;

    #[test]
    fn test_find_sequences() {
        let x = find_sequences(123);
        let a = x.iter().find(|pi| pi.seq == Seq::from(&[-1, -1, 0, 2]));
        assert_eq!(a.unwrap().price, 6);
        assert_eq!(a.unwrap().i, 2);

        let x = find_sequences(1);
        let a = x.iter().find(|pi| pi.seq == Seq::from(&[-2, 1, -1, 3]));
        //assert_eq!(a.unwrap().price, 7);

        let mut seed = 1;
        let prices = (0..2001)
            .map(|_| {
                let last = seed;
                seed = next_seed(seed);
                (last % 10) as i8
            })
            .collect::<Vec<_>>();

        let price_diffs = (0..2001 - 1)
            .map(|i| (prices[i + 1] - prices[i]) as i8)
            .collect::<Vec<_>>();

        // Cache holding all the seen sequences.
        // Used to ensure only the first instance of the sequence is used
        let seq_cache: SeqCache = (0..(2001 - 4 - 1)).fold(SeqCache::new(), |mut cache, i| {
            cache
                .entry(Seq::from(&price_diffs[i..i + 4]))
                .or_insert((i, prices[i]));
            cache
        });

        let x = seq_cache.get(&Seq::from(&[-2, 1, -1, 3])).unwrap();
        //assert!(x.is_some());
        println!("{:?}", x);
        println!("{:?}", &prices[x.0 - 2..x.0 + 6]);
        println!("{:?}", &price_diffs[x.0 - 2..x.0 + 6]);
        //assert_eq!(seq_cache.get(&Seq::from(&[-1, -1, 0, 2])), Some(&(2, 6)))

        prices
            .iter()
            .enumerate()
            .filter(|x| *x.1 == 7 && prices[x.0 + 4] == 7)
            .for_each(|x| {
                println!(
                    "{}: {:?} ... {:?}",
                    x.0,
                    &prices[(0i64.max(x.0 as i64) as usize)..2000.min(x.0 + 5)],
                    &price_diffs[(0i64.max(x.0 as i64) as usize)..2000.min(x.0 + 4)]
                )
            });
    }

    #[test]
    fn test_next_secret() {
        let secrets: [usize; 10] = [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        let mut seed = 123;
        let test_secrets = (0..10)
            .map(|_| {
                seed = next_seed(seed);
                seed
            })
            .collect::<Vec<_>>();
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
