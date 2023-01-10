use std::collections::VecDeque;

fn mix_nth(data: &mut VecDeque<(usize, i64)>, nth: usize) {
    let len = data.len() - 1;
    let nth_out = data.iter().position(|k| k.0 == nth).unwrap();
    let d = data[nth_out];
    let mix_n = (d.1 + nth_out as i64).rem_euclid(len as i64);
    data.remove(nth_out);
    data.insert(mix_n as usize, d);
}

fn mix(it: usize, key: i64, data: &Vec<i64>) -> VecDeque<i64> {
    let len = data.len();
    let mut outdata = data.iter().enumerate().map(|x| (x.0, x.1 * key)).collect();
    for _ in 0..it {
        for i in 0..len {
            mix_nth(&mut outdata, i);
        }
    }
    outdata.iter().map(|x| x.1).collect()
}

fn grove_coords(data: &VecDeque<i64>) -> i64 {
    let pos = data.iter().position(|p| *p == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|a| data[(a + pos) % data.len()])
        .sum()
}

fn parse(input: &str) -> Vec<i64> {
    input
        .split('\n')
        .flat_map(|s| s.parse())
        .collect::<Vec<i64>>()
}

fn solve_part1(input: &str) -> String {
    grove_coords(&mix(1, 1, &parse(input))).to_string()
}

fn solve_part2(input: &str) -> String {
    grove_coords(&mix(10, 811589153, &parse(input))).to_string()
}

fn main() {
    let res = solve_part1(include_str!("input.txt"));
    println!("Part1: {res}");
    let res = solve_part2(include_str!("input.txt"));
    println!("Part2: {res}");
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(include_str!("test.txt")), 3.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(include_str!("test.txt")), 0.to_string());
    }

    fn relatively_eq(a: &VecDeque<i64>, b: &VecDeque<i64>) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let s = a[0];
        let mut i_b = b.iter().position(|p| *p == s).unwrap();
        for i_a in 0..a.len() {
            if a[i_a] != b[i_b] {
                println!("{:?} != {:?}", a, b);
                return false;
            }
            i_b = (i_b + 1) % a.len();
        }
        true
    }

    #[test]
    fn test_mix() {
        let mixed_data: VecDeque<i64> = [1, 2, -3, 4, 0, 3, -2].into();
        let mut data = parse(include_str!("test.txt"));
        assert!(relatively_eq(&mix(1, 1, &mut data), &mixed_data));
    }

    #[test]
    fn test_mods() {
        assert_eq!(-7 % 5, -2);
        assert_eq!((-7 % 5 + 5) % 5, 3);
        assert_eq!(-(7 % 5), -2);
    }

    #[test]
    fn test_dups() {
        let data = parse(include_str!("input.txt"));
        let dups = data
            .iter()
            .fold(HashMap::<i64, usize>::default(), |mut m, k| {
                if let Some(cnt) = m.get_mut(k) {
                    *cnt += 1;
                } else {
                    m.insert(*k, 1);
                }
                m
            })
            .into_iter()
            .filter(|(k, v)| *v > 1)
            .collect::<Vec<_>>();
        println!("{dups:?}");
        println!("Num dups = {}", dups.iter().fold(0, |s, (v, n)| s + n - 1));
    }
}
