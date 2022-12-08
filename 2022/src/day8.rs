use array2d::Array2D;
use itertools::Itertools;

use crate::aoc::Aoc;

fn to_rows(lines: &Vec<String>) -> Vec<Vec<usize>> {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c as usize - '0' as usize)
                .collect_vec()
        })
        .collect_vec()
}

fn to_matrix(lines: &Vec<String>) -> Array2D<usize> {
    Array2D::from_rows(&to_rows(lines))
}

fn is_visible(section: &[usize], h: usize, i: usize) -> bool {
    section[0..i].iter().all(|h2| *h2 < h) || section[i + 1..section.len()].iter().all(|h2| *h2 < h)
}

fn num_visible(section: &[usize], h: &usize) -> usize {
    section.iter().rev().take_while(|h2| *h2 < h).count()
}

pub struct Day8_1;
impl Aoc for Day8_1 {
    fn day(&self) -> u32 {
        8
    }
    fn puzzle_name(&self) -> &str {
        "Tree House"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        let m = to_matrix(lines);
        let mut visible = 2 * (m.num_rows() + m.num_columns() - 2);

        for x in 1..m.num_columns() - 1 {
            for y in 1..m.num_rows() - 1 {
                let h = m.get(y, x).unwrap();
                if is_visible(&m.as_rows()[y], *h, x) || is_visible(&m.as_columns()[x], *h, y) {
                    visible += 1;
                }
            }
        }
        visible.to_string()
    }
}

fn score(rows: &Vec<Vec<usize>>, x: usize, y: usize) -> usize {
    let h = rows[y][x];
    let mut visible = [0, 0, 0, 0];
    (0..x).rev().all(|c| {
        visible[0] += 1;
        rows[y][c] < h
    });
    (x + 1..rows[0].len()).all(|c| {
        visible[1] += 1;
        rows[y][c] < h
    });
    (0..y).rev().all(|r| {
        visible[2] += 1;
        rows[r][x] < h
    });
    (y + 1..rows.len()).all(|r| {
        visible[3] += 1;
        rows[r][x] < h
    });
    //println!("({x}, {y}) => {:?}", visible);
    visible.iter().product()
}
pub struct Day8_2;
impl Aoc for Day8_2 {
    fn day(&self) -> u32 {
        8
    }
    fn puzzle_name(&self) -> &str {
        "Tree House 2"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        let rows = to_rows(lines);
        let mut s = 0;
        for x in 1..rows[0].len() - 1 {
            for y in 1..rows.len() - 1 {
                s = s.max(score(&rows, x, y));
            }
        }

        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 5] = ["30373", "25512", "65332", "33549", "35390"];

    #[test]
    fn test_soln() {
        let input_strs = INPUT
            .map(|s| String::from(s))
            .into_iter()
            .collect::<Vec<String>>();

        assert_eq!(Day8_1.solve(&input_strs), 21.to_string());
    }

    #[test]
    fn test_soln2() {
        let input_strs = INPUT
            .map(|s| String::from(s))
            .into_iter()
            .collect::<Vec<String>>();

        assert_eq!(Day8_2.solve(&input_strs), 8.to_string());
    }
}
