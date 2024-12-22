use std::{
    collections::{HashMap, VecDeque},
    usize,
};

use aoc_utils::{
    dir::{Dir, Dir4},
    grid::{Grid, GridPos},
    str::AocStr,
};

static KEYPAD: &str = "789\n456\n123\n#0A";
static DIRPAD: &str = "#^A\n<v>";
static UP: &str = "^";
static DOWN: &str = "v";
static LEFT: &str = "<";
static RIGHT: &str = ">";

struct Controls<'a> {
    codes: Vec<&'a str>,
    keypad: Grid<char>,
    dirpad: Grid<char>,
}

fn key_press_from_dir(dir: Dir4) -> &'static str {
    match dir {
        Dir4::N => UP,
        Dir4::E => RIGHT,
        Dir4::S => DOWN,
        Dir4::W => LEFT,
    }
}
impl<'a> Controls<'a> {
    fn parse(input: &str) -> Controls {
        let codes: Vec<&str> = input.lines().collect();
        let keypad = Grid::<char>::parse(KEYPAD);
        let dirpad = Grid::<char>::parse(DIRPAD);
        Controls {
            codes,
            keypad,
            dirpad,
        }
    }

    fn key_shortest_paths(&self, pad: &Grid<char>, start_c: char, c: char) -> Vec<String> {
        let start = pad.find_pos(start_c).unwrap();
        let end = pad.find_pos(c).unwrap();
        let mut visited: HashMap<GridPos, usize> = HashMap::new();
        let mut possies: VecDeque<(GridPos, String)> = VecDeque::from([(start, String::new())]);
        let mut paths: Vec<String> = Vec::new();
        let mut shortest = usize::MAX;
        while let Some((pos, path)) = possies.pop_front() {
            if shortest < path.len() {
                continue;
            } else if let Some(&len) = visited.get(&pos) {
                if len < path.len() {
                    continue;
                }
            }
            if pos == end {
                assert!(shortest >= path.len());
                shortest = path.len();
                paths.push(path + "A");
                continue;
            } else {
                visited.insert(pos, path.len());
                Dir4::cw()
                    .filter_map(|dir| {
                        let new_pos = pos + dir;
                        let key = pad.at_pos(&new_pos)?;
                        if key != '#' {
                            Some((new_pos, key_press_from_dir(dir)))
                        } else {
                            None
                        }
                    })
                    .for_each(|c| possies.push_back((c.0, path.clone() + c.1)));
            }
        }
        paths
    }

    fn keypad_shortest_segments(&self, pad: &Grid<char>, code: &str) -> Vec<Vec<String>> {
        let mut next_start_pos = 'A';
        code.chars()
            .map(|c| {
                let start_pos = next_start_pos;
                next_start_pos = c;
                self.key_shortest_paths(pad, start_pos, c)
            })
            .collect::<Vec<_>>()
    }

    fn keypad_shortest_paths(&self, pad: &Grid<char>, code: &str) -> Vec<String> {
        let segments = self.keypad_shortest_segments(pad, code);
        let len = segments.iter().map(|v| v.len()).product();
        let mut paths = Vec::<String>::new();
        segments.into_iter().for_each(|seg| {
            let mut new_paths = Vec::<String>::with_capacity(len);
            seg.iter().for_each(|end_p| {
                if paths.is_empty() {
                    new_paths.push(end_p.clone());
                } else {
                    paths
                        .iter()
                        .for_each(|start_p| new_paths.push(start_p.clone() + end_p));
                }
            });
            paths = new_paths;
        });
        assert!(paths.iter().all(|p| p.len() == paths[0].len()));
        paths
    }

    fn keypad_nth_order_shortest_path(&self, code: &str, num_robots: usize) -> Vec<String> {
        let mut paths = self.keypad_shortest_paths(&self.keypad, code);
        for _ in 0..num_robots {
            let mut paths_2nd = Vec::<String>::new();
            let mut shortest = usize::MAX;
            paths.into_iter().for_each(|p| {
                let mut paths2 = self.keypad_shortest_paths(&self.dirpad, &p);
                if paths2[0].len() < shortest {
                    shortest = paths2[0].len();
                    paths_2nd = paths2;
                } else if paths2[0].len() == shortest {
                    paths_2nd.append(&mut paths2);
                }
            });

            paths = paths_2nd;
        }

        paths
    }

    fn sequence_complexity_old(&self, code: &str, num_robots: usize) -> usize {
        let paths = self.keypad_nth_order_shortest_path(code, num_robots);
        let len = paths[0].len();
        let code_num = String::from(code.strip_suffix("A").unwrap())
            .parse::<usize>()
            .unwrap();

        len * code_num
    }

