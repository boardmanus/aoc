use std::collections::HashMap;

use aoc_utils::{
    dir::Dir4,
    grud::{Grid, GridPos},
};

static NUM_PAD: &str = "789\n456\n123\n#0A";
static DIR_PAD: &str = "#^A\n<v>";
static UP: &str = "^";
static DOWN: &str = "v";
static LEFT: &str = "<";
static RIGHT: &str = ">";

type KeyPad = Grid<char, Dir4>;

struct Controls<'a> {
    codes: Vec<&'a str>,
    cache: HashMap<(char, char), Vec<String>>,
}

impl<'a> Controls<'a> {
    fn cache_keypad(pad: &KeyPad, cache: &mut HashMap<(char, char), Vec<String>>) {
        for (start_pos, start_key) in pad.iter_pair() {
            for (end_pos, end_key) in pad.iter_pair() {
                if start_key == '#' || end_key == '#' {
                    continue;
                }

                if start_key == end_key {
                    cache.insert((start_key, end_key), vec!["A".to_string()]);
                    continue;
                }

                let v = end_pos - start_pos;
                let vertical = if v.y > 0 { DOWN } else { UP }.repeat(v.y.unsigned_abs() as usize);
                let horizontal =
                    if v.x > 0 { RIGHT } else { LEFT }.repeat(v.x.unsigned_abs() as usize);

                let mut paths = vec![];
                if v.x != 0 && pad.at(&GridPos::new(end_pos.x, start_pos.y)) != Some('#') {
                    paths.push(horizontal.clone() + &vertical + "A");
                }

                if v.y != 0 && pad.at(&GridPos::new(start_pos.x, end_pos.y)) != Some('#') {
                    paths.push(vertical + &horizontal + "A");
                }

                cache.insert((start_key, end_key), paths);
            }
        }
    }

    fn presses(&self, start: char, end: char) -> std::slice::Iter<'_, String> {
        self.cache[&(start, end)].iter()
    }

    fn parse(input: &'a str) -> Controls<'a> {
        let codes = input.lines().collect::<Vec<&str>>();
        let keypad = KeyPad::parse(NUM_PAD);
        let dirpad = KeyPad::parse(DIR_PAD);
        let mut cache = HashMap::new();
        Controls::cache_keypad(&keypad, &mut cache);
        Controls::cache_keypad(&dirpad, &mut cache);

        Controls { codes, cache }
    }

    fn shortest_path_r(
        &self,
        code: &str,
        num_robots: usize,
        shortest_path_cache: &mut HashMap<(char, char, usize), usize>,
    ) -> usize {
        if num_robots == 0 {
            return code.len();
        }

        let mut all_presses = 0;
        let mut start = 'A';
        for end in code.chars() {
            let cache_key = (start, end, num_robots);

            all_presses += shortest_path_cache
                .get(&cache_key)
                .copied()
                .unwrap_or_else(|| {
                    let presses = self
                        .presses(start, end)
                        .map(|path| self.shortest_path_r(path, num_robots - 1, shortest_path_cache))
                        .min()
                        .expect("at least one path");
                    shortest_path_cache.insert(cache_key, presses);

                    presses
                });
            start = end;
        }

        all_presses
    }

    fn shortest_path(&self, code: &str, num_robots: usize) -> usize {
        self.shortest_path_r(code, num_robots + 1, &mut HashMap::new())
    }

    fn sequence_complexity(&self, code: &str, shortest_path: usize) -> usize {
        let code_num = String::from(code.strip_suffix("A").unwrap())
            .parse::<usize>()
            .unwrap();

        shortest_path * code_num
    }

    fn total_complexity(&self, num_robots: usize) -> usize {
        self.codes
            .iter()
            .map(|&code| self.sequence_complexity(code, self.shortest_path(code, num_robots)))
            .sum()
    }
}

pub fn part1(input: &str) -> usize {
    let controls = Controls::parse(input);
    controls.total_complexity(2)
}

pub fn part2(input: &str) -> usize {
    let controls = Controls::parse(input);
    controls.total_complexity(25)
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const PART1_RESULT: usize = 126384;

    #[test]
    fn test_num_keypad() {
        let controls = Controls::parse(TEST_INPUT);
        let sp = controls.presses('A', '0').cloned().collect::<Vec<_>>();
        assert_eq!(sp, vec!["<A".to_string()]);

        let sp = controls.presses('0', '2').cloned().collect::<Vec<_>>();
        assert_eq!(sp, vec!["^A".to_string()]);

        let sp = controls.presses('2', '9').cloned().collect::<Vec<_>>();
        assert_eq!(sp, vec![">^^A".to_string(), "^^>A".to_string(),]);

        let sp = controls.presses('9', 'A').cloned().collect::<Vec<_>>();
        assert_eq!(sp, vec!["vvvA".to_string()]);

        let sp = controls.presses('9', 'A').cloned().collect::<Vec<_>>();
        assert_eq!(sp, vec!["vvvA".to_string()]);

        let sp = controls.presses('6', '4').cloned().collect::<Vec<_>>();
        assert_eq!(sp, vec!["<<A".to_string()]);

        let sp = controls.presses('1', '0').cloned().collect::<Vec<_>>();
        assert_eq!(sp, vec![">vA".to_string()]);

        let sp = controls.presses('A', '<').cloned().collect::<Vec<_>>();
        assert_eq!(sp, vec!["v<<A".to_string()]);

        let sp = controls.presses('^', '>').cloned().collect::<Vec<_>>();
        assert_eq!(sp, vec![">vA".to_string(), "v>A".to_string()]);
    }

    #[test]
    fn test_part1() {
        let result = part1(TEST_INPUT);
        assert_eq!(result, PART1_RESULT);
    }
}
