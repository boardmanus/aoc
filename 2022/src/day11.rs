use std::collections::HashMap;

use crate::aoc::Aoc;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

type Op = Box<dyn Fn(usize) -> usize>;
type ThrowUpdate = HashMap<usize, Vec<usize>>;

struct Monkey {
    num: usize,
    items: Vec<usize>,
    op: Op,
    div_by: usize,
    to_monkey: [usize; 2],
    num_inspected: usize,
}

fn parse_monkey_num(line: &str) -> usize {
    lazy_static! {
        static ref MONKEY_NUM: Regex = Regex::new(r"^Monkey (\d+):$").unwrap();
    }
    let caps = MONKEY_NUM.captures(line).unwrap();
    caps.get(1).unwrap().as_str().parse::<usize>().unwrap()
}

fn parse_starting_items(line: &str) -> Vec<usize> {
    let list_str = line.split_once(": ").unwrap().1;
    let items = list_str.split(", ");
    items
        .map(|istr| istr.parse::<usize>().unwrap())
        .collect_vec()
}

fn parse_operation(line: &str) -> Op {
    let tail_str = line.split_once("= old ").unwrap().1;
    let op_str = tail_str.split_once(' ').unwrap();
    match op_str {
        ("+", "old") => Box::new(move |old: usize| old + old),
        ("*", "old") => Box::new(move |old: usize| old * old),
        ("+", n_str) => {
            let n = n_str.parse::<usize>().unwrap();
            Box::new(move |old| old + n)
        }
        ("*", n_str) => {
            let n = n_str.parse::<usize>().unwrap();
            Box::new(move |old| old * n)
        }
        (_, _) => panic!(),
    }
}

fn parse_div_by(line: &str) -> usize {
    lazy_static! {
        static ref DIV_BY: Regex = Regex::new(r"^\s+Test: divisible by (\d+)$").unwrap();
    }
    let caps = DIV_BY.captures(line).unwrap();
    caps.get(1).unwrap().as_str().parse::<usize>().unwrap()
}

fn parse_throw_num(line: &str) -> usize {
    lazy_static! {
        static ref RES_NUM: Regex = Regex::new(r"^\s+If .+: throw to monkey (\d+)$").unwrap();
    }
    let caps = RES_NUM.captures(line).unwrap();
    caps.get(1).unwrap().as_str().parse::<usize>().unwrap()
}

fn parse_monkey(lines: &[String]) -> Monkey {
    let num = parse_monkey_num(&lines[0]);
    let items = parse_starting_items(&lines[1]);
    let op = parse_operation(&lines[2]);
    let div_by = parse_div_by(&lines[3]);
    let to_monkey = [parse_throw_num(&lines[5]), parse_throw_num(&lines[4])];

    Monkey {
        num,
        items,
        op,
        div_by,
        to_monkey,
        num_inspected: 0,
    }
}

fn parse_monkeys(lines: &Vec<String>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Default::default();
    for i in 0..(lines.len() + 1) / 7 {
        monkeys.push(parse_monkey(&lines[i * 7..i * 7 + 6]));
    }
    monkeys
}

fn update_monkey(part: usize, throws: &mut ThrowUpdate, m: &mut Monkey, worry_div: usize) {
    for t in throws.get(&m.num).unwrap() {
        m.items.push(*t);
    }
    throws.get_mut(&m.num).unwrap().clear();

    for item in &m.items {
        let pre_worry: usize = (m.op)(*item);
        let worry = if part == 1 {
            pre_worry / worry_div
        } else {
            pre_worry % worry_div
        };

        m.num_inspected += 1;
        let test = ((worry != 0) && (worry % m.div_by == 0)) as usize;
        throws.get_mut(&m.to_monkey[test]).unwrap().push(worry);
    }
    m.items.clear();
}

pub struct Day11_1;
impl Aoc for Day11_1 {
    fn day(&self) -> u32 {
        11
    }
    fn puzzle_name(&self) -> &str {
        "Monkey in the Middle"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        let mut monkeys = parse_monkeys(lines);
        let mut throws: ThrowUpdate = Default::default();
        for m in &monkeys {
            throws.insert(m.num, Default::default());
        }

        for _ in 0..20 {
            for m in &mut monkeys {
                update_monkey(1, &mut throws, m, 3);
            }
        }

        let mut a = monkeys.iter().map(|m| m.num_inspected).collect_vec();
        a.sort_by(|a, b| b.cmp(a));
        (a[0] * a[1]).to_string()
    }
}

