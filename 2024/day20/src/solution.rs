use std::collections::{HashMap, HashSet, VecDeque};

use aoc_utils::{
    dir::{Dir, Dir4},
    grud::{Grid, GridPos},
    vec2d::VecSize,
};

#[derive(Debug, Clone)]
struct Maze {
    grid: Grid<char, Dir4>,
    start: GridPos,
    end: GridPos,
}

impl Maze {
    fn parse(input: &str) -> Option<Maze> {
        let grid = Grid::<char, Dir4>::parse(input);
        let start = grid.find('S')?;
        let end = grid.find('E')?;
        Some(Maze { grid, start, end })
    }

    fn shortest_path(&self) -> HashMap<GridPos, usize> {
        let mut visited: HashMap<GridPos, usize> = HashMap::new();
        let mut possies = VecDeque::<(GridPos, usize)>::from([(self.start, 0)]);
        while let Some((pos, len)) = possies.pop_front() {
            visited.insert(pos, len);
            if pos == self.end {
                break;
            } else {
                Dir4::cw()
                    .map(|dir| pos + dir)
                    .filter(|pos_next| {
                        !self.grid.matches(pos_next, '#') && !visited.contains_key(pos_next)
                    })
                    .for_each(|pos| possies.push_back((pos, len + 1)));
            }
        }
        visited
    }

    #[cfg(test)]
    fn num_walkable(&self) -> usize {
        self.grid
            .iter()
            .filter(|&c| match c {
                '.' | 'S' | 'E' => true,
                _ => false,
            })
            .count()
    }

    #[cfg(test)]
    fn shortest_path_len(&self) -> Option<usize> {
        let visited = self.shortest_path();
        Some(*visited.get(&self.end)?)
    }

    fn single_wall(&self, pos: GridPos, dir: Dir4) -> Option<(GridPos, Dir4)> {
        let poss_wall = pos + dir;
        match self.grid.at(&poss_wall)? {
            '#' => match self.grid.at(&(poss_wall + dir))? {
                '#' => None,
                _ => Some((pos, dir)),
            },
            _ => None,
        }
    }

