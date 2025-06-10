use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs,
};

use aoc_utils::{
    grif::{simple, Graph},
    str::AocStr,
};
use graphviz_rust::{cmd::Format, exec, printer::PrinterContext};

type SimpleGraph<'a> = simple::SimpleGraph<u64>;
type SimpleGraphBuilder<'a> = simple::SimpleGraphBuilder<u64>;

#[derive(Debug, Copy, Clone)]
struct Rule {
    before: u64,
    after: u64,
}

#[derive(Debug, Clone, Eq)]
struct Page {
    num: u64,
    before: HashSet<u64>,
    after: HashSet<u64>,
}

impl Page {
    fn new(num: u64) -> Page {
        Page {
            num,
            before: HashSet::<_>::new(),
            after: HashSet::<_>::new(),
        }
    }

    fn add_after(&mut self, page: u64) {
        self.after.insert(page);
    }

    fn add_before(&mut self, page: u64) {
        self.before.insert(page);
    }
}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}

type PageOrdering = HashMap<u64, Page>;

fn parse_input(input: &str) -> (Vec<Rule>, Vec<Vec<u64>>) {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    assert_eq!(sections.len(), 2);
    let rules = sections[0]
        .lines()
        .map(|line| {
            let mut a = line
                .split("|")
                .map(|page_str| page_str.parse::<u64>().unwrap());
            Rule {
                before: a.next().unwrap(),
                after: a.next().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let pages = sections[1]
        .lines()
        .map(|line| {
            line.split(",")
                .map(|page_str| page_str.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (rules, pages)
}

fn page_ordering(rules: &Vec<Rule>) -> PageOrdering {
    rules.iter().fold(PageOrdering::new(), |mut coll, rule| {
        coll.entry(rule.before)
            .or_insert(Page::new(rule.before))
            .add_after(rule.after);
        coll.entry(rule.after)
            .or_insert(Page::new(rule.after))
            .add_before(rule.before);
        coll
    })
}

fn is_before(ordering: &PageOrdering, page_nums: &Vec<u64>, page_a: u64, page_b: u64) -> bool {
    let mut afters_seen = HashSet::<u64>::from([page_a]);
    let mut afters = HashSet::<u64>::from([page_a]);

    while !afters.is_empty() {
        if afters.contains(&page_b) {
            return true;
        }

        afters = afters
            .into_iter()
            .map(|a| &ordering.get(&a).unwrap().after)
            .flatten()
            .filter(|&a| !afters_seen.contains(a) && page_nums.contains(a))
            .fold(HashSet::<u64>::new(), |mut coll, a| {
                coll.insert(*a);
                coll
            });
        afters.iter().for_each(|a| {
            afters_seen.insert(*a);
        });
    }

    false
}

fn good_order(ordering: &PageOrdering, page_nums: &Vec<u64>) -> bool {
    (0..page_nums.len() - 1)
        .all(|index| is_before(ordering, page_nums, page_nums[index], page_nums[index + 1]))
}

fn good_order_graph(
    graph: &SimpleGraph,
    pages: &Vec<u64>,
    cache: &mut HashMap<(u64, u64), bool>,
) -> bool {
    let good = pages
        .iter()
        .zip(pages.iter().skip(1))
        .all(|(&before, &after)| {
            if let Some(&good_order) = cache.get(&(before, after)) {
                good_order
            } else {
                let good_order = graph
                    .dfs(before, |n| pages.contains(&n))
                    .find(|&x| x == after)
                    .is_some()
                    || graph
                        .dfs(after, |n| pages.contains(&n))
                        .find(|&x| x == before)
                        .is_none();

                cache.insert((before, after), good_order);
                cache.insert((after, before), !good_order);
                good_order
            }
        });
    good
}

fn reorder_pages(ordering: &PageOrdering, page_nums: &Vec<u64>) -> Vec<u64> {
    let mut good_order = page_nums.clone();
    good_order.sort_by(|&a, &b| {
        if is_before(ordering, page_nums, a, b) {
            Ordering::Less
        } else if a == b {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    good_order
}

fn reorder_pages_graph(graph: &SimpleGraph, page_nums: &Vec<u64>) -> Vec<u64> {
    let mut good_order = page_nums.clone();
    good_order.sort_by(|&a, &b| {
        if graph
            .dfs(a, |n| page_nums.contains(&n))
            .find(|&x| x == b)
            .is_some()
        {
            Ordering::Less
        } else if a == b {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    good_order
}

pub fn parse_graph(input: &str) -> (SimpleGraph, Vec<Vec<u64>>) {
    let mut sections = input.split("\n\n");
    let rule_input = sections.next().unwrap();
    let page_input = sections.next().unwrap();
    assert!(
        sections.next().is_none(),
        "Input should have exactly two sections"
    );

    let graph = SimpleGraphBuilder::parse_directed("pageorder", rule_input, "|").unwrap();
    let pages = page_input.parse_lines(|line| line.parse_sep_nums::<u64>(","));
    (graph, pages)
}

fn draw_graph(graph: &SimpleGraph) {
    let viz = graph.to_viz(true);
    let graph_svg = exec(
        viz,
        &mut PrinterContext::default(),
        vec![Format::Svg.into()],
    )
    .unwrap();
    fs::write("graph.svg", graph_svg).expect("Unable to write file");
}

pub fn part1_graph(input: &str) -> u64 {
    let (graph, page_nums) = parse_graph(input);
    println!("Graph: {}", graph);
    let mut cache = HashMap::<(u64, u64), bool>::new();
    page_nums
        .iter()
        .filter(|pages| good_order_graph(&graph, pages, &mut cache))
        .map(|pages| pages[pages.len() / 2])
        .sum()
}

pub fn part1_orig(input: &str) -> u64 {
    let (rules, page_nums) = parse_input(input);
    let page_order = page_ordering(&rules);
    page_nums
        .iter()
        .filter(|&pages| good_order(&page_order, pages))
        .map(|pages| pages[pages.len() / 2])
        .sum::<u64>()
}
pub fn part1(input: &str) -> u64 {
    part1_graph(input)
}

pub fn part2_graph(input: &str) -> u64 {
    let (graph, page_nums) = parse_graph(input);
    let mut cache = HashMap::<(u64, u64), bool>::new();
    page_nums
        .iter()
        .filter(|pages| !good_order_graph(&graph, pages, &mut cache))
        .map(|bo| reorder_pages_graph(&graph, bo))
        .map(|pages| pages[pages.len() / 2])
        .sum()
}

pub fn part2_orig(input: &str) -> u64 {
    let (rules, page_nums) = parse_input(input);
    let page_order = page_ordering(&rules);
    page_nums
        .iter()
        .filter(|&pages| !good_order(&page_order, pages))
        .map(|bo| reorder_pages(&page_order, bo))
        .map(|pages| pages[pages.len() / 2])
        .sum()
}

pub fn part2(input: &str) -> u64 {
    part2_graph(input)
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;
    use graphviz_rust::cmd::Format;
    use graphviz_rust::exec;
    use graphviz_rust::printer::PrinterContext;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: u64 = 143;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: u64 = 123;

    #[test]
    fn test_page_properties() {
        let (_rules, page_nums) = parse_input(include_str!("data/input"));

        page_nums.iter().for_each(|v| {
            let hm = v.iter().collect::<HashSet<_>>();
            assert_eq!(hm.len(), v.len());
        });
    }

    #[test]
    fn test_draw_graph() {
        let (graph, _page_nums) = parse_graph(TEST_INPUT);
        let viz = graph.to_viz(false);
        let graph_svg = exec(
            viz,
            &mut PrinterContext::default(),
            vec![Format::Svg.into()],
        )
        .unwrap();
        fs::write("graph.svg", graph_svg).expect("Unable to write file");
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
}
