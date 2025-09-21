use std::collections::HashSet;

use aoc_utils::{
    dir::Dir4,
    grud::{Grid, GridPos},
    vec2d::Vec2d,
};
use lazy_regex::regex;

type Pos = GridPos;
type Vel = Vec2d<i64>;

fn normalize_pos(pos: &Pos, width: usize, height: usize) -> Pos {
    let x = pos.x.rem_euclid(width as i64);
    let y = pos.y.rem_euclid(height as i64);
    Pos::new(x, y)
}
struct Robot {
    pos: Pos,
    vel: Vel,
}

impl Robot {
    fn new(pos: Pos, vel: Vel) -> Robot {
        Robot { pos, vel }
    }

    fn moves(&mut self, num_moves: usize, width: usize, height: usize) {
        let new_pos = self.pos + self.vel * (num_moves as i64);
        self.pos = normalize_pos(&new_pos, width, height);
    }

    fn quadrant(&self, grid: &RobotGrid) -> Option<usize> {
        let width = grid.grid.width() as i64;
        let height = grid.grid.height() as i64;
        match self.pos {
            Pos { x, y, .. } if x < width / 2 && y < height / 2 => Some(0),
            Pos { x, y, .. } if x > width / 2 && y < height / 2 => Some(1),
            Pos { x, y, .. } if x < width / 2 && y > height / 2 => Some(2),
            Pos { x, y, .. } if x > width / 2 && y > height / 2 => Some(3),
            _ => None,
        }
    }
}

struct RobotGrid {
    robots: Vec<Robot>,
    grid: Grid<char, Dir4>,
}

impl RobotGrid {
    fn parse(input: &str, width: usize, height: usize) -> RobotGrid {
        let re = regex!(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)");
        let robots = input
            .lines()
            .filter_map(|line| {
                let c = re.captures(line)?;
                let px = c.get(1)?.as_str().parse::<i64>().ok()?;
                let py = c.get(2)?.as_str().parse::<i64>().ok()?;
                let vx = c.get(3)?.as_str().parse::<i64>().ok()?;
                let vy = c.get(4)?.as_str().parse::<i64>().ok()?;
                Some(Robot::new(Pos::new(px, py), Vel::new(vx, vy)))
            })
            .collect::<Vec<_>>();
        let grid = Grid::new('.', width, height);
        let mut rg = RobotGrid { robots, grid };
        rg.update_grid();
        rg
    }

    fn update_grid(&mut self) {
        self.grid.fill('.');
        self.robots.iter().for_each(|robot| {
            if let Some(c) = self.grid.at(&robot.pos) {
                let new_c = match c {
                    '.' => '1',
                    _ => ((c as u8) + 1) as char,
                };
                self.grid.set(&robot.pos, new_c);
            }
        });
    }
    fn count_quadrants(&self) -> [usize; 4] {
        self.robots.iter().fold([0, 0, 0, 0], |mut acc, robot| {
            if let Some(q) = robot.quadrant(self) {
                acc[q] += 1;
            }
            acc
        })
    }

    fn move_all(&mut self, num_moves: usize) {
        let width = self.grid.width();
        let height = self.grid.height();
        self.robots
            .iter_mut()
            .for_each(|robot| robot.moves(num_moves, width, height));
    }

    fn has_overlaps(&self) -> bool {
        let mut positions: HashSet<Pos> = HashSet::new();
        !self.robots.iter().all(|robot| positions.insert(robot.pos))
    }

    fn find_xmas_tree(&mut self) -> usize {
        let mut iterations = 0usize;
        while self.has_overlaps() {
            self.move_all(1);
            iterations += 1;
        }
        self.update_grid();
        println!("{}", self.grid);

        iterations
    }
}

pub fn part1(input: &str) -> usize {
    let mut robots = RobotGrid::parse(input, 101, 103);
    robots.move_all(100);
    robots.count_quadrants().iter().product()
}

pub fn part2(input: &str) -> usize {
    let mut robots = RobotGrid::parse(input, 101, 103);
    robots.find_xmas_tree()
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const INPUT: &str = include_str!("data/input");
    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 12;

    #[test]
    fn test_part1() {
        let mut robots = RobotGrid::parse(TEST_INPUT, 11, 7);
        robots.move_all(100);
        let count: usize = robots.count_quadrants().iter().product();
        assert_eq!(count, TEST_ANSWER)
    }

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
        let mut robots = RobotGrid::parse(TEST_INPUT, 11, 7);
        robots.move_all(100);
        robots.update_grid();
        let grid_str = robots.grid.to_string();
        println!("{grid_str}");
        assert_eq!(grid_str, res_str);
    }

    #[test]
    fn test_count_quadrants() {
        let mut robots = RobotGrid::parse(TEST_INPUT, 11, 7);
        robots.move_all(100);
        assert_eq!(robots.count_quadrants(), [1usize, 3, 4, 1]);
    }

    #[test]
    fn test_find_xmas_tree() {
        let mut robots = RobotGrid::parse(INPUT, 101, 103);
        let _ = robots.find_xmas_tree();
    }
}
