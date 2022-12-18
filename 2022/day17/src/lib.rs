use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fmt,
    fmt::Display,
    hash::{Hash, Hasher},
};

use itertools::Itertools;

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

type Pos = (u64, u64);

struct Chamber {
    rows: Vec<u8>,
    rock_seq: usize,
    rock: Rock,
}

impl Chamber {
    fn new() -> Chamber {
        Chamber {
            rows: Default::default(),
            rock_seq: 1,
            rock: Rock::spawn(0, ROCK_ORDER[0]),
        }
    }

    fn spawn_rock(&mut self) -> Rock {
        let rock = Rock::spawn(self.height(), ROCK_ORDER[self.rock_seq]);
        self.rock_seq = (self.rock_seq + 1) % ROCK_ORDER.len();
        rock
    }

    fn overlaps(&self, new_pos: &Pos) -> bool {
        (0..self.rock.height()).any(|o| {
            let rock_row = self.rock.row_from(o, new_pos);
            let chamber_row = self.row(new_pos.1 + o);
            (chamber_row & rock_row) != 0
        })
    }

    fn blow(&mut self, jet: Jet) -> Option<Pos> {
        let new_pos = match jet {
            Jet::Left => (self.rock.pos.0.checked_sub(1)?, self.rock.pos.1),
            Jet::Right => (self.rock.pos.0 + 1, self.rock.pos.1),
        };
        if self.overlaps(&new_pos) {
            None
        } else {
            self.rock.pos = new_pos;
            Some(new_pos)
        }
    }

    fn height(&self) -> u64 {
        self.rows.len() as u64
    }

    fn row(&self, y: u64) -> u8 {
        const WALL: u8 = 0b10000000;
        let r = if y < self.height() {
            self.rows[y as usize]
        } else {
            0
        };
        r | WALL
    }

    // Fall, maybe returning a rest position
    fn fall(&mut self) -> Option<Pos> {
        if self.rock.pos.1 == 0 || self.overlaps(&(self.rock.pos.0, self.rock.pos.1 - 1)) {
            Some(self.rock.pos)
        } else {
            self.rock.pos.1 -= 1;
            None
        }
    }

    fn embed(&mut self) {
        for i in 0..self.rock.height() {
            let y = self.rock.pos.1 + i;
            let rock_row = self.rock.row(i);
            if y >= self.height() {
                self.rows.push(rock_row);
            } else {
                self.rows[y as usize] |= rock_row;
            }
        }
        self.rock = self.spawn_rock();
    }
}
struct Rock {
    shape: &'static [u8],
    pos: Pos, // bottom-left
}

impl Display for Rock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, ".......")?;
        for i in 0..self.height() {
            let r = self.row(self.height() - i - 1);
            for i in 0..7 {
                let c = if (r & (1 << i)) == 0 { '.' } else { '@' };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        writeln!(f, ".......")
    }
}
impl Rock {
    fn spawn(height: u64, shape: &'static [u8]) -> Rock {
        Rock {
            shape,
            pos: (2, 3u64 + height),
        }
    }

    fn height(&self) -> u64 {
        self.shape.len() as u64
    }

    fn row(&self, y: u64) -> u8 {
        if y < self.height() {
            self.shape[y as usize].rotate_left(self.pos.0 as u32)
        } else {
            0
        }
    }

    fn row_from(&self, y: u64, new_pos: &Pos) -> u8 {
        if y >= self.height() {
            0
        } else {
            self.shape[y as usize].rotate_left(new_pos.0 as u32)
        }
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

#[allow(dead_code)]
impl Display for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let maxy = (self.rock.pos.1 + self.rock.height() + 1).max(self.height() + 1);
        let miny = maxy.saturating_sub(20);
        let diff = maxy - miny;

        for dy in 0..=diff {
            let h = maxy - dy;
            let rock_row = self
                .rock
                .row(h.checked_sub(self.rock.pos.1).unwrap_or(u64::MAX));
            let chamber_row = self.row(h);
            write!(f, "{h:>3} |")?;
            for bit in 0..7 {
                let v = 1 << bit;
                let rr = (rock_row & v) != 0;
                let cr = (chamber_row & v) != 0;
                let c = if rr && cr {
                    'X'
                } else if rr {
                    '@'
                } else if cr {
                    '#'
                } else {
                    '.'
                };
                write!(f, "{c}")?;
            }
            writeln!(f, "|")?;
        }
        if miny == 0 {
            writeln!(f, "    +-------+")?;
        }
        writeln!(f)
    }
}