pub struct Day11_2;
impl Aoc for Day11_2 {
    fn day(&self) -> u32 {
        11
    }
    fn puzzle_name(&self) -> &str {
        "Monkey in the Middle 2"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        let mut monkeys = parse_monkeys(lines);
        let mut throws: ThrowUpdate = Default::default();

        // Generate a common factor for all the monkey mod operations... to stop
        // the xplodz
        let mod_val: usize = monkeys.iter().fold(1, |new_mod, m| new_mod * m.div_by);

        for m in &monkeys {
            throws.insert(m.num, Default::default());
        }

        for _ in 0..10000 {
            for m in &mut monkeys {
                update_monkey(2, &mut throws, m, mod_val);
            }
        }

        let mut a = monkeys.iter().map(|m| m.num_inspected).collect_vec();
        a.sort_by(|a, b| b.cmp(a));
        (a[0] * a[1]).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_monkey_num() {
        assert_eq!(parse_monkey_num("Monkey 0:"), 0);
    }

    #[test]
    fn test_parse_starting_items() {
        assert_eq!(
            parse_starting_items("  Starting items: 75, 70, 82, 83, 96, 64, 62"),
            vec![75, 70, 82, 83, 96, 64, 62]
        );
    }

    #[test]
    fn test_parse_op() {
        assert_eq!(parse_operation("  Operation: new = old + 8")(7), 15);
        assert_eq!(parse_operation("  Operation: new = old * 28")(2), 56);
        assert_eq!(parse_operation("  Operation: new = old + old")(11), 22);
        assert_eq!(parse_operation("  Operation: new = old * old")(4), 16);
    }

    #[test]
    fn test_parse_div_by() {
        assert_eq!(parse_div_by("  Test: divisible by 17"), 17);
    }

    #[test]
    fn test_parse_throw_num() {
        assert_eq!(parse_throw_num("    If true: throw to monkey 7"), 7);
        assert_eq!(parse_throw_num("    If false: throw to monkey 76"), 76);
    }

    const INPUT: [&str; 27] = [
        "Monkey 0:",
        "  Starting items: 79, 98",
        "  Operation: new = old * 19",
        "  Test: divisible by 23",
        "    If true: throw to monkey 2",
        "    If false: throw to monkey 3",
        "",
        "Monkey 1:",
        "  Starting items: 54, 65, 75, 74",
        "  Operation: new = old + 6",
        "  Test: divisible by 19",
        "    If true: throw to monkey 2",
        "    If false: throw to monkey 0",
        "",
        "Monkey 2:",
        "  Starting items: 79, 60, 97",
        "  Operation: new = old * old",
        "  Test: divisible by 13",
        "    If true: throw to monkey 1",
        "    If false: throw to monkey 3",
        "",
        "Monkey 3:",
        "  Starting items: 74",
        "  Operation: new = old + 3",
        "  Test: divisible by 17",
        "    If true: throw to monkey 0",
        "    If false: throw to monkey 1",
    ];

    #[test]
    fn test_parse_monkeys() {
        let input_strs = INPUT
            .map(|s| String::from(s))
            .into_iter()
            .collect::<Vec<String>>();

        let monkeys = parse_monkeys(&input_strs);
        assert_eq!(monkeys.len(), 4);
        assert_eq!(monkeys[2].items.len(), 3);
    }

    #[test]
    fn test_soln() {
        let input_strs = INPUT
            .map(|s| String::from(s))
            .into_iter()
            .collect::<Vec<String>>();

        assert_eq!(Day11_1.solve(&input_strs), 10605.to_string());
    }

    #[test]
    fn test_soln2() {
        let input_strs = INPUT
            .map(|s| String::from(s))
            .into_iter()
            .collect::<Vec<String>>();

        assert_eq!(Day11_2.solve(&input_strs), 2713310158usize.to_string());
    }
}