    fn cheats_for_pos(&self, pos: GridPos, time: i64) -> Vec<GridPos> {
        // find all path positions reachable in the time frame
        let mut cheat_pos: Vec<GridPos> = vec![];
        for y in (pos.y - time)..=(pos.y + time) {
            let xmax = time - (y - pos.y).abs();
            assert!(xmax >= 0 && xmax <= time);
            for x in (pos.x - xmax)..=(pos.x + xmax) {
                let new_pos = GridPos::new(x, y);
                if new_pos == pos {
                    continue;
                }
                if let Some(c) = self.grid.at(&new_pos) {
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
        visited: &HashMap<GridPos, usize>,
        max_len: usize,
        start_pos: GridPos,
        cheat_pos: GridPos,
    ) -> Option<usize> {
        let start_len = visited.get(&start_pos)?;
        let end_len = max_len - visited.get(&cheat_pos)?;
        let v = cheat_pos - start_pos;
        let cheat_len = v.manhattan() as usize;
        Some(start_len + cheat_len + end_len)
    }

    fn num_cheat_paths_for_pos(
        &self,
        visited: &HashMap<GridPos, usize>,
        max_len: usize,
        pos: GridPos,
        time: i64,
        min_reduction: usize,
    ) -> usize {
        let cheats = self.cheats_for_pos(pos, time);
        cheats
            .iter()
            .filter_map(|&cheat_pos| self.adv_cheat_length(visited, max_len, pos, cheat_pos))
            .filter(|&cheat_len| max_len.saturating_sub(cheat_len) >= min_reduction)
            .count()
    }

    #[cfg(test)]
    fn cheat_paths_for_pos(
        &self,
        visited: &HashMap<GridPos, usize>,
        max_len: usize,
        pos: GridPos,
        time: i64,
        min_reduction: usize,
    ) -> Vec<(GridPos, usize)> {
        let cheats = self.cheats_for_pos(pos, time);
        cheats
            .iter()
            .filter_map(|&cheat_pos| {
                if let Some(cheat_len) = self.adv_cheat_length(visited, max_len, pos, cheat_pos) {
                    if max_len.saturating_sub(cheat_len) >= min_reduction {
                        Some((cheat_pos, max_len.saturating_sub(cheat_len)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    #[cfg(test)]
    fn filtered_cheat_paths_for_pos(
        &self,
        visited: &HashMap<GridPos, usize>,
        time: i64,
        min_reduction: usize,
    ) -> Vec<(GridPos, Vec<(GridPos, usize)>)> {
        let Some(&max_len) = visited.get(&self.end) else {
            return vec![];
        };
        visited
            .iter()
            .map(|(&pos, _path_len)| {
                (
                    pos,
                    self.cheat_paths_for_pos(visited, max_len, pos, time, min_reduction),
                )
            })
            .filter(|(_pos, cheats)| !cheats.is_empty())
            .collect()
    }

    fn find_num_adv_cheat_paths(
        &self,
        visited: &HashMap<GridPos, usize>,
        time: i64,
        min_reduction: usize,
    ) -> usize {
        let Some(&max_len) = visited.get(&self.end) else {
            return 0;
        };
        visited
            .iter()
            .map(|(&pos, _)| {
                self.num_cheat_paths_for_pos(visited, max_len, pos, time, min_reduction)
            })
            .sum()
    }

    fn removable_walls(&self, visited: &HashMap<GridPos, usize>) -> Vec<(GridPos, Dir4)> {
        let mut walls = HashSet::<(GridPos, Dir4)>::new();
        visited.keys().for_each(|&pos| {
            Dir4::cw()
                .filter_map(|dir| self.single_wall(pos, dir))
                .for_each(|pos| _ = walls.insert(pos));
        });
        walls.iter().copied().collect()
    }

    fn cheat_length(
        &self,
        max_len: usize,
        visited: &HashMap<GridPos, usize>,
        wall: (GridPos, Dir4),
    ) -> Option<usize> {
        let start_len = visited.get(&wall.0)?;
        let cheat_pos = wall.0 + wall.1 + wall.1;
        let cheat_len = visited.get(&cheat_pos)?;
        let end_len = max_len - cheat_len + 1;
        Some(start_len + 1 + end_len)
    }

    fn find_num_cheat_paths(
        &self,
        visited: &HashMap<GridPos, usize>,
        min_reduction: usize,
    ) -> usize {
        let Some(&max_path_len) = visited.get(&self.end) else {
            return 0;
        };
        let walls = self.removable_walls(visited);
        let min_len = max_path_len - min_reduction;
        let num_cheats = walls
            .iter()
            .filter_map(|&wall| self.cheat_length(max_path_len, visited, wall))
            .filter(|&len| len <= min_len)
            .count();

        num_cheats
    }
}

// 1. find shortest path: t=len(shortest)
// 2. find cheat such that tc < t - 100
//    - only look at walls adjacent to shortest path
//    - ignore "double" walls
pub fn part1(input: &str) -> usize {
    let maze = Maze::parse(input).expect("valid maze");
    let visited = maze.shortest_path();
    maze.find_num_cheat_paths(&visited, 100)
}

pub fn part2(input: &str) -> usize {
    let maze = Maze::parse(input).expect("valid maze");
    let visited = maze.shortest_path();
    maze.find_num_adv_cheat_paths(&visited, 20, 100)
}

#[cfg(test)]
mod tests {

    use super::*;
    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const HACKED_TEST_INPUT: &str = include_str!("data/input_example_hacked");

    #[test]
    fn test_cheats_for_pos() {
        let maze = Maze::parse(TEST_INPUT).unwrap();
        let test_pos = vec![
            GridPos::new(1, 3),
            GridPos::new(1, 2),
            GridPos::new(3, 3),
            GridPos::new(3, 2),
            GridPos::new(3, 1),
        ];

        for pos in test_pos {
            let cheats = maze.cheats_for_pos(pos, 6);
            cheats.iter().for_each(|cheat_pos| {
                let md = (cheat_pos.x - pos.x).abs() + (cheat_pos.y - pos.y).abs();
                assert!(md <= 6);
            });
            println!("{:?} => {:?}", pos, cheats);
        }
    }

    use aoc_utils::lust::Lust;

    #[test]
    fn test_num_paths() {
        let maze = Maze::parse(HACKED_TEST_INPUT).unwrap();
        let mut to_visit: VecDeque<Lust<GridPos>> = VecDeque::from([Lust::new(maze.start)]);
        let mut paths: Vec<Lust<GridPos>> = vec![];
        while let Some(path) = to_visit.pop_front() {
            if let Some(&pos) = path.data() {
                if pos == maze.end {
                    paths.push(path);
                } else {
                    Dir4::cw()
                        .map(|dir| pos + dir)
                        .filter(|pos_next| {
                            !maze.grid.matches(pos_next, '#') && !path.contains(pos_next)
                        })
                        .for_each(|pos_next| {
                            to_visit.push_back(path.append(pos_next));
                        });
                }
            }
        }
        println!("num paths: {}", paths.len());
        for path in paths {
            println!("{}", path);
        }
    }

    #[test]
    fn test_shortest_path() {
        let maze = Maze::parse(TEST_INPUT).expect("valid maze");
        assert_eq!(maze.shortest_path_len(), Some(84));
    }

    #[test]
    fn test_num_walkable() {
        let maze = Maze::parse(TEST_INPUT).expect("valid maze");
        assert_eq!(maze.num_walkable(), maze.shortest_path().len());
    }

    #[test]
    fn test_removable_walls() {
        let maze = Maze::parse(TEST_INPUT).expect("valid maze");
        let visited = maze.shortest_path();
        let x = maze.removable_walls(&visited);
        println!("{:?}", x);
        assert!(x.contains(&(GridPos::new(3, 1), Dir4::E)));
        assert!(x.contains(&(GridPos::new(12, 5), Dir4::S)));
        assert!(x.contains(&(GridPos::new(12, 7), Dir4::N)));
    }

    #[test]
    fn test_cheat_len() {
        let maze = Maze::parse(TEST_INPUT).expect("valid maze");
        let visited = maze.shortest_path();
        let mut test = visited.iter().map(|x| (*x.0, *x.1)).collect::<Vec<_>>();
        test.sort_by(|a, b| a.1.cmp(&b.1));
        println!("{:?}", test);
        let max_len = *visited.get(&maze.end).unwrap();
        assert_eq!(
            maze.cheat_length(max_len, &visited, (GridPos::new(7, 7), Dir4::W))
                .expect("wall"),
            20
        );
        assert_eq!(
            maze.cheat_length(max_len, &visited, (GridPos::new(8, 7), Dir4::S))
                .expect("wall"),
            max_len - 38
        );
    }

    #[test]
    fn test_find_num_cheat_paths() {
        let maze = Maze::parse(TEST_INPUT).expect("valid maze");
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
        let maze = Maze::parse(TEST_INPUT).expect("valid maze");
        let visited = maze.shortest_path();
        let max_len = *visited.get(&maze.end).expect("end");
        assert_eq!(
            maze.adv_cheat_length(&visited, max_len, GridPos::new(1, 3), GridPos::new(3, 7)),
            Some(max_len - 76)
        );
    }

    #[test]
    fn test_cheat_paths_for_pos() {
        let maze = Maze::parse(TEST_INPUT).expect("valid maze");
        let visited = maze.shortest_path();
        let max_len = *visited.get(&maze.end).unwrap();
        let test_pos = vec![
            GridPos::new(1, 3),
            GridPos::new(1, 2),
            GridPos::new(3, 3),
            GridPos::new(3, 2),
            GridPos::new(3, 1),
        ];
        test_pos.iter().for_each(|pos| {
            let cheats = maze.cheat_paths_for_pos(&visited, max_len, *pos, 6, 20);
            println!("{} => {} - {:?}", pos, cheats.len(), cheats);
        });
    }

    #[test]
    fn test_filtered_cheat_paths_for_pos() {
        let maze = Maze::parse(TEST_INPUT).expect("valid maze");
        let visited = maze.shortest_path();
        let a = maze.filtered_cheat_paths_for_pos(&visited, 20, 75);
        for (pos, cheats) in a {
            println!("{} => {} - {:?}", pos, cheats.len(), cheats);
        }
    }

    #[test]
    fn test_find_num_adv_cheat_paths() {
        let expected_results = vec![
            (3, 76),
            (4, 74),
            (22, 72),
            (12, 70),
            (14, 68),
            (12, 66),
            (19, 64),
            (20, 62),
            (23, 60),
            (25, 58),
            (39, 56),
            (29, 54),
            (31, 52),
            (32, 50),
        ];
        let maze = Maze::parse(TEST_INPUT).expect("valid maze");
        let visited = maze.shortest_path();
        println!("Shortest path length: {}", visited.len());
        println!(
            "End pos: {:?} = {}",
            maze.end,
            visited.get(&maze.end).unwrap()
        );
        expected_results
            .iter()
            .for_each(|(expected_cheats, reduction)| {
                let x = maze.filtered_cheat_paths_for_pos(&visited, 20, *reduction);
                println!("{:?}", x);
                let num: usize = x
                    .iter()
                    .map(|(_pos, cheats)| {
                        cheats
                            .iter()
                            .filter(|(_pos, size)| *size == *reduction)
                            .count()
                    })
                    .sum();
                assert_eq!(num, *expected_cheats, "reduction={}", reduction);
            });
    }
}
