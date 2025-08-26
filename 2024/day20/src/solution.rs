use std::collections::{HashMap, HashSet, VecDeque};

use aoc_utils::{
    dir::{Dir, Dir4},
    grid::{Grid, Index},
    grif, grud,
};

#[derive(Debug, Clone)]
struct GrudMaze {
    grid: grud::Grid<char>,
    start: grud::GridPos,
    end: grud::GridPos,
}

impl GrudMaze {
    fn parse(input: &str) -> Option<GrudMaze> {
        let grid = grud::Grid::<char>::parse(input);
        let start = grid.find('S')?;
        let end = grid.find('E')?;
        Some(GrudMaze { grid, start, end })
    }

    fn walkable(&self, _from: &grud::GridPos, to: &grud::GridPos) -> bool {
        self.grid.at(to) != Some('#')
    }

    fn shortest_path(&self) -> HashMap<Index, usize> {
        let sp = grif::shortest_path(&self.node_at(&self.start), &self.node_at(&self.end));
        let mut visited: HashMap<Index, usize> = HashMap::new();
        let mut possies = VecDeque::<(grud::GridPos, usize)>::from([(self.start, 0)]);
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
}

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

    fn cheats_for_pos(&self, pos: Index, time: i64) -> Vec<Index> {
        // find all path positions reachable in the time frame
        let mut cheat_pos: Vec<Index> = vec![];
        for y in (pos.1 - time)..=(pos.1 + time) {
            let xmax = time - (y - pos.1).abs();
            assert!(xmax >= 0 && xmax <= time);
            for x in (pos.0 - xmax)..=(pos.0 + xmax) {
                let new_pos = Index(x, y);
                if new_pos == pos {
                    continue;
                }
                if let Some(c) = self.grid.at(new_pos) {
                    if c != '#' {
                        cheat_pos.push(new_pos);
                    }
                }
            }
        }
        cheat_pos
    }

    fn adv_cheat_length(
        &self,
        visited: &HashMap<Index, usize>,
        max_len: usize,
        start_pos: Index,
        cheat_pos: Index,
    ) -> usize {
        let start_len = visited.get(&start_pos).unwrap();
        let end_len = max_len - visited.get(&cheat_pos).unwrap();
        let md = ((start_pos.0 - cheat_pos.0).abs() + (start_pos.1 - cheat_pos.1).abs()) as usize;
        start_len + md + end_len
    }

    fn cheat_paths_for_pos(
        &self,
        visited: &HashMap<Index, usize>,
        max_len: usize,
        pos: Index,
        time: i64,
        cheat_map: &mut HashMap<usize, usize>,
    ) {
        let cheats = self.cheats_for_pos(pos, time);
        cheats.iter().for_each(|&cheat_pos| {
            let total = self.adv_cheat_length(visited, max_len, pos, cheat_pos);
            if max_len >= total {
                let savings = max_len - total;
                *cheat_map.entry(savings).or_default() += 1;
            }
        })
    }

    fn find_num_adv_cheat_paths(
        &self,
        visited: &HashMap<Index, usize>,
        time: i64,
        min_reduction: usize,
    ) -> usize {
        let max_len = *visited.get(&self.end).unwrap();
        let mut cheat_map: HashMap<usize, usize> = HashMap::new();
        visited.iter().for_each(|(&pos, _)| {
            self.cheat_paths_for_pos(visited, max_len, pos, time, &mut cheat_map);
        });

        cheat_map
            .iter()
            .filter(|x| *x.0 >= min_reduction)
            .map(|x| x.1)
            .sum()
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
        let num_cheats = walls
            .iter()
            .filter(|&wall| self.cheat_length(max_path_len, &visited, *wall) <= min_len)
            .count();

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

pub fn part2(input: &str) -> usize {
    let maze = Maze::parse(input).unwrap();
    let visited = maze.shortest_path();
    maze.find_num_adv_cheat_paths(&visited, 20, 100)
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");

    #[test]
    fn test_cheats_for_pos() {
        let maze = Maze::parse(TEST_INPUT).unwrap();
        let test_pos = vec![
            Index(1, 3),
            Index(1, 2),
            Index(3, 3),
            Index(3, 2),
            Index(3, 1),
        ];

        for pos in test_pos {
            let cheats = maze.cheats_for_pos(pos, 6);
            cheats.iter().for_each(|cheat_pos| {
                let md = (cheat_pos.0 - pos.0).abs() + (cheat_pos.1 - pos.1).abs();
                assert!(md <= 6);
            });
            println!("{:?} => {:?}", pos, cheats);
        }
    }

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

    #[test]
    fn test_adv_cheat_len() {
        let maze = Maze::parse(TEST_INPUT).unwrap();
        let visited = maze.shortest_path();
        let max_len = *visited.get(&maze.end).unwrap();
        assert_eq!(
            maze.adv_cheat_length(&visited, max_len, Index(1, 3), Index(3, 7)),
            max_len - 76
        );
    }

    #[test]
    fn test_cheat_paths_for_pos() {
        let maze = Maze::parse(TEST_INPUT).unwrap();
        let visited = maze.shortest_path();
        let max_len = *visited.get(&maze.end).unwrap();
        let test_pos = vec![
            Index(1, 3),
            Index(1, 2),
            Index(3, 3),
            Index(3, 2),
            Index(3, 1),
        ];
        test_pos.iter().for_each(|pos| {
            let mut cheat_map: HashMap<usize, usize> = HashMap::new();
            maze.cheat_paths_for_pos(&visited, max_len, *pos, 6, &mut cheat_map);
            println!("{:?} => {:?}", pos, cheat_map);
        });
    }

    #[test]
    fn test_find_num_adv_cheat_paths() {
        let maze = Maze::parse(TEST_INPUT).unwrap();
        let visited = maze.shortest_path();
        assert_eq!(maze.find_num_adv_cheat_paths(&visited, 6, 76), 3);
    }
}
