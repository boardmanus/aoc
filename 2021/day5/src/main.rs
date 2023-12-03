use regex::Regex;
use std::{collections::HashMap, fmt::Display, num::ParseIntError, str::FromStr};

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

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: u64,
    y: u64,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "P({}, {})", self.x, self.y)
    }
}

impl Point {
    fn new(x: u64, y: u64) -> Point {
        Point { x, y }
    }
}
#[derive(Copy, Clone)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn new(a: Point, b: Point) -> Line {
        Line { a, b }
    }

    fn iter(&self) -> LineIter {
        LineIter {
            line: self,
            point: Some(self.a),
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "L({}, {})", self.a, self.b)
    }
}

impl FromStr for Line {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$")?;
        let cap = re.captures(line).ok_or(ParseError::NoMatch)?;
        let a = Point::new(u64::from_str(&cap[1])?, u64::from_str(&cap[2])?);
        let b = Point::new(u64::from_str(&cap[3])?, u64::from_str(&cap[4])?);
        Ok(Line::new(a, b))
    }
}

struct LineIter<'a> {
    line: &'a Line,
    point: Option<Point>,
}

impl<'a> Iterator for LineIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(point) = self.point {
            let x = if point.x != self.line.b.x {
                if point.x < self.line.b.x {
                    point.x + 1
                } else {
                    point.x - 1
                }
            } else {
                point.x
            };
            let y = if point.y != self.line.b.y {
                if point.y < self.line.b.y {
                    point.y + 1
                } else {
                    point.y - 1
                }
            } else {
                point.y
            };
            self.point = if x == point.x && y == point.y {
                None
            } else {
                Some(Point::new(x, y))
            };
            Some(point)
        } else {
            None
        }
    }
}

fn parse(input: &str) -> Result<Vec<Line>, ParseError> {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| Line::from_str(line))
        .collect()
}

fn hv_lines(lines: &[Line]) -> Vec<Line> {
    lines
        .iter()
        .filter(|line| line.a.x == line.b.x || line.a.y == line.b.y)
        .map(|line| *line)
        .collect()
}

fn count(lines: &[Line]) -> usize {
    let mut pmap: HashMap<Point, u64> = Default::default();
    for line in lines {
        line.iter().for_each(|point| {
            pmap.entry(point).and_modify(|v| *v += 1).or_insert(1);
        });
    }
    pmap.iter().filter(|v| *v.1 > 1).count()
}

fn solve_part1(lines: &[Line]) -> usize {
    count(&hv_lines(lines))
}

fn solve_part2(lines: &[Line]) -> usize {
    count(lines)
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let all_lines = parse(INPUT).expect("Good input");
    let part1 = solve_part1(&all_lines);
    println!("Part1: {part1}");
    let part2 = solve_part2(&all_lines);
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let lines = parse(TEST_INPUT).expect("Good input");
        lines.iter().for_each(|line| println!("{line}"));
        assert_eq!(solve_part1(&lines), 5);
    }

    #[test]
    fn test_part2() {
        let lines = parse(TEST_INPUT).expect("Good input");
        assert_eq!(solve_part2(&lines), 12);
    }
}
