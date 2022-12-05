use std::process;

mod aoc;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
use aoc::Aoc;

fn main() {
    let puzzles: Vec<&'static dyn Aoc<u32>> = vec![
        &day1::Day1_1,
        &day1::Day1_2,
        &day2::Day2_1,
        &day2::Day2_1_2,
        &day2::Day2_2,
        &day2::Day2_2_2,
        &day3::Day3_1,
        &day3::Day3_2,
        &day4::Day4_1,
        &day4::Day4_2,
        &day5::Day5_1,
    ];

    puzzles.into_iter().for_each(|puzzle| {
        let lines = aoc::read_lines(puzzle.input_name().as_str()).unwrap_or_else(|err| {
            println!(
                "ERROR: couldn't read lines from {:} ({err})",
                puzzle.input_name()
            );
            process::exit(1);
        });
        let res = puzzle.solve(&lines);
        println!("Day {:}: {:} => {res}", puzzle.day(), puzzle.puzzle_name());
    });
}
