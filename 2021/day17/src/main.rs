use std::{fmt::Display, num::ParseIntError, str::FromStr};

use regex::Regex;

#[derive(Debug)]
enum ParseError {
    Regex(regex::Error),
    Int(ParseIntError),
    NoMatch,
}

impl From<regex::Error> for ParseError {
    fn from(err: regex::Error) -> ParseError {
        ParseError::Regex(err)
    }
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> ParseError {
        ParseError::Int(err)
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "P({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone)]
struct Rect {
    tl: Point,
    br: Point,
}

impl Rect {
    fn new(tl: Point, br: Point) -> Rect {
        assert!(tl.y >= br.y && tl.x <= br.x);
        Rect { tl, br }
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "R({}, {})", self.tl, self.br)
    }
}

impl FromStr for Rect {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)$")?;
        let cap = re.captures(line).ok_or(ParseError::NoMatch)?;
        let x1 = i64::from_str(&cap[1])?;
        let x2 = i64::from_str(&cap[2])?;
        let y1 = i64::from_str(&cap[3])?;
        let y2 = i64::from_str(&cap[4])?;
        let tl = Point::new(x1.min(x2), y1.max(y2));
        let br = Point::new(x1.max(x2), y1.min(y2));
        Ok(Rect::new(tl, br))
    }
}

fn max_y_height(rect: &Rect) -> i64 {
    let yv = max_y_start(rect);
    (yv + 1) * yv / 2
}

fn max_y_start(rect: &Rect) -> i64 {
    let yv = if rect.tl.y < rect.br.y.abs() {
        rect.br.y.abs() - 1
    } else {
        rect.tl.y
    };
    yv
}

fn min_x_start(rect: &Rect) -> i64 {
    let xd = rect.tl.x;
    // tl = (x + 1)x/2
    // x^2 + x - 2*tl = 0
    // x = (-a +- sqrt(b*b - 4ac))/2
    let d = ((1 + 8 * xd) as f64).sqrt();
    let x = (d - 1.0) / 2.0;
    x as i64
}

fn max_x_start(rect: &Rect) -> i64 {
    rect.br.x
}

fn parse_input(input: &str) -> Rect {
    Rect::from_str(input).expect("good input")
}

fn solve_part1(input: &str) -> i64 {
    let r = parse_input(input);
    max_y_height(&r)
}

fn solve_part2(input: &str) -> usize {
    let r = parse_input(input);
    let minx = (1.0 + 8.0 * (r.tl.x as f32)).sqrt().ceil() as i64;
    let maxx = (1.0 + 8.0 * (r.br.x as f32)).sqrt().ceil() as i64;
    let miny = r.tl.y as f32;
    /*
        for x in minx..maxx {
            let maxy = ys.iter().max().unwrap().abs() as usize;
        }
        let yv = if miny > maxy { miny - 1 } else { maxy };
        (yv + 1) * yv / 2
    */
    0
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let part1 = solve_part1(INPUT);
    println!("Part1: {part1}");
    let part2 = solve_part2(INPUT);
    println!("Part2: {part2}");
    solve_part2(INPUT);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let res = solve_part1("target area: x=20..30, y=-10..-5");
        assert_eq!(res, 45);
    }

    #[test]
    fn test_part2() {
        let res = solve_part1("target area: x=20..30, y=-10..-5");
        assert_eq!(res, 112);
    }
}
