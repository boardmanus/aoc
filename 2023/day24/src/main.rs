use std::{num::ParseFloatError, str::FromStr};

#[derive(Debug)]
enum ParseErr {
    NotEnoughElements,
    F64(ParseFloatError),
}

impl From<ParseFloatError> for ParseErr {
    fn from(e: ParseFloatError) -> Self {
        ParseErr::F64(e)
    }
}

fn tuple_from_str(s: &str) -> Result<(f64, f64, f64), ParseErr> {
    s.split(", ")
        .map(|s| s.trim().parse::<f64>())
        .collect::<Result<Vec<_>, _>>()
        .map(|v| {
            Ok((
                *v.get(0).ok_or(ParseErr::NotEnoughElements)?,
                *v.get(1).ok_or(ParseErr::NotEnoughElements)?,
                *v.get(2).ok_or(ParseErr::NotEnoughElements)?,
            ))
        })?
}

#[derive(Debug, PartialEq)]
struct Pos {
    x: f64,
    y: f64,
    z: f64,
}

impl Pos {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Pos { x, y, z }
    }

    fn from_tuple(t: (f64, f64, f64)) -> Self {
        Pos::new(t.0, t.1, t.2)
    }
}

impl FromStr for Pos {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pos::from_tuple(tuple_from_str(s)?))
    }
}
#[derive(Debug, PartialEq)]

struct Vel {
    x: f64,
    y: f64,
    z: f64,
}

impl Vel {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vel { x, y, z }
    }

    fn from_tuple(t: (f64, f64, f64)) -> Self {
        Vel::new(t.0, t.1, t.2)
    }
}

impl FromStr for Vel {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Vel::from_tuple(tuple_from_str(s)?))
    }
}

#[derive(Debug, PartialEq)]
struct Hail {
    pos: Pos,
    vel: Vel,
}

impl Hail {
    fn new(pos: Pos, vel: Vel) -> Self {
        Hail { pos, vel }
    }

    fn path_intersect2d(&self, other: &Hail) -> Option<Pos> {
        let m0 = self.vel.y / self.vel.x;
        let m1 = other.vel.y / other.vel.x;
        let c0 = self.pos.y - m0 * self.pos.x;
        let c1 = other.pos.y - m1 * other.pos.x;
        let denom = m0 - m1;
        if denom.abs() < f64::EPSILON {
            return None;
        }

        let x = (c1 - c0) / denom;
        let y = m0 * x + c0;

        let n0 = ((x - self.pos.x) / self.vel.x, (y - self.pos.y) / self.vel.y);

        let n1 = (
            (x - other.pos.x) / other.vel.x,
            (y - other.pos.y) / other.vel.y,
        );

        if n0.0 < 0.0 || n0.1 < 0.0 || n1.0 < 0.0 || n1.1 < 0.0 {
            return None;
        }

        Some(Pos::new(x, y, 0.0))
    }
}
impl FromStr for Hail {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" @ ");
        let pos = split
            .next()
            .ok_or(ParseErr::NotEnoughElements)?
            .parse::<Pos>()?;
        let vel = split
            .next()
            .ok_or(ParseErr::NotEnoughElements)?
            .parse::<Vel>()?;
        Ok(Hail::new(pos, vel))
    }
}

struct Area {
    min: Pos,
    max: Pos,
}

impl Area {
    fn new(min: Pos, max: Pos) -> Self {
        Area { min, max }
    }

    fn contains(&self, pos: &Pos) -> bool {
        pos.x >= self.min.x && pos.x <= self.max.x && pos.y >= self.min.y && pos.y <= self.max.y
    }
}

struct Storm {
    hail: Vec<Hail>,
}

