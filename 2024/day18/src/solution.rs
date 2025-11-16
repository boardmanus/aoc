use aoc_utils::{
    dir::Dir4,
    grif::Graph,
    grud::{Grid, GridPos},
};

fn parse_input(input: &str) -> Vec<GridPos> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(",").map(|s| s.parse::<i64>().unwrap());
            GridPos::new(it.next().unwrap(), it.next().unwrap())
        })
        .collect()
}

fn create_grid(blocks: &[GridPos], width: usize, height: usize) -> Grid<char, Dir4> {
    let mut grid = Grid::new_walkable('.', width, height, |g, _, to| g.at(to) == Some('.'));
    for &b in blocks {
        grid.set(&b, '#');
    }
    grid
}

fn shortest_path(grid: &Grid<char, Dir4>, start: GridPos, end: GridPos) -> Option<usize> {
    grid.bfs(start)
        .find_map(|(pos, level)| if pos == end { Some(level) } else { None })
}

#[allow(dead_code)]
fn first_blocker(grid: &mut Grid<char, Dir4>, blocks: &[GridPos]) -> Option<GridPos> {
    let start = GridPos::new(0, 0);
    let end = GridPos::new(grid.width() as i64 - 1, grid.height() as i64 - 1);

    for &block in blocks {
        grid.set(&block, '#');
        if shortest_path(grid, start, end).is_none() {
            return Some(block);
        }
    }
    None
}

fn first_blocker_binary_search(grid: &mut Grid<char, Dir4>, blocks: &[GridPos]) -> Option<GridPos> {
    let start = GridPos::new(0, 0);
    let end = GridPos::new(grid.width() as i64 - 1, grid.height() as i64 - 1);

    let mut low = 0;
    let mut high = blocks.len();
    let mut mid = (low + high) / 2;

    while low < mid {
        for &block in &blocks[low..mid] {
            grid.set(&block, '#');
        }

        for &block in &blocks[mid..high] {
            grid.set(&block, '.');
        }

        if shortest_path(grid, start, end).is_some() {
            low = mid;
        } else {
            high = mid;
        }
        mid = (low + high) / 2;
    }

    if low < blocks.len() {
        Some(blocks[low])
    } else {
        None
    }
}

pub fn part1(input: &str) -> usize {
    let blocks = parse_input(input);
    let grid = create_grid(&blocks[0..1024], 71, 71);
    shortest_path(&grid, GridPos::new(0, 0), GridPos::new(70, 70)).unwrap()
}

pub fn part2(input: &str) -> String {
    let blocks = parse_input(input);
    let mut grid = create_grid(&blocks[0..1024], 71, 71);
    let blocker = first_blocker_binary_search(&mut grid, &blocks[1024..]).unwrap();
    format!("{},{}", blocker.x, blocker.y)
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 22;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: GridPos = GridPos { x: 6, y: 1 };

    #[test]
    fn test_create_grid() {
        let blocks = parse_input(TEST_INPUT);
        let grid = create_grid(&blocks[0..12], 7, 7);
        assert_eq!(grid.to_string(), include_str!("data/grid_example"));
    }

    #[test]
    fn test_part1() {
        let blocks = parse_input(TEST_INPUT);
        let grid = create_grid(&blocks[0..12], 7, 7);
        assert_eq!(
            shortest_path(&grid, GridPos::new(0, 0), GridPos::new(6, 6)).unwrap(),
            TEST_ANSWER
        );
    }

    #[test]
    fn test_part2() {
        let blocks = parse_input(TEST_INPUT_2);
        let mut grid = create_grid(&blocks[0..12], 7, 7);
        let blocker = first_blocker_binary_search(&mut grid, &blocks[12..]).unwrap();
        assert_eq!(blocker, TEST_ANSWER_2)
    }
}