    fn shortest_path_r(
        &self,
        code: &str,
        num_robots: usize,
        cache: &mut HashMap<(String, usize), String>,
    ) -> String {
        if code == "A" {
            "A".to_string()
        } else if let Some(sp) = cache.get(&(code.to_string(), num_robots)) {
            if num_robots == 1 {
                sp.clone()
            } else {
                self.shortest_path_r(&sp.clone(), num_robots - 1, cache)
            }
        } else {
            let path = if code.len() == 2 {
                self.key_shortest_paths(&self.dirpad, code.nth(0), code.nth(1))
                    .into_iter()
                    .fold(String::new(), |sp, p| {
                        if !sp.is_empty() && sp.len() <= p.len() {
                            sp
                        } else {
                            p
                        }
                    })
            } else {
                (0..code.len() - 1)
                    .map(|i| self.shortest_path_r(&code[i..=i + 1], num_robots, cache))
                    .collect::<String>()
            };
            //assert_eq!(path.first(), Some('A'));
            if num_robots == 1 {
                path
            } else {
                assert!(path.len() > 1 || path == "A");
                cache.insert((code.to_string(), num_robots - 1), path.clone());
                self.shortest_path_r(code, num_robots - 1, cache)
            }
        }
    }

    fn shortest_path(&self, code: &str, num_robots: usize) -> String {
        let mut cache: HashMap<(String, usize), String> = HashMap::new();
        self.keypad_shortest_paths(&self.keypad, code)
            .into_iter()
            .map(|path| "A".to_string() + &path)
            .map(|dir_code| self.shortest_path_r(&dir_code, num_robots, &mut cache))
            .fold(String::new(), |sp, p| {
                if !sp.is_empty() && sp.len() <= p.len() {
                    sp
                } else {
                    p
                }
            })
    }

    fn sequence_complexity(&self, code: &str, shortest_path: &str) -> usize {
        let len = shortest_path.len();
        let code_num = String::from(code.strip_suffix("A").unwrap())
            .parse::<usize>()
            .unwrap();

        len * code_num
    }
    fn total_complexity(&self, num_robots: usize) -> usize {
        self.codes
            .iter()
            .map(|&code| self.sequence_complexity(code, &self.shortest_path(code, num_robots)))
            .sum()
    }

    fn total_complexity_old(&self, num_robots: usize) -> usize {
        self.codes
            .iter()
            .map(|c| self.sequence_complexity_old(c, num_robots))
            .sum()
    }
}

pub fn part1(input: &str) -> usize {
    let controls = Controls::parse(input);
    controls.total_complexity_old(2)
}

pub fn part2(input: &str) -> usize {
    let controls = Controls::parse(input);
    controls.total_complexity(25)
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");

    #[test]
    fn test_key_shorest_paths() {
        let controls = Controls::parse(TEST_INPUT);
        let sp = controls.key_shortest_paths(&controls.keypad, 'A', '0');
        assert_eq!(sp, vec!["<A".to_string()]);

        let sp = controls.key_shortest_paths(&controls.keypad, '0', '2');
        assert_eq!(sp, vec!["^A".to_string()]);

        let mut sp = controls.key_shortest_paths(&controls.keypad, '2', '9');
        sp.sort();
        let mut res = vec![">^^A".to_string(), "^>^A".to_string(), "^^>A".to_string()];
        res.sort();
        assert_eq!(sp, res);

        let sp = controls.key_shortest_paths(&controls.keypad, '9', 'A');
        assert_eq!(sp, vec!["vvvA".to_string()]);
    }

    #[test]
    fn test_keypad_shortest_paths() {
        let controls = Controls::parse(TEST_INPUT);
        let mut paths = controls.keypad_shortest_paths(&controls.keypad, "029A");
        paths.sort();
        let mut ans = vec!["<A^A>^^AvvvA", "<A^A^>^AvvvA", "<A^A^^>AvvvA"];
        ans.sort();
        assert_eq!(paths, ans);
    }

    #[test]
    fn test_2nd_keypad_shortest_paths() {
        let controls = Controls::parse(TEST_INPUT);
        let paths = controls.keypad_shortest_paths(&controls.keypad, "029A");
        let mut new_paths = Vec::<String>::new();
        paths.iter().for_each(|p| {
            let mut paths2 = controls.keypad_shortest_paths(&controls.dirpad, p);
            new_paths.append(&mut paths2);
        });

        let h = new_paths.iter().collect::<HashSet<_>>();
        assert_eq!(h.len(), new_paths.len());

        assert!(new_paths.contains(&"v<<A>>^A<A>AvA<^AA>A<vAAA>^A".to_string()))
    }

    #[test]
    fn test_shortest_path() {
        let controls = Controls::parse(TEST_INPUT);
        let path = controls.shortest_path("029A", 2);
        println!("shortest-path: {:?}", path);
        assert_eq!(path.len(), "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len());

        //for num_robots in 2..=25 {
        let path = controls.shortest_path("029A", 10); //num_robots);
        println!("{}: {:?}", path.len(), path);
        //}
    }

    #[test]
    fn test_sequence_complexity() {
        let controls = Controls::parse(TEST_INPUT);
        let paths = controls.keypad_nth_order_shortest_path("029A", 2);
        let h = paths.iter().collect::<HashSet<_>>();
        assert_eq!(h.len(), paths.len());
    }

    #[test]
    fn test_3rd_keypad_shortest_paths() {
        let controls = Controls::parse(TEST_INPUT);
        let paths = controls.keypad_nth_order_shortest_path("029A", 2);

        assert!(paths.contains(
            &"<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".to_string()
        ));

        assert!(paths.iter().all(|p| p.len() == paths[0].len()));
    }
}
