#![feature(hash_set_entry)]

use enum_iterator::Sequence;
use regex;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Display;
use std::num::ParseIntError;
use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, Sequence, Clone, Copy, PartialEq, Eq, Hash)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Into<usize> for Resource {
    fn into(self) -> usize {
        self as usize
    }
}

impl From<usize> for Resource {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Ore,
            1 => Self::Clay,
            2 => Self::Obsidian,
            3 => Self::Geode,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Resources {
    quantity: [usize; 4],
}

type Recipe = Resources;
type Robots = Resources;

impl Resources {
    fn new(ore: usize, clay: usize, obsidian: usize, geode: usize) -> Resources {
        Resources {
            quantity: [ore, clay, obsidian, geode],
        }
    }
    fn single(resource: Resource) -> Resources {
        let mut resources = Resources::default();
        resources.quantity[resource as usize] = 1;
        resources
    }

    fn contains(&self, resources: &Resources) -> bool {
        self.quantity[0] >= resources.quantity[0]
            && self.quantity[1] >= resources.quantity[1]
            && self.quantity[2] >= resources.quantity[2]
            && self.quantity[3] >= resources.quantity[3]
    }
}

impl Default for Resources {
    fn default() -> Self {
        Resources::new(0, 0, 0, 0)
    }
}

impl Add for Resources {
    type Output = Resources;
    fn add(self, rhs: Self) -> Self::Output {
        Resources::new(
            self.quantity[0] + rhs.quantity[0],
            self.quantity[1] + rhs.quantity[1],
            self.quantity[2] + rhs.quantity[2],
            self.quantity[3] + rhs.quantity[3],
        )
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Resources {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.quantity[0].saturating_sub(rhs.quantity[0]),
            self.quantity[1].saturating_sub(rhs.quantity[1]),
            self.quantity[2].saturating_sub(rhs.quantity[2]),
            self.quantity[3].saturating_sub(rhs.quantity[3]),
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
struct BluePrint {
    id: usize,
    robot_recipes: [Recipe; 4],
    max_robots: Robots,
}

#[derive(Debug)]
enum BluePrintError {
    Regex(regex::Error),
    Parse(ParseIntError),
}
impl Display for BluePrintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BluePrintError::Regex(e) => write!(f, "{e}"),
            BluePrintError::Parse(e) => write!(f, "{e}"),
        }
    }
}
impl std::error::Error for BluePrintError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            BluePrintError::Regex(ref e) => Some(e),
            BluePrintError::Parse(ref e) => Some(e),
        }
    }
}
impl From<regex::Error> for BluePrintError {
    fn from(e: regex::Error) -> BluePrintError {
        BluePrintError::Regex(e)
    }
}
impl From<ParseIntError> for BluePrintError {
    fn from(e: ParseIntError) -> BluePrintError {
        BluePrintError::Parse(e)
    }
}

impl BluePrint {
    fn new(
        id: usize,
        ore: Resources,
        clay: Resources,
        obsidian: Resources,
        geode: Resources,
    ) -> BluePrint {
        let robot_recipes = [ore, clay, obsidian, geode];
        let max_robots = Resources::new(
            robot_recipes.iter().map(|rr| rr.quantity[0]).max().unwrap(),
            robot_recipes.iter().map(|rr| rr.quantity[1]).max().unwrap(),
            robot_recipes.iter().map(|rr| rr.quantity[2]).max().unwrap(),
            usize::MAX,
        );
        BluePrint {
            id,
            robot_recipes,
            max_robots,
        }
    }
    fn parse(input: &str) -> Result<BluePrint, BluePrintError> {
        // Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
        let re = regex::Regex::new(
            r"^Blueprint (\d+): .* ore robot costs (\d+) ore.* clay robot costs (\d+) ore.* obsidian robot costs (\d+) ore and (\d+) clay.* geode robot costs (\d+) ore and (\d+) obsidian.$",
        )?;
        let c = re.captures(input).unwrap();
        Ok(BluePrint::new(
            c[1].parse()?,
            Resources::new(c[2].parse()?, 0, 0, 0),
            Resources::new(c[3].parse()?, 0, 0, 0),
            Resources::new(c[4].parse()?, c[5].parse()?, 0, 0),
            Resources::new(c[6].parse()?, 0, c[7].parse()?, 0),
        ))
    }

