use core::num;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char) -> Card {
        match c {
            'j' => Card::Joker,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard(Card),
    OnePair(Card),
    TwoPairs(Card, Card),
    ThreeOfAKind(Card),
    FullHouse(Card, Card),
    FourOfAKind(Card),
    FiveOfAKind(Card),
}

impl HandType {
    fn strength(&self) -> u64 {
        match self {
            HandType::HighCard(_) => 1,
            HandType::OnePair(_) => 2,
            HandType::TwoPairs(_, _) => 3,
            HandType::ThreeOfAKind(_) => 4,
            HandType::FullHouse(_, _) => 5,
            HandType::FourOfAKind(_) => 6,
            HandType::FiveOfAKind(_) => 7,
        }
    }
}

fn parse(input: &str) -> Vec<(Vec<Card>, u64)> {
    input
        .lines()
        .map(|line| {
            let mut bits = line.split_whitespace();
            let cards = bits
                .next()
                .unwrap()
                .chars()
                .map(|c| Card::from_char(c))
                .collect::<Vec<_>>();
            let bid = bits.next().unwrap().parse::<u64>().unwrap();
            (cards, bid)
        })
        .collect()
}

fn count_to_hand_type(card: Card, count: u64) -> Option<HandType> {
    match count {
        1 => Some(HandType::HighCard(card)),
        2 => Some(HandType::OnePair(card)),
        3 => Some(HandType::ThreeOfAKind(card)),
        4 => Some(HandType::FourOfAKind(card)),
        5 => Some(HandType::FiveOfAKind(card)),
        _ => None,
    }
}

fn condense_hand_types(hand_types: &Vec<Option<HandType>>) -> Option<HandType> {
    let a = match hand_types.len() {
        0 => None,
        1 => Some(hand_types[0]?),
        2 => match (hand_types[0]?, hand_types[1]?) {
            (HandType::OnePair(card1), HandType::OnePair(card2)) => {
                Some(HandType::TwoPairs(card1, card2))
            }
            (HandType::OnePair(pair), HandType::ThreeOfAKind(toak)) => {
                Some(HandType::FullHouse(pair, toak))
            }
            _ => None,
        },
        _ => None,
    };
    println!("Condensing hand types={:?} => {:?}", hand_types, a);
    a
}

fn winnings(hands: Vec<(Vec<Card>, u64)>) -> u64 {
    let mut ordered_hands = hands
        .iter()
        .map(|hand| {
            let (cards, bid) = &hand;
            let mut sorted_cards = cards
                .iter()
                .filter(|card| **card != Card::Joker)
                .map(|card| *card)
                .collect::<Vec<_>>();
            sorted_cards.sort();
            let num_jokers = cards.len() - sorted_cards.len();
            let mut last_card = None;
            let mut count = 0;
            let mut hand_types = vec![];
            for card in sorted_cards {
                if let Some(last_card) = last_card {
                    if last_card == card {
                        count += 1;
                    } else {
                        if count > 1 {
                            hand_types.push((last_card, count));
                        }
                        count = 1;
                    }
                } else {
                    count = 1;
                }
                last_card = Some(card);
            }
            if count > 1 {
                hand_types.push((last_card.unwrap(), count));
            }
            if hand_types.len() > 0 {
                hand_types.sort_by(|a, b| a.1.cmp(&b.1));
                hand_types.last_mut().unwrap().1 += num_jokers as u64;
            } else if num_jokers > 0 {
                hand_types.push((Card::Joker, 5.min(1 + num_jokers as u64)));
            }
            let hand_types = hand_types
                .iter()
                .map(|(card, count)| count_to_hand_type(*card, *count))
                .collect();
            let condensed: Option<HandType> = condense_hand_types(&hand_types);
            (condensed, cards, bid)
        })
        .collect::<Vec<_>>();

    ordered_hands.sort_by(|a, b| {
        match a
            .0
            .unwrap_or(HandType::HighCard(Card::Ace))
            .strength()
            .cmp(&b.0.unwrap_or(HandType::HighCard(Card::Ace)).strength())
        {
            Ordering::Equal => a.1.cmp(&b.1),
            other => other,
        }
    });

    println!("Ordered Hands={:?}", ordered_hands);

    ordered_hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            let (_, _, bid) = hand;
            *bid * (i as u64 + 1)
        })
        .sum()
}

fn solve_part1(input: &str) -> u64 {
    winnings(parse(input))
}

fn solve_part2(input: &str) -> u64 {
    winnings(parse(&input.replace('J', "j")))
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let part1 = solve_part1(INPUT);
    println!("Part1: {part1}");
    let part2 = solve_part2(INPUT);
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
    const TEST_INPUT_2: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 5905);
    }

    #[test]
    fn test_parse() {
        let hands = parse(TEST_INPUT);
        assert_eq!(hands.len(), 5);
        assert_eq!(
            hands[3], //KTJJT 220
            (
                vec![Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten],
                220
            )
        );
    }
}
