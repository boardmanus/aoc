use std::collections::HashSet;

use aoc_utils::grid::{Grid, Index};
use euclid::{Point2D, Vector2D};
use lazy_regex::regex;

enum Tiles {}

type Pos = Point2D<i64, Tiles>;
type Vel = Vector2D<i64, Tiles>;

struct Robot {
    pos: Pos,
    vel: Vel,
}

impl Robot {
    fn new(pos: Pos, vel: Vel) -> Robot {
        Robot { pos, vel }
    }

    fn parse(input: &str) -> Vec<Robot> {
        let re = regex!(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)");
        input
            .lines()
            .map(|line| {
                let c = re.captures(line).unwrap();
                let px = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let py = c.get(2).unwrap().as_str().parse::<i64>().unwrap();
                let vx = c.get(3).unwrap().as_str().parse::<i64>().unwrap();
                let vy = c.get(4).unwrap().as_str().parse::<i64>().unwrap();
                Robot::new(Pos::new(px, py), Vel::new(vx, vy))
            })
            .collect()
    }

    fn moves(&mut self, num_moves: usize, width: usize, height: usize) {
        let new_pos = self.pos + self.vel * (num_moves as i64);
        self.pos = normalize_pos(&new_pos, width, height);
    }

    fn quadrant(&self, width: usize, height: usize) -> Option<usize> {
        let width = width as i64;
        let height = height as i64;
        match self.pos {
            Pos { x, y, .. } if x < width / 2 && y < height / 2 => Some(0),
            Pos { x, y, .. } if x > width / 2 && y < height / 2 => Some(1),
            Pos { x, y, .. } if x < width / 2 && y > height / 2 => Some(2),
            Pos { x, y, .. } if x > width / 2 && y > height / 2 => Some(3),
            _ => None,
        }
    }
}

fn robot_grid(robots: &Vec<Robot>, width: usize, height: usize) -> Grid<char> {
    let mut grid = Grid::new('.', width, height);
    robots.iter().for_each(|robot| {
        let index = Index(robot.pos.x, robot.pos.y);
        if let Some(c) = grid.at(index) {
            let new_c = match c {
                '.' => '1',
                _ => ((c as u8) + 1) as char,
            };
            grid.set(index, new_c);
        }
    });
    grid
}

fn normalize_pos(pos: &Pos, width: usize, height: usize) -> Pos {
    let x = pos.x.rem_euclid(width as i64);
    let y = pos.y.rem_euclid(height as i64);
    Pos::new(x, y)
}

fn count_quadrants(robots: &Vec<Robot>, width: usize, height: usize) -> [usize; 4] {
    robots.iter().fold([0, 0, 0, 0], |mut acc, robot| {
        if let Some(q) = robot.quadrant(width, height) {
            acc[q] += 1;
        }
        acc
    })
}

fn move_all(robots: &mut Vec<Robot>, num_moves: usize, width: usize, height: usize) {
    robots
        .iter_mut()
        .for_each(|robot| robot.moves(num_moves, width, height));
}

fn has_overlaps(robots: &Vec<Robot>) -> bool {
    let mut positions: HashSet<Pos> = HashSet::new();
    !robots.iter().all(|robot| positions.insert(robot.pos))
}

fn find_xmas_tree(robots: &mut Vec<Robot>, width: usize, height: usize) -> usize {
    let mut iterations = 0usize;
    while has_overlaps(&robots) {
        move_all(robots, 1, width, height);
        iterations += 1;
    }
    println!("{}", robot_grid(robots, width, height));

    iterations
}

pub fn part1(input: &str) -> usize {
    let mut robots = Robot::parse(input);
    move_all(&mut robots, 100, 101, 103);
    count_quadrants(&robots, 101, 103).iter().product()
}

pub fn part2(input: &str) -> usize {
    let mut robots = Robot::parse(input);
    find_xmas_tree(&mut robots, 101, 103)
}

#[cfg(test)]
mod tests {

    use aoc_utils::grid::{Grid, Index};

    use super::*;

    pub const INPUT: &str = include_str!("data/input");
    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 12;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: &str = "part2";

    #[test]
    fn test_normalize_pos() {
        assert_eq!(normalize_pos(&Pos::new(100, 50), 100, 50), Pos::new(0, 0));
        assert_eq!(
            normalize_pos(&Pos::new(100, 50), 101, 51),
            Pos::new(100, 50)
        );
        assert_eq!(normalize_pos(&Pos::new(-1, -1), 100, 50), Pos::new(99, 49));
    }

    #[test]
    fn test_move_robot() {
        let mut robot = Robot::new(Pos::new(2, 4), Vel::new(2, -3));
        robot.moves(5, 11, 7);
        assert_eq!(robot.pos, Pos::new(1, 3));
    }

    #[test]
    fn test_move_all() {
        let res_str = "......2..1.
...........
1..........
.11........
.....1.....
...12......
.1....1....
";
        let mut grid = Grid::new('.', 11, 7);
        let mut robots = Robot::parse(TEST_INPUT);
        move_all(&mut robots, 100, 11, 7);
        robots.iter().for_each(|robot| {
            let index = Index(robot.pos.x, robot.pos.y);
            if let Some(c) = grid.at(index) {
                let new_c = match c {
                    '.' => '1',
                    _ => ((c as u8) + 1) as char,
                };
                grid.set(index, new_c);
            }
        });
        let grid_str = grid.to_string();
        println!("{grid_str}");
        assert_eq!(grid_str, res_str);
    }

    #[test]
    fn test_count_quadrants() {
        let mut robots = Robot::parse(TEST_INPUT);
        move_all(&mut robots, 100, 11, 7);
        assert_eq!(count_quadrants(&robots, 11, 7), [1usize, 3, 4, 1]);
    }

    #[test]
    fn test_find_xmas_tree() {
        let mut robots = Robot::parse(INPUT);
        let i = find_xmas_tree(&mut robots, 101, 103);
        assert_eq!(i, 10);
    }
}
