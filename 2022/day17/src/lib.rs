use std::{
    cmp::{Ordering, Reverse},
    collections::{hash_map::DefaultHasher, BinaryHeap, HashMap, HashSet, VecDeque},
    fmt,
    fmt::Display,
    hash::{Hash, Hasher},
    str::FromStr,
};

use itertools::Itertools;
use nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{alpha0, digit1},
        is_alphabetic,
    },
    combinator::map_res,
    multi::separated_list0,
    sequence::{delimited, pair, preceded, separated_pair},
    IResult,
};
use pathfinding::directed::bfs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Jet {
    Left,
    Right,
}

// Note: bits arranged so lowest index is bottom, left
const ROCK_MINUS: [u8; 1] = [0b1111];
const ROCK_PLUS: [u8; 3] = [0b010, 0b111, 0b010];
const ROCK_L: [u8; 3] = [0b111, 0b100, 0b100];
const ROCK_PIPE: [u8; 4] = [0b1, 0b1, 0b1, 0b1];
const ROCK_SQUARE: [u8; 2] = [0b11, 0b11];

const ROCK_ORDER: [&[u8]; 5] = [&ROCK_MINUS, &ROCK_PLUS, &ROCK_L, &ROCK_PIPE, &ROCK_SQUARE];

type Pos = (i64, i64);

struct Chamber {
    width: usize,
    //rows: VecDeque<u8>,
    rows: Vec<u8>,
    rock_seq: usize,
}

impl Chamber {
    fn new(width: usize) -> Chamber {
        let v = Vec::<u8>::with_capacity(100000000);
        Chamber {
            width,
            rows: v,
            rock_seq: 0,
        }
    }

    fn spawn_rock(&mut self) -> Rock {
        let rock = Rock::spawn(self.height(), ROCK_ORDER[self.rock_seq]);
        self.rock_seq = (self.rock_seq + 1) % ROCK_ORDER.len();
        rock
    }

    fn height(&self) -> i64 {
        self.rows.len() as i64
    }

    fn maybe_row(&self, y: i64) -> Option<u8> {
        if y < 0 || y >= self.height() {
            None
        } else {
            Some(self.rows[y as usize])
        }
    }

    fn row(&self, offset: usize) -> Option<u8> {
        if offset < self.rows.len() {
            Some(self.rows[self.rows.len() - offset - 1])
        } else {
            None
        }
    }

    fn embed(&mut self, rock: &Rock) {
        for i in 0..rock.height() {
            let y = rock.pos.1 + i;
            let rock_row = rock.row(i as usize);
            if y >= self.height() {
                self.rows.push(rock_row);
                //self.rows.push_back(rock_row);
            } else {
                self.rows[y as usize] = self.rows[y as usize] | rock_row;
            }
        }
    }
}
struct Rock {
    shape: &'static [u8],
    pos: Pos, // bottom-left
}

impl Display for Rock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, ".......");
        for i in 0..self.height() {
            let r = self.row((self.height() - i - 1) as usize);
            for i in 0..7 {
                let c = if (r & (1 << i)) == 0 { '.' } else { '@' };
                write!(f, "{c}");
            }
            writeln!(f, "");
        }
        writeln!(f, ".......")
    }
}
impl Rock {
    fn spawn(height: i64, shape: &'static [u8]) -> Rock {
        Rock {
            shape,
            pos: (2, 3 + height),
        }
    }

    fn height(&self) -> i64 {
        self.shape.len() as i64
    }

    fn row(&self, y: usize) -> u8 {
        self.shape[y as usize] << self.pos.0
    }

    fn row_from(&self, y: i64, new_pos: &Pos) -> u8 {
        if y < 0 || y >= self.height() {
            0
        } else {
            self.shape[y as usize] << new_pos.0
        }
    }

    fn maybe_row(&self, y: i64) -> Option<u8> {
        if y < 0 || y >= self.height() {
            None
        } else {
            Some(self.shape[y as usize] << self.pos.0)
        }
    }

