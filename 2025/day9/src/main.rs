use std::iter::once;

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
    let it = pts.iter();
    let mut verts = it
        .clone()
        .zip(it.skip(1).chain(once(&pts[0])))
        .filter_map(|(a, b)| {
            if a.y == b.y {
                None
            } else {
                Some((a, b.y - a.y))
            }
        })
        .collect::<Vec<_>>();
    verts.sort_by_key(|x| x.0.x);

    verts.iter().for_each(|v| println!("{:?}", v));

    (0..pts.len()).fold(0_i64, |max, i| {
        (i + 1..pts.len()).fold(max, |max, j| {
            let a = pts[i];
            let b = pts[j];
            let v = b - a;
            if point_range_inside(a, b.x, &verts) && point_range_inside(b, a.x, &verts) {
                let area = (v.x.abs() + 1) * (v.y.abs() + 1);
                if area > max {
                    println!("new max: a={a}, b={b}, area={area}");
                    area
                } else {
                    max
                }
            } else {
                max
            }
        })
    }) as usize
}

fn point_range_inside(a: Pos2d<i64>, x: i64, verts: &[(&Pos2d<i64>, i64)]) -> bool {
    let x0 = a.x.min(x);
    let x1 = a.x.max(x);
    let h = verts
        .iter()
        .filter(|v| x0 <= v.0.x && x1 >= v.0.x)
        .collect::<Vec<_>>();
    let v = h
        .iter()
        .filter(|v| {
            let y0 = v.0.y.min(v.0.y + v.1);
            let y1 = v.0.y.max(v.0.y + v.1);
            y0 <= a.y && y1 >= a.y
        })
        .collect::<Vec<_>>();
    if v.len() == 2 {
        println!("a={a},x={x},x0={x0},x1={x1},v={:?}", v);
        true
    } else {
        false
    }
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
