use std::collections::VecDeque;

fn mix_nth(data: &mut VecDeque<(usize, i64)>, nth: usize) {
    let len = data.len() - 1;
    let nth_out = data.iter().position(|k| k.0 == nth).unwrap();
    let d = data[nth_out];
    let mix_n = (d.1 + nth_out as i64).rem_euclid(len as i64);
    data.remove(nth_out);
    data.insert(mix_n as usize, d);

    //let mix_n = (d.1.rem_euclid((len - 1) as i64) as usize + nth_out) % len;
    //data[nth_out] = data[mix_n];
    //data[mix_n] = d;
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
    /*
        #[test]
        fn test_extremes() {
            let data = vec![-21i64, 21, 22, -22, 14, -20, 20, 23];
            let mut out: VecDeque<_> = data.clone().into();
            assert!(relatively_eq(
                mix_nth(&data, &mut out, 0),
                &[-21i64, 21, 22, -22, 14, -20, 20, 23].into()
            ));
            let mut out: VecDeque<_> = data.clone().into();
            assert!(relatively_eq(
                mix_nth(&data, &mut out, 1),
                &[-21i64, 21, 22, -22, 14, -20, 20, 23].into()
            ));
            let mut out: VecDeque<_> = data.clone().into();
            assert!(relatively_eq(
                mix_nth(&data, &mut out, 2),
                &[-21i64, 21, -22, 22, 14, -20, 20, 23].into()
            ));
            let mut out: VecDeque<_> = data.clone().into();
            assert!(relatively_eq(
                mix_nth(&data, &mut out, 3),
                &[-21i64, 21, -22, 22, 14, -20, 20, 23].into()
            ));
            let mut out: VecDeque<_> = data.clone().into();
            assert!(relatively_eq(
                mix_nth(&data, &mut out, 4),
                &[-21i64, 21, 22, -22, 14, -20, 20, 23].into()
            ));
            let mut out: VecDeque<_> = data.clone().into();
            assert!(relatively_eq(
                mix_nth(&data, &mut out, 5),
                &[-21i64, 21, 22, -22, 14, 20, -20, 23].into()
            ));
            let mut out: VecDeque<_> = data.clone().into();
            assert!(relatively_eq(
                mix_nth(&data, &mut out, 6),
                &[-21i64, 21, 22, -22, 14, 20, -20, 23].into()
            ));
            let mut out: VecDeque<_> = data.clone().into();
            assert!(relatively_eq(
                mix_nth(&data, &mut out, 7),
                &[-21i64, 21, 23, 22, -22, 14, -20, 20].into()
            ));
        }
        #[test]
        fn test_mix_nth() {
            let data: Vec<i64> = [1, 2, -3, 3, -2, 0, 4].into();

            let mut outdata: VecDeque<i64> = [2, 1, -3, 3, -2, 0, 4].into();
            let mixed: VecDeque<i64> = [2, 1, -2, -3, 3, 0, 4].into();
            assert!(relatively_eq(mix_nth(&data, &mut outdata, 4), &mixed));

            let mut outdata: VecDeque<i64> = [2, 1, -3, 3, -2, 0, 4].into();
            let mixed: VecDeque<i64> = [1, -3, 2, 3, -2, 0, 4].into();
            assert!(relatively_eq(mix_nth(&data, &mut outdata, 1), &mixed));

            let mut outdata: VecDeque<i64> = [1, -3, 2, 3, -2, 0, 4].into();
            let mixed: VecDeque<i64> = [1, 2, 3, -2, -3, 0, 4].into();
            assert!(relatively_eq(mix_nth(&data, &mut outdata, 2), &mixed));

            let mut outdata: VecDeque<i64> = [1, 2, -2, -3, 0, 3, 4].into();
            let mixed: VecDeque<i64> = [1, 2, -3, 0, 3, 4, -2].into();
            assert!(relatively_eq(mix_nth(&data, &mut outdata, 4), &mixed));

            let mut outdata: VecDeque<i64> = [1, 2, -3, 0, 3, 4, -2].into();
            let mixed: VecDeque<i64> = [1, 2, -3, 0, 3, 4, -2].into();
            assert!(relatively_eq(mix_nth(&data, &mut outdata, 5), &mixed));

            let mut outdata: VecDeque<i64> = [1, 2, -3, 0, 3, 4, -2].into();
            let mixed: VecDeque<i64> = [1, 2, -3, 4, 0, 3, -2].into();
            assert!(relatively_eq(mix_nth(&data, &mut outdata, 6), &mixed));
        }
    */
    #[test]
    fn test_mix() {
        let mixed_data: VecDeque<i64> = [1, 2, -3, 4, 0, 3, -2].into();
        let mut data = parse(include_str!("test.txt"));
        assert!(relatively_eq(&mix(&mut data), &mixed_data));
    }

    #[test]
    fn test_mods() {
        assert_eq!(-7 % 5, -2);
        assert_eq!((-7 % 5 + 5) % 5, 3);
        assert_eq!(-(7 % 5), -2);
    }
}
