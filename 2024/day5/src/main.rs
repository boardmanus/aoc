use std::env;

use day5::solution::{part1, part2};

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

    const INPUT: &str = include_str!("data/input");
    if parts.0 {
        println!("Part1 Solution:\n{}", part1(INPUT));
    }
    if parts.1 {
        println!("Part2 Solution:\n{}", part2(INPUT));
    }
}
