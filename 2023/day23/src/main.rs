use core::time;
use std::{
    collections::HashMap,
    io::{stdout, Stdout, Write},
    ops::Add,
    str::FromStr,
    thread,
};

use crossterm::{
    cursor::{self, MoveDown, MoveRight, RestorePosition, SavePosition},
    execute,
    style::{self, Color, Print, ResetColor, SetForegroundColor},
    QueueableCommand,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Down,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Ramp(Dir),
}

impl Cell {
    fn from_char(c: char) -> Result<Self, ()> {
        match c {
            '.' => Ok(Cell::Empty),
            '#' => Ok(Cell::Wall),
            'v' => Ok(Cell::Ramp(Dir::Down)),
            '>' => Ok(Cell::Ramp(Dir::Right)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Pos {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Grid {
    width: usize,
    height: usize,
    data: Vec<Cell>,
}

impl Grid {
    fn new(width: usize, height: usize, data: Vec<Cell>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    fn get(&self, pos: &Pos) -> Option<Cell> {
        match pos {
            Pos { x, y } if *x < 0 || *y < 0 => None,
            Pos { x, y } if *x >= self.width as i32 || *y >= self.height as i32 => None,
            _ => Some(self.data[pos.y as usize * self.width + pos.x as usize]),
        }
    }

    fn neighbours(&self, pos: &Pos, ignore_slopes: bool) -> Vec<Pos> {
        const DIRECTIONS: [Pos; 4] = [
            Pos { x: 0, y: -1 },
            Pos { x: 0, y: 1 },
            Pos { x: -1, y: 0 },
            Pos { x: 1, y: 0 },
        ];
        let cell = self.get(pos);
        DIRECTIONS
            .iter()
            .filter(|d| match cell {
                Some(Cell::Wall) | Some(Cell::Empty) | None => true,
                Some(Cell::Ramp(Dir::Down)) => ignore_slopes || d.y == 1,
                Some(Cell::Ramp(Dir::Right)) => ignore_slopes || d.x == 1,
            })
            .map(|d| (d, *pos + *d))
            .filter(|&(d, p)| match self.get(&p) {
                Some(Cell::Empty) => true,
                Some(Cell::Wall) | None => false,
                Some(Cell::Ramp(Dir::Down)) => ignore_slopes || d.y != -1,
                Some(Cell::Ramp(Dir::Right)) => ignore_slopes || d.x != -1,
            })
            .map(|(_, p)| p)
            .collect()
    }

    fn longest_path(&self, start: &Pos, end: &Pos, delay: u64, ignore_slopes: bool) -> Vec<Pos> {
        // Depth first search
        let mut stdout = stdout();
        let mut visited: HashMap<Pos, Vec<Pos>> = HashMap::new();
        let mut queue = vec![vec![*start]];
        while let Some(path) = queue.pop() {
            if delay > 0 {
                self.display_path(&mut stdout, start, end, &path, delay);
            }
            let pos = path.last().unwrap();
            let mut last_len = 0;
            if let Some(longest_path) = visited.get_mut(pos) {
                if longest_path.len() >= path.len() {
                    continue;
                }
                last_len = longest_path.len();
                *longest_path = path.clone();
            } else {
                visited.insert(*pos, path.clone());
            }

            if pos == end {
                let pdist = last_len;
                let dist = path.len();
                execute!(
                    stdout,
                    SavePosition,
                    MoveRight(self.width as u16 + 1),
                    Print(format!("Update end dist: {} => {}", pdist, dist)),
                    RestorePosition,
                )
                .unwrap();
            } else {
                execute!(
                    stdout,
                    SavePosition,
                    MoveRight(self.width as u16 + 1),
                    MoveDown(1),
                    Print(format!("Current dist: {}", path.len())),
                    RestorePosition,
                )
                .unwrap();
            }

            // Add all the neighbours with updated distance
            queue.extend(
                self.neighbours(&pos, ignore_slopes)
                    .iter()
                    .filter(|&p| !path.contains(p))
                    .map(|p| {
                        let mut new_path = path.clone();
                        new_path.push(*p);
                        new_path
                    }),
            );
        }

        if let Some(path) = visited.get(end) {
            path.clone()
        } else {
            vec![]
        }
    }

    fn display_path(&self, stdout: &mut Stdout, start: &Pos, end: &Pos, path: &[Pos], delay: u64) {
        //let mut s = String::new();
        //s.reserve((self.width + 1) * self.height);
        stdout.queue(cursor::SavePosition).unwrap();
        stdout.queue(style::ResetColor).unwrap();
        let last_pos = *path.last().unwrap();
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Pos::new(x as i32, y as i32);
                let cell = self.get(&pos);
                let in_path = path.contains(&pos);
                if in_path {
                    stdout.queue(SetForegroundColor(Color::Green)).unwrap();
                }
                if pos == last_pos {
                    stdout.queue(SetForegroundColor(Color::Red)).unwrap();
                    stdout.queue(Print("X")).unwrap();
                    //s.push('X');
                } else if pos == *start {
                    stdout.queue(SetForegroundColor(Color::DarkGreen)).unwrap();
                    stdout.queue(Print("S")).unwrap();
                    //s.push('S');
                } else if pos == *end {
                    stdout.queue(SetForegroundColor(Color::DarkBlue)).unwrap();
                    stdout.queue(Print("E")).unwrap();
                    //s.push('E');
                } else if in_path {
                    let cell_str = match cell {
                        Some(Cell::Ramp(Dir::Down)) => "v",
                        Some(Cell::Ramp(Dir::Right)) => ">",
                        _ => "O",
                    };
                    stdout.queue(Print(cell_str)).unwrap();
                    stdout.queue(ResetColor).unwrap();
                    //s.push('O');
                } else {
                    match cell {
                        Some(Cell::Empty) => stdout.queue(Print(".")), //s.push('.'),
                        Some(Cell::Wall) => stdout.queue(Print("#")),  //s.push('#'),
                        Some(Cell::Ramp(Dir::Down)) => stdout.queue(Print("v")), //s.push('v'),
                        Some(Cell::Ramp(Dir::Right)) => stdout.queue(Print(">")), //s.push('>'),
                        _ => stdout.queue(Print("?")),                 //s.push('?'),
                    }
                    .unwrap();
                }
                stdout.queue(ResetColor).unwrap();
            }
            stdout.queue(Print("\n")).unwrap();
            //s.push('\n');
        }
        //stdout.write_all(s.as_bytes()).unwrap();
        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.flush().unwrap();
        thread::sleep(time::Duration::from_millis(delay));
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().ok_or(())?.len();
        let height = s.lines().count();
        let data = s
            .lines()
            .flat_map(|line| line.chars().map(|c| Cell::from_char(c)))
            .collect::<Result<_, _>>()?;
        Ok(Grid::new(width, height, data))
    }
}

fn solve_part1(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    let start = Pos::new(1, 0);
    let end = Pos::new(grid.width as i32 - 2, grid.height as i32 - 1);
    grid.longest_path(&start, &end, 0, false).len() - 1
}

fn solve_part2(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    let start = Pos::new(1, 0);
    let end = Pos::new(grid.width as i32 - 2, grid.height as i32 - 1);
    grid.longest_path(&start, &end, 0, true).len() - 1
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

    use crossterm::{
        cursor::{Hide, MoveDown, Show},
        ExecutableCommand,
    };

    use super::*;

    const INPUT: &str = include_str!("input.txt");
    const TEST_INPUT: &str = include_str!("test_input.txt");
    const TEST_INPUT_2: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 94);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 154);
    }

    #[test]
    fn test_grid_from_str() {
        assert!(Grid::from_str(TEST_INPUT).is_ok());
        assert!(Grid::from_str(INPUT).is_ok());
    }

    #[test]
    fn test_longest_path_with_slopes() {
        let grid = Grid::from_str(TEST_INPUT).unwrap();
        let mut stdout = stdout();
        execute!(stdout, Hide).unwrap();
        let start = Pos::new(1, 0);
        let end = Pos::new(grid.width as i32 - 2, grid.height as i32 - 1);
        let path = grid.longest_path(&start, &end, 10, false);
        grid.display_path(&mut stdout, &start, &end, &path, 1000);
        execute!(stdout, MoveDown(grid.height as u16), Show).unwrap();
    }

    #[test]
    fn test_longest_path_without_slopes() {
        let grid = Grid::from_str(TEST_INPUT).unwrap();
        let mut stdout = stdout();
        execute!(stdout, Hide).unwrap();
        let start = Pos::new(1, 0);
        let end = Pos::new(grid.width as i32 - 2, grid.height as i32 - 1);
        let path = grid.longest_path(&start, &end, 10, true);
        grid.display_path(&mut stdout, &start, &end, &path, 1000);
        execute!(stdout, MoveDown(grid.height as u16), Show).unwrap();
    }

    #[test]
    fn test_grid() {
        let grid = Grid::from_str(TEST_INPUT).unwrap();
        assert_eq!(grid.get(&Pos::new(0, 0)), Some(Cell::Wall));
        assert_eq!(grid.get(&Pos::new(20, 11)), Some(Cell::Ramp(Dir::Right)));
        assert_eq!(grid.get(&Pos::new(3, 6)), Some(Cell::Ramp(Dir::Down)));
        assert_eq!(grid.get(&Pos::new(21, 22)), Some(Cell::Empty));
        assert_eq!(grid.get(&Pos::new(0, -1)), None);
        assert_eq!(grid.get(&Pos::new(-1, (grid.width - 1) as i32)), None);
        assert_eq!(grid.get(&Pos::new(0, grid.width as i32)), None);
        assert_eq!(grid.get(&Pos::new(grid.width as i32, 0)), None);
    }

    #[test]
    fn test_neighbours() {
        let grid = Grid::from_str(TEST_INPUT).unwrap();
        assert_eq!(
            grid.neighbours(&Pos::new(1, 0), false),
            vec![Pos::new(1, 1)]
        );
        assert_eq!(
            grid.neighbours(&Pos::new(11, 19), false),
            vec![Pos::new(11, 20), Pos::new(12, 19)]
        );
        assert_eq!(
            grid.neighbours(&Pos::new(3, 5), false),
            vec![Pos::new(3, 6), Pos::new(4, 5)]
        );
        assert_eq!(
            grid.neighbours(&Pos::new(3, 5), true),
            vec![Pos::new(3, 4), Pos::new(3, 6), Pos::new(4, 5)]
        );

        assert_eq!(
            grid.neighbours(&Pos::new(3, 4), false),
            vec![Pos::new(3, 5)]
        );
        assert_eq!(
            grid.neighbours(&Pos::new(12, 13), false),
            vec![Pos::new(13, 13)]
        );
        // Can't step left onto >
        assert_eq!(
            grid.neighbours(&Pos::new(7, 13), false),
            vec![Pos::new(7, 14)]
        );
        assert_eq!(
            grid.neighbours(&Pos::new(7, 13), true),
            vec![Pos::new(7, 14), Pos::new(6, 13),]
        );
        assert_eq!(
            grid.neighbours(&Pos::new(11, 3), false),
            vec![Pos::new(11, 4), Pos::new(12, 3)]
        )
    }
}
