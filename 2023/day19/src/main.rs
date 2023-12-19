use std::{collections::HashMap, str::FromStr};

use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Range { start, end }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ParseError {
    InvalidCategory,
    InvalidOp,
    InvalidWorkflow,
    InvalidRating,
    InvalidRule,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn idx(&self) -> usize {
        *self as usize
    }
}

impl FromStr for Category {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Category::X),
            "m" => Ok(Category::M),
            "a" => Ok(Category::A),
            "s" => Ok(Category::S),
            _ => Err(ParseError::InvalidCategory),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Rule {
    Pass(String),
    LT(Category, u64, String),
    GT(Category, u64, String),
}

impl Rule {
    fn apply(&self, rating: &Rating) -> bool {
        match self {
            Rule::Pass(_) => true,
            Rule::LT(category, value, _) => rating.xmas[category.idx()] < *value,
            Rule::GT(category, value, _) => rating.xmas[category.idx()] > *value,
        }
    }

    fn workflow_name(&self) -> &str {
        match self {
            Rule::Pass(name) => name.as_str(),
            Rule::LT(_, _, name) => name.as_str(),
            Rule::GT(_, _, name) => name.as_str(),
        }
    }

    fn split_range(&self, ratings: &AllRatings) -> (Option<AllRatings>, Option<AllRatings>) {
        let mut accepted = ratings.clone();
        let mut rejected = ratings.clone();
        match self {
            Rule::Pass(_) => (Some(accepted), None),
            Rule::LT(category, value, _) => {
                let range = ratings.xmas[category.idx()];
                let arange = Range::new((*value - 1).min(range.start), (*value).min(range.end));
                let rrange = Range::new((*value).max(range.start), (*value).max(range.end));
                accepted.xmas[category.idx()] = arange;
                rejected.xmas[category.idx()] = rrange;
                (Some(accepted), Some(rejected))
            }
            Rule::GT(category, value, _) => {
                let range = ratings.xmas[category.idx()];
                let rrange = Range::new((*value).min(range.start), (*value + 1).min(range.end));
                let arange = Range::new((*value + 1).max(range.start), (*value + 1).max(range.end));
                accepted.xmas[category.idx()] = arange;
                rejected.xmas[category.idx()] = rrange;
                (Some(accepted), Some(rejected))
            }
        }
    }
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rule_re: Regex = Regex::new(r"^(\w)(.)(\d+):(\w+)$").unwrap();
        let target_re = Regex::new(r"^(\w+)$").unwrap();
        if let Some(cap) = target_re.captures(s) {
            let workflow = cap.get(1).ok_or(ParseError::InvalidRule)?.as_str();
            Ok(Rule::Pass(workflow.to_string()))
        } else {
            let cap = rule_re.captures(s).ok_or(ParseError::InvalidRule)?;
            let category =
                Category::from_str(cap.get(1).ok_or(ParseError::InvalidCategory)?.as_str())?;
            let op = cap.get(2).ok_or(ParseError::InvalidOp)?.as_str();
            let value_str = cap.get(3).ok_or(ParseError::InvalidRule)?.as_str();
            let value = value_str.parse().map_err(|_| ParseError::InvalidRule)?;
            let workflow = cap.get(4).ok_or(ParseError::InvalidRule)?.as_str();
            match op {
                "<" => Ok(Rule::LT(category, value, workflow.to_string())),
                ">" => Ok(Rule::GT(category, value, workflow.to_string())),
                _ => Err(ParseError::InvalidOp),
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn flow(&self, rating: &Rating) -> &str {
        self.rules
            .iter()
            .find(|rule| rule.apply(rating))
            .unwrap()
            .workflow_name()
    }

    fn possibilities(&self, possibles: &AllRatings) -> Vec<(&str, AllRatings)> {
        self.rules
            .iter()
            .fold((Some(*possibles), vec![]), |mut acc, rule| {
                if let Some(possibles) = acc.0 {
                    let (accepted, rejected) = rule.split_range(&possibles);
                    if let Some(accepted) = accepted {
                        acc.1.push((rule.workflow_name(), accepted));
                    }
                    acc.0 = rejected;
                }
                acc
            })
            .1
    }
}
impl FromStr for Workflow {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\w+)\{(.*)\}").unwrap();
        let cap = re.captures(s).ok_or(ParseError::InvalidWorkflow)?;
        let name = cap.get(1).ok_or(ParseError::InvalidWorkflow)?.as_str();
        let rule_strs = cap.get(2).ok_or(ParseError::InvalidWorkflow)?.as_str();
        let rules = rule_strs
            .split(',')
            .map(|rule_str| Rule::from_str(rule_str))
            .collect::<Result<Vec<Rule>, _>>()?;

        Ok(Workflow {
            name: name.to_string(),
            rules,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct AllRatings {
    xmas: [Range; 4],
}

impl AllRatings {
    fn new() -> Self {
        AllRatings {
            xmas: [
                Range::new(1, 4001),
                Range::new(1, 4001),
                Range::new(1, 4001),
                Range::new(1, 4001),
            ],
        }
    }
    fn product(&self) -> usize {
        self.xmas
            .iter()
            .map(|x| (x.end - x.start) as usize)
            .product()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Rating {
    xmas: [u64; 4],
}

impl Rating {
    fn sum(&self) -> usize {
        self.xmas.iter().map(|x| *x as usize).sum()
    }
}

impl FromStr for Rating {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
        let cap = re.captures(s).ok_or(ParseError::InvalidRating)?;
        let mut xmas = [0; 4];
        for i in 0..4 {
            let c = cap.get(i + 1).ok_or(ParseError::InvalidRating)?.as_str();
            xmas[i] = c.parse().map_err(|_| ParseError::InvalidRating)?;
        }
        Ok(Rating { xmas })
    }
}

struct State {
    workflows: HashMap<String, Workflow>,
    ratings: Vec<Rating>,
}

impl State {
    fn flow(&self) -> Vec<&Rating> {
        let in_wf = self.workflows.get("in");
        self.ratings.iter().fold(vec![], |mut acc, rating| {
            let mut maybe_wf = in_wf;
            while let Some(wf) = maybe_wf {
                maybe_wf = match wf.flow(rating) {
                    "A" => {
                        acc.push(rating);
                        None
                    }
                    "R" => None,
                    name => self.workflows.get(name),
                };
            }
            acc
        })
    }

    fn num_possible_accepted(&self) -> usize {
        let mut q = vec![("in", AllRatings::new())];
        let mut count = 0;

        while let Some(path) = q.pop() {
            match path.0 {
                "A" => count += path.1.product(),
                "R" => (),
                _ => {
                    let workflow = self.workflows.get(path.0).unwrap();
                    let pathways = workflow.possibilities(&path.1);
                    q.extend(pathways);
                }
            }
        }

        count
    }
}

impl FromStr for State {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("\n\n");
        let workflow_lines = parts.next().ok_or(ParseError::InvalidWorkflow)?;
        let rating_lines = parts.next().ok_or(ParseError::InvalidRating)?;
        let workflows = workflow_lines
            .lines()
            .map(|line| Workflow::from_str(line))
            .fold(HashMap::new(), |mut acc, wf| {
                let w = &wf.unwrap();
                acc.insert(w.name.clone(), w.clone());
                acc
            });

        let ratings = rating_lines
            .lines()
            .map(|line| Rating::from_str(line))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(State { workflows, ratings })
    }
}

fn solve_part1(input: &str) -> usize {
    let state = State::from_str(input).unwrap();
    state.flow().iter().map(|rating| rating.sum()).sum()
}

fn solve_part2(input: &str) -> usize {
    let state = State::from_str(input).unwrap();
    state.num_possible_accepted()
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let part1 = solve_part1(INPUT);
    println!("Part1: {part1}");
    let part2 = solve_part2(INPUT);
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
    const TEST_INPUT_2: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 19114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 167409079868000);
    }

    #[test]
    fn test_parse() {
        const INPUT: &str = include_str!("input.txt");

        let state = State::from_str(INPUT).unwrap();
    }
    #[test]
    fn test_parse_workflow() {
        let workflow = Workflow::from_str("px{a<2006:qkq,m>2090:A,rfg}").unwrap();
        assert_eq!(workflow.name, "px");
        assert_eq!(
            workflow.rules,
            vec![
                Rule::LT(Category::A, 2006, "qkq".to_string()),
                Rule::GT(Category::M, 2090, "A".to_string()),
                Rule::Pass("rfg".to_string()),
            ]
        );
    }

    #[test]
    fn test_parse_rating() {
        let rating = Rating::from_str("{x=2127,m=1623,a=2188,s=1013}").unwrap();
        assert_eq!(
            rating,
            Rating {
                xmas: [2127, 1623, 2188, 1013]
            }
        );
        assert_eq!(rating.sum(), 6951);
    }
}
