use std::{fmt, fmt::Display, str::FromStr};

use crate::aoc::Aoc;
use itertools::Itertools;
use nom::{
    self,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list0,
    sequence::{delimited, pair, preceded, separated_pair},
    IResult,
};
use rangemap::RangeSet;

type Pos = (i32, i32);

#[derive(PartialEq, Debug)]
struct Sensor {
    s_pos: Pos,
    b_pos: Pos,
}

impl Sensor {
    fn parse(s: &str) -> Sensor {
        parse_sensor(s).unwrap().1
    }

    fn man_dist(&self) -> u32 {
        self.s_pos.0.abs_diff(self.b_pos.0) + self.s_pos.1.abs_diff(self.b_pos.1)
    }

    fn covered_range(&self, y: i32) -> Option<core::ops::Range<i32>> {
        let md = self.man_dist();
        let dy = self.s_pos.1.abs_diff(y);
        if md < dy {
            None
        } else {
            let x = md.abs_diff(dy) as i32;
            Some((self.s_pos.0 - x)..(self.s_pos.0 + x + 1))
        }
    }

    fn in_range(&self, pos: &Pos) -> bool {
        let dx = self.s_pos.0.abs_diff(pos.0);
        let dy = self.s_pos.1.abs_diff(pos.1);
        self.man_dist() >= dx + dy
    }

    fn all_in_range(&self, amin: &Pos, amax: &Pos) -> bool {
        self.in_range(amin)
            && self.in_range(amax)
            && self.in_range(&(amin.0, amax.1))
            && self.in_range(&(amax.0, amin.1))
    }
}

fn parse_sensor(s: &str) -> IResult<&str, Sensor> {
    let (s2, s_pos) = pair(
        delimited(tag("Sensor at x="), nom::character::complete::i32, tag(",")),
        delimited(tag(" y="), nom::character::complete::i32, tag(":")),
    )(s)?;
    let (s3, b_pos) = pair(
        delimited(
            tag(" closest beacon is at x="),
            nom::character::complete::i32,
            tag(","),
        ),
        preceded(tag(" y="), nom::character::complete::i32),
    )(s2)?;
    Ok((s3, Sensor { s_pos, b_pos }))
}

fn covered_ranges(sensors: &[Sensor], y: i32) -> RangeSet<i32> {
    let mut ranges = RangeSet::new();
    sensors.iter().for_each(|s| {
        if let Some(r) = s.covered_range(y) {
            ranges.insert(r);
        }
    });
    ranges
}
fn covered_size(sensors: &[Sensor], y: i32) -> usize {
    let ranges = covered_ranges(sensors, y);
    let beacons = sensors
        .iter()
        .filter(|s| s.b_pos.1 == y)
        .map(|s| s.b_pos)
        .unique()
        .collect_vec();
    print_range(-4..27, &ranges, &beacons);
    ranges.iter().map(|r| r.len()).sum::<usize>() - beacons.len()
}

fn min_max_becaon_loc(sensors: &[Sensor]) -> (Pos, Pos) {
    let mut min = (i32::MAX, i32::MAX);
    let mut max = (i32::MIN, i32::MIN);
    sensors.iter().for_each(|s| {
        let dist = s.man_dist() as i32;
        min.0 = min.0.min(s.s_pos.0 - dist);
        max.0 = max.0.max(s.s_pos.0 + dist);
        min.1 = min.1.min(s.s_pos.1 - dist);
        max.1 = max.1.max(s.s_pos.1 + dist);
    });
    println!("sensor min={min:?}, max={max:?}");
    (min, max)
}
/*
fn lines_from_pt(pos: &Pos, dist: i32) -> impl Iterator<Item = (i32, i32)> + '_ {
    [dist, -dist]
        .iter()
        .map(move |dx| {
            [1, -1]
                .iter()
                .map(move |m| (m * 1, pos.1 - m * (pos.0 + dx)))
        })
        .flatten()
}
fn check_points(a: &Sensor, b: &Sensor) -> Vec<Pos> {
    let a_dist = a.man_dist() as i32;
    let b_dist = b.man_dist() as i32;
    let a_lines = [a_dist, -a_dist]
        .iter()
        .map(|dx| {
            let t = [1, -1]
                .iter()
                .map(|m| (*m, a.s_pos.1 - m * (a.s_pos.0 + dx)));
            t
        })
        .flatten();
    let b_lines = [(b_dist, 0), (-b_dist, 0), (0, b_dist), (0, -b_dist)];

    let pos = (a.s_pos.0 + a_dist, a.s_pos.1);
}
fn perm(sensors: &[Sensor]) {
    // doesn't get rid of dups
    let all_ab = sensors.iter().permutations(2);
    let all_pts = p.map(|ab| check_points(ab[0], ab[1]));
}
*/

