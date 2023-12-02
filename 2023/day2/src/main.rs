use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Colour {
    Blue,
    Red,
    Green,
}

type HandFull = HashMap<Colour, usize>;

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

    let colour = match captures
        .get(2)
        .expect("Invalid card capture")
        .as_str()
        .to_lowercase()
        .as_str()
    {
        "blue" => Colour::Blue,
        "red" => Colour::Red,
        "green" => Colour::Green,
        _ => panic!("Invalid colour"),
    };

    (colour, num)
}

fn parse_hand(hand: &str) -> HandFull {
    hand.split(',')
        .into_iter()
        .map(|h| parse_card(h))
        .collect::<HashMap<Colour, usize>>()
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
    let max_colours = HashMap::from([(Colour::Blue, 14), (Colour::Red, 12), (Colour::Green, 13)]);

    input
        .lines()
        .map(|line| {
            let game = parse_game(line);
            let possible = game.hands.iter().all(|hand| {
                hand.iter()
                    .all(|(colour, num)| num <= max_colours.get(colour).unwrap())
            });

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
            let max_colours = game.hands.iter().fold(HashMap::new(), |mut acc, hand| {
                for (colour, num) in hand.iter() {
                    let current = acc.entry(colour).or_insert(0);
                    if *current < *num {
                        *current = *num;
                    }
                }
                acc
            });
            max_colours.values().product::<usize>()
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
        assert_eq!(hand[&Colour::Blue], 1);
        assert_eq!(hand[&Colour::Red], 2);
        assert_eq!(hand[&Colour::Green], 3);
    }

    #[test]
    fn test_parse_game() {
        let game = parse_game("Game 134: 1 blue, 2 red, 3 green; 6 blue, 5 red; 56 green");
        assert_eq!(game.num, 134);
        assert_eq!(game.hands.len(), 3);
        assert_eq!(game.hands[0][&Colour::Blue], 1);
        assert_eq!(game.hands[0][&Colour::Red], 2);
        assert_eq!(game.hands[0][&Colour::Green], 3);
        assert_eq!(game.hands[1][&Colour::Blue], 6);
        assert_eq!(game.hands[1][&Colour::Red], 5);
        assert_eq!(game.hands[2][&Colour::Green], 56);
    }
}