impl FromStr for Storm {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Storm {
            hail: s
                .lines()
                .map(|l| l.parse::<Hail>())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn count_inbound_intersections(storm: &Storm, bounds: &Area) -> usize {
    storm.hail[0..storm.hail.len() - 1]
        .iter()
        .enumerate()
        .fold(0, |intersections, (i, hail)| {
            let intersecting_paths = storm.hail[i + 1..]
                .iter()
                .filter(|hail2| {
                    hail.path_intersect2d(hail2)
                        .map(|p| bounds.contains(&p))
                        .unwrap_or(false)
                })
                .collect::<Vec<_>>();

            println!("Intersecting paths @ {:?}: {:?}", hail, intersecting_paths);

            intersections + intersecting_paths.len()
        })
}

fn solve_part1(input: &str) -> usize {
    count_inbound_intersections(
        &Storm::from_str(input).unwrap(),
        &Area::new(
            Pos::new(200000000000000.0, 200000000000000.0, 0.0),
            Pos::new(400000000000000.0, 400000000000000.0, 0.0),
        ),
    )
}

fn solve_part2(input: &str) -> usize {
    let storm = Storm::from_str(input).unwrap();
    let start_pos = Pos::new(200000000000000.0, 200000000000000.0, 0.0);
    (start_pos.x + start_pos.y + start_pos.z) as usize
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
        assert_eq!(solve_part1(TEST_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 47);
    }

    #[test]
    fn test_parse_pos() {
        let pos = Pos::from_str("1, 2, 3").unwrap();
        assert_eq!(pos, Pos::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_parse() {
        let storm = Storm::from_str(TEST_INPUT).unwrap();
        assert_eq!(storm.hail.len(), 5);
        assert_eq!(
            storm.hail[2],
            Hail::new(Pos::new(20.0, 25.0, 34.0), Vel::new(-2.0, -2.0, -4.0))
        );
    }

    fn approx_eq_2d(p0: &Pos, p1: &Pos) -> bool {
        format!("{:.3}", p0.x) == format!("{:.3}", p1.x)
            && format!("{:.3}", p0.y) == format!("{:.3}", p1.y)
    }

    #[test]
    fn test_path_intersect() {
        // Hailstone A: 19, 13, 30 @ -2, 1, -2
        // Hailstone B: 18, 19, 22 @ -1, -1, -2
        // Hailstones' paths will cross inside the test area (at x=14.333, y=15.333).
        let hail0 = Hail::from_str("19, 13, 30 @ -2, 1, -2").unwrap();
        let hail1 = Hail::from_str("18, 19, 22 @ -1, -1, -2").unwrap();
        let p = hail0.path_intersect2d(&hail1).unwrap();
        assert!(approx_eq_2d(&p, &Pos::new(14.333, 15.333, 0.0)));

        // Hailstone A: 19, 13, 30 @ -2, 1, -2
        // Hailstone B: 20, 19, 15 @ 1, -5, -3
        // Hailstones' paths crossed in the past for hailstone A.
        let hail0 = Hail::from_str("19, 13, 30 @ -2, 1, -2").unwrap();
        let hail1 = Hail::from_str("20, 19, 15 @ 1, -5, -3").unwrap();
        assert_eq!(hail0.path_intersect2d(&hail1), None);

        // Hailstone A: 18, 19, 22 @ -1, -1, -2
        // Hailstone B: 12, 31, 28 @ -1, -2, -1
        // Hailstones' paths will cross outside the test area (at x=-6, y=-5).
        let hail0 = Hail::from_str("18, 19, 22 @ -1, -1, -2").unwrap();
        let hail1 = Hail::from_str("12, 31, 28 @ -1, -2, -1").unwrap();
        assert_eq!(
            hail0.path_intersect2d(&hail1),
            Some(Pos::new(-6.0, -5.0, 0.0))
        );

        let hail0 = Hail::from_str("18, 19, 22 @ -1, -1, -2").unwrap();
        let hail1 = Hail::from_str("20, 25, 34 @ -2, -2, -4").unwrap();
        assert_eq!(hail0.path_intersect2d(&hail1), None);
    }

    #[test]
    fn test_area_contains() {
        let area = Area::new(Pos::new(7.0, 7.0, 0.0), Pos::new(27.0, 27.0, 0.0));
        assert!(area.contains(&Pos::new(14.333, 15.333, 0.0)));
        assert!(area.contains(&Pos::new(7.0, 7.0, 0.0)));
        assert!(area.contains(&Pos::new(27.0, 27.0, 0.0)));
        assert!(!area.contains(&Pos::new(6.999, 16.0, 0.0)));
        assert!(!area.contains(&Pos::new(16.0, 27.001, 0.0)));
    }
}
