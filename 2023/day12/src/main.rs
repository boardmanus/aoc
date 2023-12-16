use std::{collections::HashMap, time::Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Working,
    Broken,
    Unknown,
}

impl State {
    fn from_char(c: char) -> Self {
        match c {
            '.' => State::Working,
            '#' => State::Broken,
            '?' => State::Unknown,
            _ => panic!("Invalid char"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            State::Working => '.',
            State::Broken => '#',
            State::Unknown => '?',
        }
    }

    fn from_str(s: &str) -> Vec<Self> {
        s.chars().map(|c| State::from_char(c)).collect()
    }

    fn to_string(conditions: &[State]) -> String {
        conditions.iter().map(|c| c.to_char()).collect()
    }
}

type Conditions = Vec<State>;
type GroupsNums = Vec<usize>;
type Groups = Vec<State>;

fn parse(input: &str) -> Vec<(Vec<State>, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split_whitespace();
            let conditions = it
                .next()
                .unwrap()
                .chars()
                .map(|c| State::from_char(c))
                .collect::<Vec<_>>();
            let groups = it
                .next()
                .unwrap()
                .split(',')
                .map(|group| group.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (conditions, groups)
        })
        .collect()
}

fn next_conditions(conditions: &[State], group_num: usize) -> Option<&[State]> {
    let mut matches = 0;
    for (i, condition) in conditions.iter().enumerate() {
        match condition {
            State::Working => {
                if matches == group_num {
                    return Some(&conditions[i + 1..]);
                } else {
                    return None;
                }
            }
            State::Broken => {
                if matches < group_num {
                    matches += 1;
                } else {
                    return None;
                }
            }
            State::Unknown => {
                if matches < group_num {
                    matches += 1;
                } else if matches == group_num {
                    return Some(&conditions[i + 1..]);
                } else {
                    return None;
                }
            }
        }
    }
    if matches == group_num {
        Some(&conditions[conditions.len()..])
    } else {
        None
    }
}

fn permutations2<'a>(
    conditions: &'a [State],
    group_nums: &'a [usize],
    cache: &mut HashMap<(&'a [State], &'a [usize]), usize>,
) -> usize {
    if let Some(p) = cache.get(&(conditions, group_nums)) {
        return *p;
    }

    let mut p = 0;
    for (i, condition) in conditions.iter().enumerate() {
        p += match condition {
            State::Working => 0,
            State::Broken => {
                let new_p =
                    if let Some(next_cond) = next_conditions(&conditions[i..], group_nums[0]) {
                        if group_nums.len() > 1 {
                            p + permutations2(next_cond, &group_nums[1..], cache)
                        } else {
                            if next_cond.iter().any(|c| *c == State::Broken) {
                                p
                            } else {
                                p + 1
                            }
                        }
                    } else {
                        p
                    };
                cache.insert((conditions, group_nums), new_p);
                return new_p;
            }
            State::Unknown => {
                if let Some(next_cond) = next_conditions(&conditions[i..], group_nums[0]) {
                    if group_nums.len() > 1 {
                        permutations2(next_cond, &group_nums[1..], cache)
                    } else {
                        if next_cond.iter().any(|c| *c == State::Broken) {
                            0
                        } else {
                            1
                        }
                    }
                } else {
                    0
                }
            }
        }
    }
    cache.insert((conditions, group_nums), p);
    p
}

fn permutations(conditions: &[State], group_nums: &[usize]) -> usize {
    let mut cache: HashMap<(&[State], &[usize]), usize> = Default::default();
    permutations2(conditions, group_nums, &mut cache)
}

fn unfold_conditions(conditions: &[State]) -> Conditions {
    (0..5).fold(Vec::<State>::new(), |mut acc, i| {
        if i != 0 {
            acc.push(State::Unknown);
        }
        for c in conditions {
            acc.push(*c);
        }
        acc
    })
}

fn unfold_groups(groups: &[usize]) -> GroupsNums {
    groups
        .iter()
        .cycle()
        .take(5 * groups.len())
        .map(|g| *g)
        .collect()
}

fn unfold(input: Vec<(Conditions, GroupsNums)>) -> Vec<(Conditions, GroupsNums)> {
    input
        .iter()
        .map(|p| (unfold_conditions(&p.0), unfold_groups(&p.1)))
        .collect()
}

