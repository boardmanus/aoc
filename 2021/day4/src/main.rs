#![feature(slice_as_chunks)]

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};
use std::{collections::HashMap, num::ParseIntError};
use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
enum BingoParseError {
    Fail,
    Int(ParseIntError),
    Nom,
}

impl From<ParseIntError> for BingoParseError {
    fn from(err: ParseIntError) -> BingoParseError {
        BingoParseError::Int(err)
    }
}

fn row_parser(line: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        multispace0,
        separated_list1(multispace1, digit1),
        multispace0,
    )(line)
}

fn call_parser(line: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(","), digit1)(line)
}

fn as_u64(s: &str) -> Result<u64, BingoParseError> {
    let val = u64::from_str(s)?;
    Ok(val)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Card([[u64; 5]; 5]);
impl Card {
    fn new(rows: &[Vec<u64>; 5]) -> Self {
        let mut row_data: [[u64; 5]; 5] = Default::default();
        rows.iter().enumerate().for_each(|row| {
            row.1
                .iter()
                .enumerate()
                .for_each(|col| row_data[row.0][col.0] = *col.1)
        });

        let card = Card(row_data);
        println!("{:?}", card);
        card
    }
}

type CallMap<'a> = HashMap<u64, Vec<CallCoord<'a>>>;

#[derive(Debug)]
struct CardState {
    row_count: [usize; 5],
    col_count: [usize; 5],
    bingod: bool,
}

impl<'a> CardState {
    fn new() -> CardState {
        CardState {
            row_count: [0; 5],
            col_count: [0; 5],
            bingod: false,
        }
    }
    fn update(&mut self, call: u64, coord: &'a CallCoord) -> Option<&'a Card> {
        if self.bingod {
            None
        } else {
            self.row_count[coord.row] += 1;
            self.col_count[coord.col] += 1;
            if self.row_count[coord.row] >= 5 {
                self.bingod = true;
                println!(
                    "Bingo: call={call}, row={}, card={:?}",
                    coord.row, coord.card
                );
                Some(coord.card)
            } else if self.col_count[coord.col] >= 5 {
                self.bingod = true;
                println!(
                    "Bingo: call={call}, col={}, card={:?}",
                    coord.col, coord.card
                );
                Some(coord.card)
            } else {
                None
            }
        }
    }
}

type CardStateMap<'a> = HashMap<&'a Card, CardState>;

#[derive(Debug)]
struct BingoState<'a> {
    calls: Vec<u64>,
    card_states: CardStateMap<'a>,
}