    fn buildable_robots<'a>(
        &'a self,
        robots: &'a Resources,
    ) -> impl Iterator<Item = Resource> + '_ {
        self.robot_recipes
            .iter()
            .enumerate()
            .filter(move |r| {
                (robots.quantity[r.0] < self.max_robots.quantity[r.0])
                    && (r.1.quantity[0] == 0 || robots.quantity[0] > 0)
                    && (r.1.quantity[1] == 0 || robots.quantity[1] > 0)
                    && (r.1.quantity[2] == 0 || robots.quantity[2] > 0)
            })
            .map(|r| Resource::from(r.0))
    }

    fn build(
        &self,
        maybe_target: Option<Resource>,
        resources: Resources,
    ) -> Option<(Recipe, Robots)> {
        if let Some(target) = maybe_target {
            let recipe = &self.robot_recipes[target as usize];
            if resources.contains(recipe) {
                Some((*recipe, Robots::single(target)))
            } else {
                None
            }
        } else {
            Some((Resources::default(), Robots::default()))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct BluePrints {
    design: Vec<BluePrint>,
}

impl BluePrints {
    fn parse(input: &str) -> Result<BluePrints, BluePrintError> {
        let mut res: (Vec<_>, Vec<_>) = input
            .split('\n')
            .filter(|s| *s != "")
            .map(|s| BluePrint::parse(s))
            .partition(Result::is_ok);
        if let Some(err) = res.1.pop() {
            Err(err.err().unwrap())
        } else {
            Ok(BluePrints {
                design: res.0.into_iter().flatten().collect::<Vec<_>>(),
            })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    factory_target: Option<Resource>,
    robots: Resources,
    resources: Resources,
}

impl Default for State {
    fn default() -> Self {
        State {
            factory_target: None,
            robots: Resources::new(1, 0, 0, 0),
            resources: Resources::new(0, 0, 0, 0),
        }
    }
}

impl State {
    fn new(factory_target: Option<Resource>, robots: Resources, resources: Resources) -> State {
        State {
            factory_target,
            robots,
            resources,
        }
    }

    fn update(&self, blueprint: &BluePrint) -> Vec<State> {
        if let Some((used_resources, created_robots)) =
            blueprint.build(self.factory_target, self.resources)
        {
            let new_robots = self.robots + created_robots;
            let new_resources = self.resources - used_resources + self.robots;
            blueprint
                .buildable_robots(&self.robots)
                .map(|r| State::new(Some(r), new_robots, new_resources))
                .collect()
        } else {
            vec![State::new(
                self.factory_target,
                self.robots,
                self.resources + self.robots,
            )]
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "State[target={:?}, robots={:?}, resources={:?}]",
            self.factory_target, self.robots, self.resources
        )
    }
}

impl BluePrints {
    fn quality_sum(&self, time: usize) -> usize {
        self.design.iter().map(|bp| bp.quality_level(time)).sum()
    }
}
impl BluePrint {
    fn quality_level(&self, time: usize) -> usize {
        self.max_geodes(time) * self.id
    }
    fn max_geodes(&self, time: usize) -> usize {
        let mut initial_state = HashSet::<State>::default();
        initial_state.insert(State::default());
        let mut all_states = vec![initial_state];
        for t in 0..time {
            println!("-------  t={t}  --------");
            let states = &all_states[t];
            let mut new_states = HashSet::<State>::default();
            states.iter().for_each(|state| {
                //println!("{state} =>");
                for s in state.update(self) {
                    //println!("  ... {s}");
                    new_states.insert(s);
                }
            });
            println!("{} new states added...", new_states.len());
            all_states.push(new_states);
        }
        all_states[time]
            .iter()
            .map(|s| s.resources.quantity[Resource::Geode as usize])
            .max()
            .unwrap()
    }
}

fn solve_part1(input: &str) -> String {
    if let Ok(blueprints) = BluePrints::parse(input) {
        blueprints.quality_sum(24).to_string()
    } else {
        "failed".to_string()
    }
}

fn solve_part2(input: &str) -> String {
    if let Ok(blueprints) = BluePrints::parse(input) {
        blueprints
            .design
            .iter()
            .take(3)
            .map(|bp| bp.max_geodes(32))
            .product::<usize>()
            .to_string()
    } else {
        "failed".to_string()
    }
}

fn main() {
    let res = solve_part1(include_str!("input.txt"));
    println!("Part1: {res}");
    let res = solve_part2(include_str!("input.txt"));
    println!("Part2: {res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT), 33.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT), "");
    }

    #[test]
    fn test_32() {
        let blueprints = BluePrints::parse(INPUT).unwrap();
        let bp = blueprints.design.iter().take(3).collect::<Vec<_>>();
        assert_eq!(bp[0].max_geodes(32), 56)
    }
    #[test]
    fn test_buildable_robots() {
        let bp = BluePrints::parse(INPUT).unwrap();
        let ql = bp.design[0]
            .buildable_robots(&Resources::new(1, 1, 0, 0))
            .collect::<Vec<_>>();
        assert_eq!(ql, vec![Resource::Ore, Resource::Clay, Resource::Obsidian]);
    }

    #[test]
    fn test_blueprint_ql() {
        let bp = BluePrints::parse(INPUT).unwrap();
        let ql = bp.design[0].quality_level(24);
        assert_eq!(ql, 9);
    }

    #[test]
    fn test_resources_contains() {
        assert!(Resources::new(10, 10, 0, 0).contains(&Resources::new(4, 0, 0, 0)));
    }

    #[test]
    fn test_resource_math() {
        assert_eq!(
            Resources::new(1, 0, 0, 0) + Resources::new(0, 1, 0, 0),
            Resources::new(1, 1, 0, 0)
        );
        assert_eq!(
            Resources::new(4, 2, 0, 0) - Resources::new(0, 1, 0, 0),
            Resources::new(4, 1, 0, 0)
        );
        assert_eq!(
            Resources::new(2, 2, 2, 2) - Resources::new(1, 1, 1, 1),
            Resources::new(1, 1, 1, 1)
        );
        assert_eq!(
            Resources::new(2, 2, 2, 2) - Resources::new(1, 1, 2, 1),
            Resources::new(1, 1, 0, 1)
        );
        assert_eq!(
            Resources::new(2, 2, 2, 2) - Resources::new(1, 1, 1, 1) + Resources::new(1, 2, 3, 4),
            Resources::new(2, 3, 4, 5)
        );
    }
}
