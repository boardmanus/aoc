use std::{collections::HashMap, f32::consts::E, ops::Add, str::FromStr};

#[derive(Hash, Eq, PartialEq, Debug)]
enum Mirror {
    HPipe, // -
    VPipe, // |
    NE,    // \
    NW,    // /
}

impl Mirror {
    fn from_char(c: char) -> Option<Mirror> {
        match c {
            '-' => Some(Mirror::HPipe),
            '|' => Some(Mirror::VPipe),
            '\\' => Some(Mirror::NE),
            '/' => Some(Mirror::NW),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Mirror::HPipe => '-',
            Mirror::VPipe => '|',
            Mirror::NE => '\\',
            Mirror::NW => '/',
        }
    }

    fn exit_dir(&self, dir: Dir) -> (Option<Dir>, Option<Dir>) {
        match (self, dir) {
            (Mirror::HPipe, Dir::N) => (Some(Dir::W), Some(Dir::E)),
            (Mirror::HPipe, Dir::S) => (Some(Dir::E), Some(Dir::W)),
            (Mirror::HPipe, dir) => (Some(dir), None),
            (Mirror::VPipe, Dir::E) => (Some(Dir::S), Some(Dir::N)),
            (Mirror::VPipe, Dir::W) => (Some(Dir::N), Some(Dir::S)),
            (Mirror::VPipe, dir) => (Some(dir), None),
            (Mirror::NE, Dir::N) => (Some(Dir::W), None),
            (Mirror::NE, Dir::S) => (Some(Dir::E), None),
            (Mirror::NE, Dir::E) => (Some(Dir::S), None),
            (Mirror::NE, Dir::W) => (Some(Dir::N), None),
            (Mirror::NW, Dir::N) => (Some(Dir::E), None),
            (Mirror::NW, Dir::S) => (Some(Dir::W), None),
            (Mirror::NW, Dir::E) => (Some(Dir::N), None),
            (Mirror::NW, Dir::W) => (Some(Dir::S), None),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    const COUNT: usize = 4;

    fn v(&self) -> (i64, i64) {
        match self {
            Dir::N => (0, -1),
            Dir::S => (0, 1),
            Dir::E => (1, 0),
            Dir::W => (-1, 0),
        }
    }

    fn index(&self) -> usize {
        *self as usize
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
}

impl Add<Dir> for Point {
    type Output = Point;

    fn add(self, dir: Dir) -> Point {
        let v = dir.v();
        Point {
            x: self.x + v.0,
            y: self.y + v.1,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Photon {
    loc: Point,
    dir: Dir,
}

impl Photon {
    fn new(loc: Point, dir: Dir) -> Photon {
        Photon { loc, dir }
    }

    fn propagate(&self, dir: Option<Dir>) -> Option<Photon> {
        if let Some(dir) = dir {
            Some(Photon::new(self.loc + dir, dir))
        } else {
            None
        }
    }
    fn propagate_all(&self, mirror: Option<&Mirror>) -> (Option<Photon>, Option<Photon>) {
        let exits = if let Some(mirror) = mirror {
            mirror.exit_dir(self.dir)
        } else {
            (Some(self.dir), None)
        };
        (self.propagate(exits.0), self.propagate(exits.1))
    }
}

struct Map {
    mirrors: HashMap<Point, Mirror>,
    width: usize,
    height: usize,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mirrors = input
            .lines()
            .enumerate()
            .fold(HashMap::new(), |acc, (y, line)| {
                line.chars().enumerate().fold(acc, |mut acc, (x, c)| {
                    if let Some(mirror) = Mirror::from_char(c) {
                        acc.insert(Point::new(x as i64, y as i64), mirror);
                    }
                    acc
                })
            });
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        Ok(Map {
            mirrors,
            width,
            height,
        })
    }
}

impl ToString for Map {
    fn to_string(&self) -> String {
        let mut map_str = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point::new(x as i64, y as i64);
                let c = match self.mirrors.get(&point) {
                    Some(Mirror::HPipe) => '-',
                    Some(Mirror::VPipe) => '|',
                    Some(Mirror::NE) => '\\',
                    Some(Mirror::NW) => '/',
                    None => '.',
                };
                map_str.push(c);
            }
            map_str.push('\n');
        }
        map_str
    }
}

struct Tile {
    counts: [usize; Dir::COUNT],
}

impl Tile {
    fn new(photon: &Photon) -> Tile {
        let mut tile = Tile {
            counts: [0; Dir::COUNT],
        };
        tile.update(photon);
        tile
    }

    fn update(&mut self, photon: &Photon) {
        self.counts[photon.dir.index()] += 1;
    }

    fn to_char(&self) -> char {
        (self.counts.iter().filter(|c| **c > 0).count() as u8 + '0' as u8) as char
    }
}

struct State {
    width: usize,
    height: usize,
    photons: Vec<Photon>,
    tiles: HashMap<Point, Tile>,
}

impl State {
    fn new(width: usize, height: usize) -> State {
        State {
            width,
            height,
            photons: vec![],
            tiles: HashMap::from([]),
        }
    }

    fn update(&mut self, photon: Option<Photon>) {
        if let Some(photon) = photon {
            // Only add photons that are inbounds
            if photon.loc.x >= 0
                && photon.loc.y >= 0
                && photon.loc.x < self.width as i64
                && photon.loc.y < self.height as i64
            {
                if let Some(tile) = self.tiles.get_mut(&photon.loc) {
                    // Only add a new photon if this tile direction as not already been visited
                    if tile.counts[photon.dir.index()] == 0 {
                        self.photons.push(photon);
                    }
                    tile.update(&photon);
                } else {
                    self.tiles.insert(photon.loc, Tile::new(&photon));
                    self.photons.push(photon);
                }
            }
        }
    }

    fn energize(&mut self, start_photon: &Photon, map: &Map) -> usize {
        self.photons.push(*start_photon);
        self.tiles.insert(start_photon.loc, Tile::new(start_photon));

        while let Some(photon) = self.photons.pop() {
            let next_photons = photon.propagate_all(map.mirrors.get(&photon.loc));
            self.update(next_photons.0);
            self.update(next_photons.1);
        }
        self.tiles.len()
    }
}

impl ToString for State {
    fn to_string(&self) -> String {
        let mut map_str = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point::new(x as i64, y as i64);
                let c = if let Some(tile) = self.tiles.get(&point) {
                    tile.to_char()
                } else {
                    '.'
                };
                map_str.push(c);
            }
            map_str.push('\n');
        }
        map_str
    }
}

fn solve_part1(input: &str) -> usize {
    let map = Map::from_str(input).unwrap();
    let mut state = State::new(map.width, map.height);
    state.energize(&Photon::new(Point::new(0, 0), Dir::E), &map)
}

fn solve_part2(input: &str) -> usize {
    let map = Map::from_str(input).unwrap();

    let start_photons = (0..map.width).fold(vec![], |mut photons, x| {
        photons.push(Photon::new(Point::new(x as i64, 0), Dir::S));
        photons.push(Photon::new(
            Point::new(x as i64, map.height as i64 - 1),
            Dir::N,
        ));
        photons
    });
    let start_photons = (0..map.height).fold(start_photons, |mut photons, y| {
        photons.push(Photon::new(Point::new(0, y as i64), Dir::E));
        photons.push(Photon::new(
            Point::new(map.width as i64 - 1, y as i64),
            Dir::W,
        ));
        photons
    });

    start_photons
        .iter()
        .map(|photon| State::new(map.width, map.height).energize(photon, &map))
        .max()
        .unwrap()
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
        assert_eq!(solve_part1(TEST_INPUT), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 51);
    }

    #[test]
    fn test_parse() {
        let map = Map::from_str(TEST_INPUT).unwrap();
        assert_eq!(map.width, 10);
        assert_eq!(map.height, 10);
        assert_eq!(map.mirrors.len(), 23);
        assert_eq!(map.to_string(), TEST_INPUT);
    }
}
