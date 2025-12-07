use std::collections::{HashMap, HashSet};

use aoc_utils::{
    dir::Dir4,
    grud::{Grid, GridPos},
};

pub fn part1(input: &str) -> usize {
    let teleporter = Grid::<char, Dir4>::parse(input);
    let start = teleporter.find('S').unwrap();
    let mut beams = vec![start];
    let mut visited: HashSet<GridPos> = HashSet::new();
    let mut num_times_split = 0;
    while let Some(beam) = beams.pop() {
        if visited.contains(&beam) {
            continue;
        }
        let mut new_beam = beam + Dir4::S;
        while let Some(x) = teleporter.at(&new_beam) {
            if visited.contains(&new_beam) {
                break;
            }
            visited.insert(new_beam);
            if x == '^' {
                beams.push(new_beam + Dir4::E);
                beams.push(new_beam + Dir4::W);
                num_times_split += 1;
                println!("Split @ {new_beam}, split={num_times_split}");
                break;
            } else {
                assert!(x == '.');
            }
            new_beam = new_beam + Dir4::S;
        }
    }
    num_times_split
}

fn beam_split_r(
    qteleporter: &Grid<char, Dir4>,
    beam: GridPos,
    visited: &mut HashMap<GridPos, usize>,
) -> usize {
    if let Some(num) = visited.get(&beam) {
        return *num;
    }

    let mut new_beam = beam + Dir4::S;
    let mut beam_locs = vec![new_beam];
    while let Some(t) = qteleporter.at(&new_beam) {
        if let Some(num) = visited.get(&new_beam) {
            return *num;
        }
        match t {
            '.' => {
                new_beam = new_beam + Dir4::S;
                beam_locs.push(new_beam);
                continue;
            }
            '^' => {
                let time_lines = beam_split_r(qteleporter, new_beam + Dir4::E, visited)
                    + beam_split_r(qteleporter, new_beam + Dir4::W, visited);
                update_visited(&beam_locs, time_lines, visited);
                return time_lines;
            }
            _ => panic!("Unexpected char"),
        }
    }
    // beam reached bottom
    update_visited(&beam_locs, 1, visited);
    1
}

fn update_visited(
    beam_locs: &Vec<GridPos>,
    time_lines: usize,
    visited: &mut HashMap<GridPos, usize>,
) {
    beam_locs.into_iter().for_each(|&beam| {
        let a = visited.insert(beam, time_lines);
        assert!(a.is_none() || a.unwrap() == time_lines);
        println!("Adding to visited: {beam}, {time_lines}");
    });
}
pub fn part2(input: &str) -> usize {
    let qteleporter = Grid::<char, Dir4>::parse(input);
    let start = qteleporter.find('S').unwrap();
    let mut visited: HashMap<GridPos, usize> = HashMap::new();
    beam_split_r(&qteleporter, start, &mut visited)
}

const INPUT: &str = include_str!("data/input");
fn main() {
    aoc_utils::run::main(INPUT, part1, part2);
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 21;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 40;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
}
