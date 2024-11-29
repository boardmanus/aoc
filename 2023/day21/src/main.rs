use std::{collections::HashSet, str::FromStr, vec};

use euclid::{Box2D, Point2D, Size2D, Vector2D};

enum Steps {}

type Point = Point2D<i64, Steps>;
type Vector = Vector2D<i64, Steps>;

const DIRS: [Vector; 4] = [
    Vector::new(0, 1),
    Vector::new(0, -1),
    Vector::new(1, 0),
    Vector::new(-1, 0),
];

struct Rocks {
    bounds: Box2D<i64, Steps>,
    locs: HashSet<Point>,
}

impl Rocks {
    fn new(width: usize, height: usize) -> Self {
        Rocks {
            bounds: Box2D::from(Size2D::new(width as i64, height as i64)),
            locs: Default::default(),
        }
    }

    fn has_rock(&self, p: &Point) -> bool {
        if self.bounds.contains(*p) {
            self.locs.contains(&Point::new(p.x, p.y))
        } else {
            false
        }
    }

    fn add_rock(&mut self, p: &Point) {
        if self.bounds.contains(*p) {
            self.locs.insert(*p);
        }
    }

    fn clr_rock(&mut self, p: &Point) {
        if self.bounds.contains(*p) {
            self.locs.remove(p);
        }
    }
}

struct Grid {
    start: Point,
    rocks: Rocks,
}

impl Grid {
    fn has_rock(&self, p: &Point) -> bool {
        self.rocks.has_rock(p)
    }

    fn walk(&self, from: &HashSet<Point>, infinite: bool) -> HashSet<Point> {
        let mut to = HashSet::new();
        for p in from {
            for d in &DIRS {
                let mut p = *p + *d;
                if infinite {
                    p = p.rem_euclid(&self.rocks.bounds.size());
                }
                if !self.has_rock(&p) {
                    to.insert(p);
                }
            }
        }
        to
    }

    fn walk2(&self, start: &Point, steps: usize, infinite: bool) -> HashSet<Point> {
        let parity = steps & 1;
        let mut to = if parity == 0 {
            HashSet::from([*start])
        } else {
            HashSet::new()
        };
        let mut seen = HashSet::from([*start]);
        let mut q = vec![*start];
        for step in 1..=steps {
            let mut last_q: Vec<_> = vec![];
            std::mem::swap(&mut last_q, &mut q);
            for p in last_q {
                for d in &DIRS {
                    let p = p + *d;
                    let rock = if infinite {
                        self.has_rock(&p.rem_euclid(&self.rocks.bounds.size()))
                    } else {
                        self.has_rock(&p)
                    };

                    if !rock && !seen.contains(&p) {
                        seen.insert(p);
                        if step & 1 == parity {
                            to.insert(p.clone());
                        }
                        q.push(p);
                    }
                }
            }
        }
        to
    }

    fn stroll(&self, start: &Point, steps: usize, infinite: bool) -> usize {
        let mut reachable = HashSet::from([*start]);

        for _ in 0..steps {
            for p in reachable.drain().collect::<Vec<_>>() {
                reachable.extend(DIRS.iter().map(|d| p + *d).filter(|p| {
                    if infinite {
                        !self.has_rock(&p.rem_euclid(&self.rocks.bounds.size()))
                    } else {
                        !self.has_rock(p)
                    }
                }));
            }
        }
        reachable.len()
    }

    // Find the number of walkable points for and odd or even fill.
    fn filled_count(&self) -> (usize, usize) {
        let mut even = 0;
        let mut odd = 0;
        for y in 0..self.rocks.bounds.height() {
            for x in 0..self.rocks.bounds.width() {
                let p = Point::new(x as i64, y as i64);
                if self.has_rock(&p) {
                    continue;
                }
                match (x + y) & 1 {
                    0 => even += 1,
                    1 => odd += 1,
                    _ => unreachable!(),
                }
            }
        }
        (even, odd)
    }

