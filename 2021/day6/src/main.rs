use std::{
    collections::{LinkedList, VecDeque},
    fmt::Display,
};

fn parse(lines: &str) -> Vec<i16> {
    lines
        .trim()
        .split(",")
        .flat_map(|vstr| i16::from_str_radix(vstr, 10))
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
struct FishTimer {
    time: i16,
    counter: i16,
}

impl Display for FishTimer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(time={}, c={})", self.time, self.counter)
    }
}

impl Iterator for FishTimer {
    type Item = FishTimer;
    fn next(&mut self) -> Option<Self::Item> {
        let rem = self.time - self.counter - 1;
        if rem >= 0 {
            self.time = rem;
            self.counter = 6;
            Some(FishTimer::new(rem, 6))
        } else {
            None
        }
    }
}

impl FishTimer {
    fn new(time: i16, counter: i16) -> FishTimer {
        FishTimer { time, counter }
    }

    fn spawn_count(&self) -> usize {
        let mut c = 1;
        let mut t = self.time - self.counter - 1;
        while t >= 0 {
            c += FishTimer::new(t, 8).spawn_count();
            t -= 7;
        }
        c
    }
}

fn spawn_count(time: i64, counter: i64) -> usize {
    let mut c = 1;
    let mut t = time - counter - 1;
    while t >= 0 {
        c += spawn_count(t, 8);
        t -= 7;
    }
    c
}

fn spawn(fish: &[i16], time: i16) -> usize {
    fish.iter()
        .map(|c| {
            //let ft = FishTimer::new(time, *c);
            let sc = spawn_count(time as i64, *c as i64); //ft.spawn_count();
            println!("{time}: {sc}");
            sc
        })
        .sum()
}
fn spawn2(fish: &[i16], time: i16) -> usize {
    let mut q = fish
        .iter()
        .map(|c| FishTimer::new(time, *c))
        .collect::<VecDeque<_>>();
    let mut count = 0;
    let mut max_size = 0;
    while let Some(ft) = q.pop_back() {
        count += 1;
        ft.for_each(|f| {
            q.push_back(FishTimer::new(f.time, 8));
        });

        max_size = std::cmp::max(max_size, q.len());
    }
    println!("Max size = {max_size}");
    count
}
fn spawn3(fish: &[i16], time: i16) -> usize {
    let mut state_data = [0; 9];
    let initial_state = fish.iter().fold(&mut state_data, |acc, f| {
        acc[*f as usize] += 1;
        acc
    });
    println!("{:?}", initial_state);

    let run_state = (0..time).fold(initial_state, |acc, day| {
        let idx = (day % 9) as usize;
        let s = acc[idx];
        acc[idx] = 0;
        acc[(idx + 8 + 1) % 9] = s;
        acc[(idx + 6 + 1) % 9] += s;
        acc
    });

    run_state.iter().sum()
}
fn solve_part1(fish: &[i16]) -> usize {
    spawn(fish, 80)
}

fn solve_part2(fish: &[i16]) -> usize {
    spawn3(fish, 256)
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let fish = parse(INPUT);
    let part1 = solve_part1(&fish);
    println!("Part1: {part1}");
    let part2 = solve_part2(&fish);
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let fish = parse(TEST_INPUT);
        assert_eq!(solve_part1(&fish), 5934);
    }

    #[test]
    fn test_part2() {
        let fish = parse(TEST_INPUT);
        assert_eq!(solve_part2(&fish), 26984457539);
    }

    #[test]
    fn test_parse() {
        let fish = parse(TEST_INPUT);
        assert_eq!(fish, [3, 4, 3, 1, 2]);
    }

    #[test]
    fn test_short_part1() {
        let fish = parse(TEST_INPUT);
        assert_eq!(spawn(&fish, 18), 26);
    }

    #[test]
    fn test_long_part1() {
        let fish = parse(TEST_INPUT);
        assert_eq!(spawn(&fish, 256), 26984457539);
    }
    #[test]
    fn test_long2_part1() {
        let fish = parse(TEST_INPUT);
        assert_eq!(spawn2(&fish, 256), 26984457539);
    }
    #[test]
    fn test_long3_part1() {
        let fish = parse(TEST_INPUT);
        assert_eq!(spawn3(&fish, 256), 26984457539);
    }
    #[test]
    fn test_fish_timer_next() {
        assert_eq!(FishTimer::new(10, 4).next(), Some(FishTimer::new(5, 6)));
        assert_eq!(FishTimer::new(10, 2).count(), 2);
    }

    #[test]
    fn test_fish_spawn_count() {
        assert_eq!(FishTimer::new(10, 4).spawn_count(), 2);
        assert_eq!(FishTimer::new(10, 2).spawn_count(), 3);
    }
}
