use aoc_utils::{
    dir::Dir4,
    grid::{Grid, Index},
};

type WarehouseGrid = Grid<char>;
type Directions = Vec<Dir4>;

fn robot_pos(grid: &WarehouseGrid) -> Index {
    grid.find('@').unwrap()
}

fn move_ops(moves: &mut Vec<Index>, grid: &WarehouseGrid, from: Index, dir: Dir4) -> bool {
    let to = from + dir;
    let to_c = grid.at(to).unwrap();
    let ok_to_move = match to_c {
        '.' => true,
        'O' => move_ops(moves, grid, to, dir),
        '[' => match dir {
            Dir4::N | Dir4::S => {
                move_ops(moves, grid, to, dir) && move_ops(moves, grid, to + Dir4::E, dir)
            }
            _ => move_ops(moves, grid, to, dir),
        },
        ']' => match dir {
            Dir4::N | Dir4::S => {
                move_ops(moves, grid, to, dir) && move_ops(moves, grid, to + Dir4::W, dir)
            }
            _ => move_ops(moves, grid, to, dir),
        },
        '#' => false,
        _ => false,
    };

    if ok_to_move && !moves.contains(&from) {
        moves.push(from);
    }

    ok_to_move
}

fn move_all(grid: &mut WarehouseGrid, directions: &Directions) {
    let mut robot = robot_pos(grid);
    directions.iter().for_each(|&dir| {
        robot = move_robot(grid, robot, dir);
    });
    println!("{grid}");
}

fn move_robot(grid: &mut WarehouseGrid, robot: Index, dir: Dir4) -> Index {
    let mut moves = Vec::<Index>::new();
    if move_ops(&mut moves, grid, robot, dir) {
        moves.into_iter().for_each(|from| {
            let to = from + dir;
            let c = grid.at(from).unwrap();
            grid.set(to, c);
            grid.set(from, '.');
        });
        robot + dir
    } else {
        robot
    }
}

fn parse_input(input: &str, double: bool) -> (WarehouseGrid, Directions) {
    let mut it = input.split("\n\n");
    let mut grid = WarehouseGrid::parse(it.next().unwrap());
    if double {
        let g = grid.row_iter().fold(Vec::new(), |mut acc, c| {
            match c {
                '#' => acc.extend("##".chars()),
                '.' => acc.extend("..".chars()),
                'O' => acc.extend("[]".chars()),
                '@' => acc.extend("@.".chars()),
                _ => {}
            };
            acc
        });
        grid = Grid::create(grid.width() * 2, grid.height(), g).unwrap();
    }

    let directions = it
        .next()
        .unwrap()
        .chars()
        .filter(|&c| "^>v<".contains(c))
        .filter_map(|c| match c {
            '^' => Some(Dir4::N),
            '>' => Some(Dir4::E),
            'v' => Some(Dir4::S),
            '<' => Some(Dir4::W),
            _ => None,
        })
        .collect();
    (grid, directions)
}

pub fn part1(input: &str) -> usize {
    let (mut grid, directions) = parse_input(input, false);
    move_all(&mut grid, &directions);
    let boxes = grid.filter_pos('O');
    boxes.iter().map(|b| (b.0 + b.1 * 100) as usize).sum()
}

pub fn part2(input: &str) -> usize {
    let (mut grid, directions) = parse_input(input, true);
    println!("{grid}");
    move_all(&mut grid, &directions);
    let boxes = grid.filter_pos('[');
    boxes.iter().map(|b| (b.0 + b.1 * 100) as usize).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 10092;
    pub const TEST_INPUT_P1_2: &str = include_str!("data/input_example_2");
    pub const TEST_ANSWER_P1_2: usize = 2028;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 9021;

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(TEST_INPUT_P1_2), TEST_ANSWER_P1_2);
    }

    #[test]
    fn test_part1_end_grid() {
        let (mut grid, directions) = parse_input(TEST_INPUT, false);
        move_all(&mut grid, &directions);
        assert_eq!(
            grid.to_string(),
            include_str!("data/input_example_end_grid")
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
}
