use core::num;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Colour {
    Blue,
    Red,
    Green,
}
const NUM_COLOURS: usize = 3;

impl FromStr for Colour {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(Colour::Blue),
            "red" => Ok(Colour::Red),
            "green" => Ok(Colour::Green),
            _ => Err(()),
        }
    }
}

impl Colour {
    fn to_str(&self) -> &str {
        match self {
            Colour::Blue => "blue",
            Colour::Red => "red",
            Colour::Green => "green",
        }
    }

    fn to_index(&self) -> usize {
        match self {
            Colour::Blue => 0,
            Colour::Red => 1,
            Colour::Green => 2,
        }
    }
}

#[derive(Debug, Default)]
struct HandFull {
    num: [usize; NUM_COLOURS],
}

struct Game {
    num: usize,
    hands: Vec<HandFull>,
}

fn parse_card(card: &str) -> (Colour, usize) {
    let regex = Regex::new(r"(\d+) (\w+)").expect("Invalid regex");

    let captures = regex.captures(card).expect("Invalid card");
    let num = captures
        .get(1)
        .expect("Invalid card capture")
        .as_str()
        .parse::<usize>()
        .expect("Invalid number");

    let colour = Colour::from_str(captures.get(2).expect("Invalid card capture").as_str()).unwrap();

    (colour, num)
}

fn parse_hand(hand: &str) -> HandFull {
    hand.split(',')
        .into_iter()
        .fold(HandFull::default(), |mut acc, card| {
            let (colour, num) = parse_card(card);
            acc.num[colour.to_index()] = num;
            acc
        })
}

fn parse_game(line: &str) -> Game {
    let regex = Regex::new(r"Game (\d+): (.*)").expect("Invalid regex");

    let captures = regex.captures(line).expect("Invalid line");
    let num = captures
        .get(1)
        .expect("Invalid game capture")
        .as_str()
        .parse::<usize>()
        .expect("Invalid number");

    let hands = captures
        .get(2)
        .expect("Invalid hands capture")
        .as_str()
        .split(';')
        .map(|hand| parse_hand(hand))
        .collect::<Vec<HandFull>>();

    Game { num, hands }
}

fn solve_part1(input: &str) -> usize {
    let max_hand = HandFull { num: [14, 13, 12] };
    input
        .lines()
        .map(|line| {
            let game = parse_game(line);
            let possible = game
                .hands
                .iter()
                .all(|hand| (0..NUM_COLOURS).all(|i| hand.num[i] <= max_hand.num[i]));

            if possible {
                game.num
            } else {
                0
            }
        })
        .sum()
}

fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let game = parse_game(line);
            let max_colours = (0..NUM_COLOURS).fold(HandFull::default(), |mut acc, i| {
                acc.num[i] = game.hands.iter().map(|hand| hand.num[i]).max().unwrap();
                acc
            });
            max_colours.num.iter().product::<usize>()
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
    const TEST_INPUT_2: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 2286);
    }

    #[test]
    fn test_parse_card() {
        assert_eq!(parse_card("1 blue"), (Colour::Blue, 1));
        assert_eq!(parse_card("2 red"), (Colour::Red, 2));
        assert_eq!(parse_card("3 green"), (Colour::Green, 3));
    }

    #[test]
    fn test_parse_hand() {
        let hand = parse_hand("1 blue, 2 red, 3 green");
        assert_eq!(hand.num[Colour::Blue.to_index()], 1);
        assert_eq!(hand.num[Colour::Red.to_index()], 2);
        assert_eq!(hand.num[Colour::Green.to_index()], 3);
    }

    #[test]
    fn test_parse_game() {
        let game = parse_game("Game 134: 1 blue, 2 red, 3 green; 6 blue, 5 red; 56 green");
        assert_eq!(game.num, 134);
        assert_eq!(game.hands.len(), 3);
        assert_eq!(game.hands[0].num[Colour::Blue.to_index()], 1);
        assert_eq!(game.hands[0].num[Colour::Red.to_index()], 2);
        assert_eq!(game.hands[0].num[Colour::Green.to_index()], 3);
        assert_eq!(game.hands[1].num[Colour::Blue.to_index()], 6);
        assert_eq!(game.hands[1].num[Colour::Red.to_index()], 5);
        assert_eq!(game.hands[2].num[Colour::Green.to_index()], 56);
    }
}