fn fully_contained(sensors: &[Sensor], amin: &(i32, i32), amax: &(i32, i32)) -> bool {
    sensors.iter().any(|s| s.all_in_range(amin, amax))
}

fn tuning_freq2(
    depth: i32,
    sensors: &[Sensor],
    amin: &(i32, i32),
    amax: &(i32, i32),
) -> Option<u64> {
    let dx = amax.0 - amin.0;
    let dy = amax.1 - amin.1;

    if fully_contained(sensors, amin, amax) {
        println!("tuning-freq: INSIDE depth={depth}, min={amin:?}, max={amax:?}, dp=({dx}, {dy})");
        None
    } else if dx <= 1 && dy <= 1 {
        println!("tuning-freq: FOUND! depth={depth}, min={amin:?}, max={amax:?}, dp=({dx}, {dy})");
        Some((amin.0 as u64) * 4000000u64 + (amin.1 as u64))
    } else {
        let halfx = 1.max(dx / 2);
        let halfy = 1.max(dy / 2);
        println!(
            "tuning-freq: SPLIT depth={depth}, min={amin:?}, max={amax:?}, dp=({dx}, {dy}), half=({halfx}, {halfy})"
        );
        let f = tuning_freq2(
            depth + 1,
            sensors,
            &(amin.0, amin.1),
            &(amin.0 + halfx, amin.1 + halfy),
        )
        .or_else(|| {
            tuning_freq2(
                depth + 1,
                sensors,
                &(amin.0 + halfx, amin.1),
                &(amax.0, amin.1 + halfy),
            )
        })
        .or_else(|| {
            tuning_freq2(
                depth + 1,
                sensors,
                &(amin.0, amin.1 + halfy),
                &(amin.0 + halfx, amax.1),
            )
        })
        .or_else(|| {
            tuning_freq2(
                depth + 1,
                sensors,
                &(amin.0, amin.1 + halfy),
                &(amax.0, amax.1),
            )
        });
        println!("tuning-freq: finished split {f:?}");
        f
    }
}

fn tuning_freq(sensors: &[Sensor], amax: &(i32, i32)) -> u64 {
    let (mut min, mut max) = min_max_becaon_loc(sensors);
    min.0 = min.0.max(0);
    min.1 = min.1.max(0);
    max.0 = max.0.min(amax.0);
    max.1 = max.1.min(amax.1);
    for y in min.1..max.1 {
        let ranges = covered_ranges(sensors, y);
        let r = ranges.gaps(&(min.0..max.0)).next();
        if let Some(p) = r {
            if p.len() > 0 {
                return (p.start as u64) * 4000000u64 + (y as u64);
            }
        }
        if y % 1000 == 0 {
            println!("y={y}");
        }
    }
    0
}

fn print_range<'a>(row: std::ops::Range<i32>, rs: &RangeSet<i32>, beacons: &[Pos]) {
    for x in row {
        let c = if let Some(_) = beacons.iter().find(|b| b.0 == x) {
            'B'
        } else if rs.contains(&x) {
            '#'
        } else {
            '.'
        };
        print!("{c}");
    }
    println!("");
}

pub struct Day15_1;
impl Aoc for Day15_1 {
    fn day(&self) -> u32 {
        15
    }
    fn puzzle_name(&self) -> &str {
        "Beacon Exclusion Zone"
    }
    fn solve(&self, lines: &[String]) -> String {
        let sensors = lines.iter().map(|line| Sensor::parse(line)).collect_vec();
        covered_size(&sensors, 2000000).to_string()
    }
}

