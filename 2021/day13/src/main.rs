use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Fold {
    X(i64),
    Y(i64),
}

#[derive(Debug)]
struct Data {
    points: HashSet<Point>,
    folds: Vec<Fold>,
}

fn foldx(points: &HashSet<Point>, x: i64) -> HashSet<Point> {
    points.iter().fold(HashSet::<Point>::new(), |mut acc, p| {
        if p.x > x {
            acc.insert(Point {
                x: 2 * x - p.x,
                y: p.y,
            });
        } else {
            acc.insert(*p);
        }
        acc
    })
}
fn foldy(points: &HashSet<Point>, y: i64) -> HashSet<Point> {
    points.iter().fold(HashSet::<Point>::new(), |mut acc, p| {
        if p.y > y {
            acc.insert(Point {
                y: 2 * y - p.y,
                x: p.x,
            });
        } else {
            acc.insert(*p);
        }
        acc
    })
}
fn fold(points: &HashSet<Point>, f: Fold) -> HashSet<Point> {
    match f {
        Fold::X(x) => foldx(points, x),
        Fold::Y(y) => foldy(points, y),
    }
}

fn parse_point(line: &str) -> Point {
    let mut line_it = line.split(',');
    let x = i64::from_str_radix(line_it.next().expect("x"), 10).expect("i64");
    let y = i64::from_str_radix(line_it.next().expect("y"), 10).expect("i64");
    Point { x, y }
}

fn parse_fold(line: &str) -> Fold {
    let mut line_it = line.split('=');
    let t = line_it
        .next()
        .expect("type")
        .chars()
        .last()
        .expect("x or y");
    let v = i64::from_str_radix(line_it.next().expect("value"), 10).expect("i64");
    match t {
        'x' => Fold::X(v),
        'y' => Fold::Y(v),
        _ => panic!("expected x or y"),
    }
}

fn parse(input: &str) -> Data {
    let mut points = HashSet::<Point>::new();
    let mut folds = Vec::<Fold>::new();
    let mut folding = false;
    input.lines().for_each(|line| {
        if line.is_empty() {
            folding = true;
        } else if folding {
            folds.push(parse_fold(line));
        } else {
            points.insert(parse_point(line));
        }
    });

    Data { points, folds }
}

fn solve_part1(input: &str) -> usize {
    let data = parse(input);
    let pts = fold(&data.points, data.folds[0]);
    pts.len()
}

fn solve_part2(input: &str) {
    let data = parse(input);

    let pts = data.folds.iter().fold(data.points, |acc, f| fold(&acc, *f));

    let max_pt = pts.iter().fold(Point { x: 0, y: 0 }, |acc, p| Point {
        x: acc.x.max(p.x),
        y: acc.y.max(p.y),
    });

    for y in 0..=max_pt.y {
        for x in 0..=max_pt.x {
            if pts.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let part1 = solve_part1(INPUT);
    println!("Part1: {part1}");
    println!("Part2:");
    solve_part2(INPUT);
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 17);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 36);
    }

    #[test]
    fn test_fold() {
        let data = parse(TEST_INPUT);
        let pts = fold(&data.points, data.folds[0]);
        assert_eq!(pts.len(), 17);
    }

    #[test]
    fn test_parse() {
        let data = parse(TEST_INPUT);
        assert_eq!(data.points.len(), 18);
        assert_eq!(data.folds.len(), 2);
        assert!(data.points.contains(&Point { x: 9, y: 0 }));
        assert_eq!(data.folds[1], Fold::X(5));
    }
}
