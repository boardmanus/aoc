use std::{io::WriterPanicked, iter::once};

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

pub fn min_max_coords(pts: &[Pos2d<i64>]) -> (Pos2d<i64>, Pos2d<i64>) {
    let min_x = pts.iter().map(|p| p.x).min().unwrap();
    let max_x = pts.iter().map(|p| p.x).max().unwrap();
    let min_y = pts.iter().map(|p| p.y).min().unwrap();
    let max_y = pts.iter().map(|p| p.y).max().unwrap();
    (Pos2d::new(min_x, min_y), Pos2d::new(max_x, max_y))
}

type Pos = Pos2d<i64>;
type Edge = (Pos, Pos, usize);

struct Edges<'a> {
    edges: &'a [Edge],
    h_bucket: Bucket<'a>,
    v_bucket: Bucket<'a>,
}

impl<'a> Edges<'a> {
    pub fn new(edges: &'a [Edge], max_coord: Pos) -> Self {
        let h_bucket = Bucket::new(edges, max_coord, |p| p.x as usize);
        let v_bucket = Bucket::new(edges, max_coord, |p| p.y as usize);
        Edges {
            edges,
            h_bucket,
            v_bucket,
        }
    }
}

struct Bucket<'a> {
    value_func: fn(&Pos) -> usize,
    bucket_size: usize,
    buckets: [Vec<usize>; 100],
    _p: std::marker::PhantomData<&'a ()>,
}

impl<'a> Bucket<'a> {
    pub fn new(edges: &'a [Edge], max_coord: Pos, value_func: fn(&Pos) -> usize) -> Self {
        let mut buckets = [const { Vec::<usize>::new() }; 100];
        let bucket_size = (value_func(&max_coord) as f64 / 100.0).ceil() as usize;
        edges.iter().enumerate().for_each(|(i, e)| {
            let a_coord = value_func(&e.0);
            let b_coord = value_func(&e.1);
            let a_bucket = (a_coord / bucket_size).min(100);
            let b_bucket = (b_coord / bucket_size).min(100);
            buckets[a_bucket].push(i);
            if a_bucket != b_bucket {
                buckets[b_bucket].push(i);
            }
        });
        buckets.iter_mut().for_each(|b| {
            b.sort_by(|&a, &b| value_func(&edges[a].0).cmp(&value_func(&edges[b].0)))
        });
        Bucket {
            value_func,
            bucket_size,
            buckets,
            _p: std::marker::PhantomData,
        }
    }

