fn more_ones(col: &str) -> bool {
    col.chars().filter(|c| *c == '1').count() >= (col.len() + 1) / 2
}

fn less_ones(col: &str) -> bool {
    !more_ones(col)
}

fn to_val(row: &str) -> u64 {
    row.chars()
        .fold(0, |val, c| (val << 1) | if c == '1' { 1 } else { 0 })
}

fn to_val_from_cols(cols: &[String], bits: fn(&str) -> bool) -> u64 {
    let bit_str: String = cols
        .iter()
        .map(|col| if bits(&col) { '1' } else { '0' })
        .collect();
    to_val(&bit_str)
}

fn gamma(cols: &[String]) -> u64 {
    to_val_from_cols(cols, more_ones)
}

fn epsilon(cols: &[String]) -> u64 {
    to_val_from_cols(cols, less_ones)
}

fn to_col(rows: &[&str], index: usize) -> String {
    rows.iter().flat_map(|row| row.chars().nth(index)).collect()
}

fn to_cols(rows: &[&str]) -> Vec<String> {
    (0..rows[0].len())
        .map(|col_index| to_col(rows, col_index))
        .collect()
}

fn filter_rows<'a>(rows: &[&'a str], index: usize, val: char) -> Vec<&'a str> {
    rows.iter()
        .filter(move |row| row.chars().nth(index) == Some(val))
        .map(|s| *s)
        .collect()
}

fn filter_all_rows<'a>(rows: &[&'a str], test: fn(&str) -> bool) -> &'a str {
    let mut filtered_rows: Vec<&str> = Vec::from(rows);
    for col_index in 0..rows[0].len() {
        if filtered_rows.len() == 1 {
            break;
        }
        let col = to_col(&filtered_rows, col_index);
        filtered_rows = filter_rows(
            &filtered_rows,
            col_index,
            if test(&col) { '1' } else { '0' },
        );
    }
    filtered_rows[0]
}

fn o2_gen_rating(rows: &[&str]) -> u64 {
    to_val(filter_all_rows(rows, more_ones))
}

fn co2_scrub_rating(rows: &[&str]) -> u64 {
    to_val(filter_all_rows(rows, less_ones))
}

fn solve_part1(cols: &[String]) -> u64 {
    gamma(cols) * epsilon(cols)
}

fn solve_part2(rows: &[&str]) -> u64 {
    o2_gen_rating(rows) * co2_scrub_rating(rows)
}

fn parse_input<'a>(input: &'a str) -> Vec<&str> {
    input.split('\n').collect::<Vec<_>>()
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let rows = parse_input(INPUT);
    let part1 = solve_part1(&to_cols(&rows));
    println!("Part1: {part1}");
    let part2 = solve_part2(&rows);
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&to_cols(&parse_input(TEST_INPUT))), 198);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&parse_input(TEST_INPUT)), 230);
    }
}
