use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Part(u64, (i64, i64)),
    Gear,
}

type EngineMap = HashMap<(i64, i64), Cell>;

#[derive(Debug)]
struct EnginePart {
    num: usize,
    row: usize,
    col: usize,
    len: usize,
}

struct Grid {
    grid: Vec<Vec<char>>,
    parts: Vec<EnginePart>,
}

fn fill_engine_map(mut em: EngineMap, line: (usize, &&str)) -> EngineMap {
    let mut num_str: &str = line.1;
    let mut id = (0i64, 0i64);
    let mut q: Vec<(i64, i64)> = Vec::new();
    num_str.chars().enumerate().for_each(|(i, c)| match c {
        '0'..='9' => {
            let pos = (i as i64, line.0 as i64);
            if q.len() == 0 {
                num_str = &line.1[i..];
                id = pos;
            }
            q.push(pos);
        }
        '*' => {
            if q.len() != 0 {
                let num = num_str[..q.len()].parse::<u64>().unwrap();
                q.iter().for_each(|(x, y)| {
                    em.insert((*x, *y), Cell::Part(num as u64, id));
                });
                q.clear();
            }
            em.insert((i as i64, line.0 as i64), Cell::Gear);
        }
        _ => {
            if q.len() != 0 {
                let num = num_str[..q.len()].parse::<u64>().unwrap();
                q.iter().for_each(|(x, y)| {
                    em.insert((*x, *y), Cell::Part(num as u64, id));
                });
                q.clear();
            }
        }
    });
    em
}

fn line_engine_parts(line: &str, row: usize) -> Vec<EnginePart> {
    let mut i = 0;
    let row_len = line.len();
    let s = line;
    let mut ep = Vec::new();
    while i < row_len {
        if let Some(j) = &s[i..].find(|c: char| c.is_numeric()) {
            if let Some(k) = &s[(i + *j)..].find(|c: char| !c.is_numeric()) {
                let num = s[(i + *j)..(i + *j + *k)].parse::<usize>().unwrap();
                ep.push(EnginePart {
                    num,
                    row,
                    col: i + *j,
                    len: *k,
                });
                i += *j + *k;
            } else {
                let num = s[(i + *j)..].parse::<usize>().unwrap();
                ep.push(EnginePart {
                    num,
                    row,
                    col: i + *j,
                    len: row_len - *j,
                });
                i = row_len;
            }
        } else {
            i = row_len;
        }
    }
    ep
}

fn is_near_symbol(ep: &EnginePart, lines: &Vec<&str>) -> bool {
    let non_symbols = "0123456789.";
    let x0 = if ep.col == 0 { 0 } else { ep.col - 1 };
    let x1 = (lines[0].len() - 1).min(ep.col + ep.len + 1);
    let mut row: &str;

    if ep.row != 0 {
        row = &(lines[ep.row - 1])[x0..x1];
        if !row.chars().all(|c| non_symbols.contains(c)) {
            return true;
        }
    }

    row = lines[ep.row];
    if !non_symbols.contains(row.chars().nth(x0).unwrap())
        || !non_symbols.contains(row.chars().nth(x1 - 1).unwrap())
    {
        return true;
    }

    if ep.row != lines.len() - 1 {
        row = &(lines[ep.row + 1])[x0..x1];
        if !row.chars().all(|c| non_symbols.contains(c)) {
            return true;
        }
    }

    false
}

fn solve_part1(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let ep = lines.iter().enumerate().fold(Vec::new(), |mut acc, line| {
        let lep = line_engine_parts(line.1, line.0);
        acc.extend(lep);
        acc
    });

    ep.iter()
        .filter(|e| is_near_symbol(e, &lines))
        .fold(0, |acc, e| {
            print!("{:?}\n", e);
            acc + e.num
        })
}

const GRID: [(i64, i64); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn solve_part2(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<_>>();

    let ep = lines
        .iter()
        .enumerate()
        .fold(EngineMap::new(), fill_engine_map);

    ep.iter()
        .filter(|(_, v)| **v == Cell::Gear)
        .map(|(k, _)| {
            let parts = GRID.iter().fold(HashSet::new(), |mut acc, (dx, dy)| {
                if let Some(Cell::Part(num, id)) = ep.get(&(k.0 + dx, k.1 + dy)) {
                    acc.insert((*num, *id));
                }
                acc
            });
            if parts.len() == 2 {
                let p = parts.iter().map(|v| v.0).product();
                println!(
                    "Found 2 parts around ({}, {}) => {:?} => {}",
                    k.0, k.1, parts, p
                );
                p
            } else {
                if parts.len() > 0 {
                    println!("Found {} parts around ({}, {})", parts.len(), k.0, k.1);
                }
                0
            }
        })
        .sum()
}

fn main() {
    let part1 = solve_part1(include_str!("input.txt"));
    println!("Part1: {part1}");
    let part2 = solve_part2(include_str!("input.txt"));
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
    const TEST_INPUT_2: &str = include_str!("test_input2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 467835);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 467835);
    }
}