    fn edges_in_range(&'a self, pos: &Pos) -> &'a [usize] {
        let coord = (self.value_func)(pos) as usize;
        let bucket = (coord / self.bucket_size).min(100);
        &self.buckets[bucket]
    }
}

pub fn part2(input: &str) -> usize {
    let pts = parse_input(input);

    let (min, max) = min_max_coords(&pts);
    println!("min={min}, max={max}");

    let edges = pts
        .iter()
        .clone()
        .zip(pts.iter().skip(1).chain(once(&pts[0])))
        .enumerate()
        .fold(vec![], |mut edges, (i, (a, b))| {
            edges.push((*a, *b, i));
            edges
        });

    let edges = Edges::new(&edges, max);

    (0..pts.len()).fold(0_i64, |max, i| {
        (i + 1..pts.len()).fold(max, |max, j| {
            let a = pts[i];
            let b = pts[j];
            let v = b - a;
            if area_inside(&a, &b, &edges) {
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

fn v_intersects(p: &Pos, edge: &Edge) -> bool {
    let y0 = edge.0.y.min(edge.1.y);
    let y1 = edge.0.y.max(edge.1.y);
    edge.0.x >= p.x && y0 <= p.y && y1 >= p.y
}

fn h_intersects(p: &Pos, edge: &Edge) -> bool {
    let x0 = edge.0.x.min(edge.1.x);
    let x1 = edge.0.x.max(edge.1.x);
    edge.0.y >= p.y && x0 <= p.x && x1 >= p.x
}

fn point_inside(a: &Pos, verts: &[Edge]) -> bool {
    let on_line = verts.iter().any(|v| {
        let y0 = v.0.y.min(v.1.y);
        let y1 = v.0.y.max(v.1.y);
        y0 <= a.y && y1 >= a.y && a.x == v.0.x
    });
    if on_line {
        println!("point on line: a={a}");
        return true;
    }

    let v = verts
        .iter()
        .filter(|v| {
            let y0 = v.0.y.min(v.1.y);
            let y1 = v.0.y.max(v.1.y);
            v.0.x >= a.x && y0 <= a.y && y1 >= a.y
        })
        .collect::<Vec<_>>();
    let winding = v.iter().fold(0, |winding, x| {
        let v = x.1 - x.0;
        winding + v.y.signum()
    });
    println!("a={a},winding={winding},v={:?}", v);
    winding != 0
}

fn filter_verts(a: &Pos, b: &Pos, verts: &[Edge]) -> Vec<Edge> {
    let x0 = a.x.min(b.x);
    let x1 = a.x.max(b.x);
    verts
        .iter()
        .filter(|v| {
            let y0 = v.0.y.min(v.1.y);
            let y1 = v.0.y.max(v.1.y);
            y0 <= a.y && y1 >= a.y && x0 <= v.0.x && x1 >= v.0.x
        })
        .cloned()
        .collect::<Vec<_>>()
}

fn filter_horz(a: &Pos, b: &Pos, horz: &[Edge]) -> Vec<Edge> {
    let y0 = a.y.min(b.y);
    let y1 = a.y.max(b.y);
    horz.iter()
        .filter(|v| {
            let x0 = v.0.x.min(v.1.x);
            let x1 = v.0.x.max(v.1.x);
            x0 <= a.x && x1 >= a.x && y0 <= v.0.y && y1 >= v.0.y
        })
        .cloned()
        .collect::<Vec<_>>()
}

fn area_inside(a: &Pos, b: &Pos, edges: &Edges) -> bool {
    println!("checking area a={a}, b={b}");
    let v = filter_verts(a, b, verts);
    if v.len() > 2 && v.len() != 0 {
        // if more than 2 vertical edges intersect, there must be a gap
        println!("area not inside: {a}-{b}, v={:?}", v);
        return false;
    }
    let h = filter_horz(a, b, horz);
    if h.len() > 2 && h.len() != 0 {
        // if more than 2 horizontal edges intersect, there must be a gap
        println!("area not inside: {a}-{b}, h={:?}", h);
        return false;
    }

    if !point_inside(&Pos2d::new(a.x, b.y), &v) {
        // If the opposite corner is not inside, the area is not fully enclosed
        println!("opposite corner not inside: ({},{})", a.x, b.y);
        return false;
    }
    println!("checked corner: ({},{})", a.x, b.y);

    if !point_inside(&Pos2d::new(b.x, a.y), &v) {
        // If the opposite corner is not inside, the area is not fully enclosed
        println!("opposite corner not inside: ({},{})", b.x, a.y);
        return false;
    }
    println!("checked corner: ({},{})", b.x, a.y);

    println!("area inside a={a}, b={b}: v={:?}, h={:?}", v, h);
    true
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
    /*
    #[test]
    fn test_area_inside() {
        let pts = parse_input(TEST_INPUT);
        let it = pts.iter();

        let it = pts.iter();
        let mut verts: Vec<(Pos2d<i64>, i64)> = vec![];
        let mut horz: Vec<(Pos2d<i64>, i64)> = vec![];
        it.clone()
            .zip(it.skip(1).chain(once(&pts[0])))
            .for_each(|(a, b)| {
                if a.y != b.y {
                    verts.push((*a, b.y - a.y))
                }
                if a.x != b.x {
                    horz.push((*a, b.x - a.x))
                }
            });

        verts.sort_by_key(|x| x.0.x);
        horz.sort_by_key(|x| x.0.y);

        let a = Pos2d::new(11, 1);
        let b = Pos2d::new(2, 5);
        let v = filter_verts(a, b, &verts);
        let c = Pos2d::new(b.x, a.y);
        let inside = point_inside(c, &verts);
        assert!(!inside);
    }
    */
}