pub struct Day15_2;
impl Aoc for Day15_2 {
    fn day(&self) -> u32 {
        15
    }
    fn puzzle_name(&self) -> &str {
        "Beacon Exclusion Zone 2"
    }
    fn solve(&self, lines: &[String]) -> String {
        let sensors = lines.iter().map(|line| Sensor::parse(line)).collect_vec();
        tuning_freq(&sensors, &(4000000, 4000000)).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::as_vstrings;

    use super::*;

    const INPUT: [&str; 14] = [
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
        "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
        "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
        "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
        "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
        "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
        "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
        "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
        "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
        "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
        "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
        "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
        "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
        "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
    ];

    #[test]
    fn test_soln() {
        let lines = as_vstrings(&INPUT[0..]);
        let sensors = lines.iter().map(|line| Sensor::parse(line)).collect_vec();
        assert_eq!(covered_size(&sensors, 10), 26);
    }

    #[test]
    fn test_soln2() {
        let lines = as_vstrings(&INPUT[0..]);
        let sensors = lines.iter().map(|line| Sensor::parse(line)).collect_vec();
        assert_eq!(tuning_freq(&sensors, &(20, 20)), 56000011);
        assert_eq!(
            tuning_freq2(0, &sensors, &(0, 0), &(20, 20)),
            Some(56000011)
        );
    }

    #[test]
    fn test_fully_contained() {
        let lines = as_vstrings(&INPUT[0..]);
        let sensors = lines.iter().map(|line| Sensor::parse(line)).collect_vec();
        assert!(fully_contained(&sensors, &(12, 10), &(15, 15)));
    }
    #[test]
    fn test_inside() {
        let s = Sensor {
            s_pos: (0, 0),
            b_pos: (10, 0),
        };

        assert!(s.in_range(&(0, 10)));
        assert!(s.in_range(&(0, -10)));
        assert!(s.in_range(&(1, 9)));
        assert!(!s.in_range(&(0, 11)));
        assert!(!s.in_range(&(0, -11)));
        assert!(!s.in_range(&(1, 10)));
    }
    #[test]
    fn test_all_inside() {
        let s = Sensor {
            s_pos: (0, 0),
            b_pos: (10, 0),
        };

        assert!(s.all_in_range(&(-1, -1), &(1, 1)));
        assert!(s.all_in_range(&(-5, -5), &(5, 5)));
        assert!(!s.all_in_range(&(-6, -6), &(5, 5)));
        assert!(!s.all_in_range(&(-5, -5), &(6, 6)));
        assert!(s.all_in_range(&(10, 0), &(10, 0)));
        assert!(!s.all_in_range(&(10, 0), &(11, 0)));
        assert!(s.all_in_range(&(0, 9), &(0, 10)));
        assert!(!s.all_in_range(&(0, 10), &(0, 11)));
        assert!(s.all_in_range(&(-10, 0), &(10, 0)));
        assert!(!s.all_in_range(&(-11, 0), &(10, 0)));
    }

    #[test]
    fn test_perm() {
        let lines = as_vstrings(&INPUT[0..]);
        let sensors = lines.iter().map(|line| Sensor::parse(line)).collect_vec();
        //perm(&sensors)

        println!("{}", 1 / 2);
    }

    #[test]
    fn test_rangeset() {
        let sensors = vec![Sensor {
            s_pos: (8, 7),
            b_pos: (2, 10),
        }];
        assert_eq!(covered_size(&sensors, 10), 13);
        assert_eq!(covered_size(&sensors, 7), 19);

        let sensors = vec![
            Sensor {
                s_pos: (0, 0),
                b_pos: (1, 0),
            },
            Sensor {
                s_pos: (10, 0),
                b_pos: (11, 0),
            },
        ];
        assert_eq!(covered_size(&sensors, 1), 2);
        assert_eq!(covered_size(&sensors, 2), 0);

        let sensors = vec![
            Sensor {
                s_pos: (0, -1),
                b_pos: (1, 0),
            },
            Sensor {
                s_pos: (10, 0),
                b_pos: (11, 1),
            },
        ];
        assert_eq!(covered_size(&sensors, -4), 0);
        assert_eq!(covered_size(&sensors, -3), 1);
        assert_eq!(covered_size(&sensors, -2), 4);
        assert_eq!(covered_size(&sensors, -1), 8);
        assert_eq!(covered_size(&sensors, 0), 8);
        assert_eq!(covered_size(&sensors, 1), 4);
        assert_eq!(covered_size(&sensors, 2), 1);
        assert_eq!(covered_size(&sensors, 3), 0);
    }
    #[test]
    fn test_range() {
        assert_eq!(
            Sensor {
                s_pos: (8, 7),
                b_pos: (2, 10)
            }
            .covered_range(10),
            Some(2..15)
        );
        assert_eq!(
            Sensor {
                s_pos: (0, 0),
                b_pos: (0, 1)
            }
            .covered_range(0),
            Some(-1..2)
        );
        assert_eq!(
            Sensor {
                s_pos: (0, 0),
                b_pos: (0, 1)
            }
            .covered_range(1),
            Some(0..1)
        );
        assert_eq!(
            Sensor {
                s_pos: (10, 0),
                b_pos: (11, 0)
            }
            .covered_range(1),
            Some(10..11)
        );
        assert_eq!(
            Sensor {
                s_pos: (10, 0),
                b_pos: (11, 0)
            }
            .covered_range(2),
            None
        );
        assert_eq!(
            Sensor {
                s_pos: (0, -1),
                b_pos: (1, 0),
            }
            .covered_range(-1),
            Some(-2..3)
        );
    }
    #[test]
    fn test_mdist() {
        assert_eq!(
            Sensor {
                s_pos: (8, 7),
                b_pos: (2, 10)
            }
            .man_dist(),
            9
        );
    }
    #[test]
    fn test_parse_sensor() {
        assert_eq!(
            parse_sensor("Sensor at x=20, y=1: closest beacon is at x=15, y=3"),
            Ok((
                "",
                Sensor {
                    s_pos: (20, 1),
                    b_pos: (15, 3)
                }
            ))
        );
    }
}