    fn blow(&mut self, jet: Jet, chamber: &Chamber) {
        let offset: i64 = match jet {
            Jet::Left => -1,
            Jet::Right => 1,
        };
        let mask = match jet {
            Jet::Left => 1 << 0,
            Jet::Right => 1 << (chamber.width - 1),
        };
        let new_pos = (self.pos.0 + offset, self.pos.1);
        for i in 0..self.shape.len() {
            if self.row(i) & mask != 0 {
                return;
            }
        }
        if self.overlaps(&new_pos, chamber) {
            return;
        }
        self.pos = new_pos;
        /*
        if (0..self.shape.len()).all(|i| (self.row(i) & mask) == 0)
            && !self.overlaps(&new_pos, chamber)
        {
            self.pos = new_pos;
        }
        */
    }

    fn overlaps(&self, new_pos: &Pos, chamber: &Chamber) -> bool {
        for o in 0..self.shape.len() as i64 {
            let rock_row = self.row_from(o, new_pos);
            if let Some(chamber_row) = chamber.maybe_row(new_pos.1 + o) {
                if (rock_row & chamber_row) != 0 {
                    return true;
                }
            } else {
                return false;
            }
        }
        false
        /*
        (0..self.shape.len() as i64).any(|o| {
            let rock_row = self.row_from(o, new_pos);
            let chamber_row = chamber.maybe_row(new_pos.1 + o).unwrap_or(0);
            (rock_row & chamber_row) != 0
        })
        */
    }
    // Fall, maybe returning a rest position
    fn fall(&mut self, chamber: &Chamber) -> Option<Pos> {
        if self.pos.1 == 0 {
            return Some(self.pos);
        }

        if self.overlaps(&(self.pos.0, self.pos.1 - 1), chamber) {
            return Some(self.pos);
        }

        self.pos.1 -= 1;
        None
    }
}

fn parse_jetstreams(input: &str) -> Vec<Jet> {
    input
        .chars()
        .flat_map(|c| match c {
            '<' => Some(Jet::Left),
            '>' => Some(Jet::Right),
            _ => None,
        })
        .collect_vec()
}

fn print_chamber_and_rock(chamber: &Chamber, rock: &Rock, max: i64) {
    let maxy = (rock.pos.1 + rock.height() + 1).max(chamber.height() + 1);
    let miny = 0.max(maxy - max);
    let diff = maxy - miny;

    for dy in 0..=diff {
        let h = maxy - dy;
        let rock_row = rock.maybe_row(h - rock.pos.1).unwrap_or(0);
        let chamber_row = chamber.maybe_row(h).unwrap_or(0);
        print!("{h:>3} |");
        for bit in 0..7 {
            let v = 1 << bit;
            let rr = (rock_row & v) != 0;
            let cr = (chamber_row & v) != 0;
            let c = if rr && cr {
                'X'
            } else if (rr) {
                '@'
            } else if (cr) {
                '#'
            } else {
                '.'
            };
            print!("{c}");
        }
        println!("|");
    }
    if miny == 0 {
        println!("    +-------+");
    } else {
        println!("");
    }
}