fn solve_part1(input: &str) -> usize {
    let data = parse(input);
    data.iter()
        .map(|(conditions, groups)| permutations(conditions, groups))
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let data = unfold(parse(input));
    data.iter()
        .enumerate()
        .map(|(i, (conditions, groups))| {
            let start = Instant::now();
            let p = permutations(conditions, groups);
            let duration = start.elapsed();
            println!(
                "{i}: {:?} | {} | {:?}",
                duration,
                State::to_string(conditions),
                groups
            );
            p
        })
        .sum()
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let part1 = solve_part1(INPUT);
    println!("Part1: {part1}");
    let part2 = solve_part2(INPUT);
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
    const TEST_INPUT_2: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 525152);
    }

    #[test]
    fn test_perumtations() {
        assert_eq!(permutations(&State::from_str("???.###"), &vec![1, 1, 3]), 1);
        assert_eq!(
            permutations(&State::from_str(".??..??...?##."), &vec![1, 1, 3]),
            4
        );
        assert_eq!(
            permutations(&State::from_str("?#?#?#?#?#?#?#?"), &vec![1, 3, 1, 6]),
            1
        );
        assert_eq!(
            permutations(&State::from_str("????.#...#..."), &vec![4, 1, 1]),
            1
        );
        assert_eq!(
            permutations(&State::from_str("????.######..#####."), &vec![1, 6, 5]),
            4
        );
        assert_eq!(
            permutations(&State::from_str("?###????????"), &vec![3, 2, 1]),
            10
        );
        assert_eq!(permutations(&State::from_str("?????"), &vec![5]), 1);
        assert_eq!(permutations(&State::from_str("#####"), &vec![5]), 1);
        assert_eq!(permutations(&State::from_str("####."), &vec![5]), 0);
        assert_eq!(permutations(&State::from_str("?????"), &vec![4]), 2);
        assert_eq!(permutations(&State::from_str("#####"), &vec![4]), 0);
        assert_eq!(permutations(&State::from_str("?###?"), &vec![4]), 2);
        assert_eq!(permutations(&State::from_str(".????"), &vec![4]), 1);
        assert_eq!(permutations(&State::from_str("????."), &vec![4]), 1);
        assert_eq!(permutations(&State::from_str("#???."), &vec![4]), 1);
        assert_eq!(permutations(&State::from_str(".???#"), &vec![4]), 1);
        assert_eq!(permutations(&State::from_str(".???#"), &vec![4]), 1);
        assert_eq!(permutations(&State::from_str("?????"), &vec![1]), 5);
        assert_eq!(permutations(&State::from_str("?.?.?"), &vec![1]), 3);
        assert_eq!(permutations(&State::from_str(".?.?."), &vec![1]), 2);
        // test('?.????##??.?#???. [2,3]'
        assert_eq!(
            permutations(&State::from_str("?.????##??.?#???"), &vec![2, 3]),
            2
        );
        assert_eq!(permutations(&State::from_str("?#.#"), &vec![2, 1]), 1);
        assert_eq!(permutations(&State::from_str("#?.#"), &vec![2, 1]), 1);
        assert_eq!(permutations(&State::from_str("?#.#"), &vec![2, 1]), 1);
        assert_eq!(permutations(&State::from_str("#.?#"), &vec![1, 2]), 1);
        assert_eq!(permutations(&State::from_str("#.#?"), &vec![1, 2]), 1);
        assert_eq!(
            permutations(&State::from_str("?.?.?.?.?"), &vec![1, 1, 1, 1, 1]),
            1
        );
        assert_eq!(
            permutations(&State::from_str("?.?.?.?.?"), &vec![1, 1, 1, 1]),
            5
        );

        assert_eq!(permutations(&State::from_str("?#??#?"), &vec![2, 2]), 3);
        assert_eq!(permutations(&State::from_str("????#?"), &vec![2]), 2);
    }

    #[test]
    fn test_expensive_op() {
        permutations(
            &State::from_str("???..???..?.????..???..?.????..???..?.????..???..?.????..???..?."),
            &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        );
    }
    #[test]
    fn test_unfold_groups() {
        assert_eq!(
            unfold_groups(&[1, 2, 3]),
            vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3]
        );
    }

    #[test]
    fn test_unfold_conditions() {
        assert_eq!(
            unfold_conditions(&[State::Working, State::Broken]),
            vec![
                State::Working,
                State::Broken,
                State::Unknown,
                State::Working,
                State::Broken,
                State::Unknown,
                State::Working,
                State::Broken,
                State::Unknown,
                State::Working,
                State::Broken,
                State::Unknown,
                State::Working,
                State::Broken
            ]
        );
    }

    #[test]
    fn test_parse() {
        let conditions = parse(TEST_INPUT);
        assert_eq!(conditions.len(), 6);
        assert_eq!(
            conditions[5].0,
            vec![
                State::Unknown,
                State::Broken,
                State::Broken,
                State::Broken,
                State::Unknown,
                State::Unknown,
                State::Unknown,
                State::Unknown,
                State::Unknown,
                State::Unknown,
                State::Unknown,
                State::Unknown
            ]
        );
        assert_eq!(conditions[5].1, vec![3, 2, 1]);
    }
}
