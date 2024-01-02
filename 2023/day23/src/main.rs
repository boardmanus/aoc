use core::time;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{stdout, Stdout, Write},
    ops::Add,
    str::FromStr,
    thread,
};

use crossterm::{
    cursor::{self, MoveDown, MoveRight, MoveToColumn, RestorePosition, SavePosition},
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
const DIRECTIONS: [Pos; 4] = [
    Pos { x: 0, y: -1 },
    Pos { x: 0, y: 1 },
    Pos { x: -1, y: 0 },
    Pos { x: 1, y: 0 },
];

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

    fn neighbours2<'a>(
        &'a self,
        pos: Pos,
        seen: &'a HashSet<Pos>,
    ) -> impl Iterator<Item = Pos> + 'a {
        DIRECTIONS.iter().filter_map(move |&dir| {
            let new_pos = pos + dir;
            if seen.contains(&new_pos) {
                None
            } else {
                match self.get(&new_pos) {
                    Some(Cell::Wall) | None => None,
                    _ => Some(new_pos),
                }
            }
        })
    }

    fn neighbours3(&self, pos: Pos, prev_pos: Option<Pos>) -> impl Iterator<Item = Pos> + '_ {
        DIRECTIONS.iter().filter_map(move |&dir| {
            let new_pos = pos + dir;
            if prev_pos != None && Some(new_pos) == prev_pos {
                None
            } else {
                match self.get(&new_pos) {
                    Some(Cell::Wall) | None => None,
                    _ => Some(new_pos),
                }
            }
        })
    }

    fn longest_path(
        &self,
        start: &Pos,
        end: &Pos,
        delay: u64,
        ignore_slopes: bool,
    ) -> HashSet<Pos> {
        // Depth first search
        let mut stdout = stdout();
        let mut end_path = HashSet::new();
        let mut queue = vec![(*start, HashSet::from([*start]))];
        while let Some((mut pos, mut path)) = queue.pop() {
            if delay > 0 {
                self.display_path(start, end, &pos, &path, delay);
            }

            if pos == *end {
                let longest_dist = end_path.len();
                let dist = path.len();
                if longest_dist < dist {
                    end_path = path.clone();
                }
                execute!(
                    stdout,
                    SavePosition,
                    MoveRight(self.width as u16 + 1),
                    Print(format!(
                        "Update end dist: {:6} => {:6}",
                        dist,
                        end_path.len()
                    )),
                    RestorePosition,
                )
                .unwrap();
            } else {
                if delay > 0 {
                    execute!(
                        stdout,
                        SavePosition,
                        MoveToColumn(self.width as u16 + 1),
                        MoveDown(1),
                        Print(format!("Current dist: {:6}", path.len())),
                        MoveToColumn(self.width as u16 + 1),
                        MoveDown(1),
                        Print(format!("Queue length: {:6}", queue.len())),
                        RestorePosition,
                    )
                    .unwrap();
                }
            }

            // Add all the neighbours with updated distance
            loop {
                let neighbours = self.neighbours(&pos, ignore_slopes);

                if neighbours.len() == 1 {
                    // If there are no alternate paths, just iterate through the positions.
                    let new_pos = neighbours[0];
                    path.insert(new_pos);
                    pos = new_pos;
                } else {
                    queue.extend(neighbours.iter().filter(|&p| !path.contains(p)).map(|p| {
                        let mut new_path = path.clone();
                        new_path.insert(*p);
                        (*p, new_path)
                    }));
                    break;
                }
            }
        }

        end_path
    }

    fn dfs(&self, pos: Pos, goal: Pos, seen: &mut HashSet<Pos>) -> Option<usize> {
        // Skip over all positions with no diversions
        let mut edge_len = 0;
        let mut edge_pos = pos;
        let mut edge = vec![];
        let mut neighbours: Vec<Pos>;
        loop {
            if edge_pos == goal {
                return Some(edge_len);
            }
            edge.push(edge_pos);
            seen.insert(edge_pos);
            edge_len += 1;
            neighbours = self.neighbours2(edge_pos, &seen).collect();
            if neighbours.len() == 1 {
                edge_pos = neighbours[0];
            } else {
                break;
            }
        }

        let longest_path = neighbours
            .iter()
            .filter_map(|&np| self.dfs(np, goal, seen).map(|ans| ans + edge_len))
            .max();

        // Remove all positions in the edge.
        edge.iter().for_each(|edge_pos| _ = seen.remove(edge_pos));

        longest_path
    }

    fn graph(&self, start: &Pos) -> HashMap<Pos, HashSet<(Pos, usize)>> {
        let mut ret: HashMap<Pos, HashSet<(Pos, usize)>> = HashMap::new();

        // cur point, from node, edge length, previous point
        let mut the_stack: Vec<(Pos, Pos, usize, Option<Pos>)> =
            vec![(start.clone(), *start, 0, None)];
        while !the_stack.is_empty() {
            let (cur, from, edge_len, prev) = the_stack.pop().unwrap();

            let neighs = self.neighbours3(cur, prev).collect::<Vec<_>>();
            if neighs.len() == 1 {
                the_stack.push((
                    neighs.first().unwrap().clone(),
                    from,
                    edge_len + 1,
                    Some(cur),
                ));
            } else {
                if ret.contains_key(&cur) && ret[&cur].iter().any(|e| e.0 == from) {
                    continue;
                }
                let mut set2 = HashSet::new();
                set2.insert((from, edge_len));
                ret.entry(cur.clone())
                    .and_modify(|v: &mut HashSet<(Pos, usize)>| {
                        v.insert((from, edge_len));
                    })
                    .or_insert(set2);
                for neigh in &neighs {
                    the_stack.push((neigh.clone(), cur, 1, Some(cur)));
                }
            }
        }
        ret
    }
    fn find_path_len(
        &self,
        start: &Pos,
        dest: &Pos,
        graph: &HashMap<Pos, HashSet<(Pos, usize)>>,
    ) -> usize {
        let mut the_stack = vec![(vec![*dest], 0)];
        let mut memory: HashMap<Pos, (Vec<Pos>, usize)> = HashMap::new();
        while let Some(cur) = the_stack.pop() {
            let head = *cur.0.last().unwrap();
            // Start node won't have an entry
            if !graph.contains_key(&head) {
                continue;
            }
            let entry = &graph[&head];
            // println!("Now at {:?}", cur);

            for edge in entry {
                if cur.0.contains(&edge.0) {
                    // println!("Skipping {:?} because it's already in the path", edge.0);
                    continue;
                }
                let mut tmp = cur.clone();
                tmp.0.push(edge.0);
                tmp.1 += edge.1;
                if !memory.contains_key(&edge.0) || tmp.1 > memory.get(&edge.0).unwrap().1 {
                    // if memory.contains_key(&edge.0) {
                    //     println!("Replacing {:?} with {:?}", memory.get(&edge.0).unwrap(), tmp);
                    // }
                    memory.insert(edge.0, tmp.clone());
                }
                the_stack.push(tmp);
            }
            // println!("-------------------");
        }
        memory[start].1
    }

    fn longest_path2(&self, start: &Pos, end: &Pos) -> usize {
        let graph = self.graph(start);
        self.find_path_len(start, end, &graph)
        //self.dfs(*start, *end, &mut HashSet::new()).unwrap()
    }

    fn display_path(
        &self,
        start: &Pos,
        end: &Pos,
        last_pos: &Pos,
        path: &HashSet<Pos>,
        delay: u64,
    ) {
        let mut stdout = stdout();
        stdout.queue(cursor::SavePosition).unwrap();
        stdout.queue(style::ResetColor).unwrap();
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Pos::new(x as i32, y as i32);
                let cell = self.get(&pos);
                let in_path = path.contains(&pos);
                if in_path {
                    stdout.queue(SetForegroundColor(Color::Green)).unwrap();
                }
                if pos == *last_pos {
                    stdout.queue(SetForegroundColor(Color::Red)).unwrap();
                    stdout.queue(Print("X")).unwrap();
                } else if pos == *start {
                    stdout.queue(SetForegroundColor(Color::DarkGreen)).unwrap();
                    stdout.queue(Print("S")).unwrap();
                } else if pos == *end {
                    stdout.queue(SetForegroundColor(Color::DarkBlue)).unwrap();
                    stdout.queue(Print("E")).unwrap();
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
        }
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
    grid.longest_path2(&start, &end)
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

    use crossterm::cursor::{Hide, MoveDown, Show};

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
        grid.display_path(&start, &end, &end, &path, 1000);
        execute!(stdout, MoveDown(grid.height as u16), Show).unwrap();
    }

    #[test]
    fn test_longest_path_without_slopes() {
        let grid = Grid::from_str(TEST_INPUT).unwrap();
        let mut stdout = stdout();
        execute!(stdout, Hide).unwrap();
        let start = Pos::new(1, 0);
        let end = Pos::new(grid.width as i32 - 2, grid.height as i32 - 1);
        let path = grid.longest_path2(&start, &end);
        //grid.display_path(&start, &end, &end, &path, 1000);
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
    #[test]
    fn test_neighbours2() {
        let grid = Grid::from_str(TEST_INPUT).unwrap();
        let mut seen = HashSet::new();
        assert_eq!(
            grid.neighbours2(Pos::new(1, 0), &seen).collect::<Vec<_>>(),
            vec![Pos::new(1, 1)]
        );
        assert_eq!(
            grid.neighbours2(Pos::new(11, 19), &seen)
                .collect::<Vec<_>>(),
            vec![Pos::new(11, 20), Pos::new(12, 19)]
        );
        assert_eq!(
            grid.neighbours2(Pos::new(3, 5), &seen).collect::<Vec<_>>(),
            vec![Pos::new(3, 4), Pos::new(3, 6), Pos::new(4, 5)]
        );

        assert_eq!(
            grid.neighbours2(Pos::new(3, 4), &seen).collect::<Vec<_>>(),
            vec![Pos::new(3, 3), Pos::new(3, 5)]
        );
        assert_eq!(
            grid.neighbours2(Pos::new(12, 13), &seen)
                .collect::<Vec<_>>(),
            vec![Pos::new(11, 13), Pos::new(13, 13)]
        );

        assert_eq!(
            grid.neighbours2(Pos::new(7, 13), &seen).collect::<Vec<_>>(),
            vec![Pos::new(7, 14), Pos::new(6, 13),]
        );
        assert_eq!(
            grid.neighbours2(Pos::new(11, 3), &seen).collect::<Vec<_>>(),
            vec![Pos::new(11, 4), Pos::new(10, 3), Pos::new(12, 3)]
        );

        seen.insert(Pos::new(11, 4));
        assert_eq!(
            grid.neighbours2(Pos::new(11, 3), &seen).collect::<Vec<_>>(),
            vec![Pos::new(10, 3), Pos::new(12, 3)]
        );
    }
}
