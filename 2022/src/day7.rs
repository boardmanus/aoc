use itertools::Itertools;
use std::collections::HashMap;

use crate::aoc::Aoc;

type FileMap = HashMap<String, usize>;

fn process_cmds<'a>(
    cmd_strs: &'a [String],
    cwd: &String,
    file_counts: &mut FileMap,
) -> (&'a [String], usize, bool) {
    let mut rem_cmd_strs = cmd_strs;
    let existing_files_size = file_counts.get_mut(cwd);

    if existing_files_size.is_some() && cwd != "/" {
        return (cmd_strs, 0, false);
    }

    let mut files_size = 0;
    while rem_cmd_strs.len() > 0 {
        let cmd_str = rem_cmd_strs.first().unwrap();
        rem_cmd_strs = &rem_cmd_strs[1..];
        let item_args = cmd_str.split(' ').collect_vec();

        match item_args[0] {
            "$" => match item_args[1] {
                "cd" => match item_args[2] {
                    "/" => {
                        if cwd != "/" {
                            return (rem_cmd_strs, files_size, true);
                        }
                    }
                    ".." => {
                        return (rem_cmd_strs, files_size, false);
                    }
                    _ => {
                        let res = process_cmds(
                            rem_cmd_strs,
                            &format!("{}{}/", cwd, &item_args[2]),
                            file_counts,
                        );
                        rem_cmd_strs = res.0;
                        files_size += res.1;
                        if res.2 {
                            file_counts.insert(cwd.to_string(), files_size);
                            return (res.0, files_size, true);
                        }
                    }
                },
                "ls" => (),
                _ => panic!(),
            },
            "dir" => (),
            _ => {
                files_size += item_args[0].parse::<usize>().unwrap();
            }
        }

        file_counts.insert(cwd.to_string(), files_size);
    }

    (rem_cmd_strs, files_size, false)
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
        let mut file_counts: FileMap = Default::default();
        process_cmds(&lines[..], &String::from("/"), &mut file_counts);
        file_counts
            .iter()
            .fold(0, |acc, size| -> usize {
                if *size.1 <= 100000 {
                    acc + size.1
                } else {
                    acc
                }
            })
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
        let mut file_counts: FileMap = Default::default();
        process_cmds(&lines[..], &String::from("/"), &mut file_counts);
        let used_space = file_counts["/"];
        let free_space = 70000000 - used_space;
        let delete_size = 30000000 - free_space;
        file_counts
            .iter()
            .filter(|a| *a.1 >= delete_size as usize)
            .fold(usize::MAX, |min, v| if *v.1 < min { *v.1 } else { min })
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