pub fn solve_part1(input_str: &str) -> String {
    const NUM_ROCKS: usize = 2022;
    const WIDTH: usize = 7;
    let jets = parse_jetstreams(input_str);
    let mut chamber = Chamber::new(WIDTH);
    let mut curr_rock = chamber.spawn_rock();
    let mut jet_idx = 0;
    let mut count = 0;

    while count < NUM_ROCKS {
        //let rock_count = jets.iter().fold(0, |count, jet| {
        let jet = jets[jet_idx];
        jet_idx = (jet_idx + 1) % jets.len();

        curr_rock.blow(jet, &chamber);
        if let Some(stuck_pos) = curr_rock.fall(&chamber) {
            chamber.embed(&curr_rock);
            curr_rock = chamber.spawn_rock();
            count += 1;
        }
    }
    //print_chamber_and_rock(&chamber, &curr_rock, 1000);
    println!(
        "possible repition = {}*{}={}",
        jets.len(),
        ROCK_ORDER.len(),
        jets.len() * ROCK_ORDER.len()
    );
    chamber.height().to_string()
}
pub fn solve_part2(input_str: &str) -> String {
    const NUM_ROCKS: usize = 1000000000000;
    //const NUM_ROCKS: usize = 1000000;
    const WIDTH: usize = 7;
    let jets = parse_jetstreams(input_str);
    let mut chamber = Chamber::new(WIDTH);
    let mut curr_rock = chamber.spawn_rock();
    let mut jet_idx = 0;
    let mut count = 0;
    let mut num_blocks = 0;
    let rep = jets.len() * ROCK_ORDER.len() * 7;
    let mut cache: HashMap<(usize, usize, u64), (usize, usize, usize)> = Default::default();
    let mut cperiod: usize = 0;
    let hashsize = 10000;
    let mut last_count = 0;
    let mut gross_period = 57; //jets.len();
    let mut height: usize = 0;

    while num_blocks < NUM_ROCKS {
        //let rock_count = jets.iter().fold(0, |count, jet| {
        let jet = jets[jet_idx];

        curr_rock.blow(jet, &chamber);
        if let Some(stuck_pos) = curr_rock.fall(&chamber) {
            let last_height = chamber.height() as usize;
            chamber.embed(&curr_rock);
            curr_rock = chamber.spawn_rock();
            count += 1;
            num_blocks += 1;
            height += (chamber.height() as usize) - last_height;
            if count >= hashsize && count == num_blocks {
                let mut s = DefaultHasher::new();
                chamber.rows[count - hashsize..count].hash(&mut s);
                let hash = (jet_idx, count % 5, s.finish());

                if cache.contains_key(&hash) {
                    let (lcount, lheight, lhits) = cache[&hash];
                    last_count = count;
                    cperiod = count - lcount;
                    let hits = lhits + 1;
                    let dh = height - lheight;
                    cache.insert(hash, (count, height, hits));
                    let max_period = cache.iter().filter(|x| x.1 .2 < 80).map(|x| x.1);
                    println!("Found match: #={hash:?}, hits={hits}, cnt={count}/{lcount} hgt={height}/{lheight}=> cperiod={cperiod}, dh={dh}, hashmapsize={}", cache.len());
                    let start = NUM_ROCKS - count;
                    let future_periods = start / cperiod;
                    let last_seg = start % cperiod;
                    height += future_periods * dh;
                    num_blocks += future_periods * cperiod;
                    println!("Skip a few: num_blocks=>{num_blocks}, height=>{height}");
                } else {
                    cache.insert(hash, (count, chamber.height() as usize, 1));
                }
            }
        }
        jet_idx = (jet_idx + 1) % jets.len();
    }
    height.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT), 3068.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT), 1514285714288u64.to_string());
    }

    #[test]
    fn test_rock() {
        let chamber = Chamber::new(7);
        assert_eq!(0b1100, 12);
        assert_eq!(0b0011 << 2, 12);
        assert_eq!(Rock::spawn(10, &ROCK_MINUS).row(0), 0b111100);

        let mut rock = Rock::spawn(10, &ROCK_MINUS);
        assert_eq!(rock.pos.0, 2);
        println!("{rock}");
        rock.blow(Jet::Left, &chamber);
        println!("{rock}");
        assert_eq!(rock.pos.0, 1);
        rock.blow(Jet::Left, &chamber);
        println!("{rock}");
        assert_eq!(rock.pos.0, 0);
        rock.blow(Jet::Left, &chamber);
        println!("{rock}");
        assert_eq!(rock.pos.0, 0);
        rock.blow(Jet::Right, &chamber);
        println!("{rock}");
        assert_eq!(rock.pos.0, 1);
        rock.blow(Jet::Right, &chamber);
        println!("{rock}");
        assert_eq!(rock.pos.0, 2);
        rock.blow(Jet::Right, &chamber);
        println!("{rock}");
        assert_eq!(rock.pos.0, 3);
        rock.blow(Jet::Right, &chamber);
        println!("{rock}");
        assert_eq!(rock.pos.0, 3);
    }
    #[test]
    fn test_parse_jetstreams() {
        assert_eq!(
            parse_jetstreams(">><\n<>"),
            vec![Jet::Right, Jet::Right, Jet::Left, Jet::Left, Jet::Right]
        )
    }
}
