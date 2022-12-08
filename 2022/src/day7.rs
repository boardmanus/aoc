use itertools::Itertools;
use std::collections::HashMap;

use crate::aoc::Aoc;

type FileMap = HashMap<String, usize>;

fn process_cmds(cmd_strs: &[String]) -> FileMap {
    let mut dir_sizes = FileMap::default();
    let mut pwd = vec![""];

    for cmd in cmd_strs {
        match cmd.split(' ').collect_vec().as_slice() {
            ["$", "cd", "/"] => pwd = vec![""],
            ["$", "cd", ".."] => {
                pwd.pop();
            }
            ["$", "cd", dir] => pwd.push(dir),
            ["$", "ls"] => (),
            ["dir", _] => (),
            [file_size_str, _filename] => {
                let file_size = file_size_str.parse::<usize>().unwrap();
                let mut prev_dir = "".to_string();
                pwd.iter()
                    .map(|dir| {
                        let path = format!("{prev_dir}{dir}/");
                        prev_dir = path.clone();
                        path
                    })
                    .for_each(|path| {
                        if let Some(dir_size) = dir_sizes.get_mut(&path) {
                            *dir_size += file_size;
                        } else {
                            dir_sizes.insert(path, file_size);
                        }
                    });
            }
            _ => panic!(),
        }
    }
    dir_sizes
}

pub struct Day7_1;
impl Aoc for Day7_1 {
    fn day(&self) -> u32 {
        7
    }
    fn puzzle_name(&self) -> &str {
        "No Space"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        process_cmds(&lines[..])
            .values()
            .filter(|size| *size < &100000)
            .sum::<usize>()
            .to_string()
    }
}

pub struct Day7_2;
impl Aoc for Day7_2 {
    fn day(&self) -> u32 {
        7
    }
    fn puzzle_name(&self) -> &str {
        "No Space 2"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        let file_counts = process_cmds(&lines[..]);
        let used_space = file_counts["/"];
        let free_space = 70000000 - used_space;
        let delete_size = 30000000 - free_space;
        file_counts
            .iter()
            .filter(|a| *a.1 >= delete_size as usize)
            .map(|v| v.1)
            .min()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    const INPUT: [&str; 23] = [
        "$ cd /",
        "$ ls",
        "dir a",
        "14848514 b.txt",
        "8504156 c.dat",
        "dir d",
        "$ cd a",
        "$ ls",
        "dir e",
        "29116 f",
        "2557 g",
        "62596 h.lst",
        "$ cd e",
        "$ ls",
        "584 i",
        "$ cd ..",
        "$ cd ..",
        "$ cd d",
        "$ ls",
        "4060174 j",
        "8033020 d.log",
        "5626152 d.ext",
        "7214296 k",
    ];

    lazy_static! {
        static ref CMD_LINES: Vec<String> = vec![
            String::from("$ cd /"),
            String::from("$ ls"),
            String::from("dir a"),
            String::from("$ cd a"),
            String::from("$ ls"),
            String::from("1 b"),
            String::from("2 c"),
        ];
    }

    #[test]
    fn test_soln() {
        let input_strs = INPUT
            .map(|s| String::from(s))
            .into_iter()
            .collect::<Vec<String>>();

        assert_eq!(Day7_1.solve(&input_strs), 95437.to_string());
    }

    #[test]
    fn test_soln2() {
        let input_strs = INPUT
            .map(|s| String::from(s))
            .into_iter()
            .collect::<Vec<String>>();

        assert_eq!(Day7_2.solve(&input_strs), 24933642.to_string());
    }
}
