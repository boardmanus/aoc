use std::str::FromStr;

use crate::aoc;

#[derive(Debug)]
enum Error {
    ParseAssignment,
    ParseAssignmentPairs,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Assignment(u32, u32);
impl FromStr for Assignment {
    type Err = Error;
    fn from_str(ass_str: &str) -> Result<Self, Error> {
        let se = ass_str
            .split('-')
            .flat_map(|se| se.parse::<u32>())
            .collect::<Vec<u32>>();
        if se.len() != 2 {
            return Err(Error::ParseAssignment);
        }
        Ok(Assignment(se[0], se[1]))
    }
}

impl Assignment {
    fn fully_contains(self, other: &Assignment) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(self, other: &Assignment) -> bool {
        (self.0 <= other.0 && self.1 >= other.0)
            || (self.0 <= other.1 && self.1 >= other.1)
            || (self.0 > other.0 && self.1 < other.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct AssignmentPair(Assignment, Assignment);
impl FromStr for AssignmentPair {
    type Err = Error;
    fn from_str(ass_str: &str) -> Result<Self, Error> {
        let ass = ass_str
            .split(',')
            .flat_map(|elf| Assignment::from_str(elf))
            .collect::<Vec<Assignment>>();
        if ass.len() != 2 {
            return Err(Error::ParseAssignmentPairs);
        }

        Ok(AssignmentPair(ass[0], ass[1]))
    }
}

impl AssignmentPair {
    fn fully_overlaps(self) -> bool {
        self.0.fully_contains(&self.1) || self.1.fully_contains(&self.0)
    }

    fn any_overlap(self) -> bool {
        self.0.overlaps(&self.1)
    }
}

pub struct Day4_1;
impl aoc::Aoc<u32> for Day4_1 {
    fn day(&self) -> u32 {
        4
    }
    fn puzzle_name(&self) -> &str {
        "Camp Cleanup"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        lines
            .iter()
            .flat_map(|line| AssignmentPair::from_str(line))
            .fold(0, |s, ap| if ap.fully_overlaps() { s + 1 } else { s })
            .to_string()
    }
}

pub struct Day4_2;
impl aoc::Aoc<u32> for Day4_2 {
    fn day(&self) -> u32 {
        4
    }
    fn puzzle_name(&self) -> &str {
        "Camp Cleanup all overlaps"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        lines
            .iter()
            .flat_map(|line| AssignmentPair::from_str(line))
            .fold(0, |s, ap| if ap.any_overlap() { s + 1 } else { s })
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assignment_from_str() {
        assert_eq!(Assignment::from_str("1-2").unwrap(), Assignment(1, 2));
        assert_eq!(
            Assignment::from_str("123-278").unwrap(),
            Assignment(123, 278)
        );
    }

    #[test]
    fn test_assignment_pair_from_str() {
        assert_eq!(
            AssignmentPair::from_str("1-2,3-4").unwrap(),
            AssignmentPair(Assignment(1, 2), Assignment(3, 4))
        );
        assert_eq!(
            AssignmentPair::from_str("126-234,333-4455").unwrap(),
            AssignmentPair(Assignment(126, 234), Assignment(333, 4455))
        );
    }

    #[test]
    fn test_assignment_fully_contains() {
        assert!(Assignment(10, 20).fully_contains(&Assignment(11, 19)));
        assert!(Assignment(10, 20).fully_contains(&Assignment(10, 20)));
        assert!(!Assignment(10, 20).fully_contains(&Assignment(9, 11)));
        assert!(!Assignment(10, 20).fully_contains(&Assignment(15, 21)));
        assert!(!Assignment(10, 20).fully_contains(&Assignment(1, 2)));
        assert!(!Assignment(10, 20).fully_contains(&Assignment(21, 22)));
    }

    #[test]
    fn test_assignments_overlap() {
        assert!(Assignment(10, 20).overlaps(&Assignment(8, 10)));
        assert!(Assignment(10, 20).overlaps(&Assignment(20, 24)));
        assert!(!Assignment(10, 20).overlaps(&Assignment(1, 4)));
        assert!(!Assignment(10, 20).overlaps(&Assignment(21, 26)));
        assert!(Assignment(10, 20).overlaps(&Assignment(9, 21)));
        assert!(Assignment(10, 20).overlaps(&Assignment(11, 19)));
    }

    #[test]
    fn test_assignment_pair_fully_overlaps() {
        assert!(AssignmentPair(Assignment(10, 20), Assignment(11, 19)).fully_overlaps());
        assert!(AssignmentPair(Assignment(11, 19), Assignment(10, 20)).fully_overlaps());
        assert!(!AssignmentPair(Assignment(9, 19), Assignment(10, 20)).fully_overlaps());
    }
}