impl<'a> BingoState<'a> {
    fn new(bingo: &'a Bingo) -> BingoState<'a> {
        let mut card_states = CardStateMap::default();
        for card in &bingo.cards {
            card_states.insert(card, CardState::new());
        }
        BingoState {
            calls: Default::default(),
            card_states,
        }
    }

    fn update_card(&mut self, call: u64, coord: &'a CallCoord) -> Option<&'a Card> {
        if let Some(card_state) = self.card_states.get_mut(coord.card) {
            card_state.update(call, coord)
        } else {
            None
        }
    }

    fn update(&mut self, bingo: &'a Bingo, call: u64) -> Vec<&'a Card> {
        if let Some(last_call) = self.calls.last() {
            if *last_call != call {
                self.calls.push(call);
            } else {
                println!("bingo state update: repeating call {call}");
            }
        } else {
            self.calls.push(call);
        }
        if let Some(coords) = bingo.call_map.get(&call) {
            coords
                .iter()
                .filter_map(|coord| self.update_card(call, coord))
                .collect()
        } else {
            Vec::default()
        }
    }

    fn sum(&self, card: &Card) -> u64 {
        let mut sum = 0;
        for row in 0..5 {
            for col in 0..5 {
                let val = card.0[row][col];
                if !self.calls.contains(&val) {
                    sum += val;
                }
            }
        }
        sum
    }

    fn answer(&self, card: &Card) -> u64 {
        if let Some(val) = self.calls.last() {
            let sum = self.sum(card);
            let ans = sum * val;
            println!("Answer = {sum} * {val} = {ans}");
            ans
        } else {
            0
        }
    }
}

#[derive(Debug)]
struct CallCoord<'a> {
    card: &'a Card,
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Bingo<'a> {
    calls: Vec<u64>,
    cards: Vec<Card>,
    call_map: CallMap<'a>,
}

impl<'a> FromStr for Bingo<'a> {
    type Err = BingoParseError;

    fn from_str(bingo_str: &str) -> Result<Self, Self::Err> {
        let mut lines = bingo_str.lines().filter(|line| !line.is_empty());

        let call_str = lines.next().ok_or(BingoParseError::Fail)?;
        let calls = call_parser(call_str)
            .unwrap()
            .1
            .iter()
            .flat_map(|s| as_u64(s))
            .collect::<Vec<u64>>();

        let card_vals = lines
            .flat_map(|line| row_parser(line))
            .map(|row| {
                row.1
                    .into_iter()
                    .flat_map(|s| as_u64(s))
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>();

        let cards = card_vals
            .as_chunks::<5>()
            .0
            .iter()
            .map(|card_lines| Card::new(card_lines))
            .collect::<Vec<Card>>();

        let bingo = Bingo::new(calls, cards);
        Ok(bingo)
    }
}

impl<'a> Bingo<'a> {
    fn new(calls: Vec<u64>, cards: Vec<Card>) -> Bingo<'a> {
        let mut bingo = Bingo {
            calls,
            cards,
            call_map: CallMap::default(),
        };
        bingo.call_map();
        println!("Bingo calls: {:?}", bingo.calls);
        bingo
    }

    fn call_map(&mut self) {
        self.cards.iter().for_each(|card| {
            for row in 0..5 {
                for col in 0..5 {
                    let val = card.0[row][col];
                    if let Some(l) = self.call_map.get_mut(&val) {
                        l.push(CallCoord { card, row, col });
                    } else {
                        self.call_map
                            .insert(val, Vec::from([CallCoord { card, row, col }]));
                    }
                }
            }
        });
    }

    fn play(&self) -> u64 {
        let mut state = BingoState::new(self);
        if let Some(card) = self.calls.iter().find_map(|call| {
            let v = state.update(self, *call);
            if v.is_empty() {
                None
            } else {
                Some(v[0])
            }
        }) {
            println!("Bingo Result: {:?}", card);
            state.answer(card)
        } else {
            0
        }
    }

    fn find_last(&self) -> u64 {
        let mut remaining_cards = self.cards.iter().map(|c| c).collect::<HashSet<_>>();
        let mut state = BingoState::new(self);
        let mut calls = self.calls.iter();
        while let Some(call) = calls.next() {
            let v = state.update(self, *call);
            for card in v {
                if remaining_cards.len() == 1 {
                    println!("Last Bingo Result: {:?}", card);
                    println!("Last Bingo Call: {}", *call);
                    return state.answer(card);
                }
                remaining_cards.remove(card);
            }
        }
        0
    }
}

fn solve_part1(bingo: &Bingo) -> u64 {
    let winner = bingo.play();
    println!("Winner: {winner}");
    winner
}

fn solve_part2(bingo: &Bingo) -> u64 {
    let winner = bingo.find_last();
    println!("Last Winner: {winner}");
    winner
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let bingo = Bingo::from_str(INPUT).expect("Valid parse string");
    let part1 = solve_part1(&bingo);
    println!("Part1: {part1}");
    let part2 = solve_part2(&bingo);
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let bingo = Bingo::from_str(TEST_INPUT).expect("Valid parse string");
        assert_eq!(solve_part1(&bingo), 4512);
    }

    #[test]
    fn test_part2() {
        let bingo = Bingo::from_str(TEST_INPUT).expect("Valid parse string");
        assert_eq!(solve_part2(&bingo), 1924);
    }
}
