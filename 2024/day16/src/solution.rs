use std::collections::{HashMap, HashSet, VecDeque};

use aoc_utils::{
    dir::{Dir, Dir4},
    grud::{Grid, GridPos},
    lust::Lust,
};

type Maze = Grid<char, Dir4>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PosDir {
    loc: GridPos,
    dir: Dir4,
}

impl PosDir {
    fn new(loc: GridPos, dir: Dir4) -> PosDir {
        PosDir { loc, dir }
    }
}

type Path = Lust<PosDir>;
type Score = usize;

#[derive(Debug, Clone)]
struct PathScore {
    path: Path,
    score: Score,
}

impl<'a> PathScore {
    fn new(path: Path, score: Score) -> Self {
        PathScore { path, score }
    }
}

/// Update the visited map with the new path and score if it's better than
/// the current score for that position and direction.
/// @param visited The map of visited positions and directions to their best scores.
/// @param path The current path to update.
/// @param score The score of the current path.
/// @return Some(PathScore) if the path was updated, None otherwise.
fn update_path_score<'a>(
    visited: &'a mut HashMap<PosDir, Score>,
    path: Path,
    score: Score,
) -> Option<PathScore> {
    let pos = path.data()?;
    let current_score = *visited.get(pos).unwrap_or(&usize::MAX);
    if score <= current_score {
        visited.insert(*pos, score);
        Some(PathScore::new(path, score))
    } else {
        None
    }
}

/// Generate next possible paths from the current path score.
/// A possible path is a single rotation and a step in the new direction, or
/// a step in the same direction.
/// @param grid The maze grid to check for walkability.
/// @param path_score The current path score to generate next paths from.
/// @return A vector of PathScore for each valid next path.
fn generate_next_paths<'a>(grid: &Maze, path_score: &PathScore) -> Vec<PathScore> {
    let score = path_score.score;
    let Some(&PosDir { loc, dir }) = path_score.path.data() else {
        return vec![];
    };
    [dir.rotate_cw(), dir, dir.rotate_ccw()]
        .into_iter()
        .filter_map(|new_dir| {
            let new_pos_dir = PosDir::new(loc + new_dir, new_dir);
            if grid.is_walkable(&loc, &new_pos_dir.loc) {
                let new_score = score + if dir == new_dir { 1 } else { 1001 };
                Some(PathScore::new(
                    path_score.path.append(new_pos_dir),
                    new_score,
                ))
            } else {
                None
            }
        })
        .collect()
}

/// Find all paths from start 'S' to end 'E' in the maze.
/// Returns a vector of PathScore for each successful path found.
/// @param grid The maze grid to search.
/// @return A vector of PathScore for each path from 'S' to 'E'.
fn find_all_paths<'a>(grid: &Maze) -> Vec<PathScore> {
    let start = grid.find('S').unwrap();
    let end = grid.find('E').unwrap();
    let mut end_runs: Vec<PathScore> = vec![];
    let start_path = PathScore::new(Path::new(PosDir::new(start, Dir4::E)), 0);
    let mut visited: HashMap<PosDir, Score> = HashMap::new();
    let mut to_visit: VecDeque<PathScore> = VecDeque::from([start_path]);

    while let Some(PathScore { path, score }) = to_visit.pop_front() {
        let pos_dir = match path.data() {
            None => continue,
            Some(pd) => *pd,
        };
        if pos_dir.loc == end {
            end_runs.push(PathScore::new(path, score));
            continue;
        }
        if let Some(path_score) = update_path_score(&mut visited, path, score) {
            for new_path_score in generate_next_paths(grid, &path_score) {
                to_visit.push_back(new_path_score);
            }
        }
    }
    end_runs
}

/// Finds the best paths (with the lowest score) from 'S' to 'E' in the maze.
/// @param grid The maze grid to search.
/// @return A vector of PathScore for each best path found.
fn find_best_paths<'a>(grid: &Maze) -> Vec<PathScore> {
    let end_runs = find_all_paths(grid);
    let Some(min_score) = end_runs.iter().map(|ps| ps.score).min() else {
        return vec![];
    };
    end_runs
        .into_iter()
        .filter(|ps| ps.score == min_score)
        .collect::<Vec<_>>()
}

/// Finds the best locations (positions) from the best paths in the maze.
/// @param grid The maze grid to search.
/// @return A HashSet of GridPos representing the best locations.
fn find_best_locations(grid: &Maze) -> HashSet<GridPos> {
    find_best_paths(grid)
        .iter()
        .fold(HashSet::new(), |mut acc, ps| {
            for pos in ps.path.iter().map(|pd| pd.loc) {
                acc.insert(pos);
            }
            acc
        })
}

pub fn part1(input: &str) -> usize {
    let grid = Maze::parse_walkable(input, |g, _, b| g.at(&b) != Some('#'));
    find_best_paths(&grid).first().unwrap().score
}

pub fn part2(input: &str) -> usize {
    let grid = Maze::parse_walkable(input, |g, _, b| g.at(&b) != Some('#'));
    let all_locations = find_best_locations(&grid);
    all_locations.len()
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 7036;
    pub const TEST_INPUT_P1_2: &str = include_str!("data/input_example_2");
    pub const TEST_ANSWER_P1_2: usize = 11048;
    pub const TEST_INPUT_P2_1: &str = include_str!("data/input_example");
    pub const TEST_OUTPUT_P2_1: &str = include_str!("data/output_part2_1");
    pub const TEST_INPUT_P2_2: &str = include_str!("data/input_example_2");
    pub const TEST_OUTPUT_P2_2: &str = include_str!("data/output_part2_2");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(TEST_INPUT_P1_2), TEST_ANSWER_P1_2);
    }

    #[test]
    fn test_part2_1() {
        //assert_eq!(part2(TEST_INPUT_P2_1), 45);
        let mut grid = Maze::parse_walkable(TEST_INPUT_P2_1, |g, _, b| g.at(&b) != Some('#'));
        let all_locations = find_best_locations(&grid);
        for loc in all_locations {
            grid.set(&loc, 'O');
        }
        println!("{grid}");
        assert_eq!(grid.to_string(), TEST_OUTPUT_P2_1);
    }

    #[test]
    fn test_part2_2() {
        let mut grid = Maze::parse_walkable(TEST_INPUT_P2_2, |g, _, b| g.at(&b) != Some('#'));
        let all_locations = find_best_locations(&grid);
        for loc in all_locations {
            grid.set(&loc, 'O');
        }
        println!("{grid}");
        assert_eq!(grid.to_string(), TEST_OUTPUT_P2_2);
        assert_eq!(part2(TEST_INPUT_P2_2), 64);
    }
}
