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

fn solve_part2(input: &str) -> usize {
    0
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
    const TEST_INPUT_2: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 2286);
    }
}
