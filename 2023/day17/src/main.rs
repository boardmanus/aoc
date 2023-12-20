use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

struct Grid {
    grid: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![0; width * height],
            width,
            height,
        }
    }

    fn get(&self, pos: &Pos) -> u8 {
        self.grid[(pos.y * self.width as i32 + pos.x) as usize]
    }

    fn in_bounds(&self, pos: &Pos) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        let mut grid = Self::new(width, height);
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid.grid[y * width + x] = c as u8 - b'0';
            }
        }
        Ok(grid)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    const ALL: [Self; 4] = [Self::N, Self::E, Self::S, Self::W];

    fn opp(&self) -> Dir {
        match self {
            Self::N => Self::S,
            Self::E => Self::W,
            Self::S => Self::N,
            Self::W => Self::E,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn stepx(&self, dir: Dir, x: i32) -> Pos {
        let mut pos = self.clone();
        match dir {
            Dir::N => pos.y -= x,
            Dir::E => pos.x += x,
            Dir::S => pos.y += x,
            Dir::W => pos.x -= x,
        }
        pos
    }
    fn step(&self, dir: Dir) -> Pos {
        self.stepx(dir, 1)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Move {
    pos: Pos,
    dir: Dir,
    hdist: u8,
    heat_loss: usize,
}

impl Move {
    fn new(pos: Pos, dir: Dir, hdist: u8, heat_loss: usize) -> Self {
        Self {
            pos,
            dir,
            hdist,
            heat_loss,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Visit {
    pos: Pos,
    hdist: u8,
    dir: Dir,
}

impl Visit {
    fn new(pos: Pos, hdist: u8, dir: Dir) -> Self {
        Self { pos, hdist, dir }
    }
}
/*
fn print_visited(visited: &HashMap<Visit, usize>, width: usize, height: usize) {
    for h in 0..4 {
        println!("H-dist={h}");
        for y in 0..height {
            for x in 0..width {
                let pos = Pos::new(x as i32, y as i32);
                let visit = Visit::new(pos, h, );
                if let Some(v) = visited.get(&visit) {
                    print!("[{:3}]", v);
                } else {
                    print!(".....");
                }
            }
            println!();
        }
    }
}
*/
fn ultra_crucible_can_turn(hlmove: &Move, dir: Dir) -> bool {
    (dir != hlmove.dir.opp() && dir != hlmove.dir && hlmove.hdist >= 4)
        || (dir == hlmove.dir && hlmove.hdist < 10)
}

fn solve_part1(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    let end_pos = Pos::new(grid.width as i32 - 1, grid.height as i32 - 1);
    let mut visited: HashMap<Visit, usize> = HashMap::from([]);
    let mut queue = VecDeque::<Move>::from([
        Move::new(Pos::new(0, 1), Dir::S, 1, 0),
        Move::new(Pos::new(1, 0), Dir::E, 1, 0),
    ]);

    while let Some(hlmove) = queue.pop_front() {
        let heat_loss = hlmove.heat_loss + grid.get(&hlmove.pos) as usize;
        let visit = Visit::new(hlmove.pos, hlmove.hdist, hlmove.dir);
        if let Some(previous_visit) = visited.get_mut(&visit) {
            if heat_loss >= *previous_visit {
                // This is a worse path
                continue;
            }
            *previous_visit = heat_loss;
        } else {
            visited.insert(visit, heat_loss);
        }

        if hlmove.pos == end_pos {
            println!("Reached end with move {:?}", hlmove);
            continue;
        }

        let dirs = Dir::ALL.iter().filter(|d| {
            (**d != hlmove.dir.opp() && **d != hlmove.dir)
                || (**d == hlmove.dir && hlmove.hdist < 3)
        });
        dirs.map(|d| {
            let pos = hlmove.pos.step(*d);
            let hdist = if *d == hlmove.dir {
                hlmove.hdist + 1
            } else {
                1
            };
            Move::new(pos, *d, hdist, heat_loss)
        })
        .for_each(|m| {
            if grid.in_bounds(&m.pos) {
                queue.push_back(m);
            }
        });
    }

    //print_visited(&visited, grid.width, grid.height);

    (0..4)
        .map(|hdist| {
            Dir::ALL
                .iter()
                .filter_map(|dir| visited.get(&Visit::new(end_pos, hdist, *dir)))
                .map(|v| *v)
                .min()
                .unwrap_or(500000)
        })
        .min()
        .unwrap_or(6000000)
    //* visited.get(&Visit::new(end_pos)).unwrap()
}

fn solve_part2(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    let end_pos = Pos::new(grid.width as i32 - 1, grid.height as i32 - 1);
    let mut visited: HashMap<Visit, usize> = HashMap::from([]);
    let mut queue = VecDeque::<Move>::from([
        Move::new(Pos::new(0, 1), Dir::S, 1, 0),
        Move::new(Pos::new(1, 0), Dir::E, 1, 0),
    ]);

    while let Some(hlmove) = queue.pop_front() {
        let heat_loss = hlmove.heat_loss + grid.get(&hlmove.pos) as usize;
        let visit = Visit::new(hlmove.pos, hlmove.hdist, hlmove.dir);
        if let Some(previous_visit) = visited.get_mut(&visit) {
            if heat_loss >= *previous_visit {
                // This is a worse path
                continue;
            }
            *previous_visit = heat_loss;
        } else {
            visited.insert(visit, heat_loss);
        }

        if hlmove.pos == end_pos {
            continue;
        }

        Dir::ALL
            .iter()
            .filter(|d| ultra_crucible_can_turn(&hlmove, **d))
            .map(|d| {
                let pos = hlmove.pos.step(*d);
                let hdist = if *d == hlmove.dir {
                    hlmove.hdist + 1
                } else {
                    1
                };
                Move::new(pos, *d, hdist, heat_loss)
            })
            .for_each(|m| {
                if grid.in_bounds(&m.pos) {
                    if m.hdist < 4 && !grid.in_bounds(&m.pos.stepx(m.dir, 4 - m.hdist as i32)) {
                    } else {
                        queue.push_back(m);
                    }
                }
            });
    }

    //print_visited(&visited, grid.width, grid.height);

    (0..10)
        .map(|hdist| {
            Dir::ALL
                .iter()
                .filter_map(|dir| {
                    let v = visited.get(&Visit::new(end_pos, hdist, *dir));
                    println!("Checking {:?} {:?} => {:?}", hdist, dir, v);
                    v
                })
                .map(|v| *v)
                .min()
                .unwrap_or(500000)
        })
        .min()
        .unwrap_or(6000000)
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
    const TEST_INPUT_2: &str = include_str!("test_input2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 102);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 94);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 71);
    }

    #[test]
    fn test_ultra_crucible_turns() {
        let hlmove = Move::new(Pos::new(0, 0), Dir::E, 1, 0);
        assert_eq!(ultra_crucible_can_turn(&hlmove, Dir::N), false);
        assert_eq!(ultra_crucible_can_turn(&hlmove, Dir::E), true);
        assert_eq!(ultra_crucible_can_turn(&hlmove, Dir::S), false);
        assert_eq!(ultra_crucible_can_turn(&hlmove, Dir::W), false);

        let hlmove = Move::new(Pos::new(0, 0), Dir::E, 4, 0);
        assert_eq!(ultra_crucible_can_turn(&hlmove, Dir::N), true);
        assert_eq!(ultra_crucible_can_turn(&hlmove, Dir::E), true);
        assert_eq!(ultra_crucible_can_turn(&hlmove, Dir::S), true);
        assert_eq!(ultra_crucible_can_turn(&hlmove, Dir::W), false);

        let hlmove = Move::new(Pos::new(0, 0), Dir::E, 9, 0);
        assert_eq!(ultra_crucible_can_turn(&hlmove, Dir::N), true);
        assert_eq!(ultra_crucible_can_turn(&hlmove, Dir::E), true);
        assert_eq!(ultra_crucible_can_turn(&hlmove, Dir::S), true);
        assert_eq!(ultra_crucible_can_turn(&hlmove, Dir::W), false);

        let hlmove = Move::new(Pos::new(0, 0), Dir::E, 10, 0);
        assert_eq!(ultra_crucible_can_turn(&hlmove, Dir::N), true);
        assert_eq!(ultra_crucible_can_turn(&hlmove, Dir::E), false);
        assert_eq!(ultra_crucible_can_turn(&hlmove, Dir::S), true);
        assert_eq!(ultra_crucible_can_turn(&hlmove, Dir::W), false);
    }
}
