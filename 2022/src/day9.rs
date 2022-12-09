use std::collections::HashSet;

use crate::aoc::Aoc;

fn to_dir(mv: &str) -> (i32, i32) {
    match mv {
        "R" => (1, 0),
        "L" => (-1, 0),
        "U" => (0, -1),
        "D" => (0, 1),
        _ => {
            println!("Unexpected input {mv}");
            panic!()
        }
    }
}

type Pos = (i32, i32);
type DPos = (i32, i32);
type Moves = (DPos, usize);

#[derive(Debug)]
struct State<const N: usize> {
    pos: [Pos; N],
}

fn to_move(line: &String) -> Moves {
    let mut move_strs = line.split_whitespace();
    let mv_dir = to_dir(move_strs.next().unwrap());
    let num = move_strs.next().unwrap().parse::<usize>().unwrap();
    (mv_dir, num)
}

fn collect_moves(moves: Moves, dhead: &mut Vec<DPos>) {
    (0..moves.1).for_each(|_i| dhead.push(moves.0));
}

fn update_head(head: &Pos, dmove: &DPos) -> Pos {
    (head.0 + dmove.0, head.1 + dmove.1)
}

fn update_tail(head: &Pos, tail: &Pos) -> Pos {
    let d_pos = (head.0 - tail.0, head.1 - tail.1);
    let abs_d_pos = (d_pos.0.abs(), d_pos.1.abs());
    match abs_d_pos {
        (_, 2) => (tail.0 + d_pos.0, tail.1 + d_pos.1.signum()),
        (2, _) => (tail.0 + d_pos.0.signum(), tail.1 + d_pos.1),
        _ => *tail,
    }
}

fn update_state<const N: usize>(state: &State<N>, dpos: &DPos) -> State<N> {
    let mut new_state: State<N> = State::<N> { pos: [(0, 0); N] };
    new_state.pos[0] = update_head(&state.pos[0], dpos);
    (1..state.pos.len()).for_each(|i| {
        new_state.pos[i] = update_tail(&new_state.pos[i - 1], &state.pos[i]);
    });
    //println!("{:?} => {:?}", dpos, new_state);
    new_state
}

pub struct Day9_1;
impl Aoc for Day9_1 {
    fn day(&self) -> u32 {
        9
    }
    fn puzzle_name(&self) -> &str {
        "Rope Bridge"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        let mut dmoves: Vec<DPos> = Default::default();
        lines
            .iter()
            .for_each(|line| collect_moves(to_move(line), &mut dmoves));

        let mut tail_positions: HashSet<Pos> = Default::default();

        dmoves
            .iter()
            .fold(State::<2> { pos: [(0, 0); 2] }, |state, dpos| -> State<2> {
                let new_state = update_state(&state, dpos);
                tail_positions.insert(new_state.pos[1]);
                new_state
            });

        tail_positions.len().to_string()
    }
}

pub struct Day9_2;
impl Aoc for Day9_2 {
    fn day(&self) -> u32 {
        9
    }
    fn puzzle_name(&self) -> &str {
        "Rope Bridge 2"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        let mut dmoves: Vec<DPos> = Default::default();
        lines
            .iter()
            .for_each(|line| collect_moves(to_move(line), &mut dmoves));

        let mut tail_positions: HashSet<Pos> = Default::default();
        let mut new_vals = 0;
        let mut dups = 0;
        dmoves.iter().fold(
            State::<10> { pos: [(0, 0); 10] },
            |state, dpos| -> State<10> {
                let new_state = update_state(&state, &dpos);
                let res = tail_positions.insert(new_state.pos[9]);
                if res {
                    new_vals += 1;
                } else {
                    dups += 1;
                }
                new_state
            },
        );
        println!(
            "new_vals={new_vals}, dups={dups}, len={}",
            tail_positions.len()
        );
        tail_positions.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 8] = ["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];
    const INPUT2: [&str; 8] = ["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"];

    #[test]
    fn test_soln() {
        let input_strs = INPUT
            .map(|s| String::from(s))
            .into_iter()
            .collect::<Vec<String>>();

        assert_eq!(Day9_1.solve(&input_strs), 13.to_string());
    }

    #[test]
    fn test_soln2() {
        let input_strs = INPUT2
            .map(|s| String::from(s))
            .into_iter()
            .collect::<Vec<String>>();

        assert_eq!(Day9_2.solve(&input_strs), 36.to_string());
    }
}