    fn guess_walk(&self, steps: usize) -> usize {
        let steps = steps as i64;
        // Assumptions about the start location, and size of grid.
        // 1. Start in the middle
        let width = self.rocks.bounds.width();
        let height = self.rocks.bounds.height();
        assert_eq!(self.start.x, width / 2);
        assert_eq!(self.start.y, height / 2);
        // 2. Grid width/height the same
        assert_eq!(width, height);
        // 3. Grid width is odd.
        //    Grids alternate between odd and even.
        assert_eq!(width & 1, 1);
        // 4. Steps is a muliple of the width (we go right to an edge)
        //    The step pattern forms a diamond shape with all the interior
        //    completely filled, and the outside missing a few bits.
        assert_eq!((steps - width / 2) % width, 0);

        // For an odd or even fill, the number of squares froms a diamond shape
        // checkerboard. The number of grids contained is the width squared.
        // However, the grids on the border, have bits of the corners missing,
        // or extra. These extra fill corners can be moved in to the missing
        // corners. After moving, each edge has one corner missing. Taking these
        // corners from one of apex pieces leaves n*n-1 grids filled, plus a
        // single grid filled to the edges.
        //  ----
        // | /\ |
        // |<  >|
        // | \/ |
        //  ----
        let grids_across = (steps - width / 2) / width;
        let num_odd_grids = (grids_across + 1) * (grids_across + 1);
        let num_even_grids = grids_across * grids_across;

        // A filled grid, in the odd positioning has the following has the following
        // walkable plots.
        let grid_plots = self.filled_count();

        // The number of walkable in the remaining partial is strolling to the edge
        let single_grid = self.stroll(&self.start, (width / 2) as usize, false);

        println!(
            "GridsAcross={}, NumFilledGrids=({}, {}), OddGridPlots={:?}, SingleGrid={}",
            grids_across, num_odd_grids, num_even_grids, grid_plots, single_grid
        );

        //let tl = self.stroll(&)
        let num_visited = num_even_grids * grid_plots.1 as i64
            + num_odd_grids * grid_plots.0 as i64
            + single_grid as i64;
        println!("NumVisited={}", num_visited);
        num_visited as usize
    }

    fn print(&self, walk: &HashSet<Point>, steps: usize) {
        let steps = steps as i64;
        let width = self.rocks.bounds.width();
        let height = self.rocks.bounds.height();
        let steps = ((steps / width) * width + width / 2);
        for y in (self.start.y - steps)..(self.start.y + steps + 1) {
            if y % height == 0 {
                ((self.start.x - steps)..(self.start.x + steps + 1)).for_each(|x| {
                    if x % width == 0 {
                        print!("+");
                    }
                    print!("-")
                });
                println!();
            }
            for x in (self.start.x - steps)..(self.start.x + steps + 1) {
                if x % width == 0 {
                    print!("|");
                }
                let p = Point::new(x as i64, y as i64).rem_euclid(&self.rocks.bounds.size());
                if p == self.start {
                    print!("S");
                } else if self.has_rock(&p) {
                    print!("#");
                } else if walk.contains(&p) {
                    if p.x & 1 == p.y & 1 {
                        print!("E");
                    } else {
                        print!("O");
                    }
                } else {
                    print!(".");
                }
            }

            println!();
        }
    }
}
impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        let mut rocks = Rocks::new(width, height);
        let mut start = None;
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => rocks.add_rock(&Point::new(x as i64, y as i64)),
                    'S' => start = Some(Point::new(x as i64, y as i64)),
                    _ => {}
                }
            }
        }
        Ok(Grid {
            start: start.unwrap(),
            rocks,
        })
    }
}

fn solve_part1(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    grid.stroll(&grid.start, 64, false)
}

fn solve_part2(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    //grid.stroll(26501365)
    grid.guess_walk(26501365)
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let part1 = solve_part1(INPUT);
    println!("Part1: {part1}");
    let part2 = solve_part2(INPUT);
    println!("Part2: {part2}");
}