pub fn solve_part1(input_str: &str) -> String {
    const NUM_ROCKS: usize = 2022;
    let jets = parse_jetstreams(input_str);
    let mut chamber = Chamber::new();
    let mut jet_idx = 0;
    let mut count = 0;

    while count < NUM_ROCKS {
        let jet = jets[jet_idx];

        chamber.blow(jet);
        if chamber.fall().is_some() {
            chamber.embed();
            count += 1;
        }
        jet_idx = (jet_idx + 1) % jets.len();
    }

    chamber.height().to_string()
}
pub fn solve_part2(input_str: &str) -> String {
    const NUM_ROCKS: usize = 1000000000000;
    let jets = parse_jetstreams(input_str);
    let mut chamber = Chamber::new();
    let mut jet_idx = 0;
    let mut count = 0;
    let mut num_blocks = 0;
    let mut cache: HashMap<(usize, usize, u64), (usize, usize, usize)> = Default::default();
    let hashsize = 1000;
    let mut height: usize = 0;

    while num_blocks < NUM_ROCKS {
        let jet = jets[jet_idx];
        chamber.blow(jet);
        if chamber.fall().is_some() {
            let last_height = chamber.height() as usize;
            chamber.embed();
            count += 1;
            num_blocks += 1;
            height += (chamber.height() as usize) - last_height;
            if count >= hashsize && count == num_blocks {
                let mut s = DefaultHasher::new();
                chamber.rows[count - hashsize..count].hash(&mut s);
                let hash = (jet_idx, count % 5, s.finish());

                if cache.contains_key(&hash) {
                    let (lcount, lheight, lhits) = cache[&hash];
                    let cperiod = count - lcount;
                    let hits = lhits + 1;
                    let dh = height - lheight;
                    cache.insert(hash, (count, height, hits));
                    println!("Found match: #={hash:?}, hits={hits}, cnt={count}/{lcount} hgt={height}/{lheight}=> cperiod={cperiod}, dh={dh}, hashmapsize={}", cache.len());
                    let start = NUM_ROCKS - count;
                    let future_periods = start / cperiod;
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
        let mut chamber = Chamber::new();

        chamber.blow(Jet::Left);
        println!("{}", chamber.rock);
        assert_eq!(chamber.rock.pos.0, 1);
        chamber.blow(Jet::Left);
        println!("{}", chamber.rock);
        assert_eq!(chamber.rock.pos.0, 0);
        chamber.blow(Jet::Left);
        println!("{}", chamber.rock);
        assert_eq!(chamber.rock.pos.0, 0);
        chamber.blow(Jet::Right);
        println!("{}", chamber.rock);
        assert_eq!(chamber.rock.pos.0, 1);
        chamber.blow(Jet::Right);
        println!("{}", chamber.rock);
        assert_eq!(chamber.rock.pos.0, 2);
        chamber.blow(Jet::Right);
        println!("{}", chamber.rock);
        assert_eq!(chamber.rock.pos.0, 3);
        chamber.blow(Jet::Right);
        println!("{}", chamber.rock);
        assert_eq!(chamber.rock.pos.0, 3);
    }
    #[test]
    fn test_parse_jetstreams() {
        assert_eq!(
            parse_jetstreams(">><\n<>"),
            vec![Jet::Right, Jet::Right, Jet::Left, Jet::Left, Jet::Right]
        )
    }
}
