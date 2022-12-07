use itertools::Itertools;
use lazy_static::{__Deref, lazy_static};
use std::{collections::HashMap, ops::DerefMut};

use crate::aoc::Aoc;

type DirMap = HashMap<String, Dir>;

#[derive(PartialEq, Debug)]
struct Dir {
    file_size: usize,
    dirs: DirMap,
}

impl Dir {
    fn new() -> Self {
        Dir {
            file_size: 0,
            dirs: Default::default(),
        }
    }
    fn make(file_size: usize, dirs: DirMap) -> Self {
        Dir { file_size, dirs }
    }
}

fn cd_to(cur_path: &str, new_dir: &str) -> String {
    format!("{cur_path}/{new_dir}")
}

fn lines_to_cmd_strs(lines: &Vec<String>) -> Vec<&[String]> {
    let mut res: Vec<&[String]> = Default::default();
    let mut start_idx = 0;
    res = lines[1..]
        .iter()
        .enumerate()
        .fold(res, |mut r, line| -> Vec<&[String]> {
            if (line.1.chars().next().unwrap() == '$') {
                r.push(&lines[start_idx..line.0 + 1]);
                start_idx = line.0 + 1;
            }
            r
        });
    res.push(&lines[start_idx..]);
    res
}

fn cd_to_dir<'a>(stack: &mut Vec<&'a mut Dir>, new_dir: &str) -> &'a mut Dir {
    let mut new_stack = stack;
    let cur_dir = new_stack.last().unwrap();

    match new_dir {
        ".." => {
            new_stack.pop();
            new_stack.last_mut().unwrap()
        }
        "/" => {
            new_stack.remove(new_stack.len() - 1);
            new_stack.first_mut().unwrap()
        }
        _ => {
            let new_dir = &mut cur_dir.dirs[new_dir];
            new_stack.push(new_dir);
            new_dir
        }
    }
}

fn ls_dir(cur_dir: &mut Dir, dir_entries: &[String]) {
    for item in dir_entries {
        let item_args = item.split(' ').collect_vec();
        match item_args[0] {
            "dir" => {
                cur_dir.dirs.insert(String::from(item_args[1]), Dir::new());
            }
            _ => {
                cur_dir.file_size += item_args[0].parse::<usize>().unwrap();
            }
        };
    }
}

fn cmd_strs_to_dirs(cmd_strs: &Vec<&[String]>) -> Dir {
    let mut root: Dir = Dir::new();
    let mut stack: Vec<&mut Dir> = vec![&mut root];
    let mut curr_dir = &mut root;
    cmd_strs.iter().fold(stack, |acc, cmd_str| {
        let args = cmd_str[0].split(' ').collect_vec();
        match args[1] {
            "cd" => curr_dir = cd_to_dir(&mut stack, args[2]),
            "ls" => ls_dir(curr_dir, &cmd_str[1..]),
        }
    });
    root
}

/*
fn filesystem_from_cmds(cmds: &Vec<String>) -> Option<Dir> {
    //let mut fs : Option<Dir> = None;
    cmds.iter()
        .fold(None, |mut acc, line| -> Option<Dir> { parse_cmd() })
}
*/
pub struct Day7_1;
impl Aoc<u32> for Day7_1 {
    fn day(&self) -> u32 {
        7
    }
    fn puzzle_name(&self) -> &str {
        "No Space"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        //let fs = filesystem_from_cmds(lines);
        Default::default()
    }
}

pub struct Day7_2;
impl Aoc<u32> for Day7_2 {
    fn day(&self) -> u32 {
        7
    }
    fn puzzle_name(&self) -> &str {
        "??? 2"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_lines_to_cmds() {
        assert_eq!(
            lines_to_cmd_strs(&CMD_LINES),
            vec![
                &CMD_LINES[0..1],
                &CMD_LINES[1..3],
                &CMD_LINES[3..4],
                &CMD_LINES[4..7]
            ]
        )
    }

    #[test]
    fn test_cmd_strs_to_dirs() {
        let root_dm = DirMap::default();
        root_dm["a"] = Dir::make(3, DirMap::default());
        assert_eq!(
            cmd_strs_to_dirs(&lines_to_cmd_strs(&CMD_LINES)),
            Dir::make(0, root_dm)
        );
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
    fn test_dir_structure() {
        let input_strs = INPUT
            .map(|s| String::from(s))
            .into_iter()
            .collect::<Vec<String>>();
    }

    #[test]
    fn test_soln2() {
        assert_eq!(1, 1);
    }
}