#[cfg(test)]
mod test_main {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let grid: Grid = Grid::from_str(TEST_INPUT).unwrap();
        assert_eq!(grid.stroll(&grid.start, 6, false), 16);
        let w2 = grid.walk2(&grid.start, 6, false);
        grid.print(&w2, (grid.rocks.bounds.width() / 2) as usize);
        assert_eq!(w2.len(), 16);
    }

    #[test]
    fn test_part2_input() {
        const INPUT: &str = include_str!("input.txt");
        let grid: Grid = Grid::from_str(INPUT).unwrap();
        let width = grid.rocks.bounds.width();
        let res = grid.stroll(&grid.start, (width / 2) as usize, false);
        let steps = 26501365;
        println!("Reachable={}", res);
        println!(
            "Steps={steps}, steps-width/2={}, steps-width/2 % width={}",
            steps - width / 2,
            (steps - width / 2) % width
        );
    }

    #[test]
    fn test_fill_count() {
        let grid: Grid = Grid::from_str(INPUT).unwrap();
        let counts = grid.filled_count();
        println!("Fill Count = {:?}", counts);
        assert_eq!(
            counts.0 + counts.1,
            (grid.rocks.bounds.width() * grid.rocks.bounds.height()) as usize
                - grid.rocks.locs.len()
        )
    }

    #[test]
    fn test_input_data() {
        let grid = Grid::from_str(INPUT).unwrap();
        let steps = 26501365;
        let width: i64 = grid.rocks.bounds.width();

        let num_grids = (steps - width / 2) / width;
        let num_diamonds = (2 * steps) / width;
        println!(
            "width={}, steps={steps}, num_grids={num_grids} (num_grids*width+width/2={}), steps/(width + width/2)={} ({})",
            width,
            num_grids * width + width / 2,
           num_diamonds,
           num_diamonds * width / 2,

        );

        //let full_walk = (0..width).fold(HashSet::from([grid.start]), |steps, i| {
        //    grid.walk(&steps, false)
        //});

        let center_walk = (0..width / 2).fold(HashSet::from([grid.start]), |steps, i| {
            grid.walk(&steps, false)
        });

        let corner_walk = (0..width / 2 - 2).fold(
            HashSet::from([
                Point::new(0, 0),
                Point::new(width - 1, width - 1),
                Point::new(0, width - 1),
                Point::new(width - 1, 0),
            ]),
            |steps, i| grid.walk(&steps, false),
        );
        /*
        let missing_walk = full_walk.iter().fold(HashSet::new(), |mut steps, p| {
            if !corner_walk.contains(p) {
                steps.insert(*p);
            }
            steps
        });
        */
        let union_walk = center_walk
            .union(&corner_walk)
            .copied()
            .collect::<HashSet<_>>();

        grid.print(&union_walk, (width / 2) as usize);

        center_walk.iter().for_each(|p| {
            if corner_walk.contains(p) {
                println!("Step in center and corner!: {:?}", p);
            }
        });

        //grid.print(&full_walk, (width / 2) as usize);
    }

    #[test]
    fn test_print() {
        let grid: Grid = Grid::from_str(TEST_INPUT).unwrap();
        let width: i64 = grid.rocks.bounds.width();
        for i in 0..10 {
            println!("i={}", i);
            let walk = grid.stroll(&grid.start, i, false);
            //grid.print(&walk, i);
        }

        println!("fill_count = {:?}", grid.filled_count());

        let walk = grid.stroll(&grid.start, width as usize, true);
        println!(
            "start=({:?}), walk_len={}, stroll length={}",
            grid.start, walk, width,
        );
    }

    #[test]
    fn test_part2() {
        let grid: Grid = Grid::from_str(TEST_INPUT).unwrap();
        assert_eq!(grid.stroll(&grid.start, 6, true), 16);

        let dist = 10;
        let walk = grid.stroll(&grid.start, dist, true);
        assert_eq!(walk, 50);
        //grid.print(&walk, dist);

        let dist = 50;
        let walk = grid.stroll(&grid.start, dist, true);
        assert_eq!(walk, 1594);
        //grid.print(&walk, dist);

        let dist = 100;
        let walk = grid.walk2(&grid.start, dist, true);
        assert_eq!(walk.len(), 6536);
        //grid.print(&walk, dist);

        let dist = 500;
        let walk = grid.walk2(&grid.start, dist, true);
        assert_eq!(walk.len(), 167004);

        let dist = 1000;
        let walk = grid.walk2(&grid.start, dist, true);
        assert_eq!(walk.len(), 668697);

        let dist = 5000;
        let walk = grid.walk2(&grid.start, dist, true);
        assert_eq!(walk.len(), 16733044);
    }
}
