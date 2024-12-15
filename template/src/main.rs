use day{{aoc_day}}::solution::{part1, part2};
use lazy_regex::regex;
use std::{env, fs};

fn check_solution(solution: &str, answer: Option<&String>) {
    if let Some(answer) = answer {
        if answer == solution {
            println!("Answer matches expected solution.");
        } else {
            println!("WRONG! Expected {answer}");
        }
    }
}

fn extract_solution(line: &str) -> Option<&str> {
    Some(
        regex!(r"^Your puzzle answer was `(.+)`\.")
            .captures(line)?
            .get(1)?
            .as_str(),
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let parts = match args.len() {
        1 => (true, true),
        2 => match args[1].as_str() {
            "1" => (true, false),
            "2" => (false, true),
            _ => panic!("Wrong part value: part = 1 | 2"),
        },
        _ => panic!("Wrong arguments: {} [part]   => part = 1|2", args[0]),
    };

    let mut answers: Vec<String> = Vec::default();
    if let Ok(puzzle_str) = fs::read_to_string("puzzle.md") {
        puzzle_str
            .lines()
            .filter_map(|line| extract_solution(line))
            .for_each(|answer| answers.push(answer.to_string()));
        println!("Found answers: {:?}", answers);
    }

    const INPUT: &str = include_str!("data/input");

    if parts.0 {
        let solution1 = part1(INPUT).to_string();
        println!("Part1 Solution:\n{}", solution1);
        check_solution(&solution1, answers.first());
    }

    if parts.1 {
        let solution2 = part2(INPUT).to_string();
        println!("Part2 Solution:\n{}", solution2);
        check_solution(&solution2, answers.get(1));
    }
}
