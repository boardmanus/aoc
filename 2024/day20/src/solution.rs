use std::collections::{HashMap, HashSet, VecDeque};

use aoc_utils::{
    dir::{Dir, Dir4},
    grid::{Grid, Index},
};

#[derive(Debug, Clone)]
struct Maze {
    grid: Grid<char>,
    start: Index,
    end: Index,
}

impl Maze {
    fn parse(input: &str) -> Option<Maze> {
        let grid = Grid::<char>::parse(input);
        let start = grid.find('S')?;
        let end = grid.find('E')?;
        Some(Maze { grid, start, end })
    }

    fn shortest_path(&self) -> HashMap<Index, usize> {
        let mut visited: HashMap<Index, usize> = HashMap::new();
        let mut possies = VecDeque::<(Index, usize)>::from([(self.start, 0)]);
        while let Some((pos, len)) = possies.pop_front() {
            if pos == self.end {
                visited.insert(pos, len);
                break;
            } else {
                Dir4::cw()
                    .map(|dir| pos + dir)
                    .filter(|pos_next| {
                        !self.grid.matches(*pos_next, '#') && !visited.contains_key(pos_next)
                    })
                    .for_each(|pos| possies.push_back((pos, len + 1)));
                visited.insert(pos, len);
            }
        }
        visited
    }

    fn shortest_path_len(&self) -> Option<usize> {
        let visited = self.shortest_path();
        Some(*visited.get(&self.end)?)
    }

    fn single_wall(&self, pos: Index, dir: Dir4) -> Option<(Index, Dir4)> {
        let poss_wall = pos + dir;
        if self.grid.at(poss_wall)? == '#' {
            let next_pos = poss_wall + dir;
            if self.grid.at(next_pos)? == '#' {
                None
            } else {
                Some((pos, dir))
            }
        } else {
            None
        }
    }

    fn removable_walls(&self, visited: &HashMap<Index, usize>) -> Vec<(Index, Dir4)> {
        let mut walls = HashSet::<(Index, Dir4)>::new();
        visited.keys().for_each(|&pos| {
            let _ = Dir4::cw()
                .filter_map(|dir| self.single_wall(pos, dir))
                .for_each(|pos| _ = walls.insert(pos));
        });
        walls.iter().map(|&x| x).collect()
    }

    fn cheat_length(
        &self,
        max_len: usize,
        visited: &HashMap<Index, usize>,
        wall: (Index, Dir4),
    ) -> usize {
        let start_len = visited.get(&wall.0).unwrap();
        let end_pos = wall.0 + wall.1 + wall.1;
        let end_len = max_len - visited.get(&end_pos).unwrap() + 1;
        start_len + 1 + end_len
    }

    fn find_num_cheat_paths(&self, visited: &HashMap<Index, usize>, min_reduction: usize) -> usize {
        let max_path_len = *visited.get(&self.end).unwrap();
        let walls = self.removable_walls(&visited);
        let min_len = max_path_len - min_reduction;
        let mut map = HashMap::<usize, usize>::new();
        let num_cheats = walls
            .iter()
            .filter(|&wall| {
                let cheat_len = self.cheat_length(max_path_len, &visited, *wall);
                if cheat_len <= max_path_len {
                    *map.entry(max_path_len - cheat_len).or_default() += 1;
                }
                cheat_len <= min_len
            })
            .count();

        let mut check = map.iter().collect::<Vec<_>>();
        check.sort_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(&b.0)
            } else {
                a.1.cmp(&b.1)
            }
        });
        println!("{:?}", check);
        check.sort_by(|a, b| {
            if a.0 == b.0 {
                a.1.cmp(&b.1)
            } else {
                a.0.cmp(&b.0)
            }
        });
        println!("{:?}\n", check);

        num_cheats
    }
}

// 1. find shortest path: t=len(shortest)
// 2. find cheat such that tc < t - 100
//    - only look at walls adjacent to shortest path
//    - ignore "double" walls
pub fn part1(input: &str) -> usize {
    let maze = Maze::parse(input).unwrap();
    let visited = maze.shortest_path();
    maze.find_num_cheat_paths(&visited, 100)
}

pub fn part2(input: &str) -> String {
    input.to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 3;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: &str = "part2";

    #[test]
    fn test_shortest_path() {
        let maze = Maze::parse(TEST_INPUT).unwrap();
        assert_eq!(maze.shortest_path_len(), Some(84));
    }

    #[test]
    fn test_removable_walls() {
        let maze = Maze::parse(TEST_INPUT).unwrap();
        let visited = maze.shortest_path();
        let x = maze.removable_walls(&visited);
        println!("{:?}", x);
        assert!(x.contains(&(Index(3, 1), Dir4::E)));
        assert!(x.contains(&(Index(12, 5), Dir4::S)));
        assert!(x.contains(&(Index(12, 7), Dir4::N)));
    }

    #[test]
    fn test_cheat_len() {
        let maze = Maze::parse(TEST_INPUT).unwrap();
        let visited = maze.shortest_path();
        let mut test = visited.iter().map(|x| (*x.0, *x.1)).collect::<Vec<_>>();
        test.sort_by(|a, b| a.1.cmp(&b.1));
        println!("{:?}", test);
        let max_len = *visited.get(&maze.end).unwrap();
        assert_eq!(
            maze.cheat_length(max_len, &visited, (Index(7, 7), Dir4::W)),
            20
        );
        assert_eq!(
            maze.cheat_length(max_len, &visited, (Index(8, 7), Dir4::S)),
            max_len - 38
        );
    }

    #[test]
    fn test_find_num_cheat_paths() {
        let maze = Maze::parse(TEST_INPUT).unwrap();
        let visited = maze.shortest_path();
        assert_eq!(maze.find_num_cheat_paths(&visited, 64), 1);
        assert_eq!(maze.find_num_cheat_paths(&visited, 40), 2);
        assert_eq!(maze.find_num_cheat_paths(&visited, 38), 3);
        assert_eq!(maze.find_num_cheat_paths(&visited, 36), 4);
        assert_eq!(maze.find_num_cheat_paths(&visited, 20), 5);
        assert_eq!(maze.find_num_cheat_paths(&visited, 12), 8);
        assert_eq!(maze.find_num_cheat_paths(&visited, 10), 10);
        assert_eq!(maze.find_num_cheat_paths(&visited, 8), 14);
        assert_eq!(maze.find_num_cheat_paths(&visited, 6), 16);
        assert_eq!(maze.find_num_cheat_paths(&visited, 4), 30);
        assert_eq!(maze.find_num_cheat_paths(&visited, 2), 44);
    }
}
