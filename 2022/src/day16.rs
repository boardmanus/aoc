use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    fmt,
    fmt::Display,
    str::FromStr,
};

use crate::aoc::Aoc;
use itertools::Itertools;
use nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{alpha0, digit1},
        is_alphabetic,
    },
    combinator::map_res,
    multi::separated_list0,
    sequence::{delimited, pair, preceded, separated_pair},
    IResult,
};
use pathfinding::directed::bfs;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone)]
struct Valve<'a> {
    name: &'a str,
    bit: u8,
    flow: usize,
    tunnels: Vec<&'a str>,
}

static mut current_bit: u8 = 0;
impl Display for Valve<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}-{}: flow={} -> {:?}",
            self.name, self.bit, self.flow, self.tunnels
        )
    }
}

impl<'a> Valve<'a> {
    fn parse(s: &'a str) -> IResult<&'a str, Valve<'a>> {
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        let name = delimited(tag("Valve "), alpha0, tag(" has "))(s)?;
        let flow = delimited(tag("flow rate="), nom::character::complete::u64, tag("; "))(name.0)?;
        let tunnels = preceded(
            alt((
                tag("tunnel leads to valve "),
                tag("tunnels lead to valves "),
            )),
            separated_list0(tag(", "), alpha0),
        )(flow.0)?;

        Ok((tunnels.0, Valve::new(name.1, flow.1 as usize, tunnels.1)))
    }
    fn new(name: &'a str, flow: usize, tunnels: Vec<&'a str>) -> Valve<'a> {
        let mut bit = 0;
        if flow > 0 {
            unsafe {
                bit = current_bit;
                current_bit += 1;
            }
        }
        Valve {
            name: name,
            bit,
            flow,
            tunnels,
        }
    }
}

type ValveMap<'a> = HashMap<&'a str, Valve<'a>>;

fn lines_to_valves(lines: &[String]) -> ValveMap {
    let mut valves: ValveMap = Default::default();
    lines
        .iter()
        .map(|l| Valve::parse(l).unwrap().1)
        .for_each(|v| {
            valves.insert(v.name, v);
        });
    valves
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Vertex<'a> {
    name: &'a str,
}

fn shortest_path(from: &str, to: &str, graph: &ValveMap) -> usize {
    let fromv = Vertex { name: from };
    if let Some(path) = bfs::bfs(
        &fromv,
        |v| graph[v.name].tunnels.iter().map(|v| Vertex { name: v }),
        |v| v.name == to,
    ) {
        //println!("{:?}", path);
        path.len()
    } else {
        usize::MAX
    }
}

fn pressurable_valves<'a>(valves: &'a ValveMap) -> impl Iterator<Item = &'a Valve<'a>> {
    valves.iter().filter(|v| v.1.flow > 0).map(|v| v.1)
}
/*
type DistMap<'a> = HashSet<&'a str, HashSet<&'a str, usize>>;
fn dist_map<'a>(valves: &ValveMap, pressurables: Vec<&Valve<'a>>) -> DistMap<'a> {
    let mut map: DistMap = Default::default();
    for v1 in &pressurables {
        for v2 in &pressurables {
            if v1.name == v2.name {
                continue;
            }
            let t = map.get(v1.name);
            t[v2.name] = shortest_path(v1.name, v2.name, valves);
        }
    }
    map
}
*/
type Cache<'a> = HashMap<(usize, &'a str, u64), usize>;
fn dfs<'a>(
    time: usize,
    valve: &'a str,
    bitmask: u64,
    valves: &ValveMap<'a>,
    cache: &mut Cache<'a>,
    pressurables: &Vec<&Valve<'a>>,
) -> usize {
    if let Some(res) = cache.get(&(time, valve, bitmask)) {
        //println!("Cachehit! {valve} t={time} {:?}", *res);
        return *res;
    }
    let mut maxval = 0;
    for neighbor in pressurables {
        if neighbor.name == valve {
            continue;
        }
        let bit: u64 = 1 << neighbor.bit;
        if (bitmask & bit) != 0 {
            continue;
        }
        let dist = shortest_path(valve, neighbor.name, valves);
        if time < dist {
            continue;
        }
        let remtime = time - dist;
        maxval = maxval.max(
            dfs(
                remtime,
                neighbor.name,
                bitmask | bit,
                valves,
                cache,
                pressurables,
            ) + neighbor.flow * remtime,
        );
    }
    cache.insert((time, valve, bitmask), maxval);
    maxval
}

fn compare_valve_path(a: &(&Valve, usize), b: &(&Valve, usize), rem_time: usize) -> Ordering {
    let aflow = a.0.flow * (rem_time - a.1);
    let bflow = b.0.flow * (rem_time - b.1);
    bflow.cmp(&aflow)
    /*if a.1 == b.1 {
        b.0.flow.cmp(&a.0.flow)
    } else if a.1 < b.1 {
        let aflow = (b.1 - a.1 + 1) * a.0.flow;
        let bflow = b.0.flow;
        bflow.cmp(&aflow)
    } else {
        let aflow = a.0.flow;
        let bflow = (a.1 - b.1 + 1) * b.0.flow;
        bflow.cmp(&aflow)
    }*/
}

#[derive(Debug)]
struct State<'a> {
    pressurable: Vec<&'a Valve<'a>>,
    start_pos: &'a str,
    start_pressure: usize,
    start_time: usize,
    total_pressure: usize,
    path_str: String,
}

impl<'a> Display for State<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}, t={}, p={}, total={}]",
            self.path_str, self.start_time, self.start_pressure, self.total_pressure
        )
    }
}
impl<'a> State<'a> {
    fn init(pressurable: Vec<&'a Valve>, start_pos: &'a str) -> State<'a> {
        State::<'a> {
            pressurable,
            start_pos,
            start_pressure: 0usize,
            start_time: 0usize,
            total_pressure: 0usize,
            path_str: start_pos.to_string(),
        }
    }
    fn move_to(&self, valve: &Valve<'a>, valves: &'a ValveMap) -> State {
        let time = shortest_path(self.start_pos, valve.name, valves);
        let pressurable: Vec<&Valve> = self
            .pressurable
            .iter()
            .filter(|v| v.name != valve.name)
            .map(|v| *v)
            .collect_vec();
        let remaining_time = 30 - self.start_time;
        if time > remaining_time {
            State {
                pressurable: Default::default(),
                start_pos: valve.name,
                start_pressure: self.start_pressure,
                start_time: self.start_time + remaining_time,
                total_pressure: self.total_pressure + remaining_time * self.start_pressure,
                path_str: format!("{}..{}", self.path_str, valve.name),
            }
        } else {
            State {
                pressurable,
                start_pos: valve.name,
                start_pressure: self.start_pressure + valve.flow,
                start_time: self.start_time + time,
                total_pressure: self.total_pressure + time * self.start_pressure,
                path_str: format!("{}->{}", self.path_str, valve.name),
            }
        }
    }
}

fn tpr(valves: &ValveMap, max_time: usize, state: State) -> usize {
    println!("Start State: {state}");
    if state.pressurable.len() == 0 {
        state.total_pressure + (max_time - state.start_time) * state.start_pressure
    } else {
        state
            .pressurable
            .iter()
            .map(|vk| tpr(valves, max_time, state.move_to(vk, valves)))
            .max()
            .unwrap()
    }
}

fn total_pressure_release(valves: &ValveMap) -> usize {
    let mut pressurable = pressurable_valves(valves).collect::<HashSet<&Valve>>();
    let mut current_pressure = 0;
    let mut total_release = 0;
    let mut mins = 0;
    let mut curr_pos = "AA";

    while pressurable.len() > 0 {
        let remaining_time = 30 - mins;
        let mut best = pressurable
            .iter()
            .map(|pvalve| {
                // dist includes start, but we want one more anyway, for the release
                let dist = shortest_path(curr_pos, pvalve.name, valves);
                (*pvalve, dist)
            })
            .sorted_by(|a, b| compare_valve_path(a, b, remaining_time));
        let best_move = best.next().unwrap();
        let remaining_time = 30 - mins;
        if best_move.1 > remaining_time {
            total_release += remaining_time * current_pressure;
            mins += remaining_time;
            println!(
                "time expired: {curr_pos} => {} ({} moves, {} flow, pressure {current_pressure}, {mins} mins, {total_release} released)",
                best_move.0.name, best_move.1, best_move.0.flow
            );
            break;
        }
        total_release += best_move.1 * current_pressure;
        current_pressure += best_move.0.flow;
        mins += best_move.1;
        println!(
            "best_move: {curr_pos} => {} ({} moves, {} flow, pressure {current_pressure}, {mins} mins, {total_release} released)",
            best_move.0.name, best_move.1, best_move.0.flow
        );
        curr_pos = best_move.0.name;
        pressurable.remove(best_move.0);
    }
    let remaining_time = 30 - mins;
    total_release += remaining_time * current_pressure;
    println!("Finished with {total_release} pressure released");
    total_release
}

pub struct Day16_1;
impl Aoc for Day16_1 {
    fn day(&self) -> u32 {
        16
    }
    fn puzzle_name(&self) -> &str {
        "Volcanium"
    }
    fn solve(&self, lines: &[String]) -> String {
        let valves = lines_to_valves(lines);
        let mut cache: Cache = Default::default();
        let pressurables = pressurable_valves(&valves).collect_vec();
        dfs(30, "AA", 0, &valves, &mut cache, &pressurables).to_string()
        /*
        tpr(
            &valves,
            30,
            State::init(pressurable_valves(&valves).collect_vec(), "AA"),
        )
        .to_string()
        */
        //total_pressure_release(&valves).to_string()
    }
}

pub struct Day16_2;
impl Aoc for Day16_2 {
    fn day(&self) -> u32 {
        16
    }
    fn puzzle_name(&self) -> &str {
        "Volcanium 2"
    }
    fn solve(&self, lines: &[String]) -> String {
        let valves = lines_to_valves(lines);
        let mut cache: Cache = Default::default();
        let pressurables = pressurable_valves(&valves).collect_vec();
        let num_bits = pressurables.len();
        let bits: u64 = (1 << num_bits) - 1;
        let mut max: usize = 0;
        for i in 0..((bits + 1) / 2) {
            if i.count_ones() < (num_bits / 2 - 2) as u32 {
                continue;
            }
            println!(
                "Trying max={max}, i={i}: elves<{i:b}>, eles<{:0b}>",
                bits ^ i
            );
            let elves = dfs(26, "AA", i, &valves, &mut cache, &pressurables);
            let elephants = dfs(26, "AA", bits ^ i, &valves, &mut cache, &pressurables);
            max = max.max(elves + elephants)
        }

        max.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::as_vstrings;

    use super::*;

    const INPUT: [&str; 10] = [
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
        "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
        "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
        "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
        "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
        "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
        "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
        "Valve HH has flow rate=22; tunnel leads to valve GG",
        "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
        "Valve JJ has flow rate=21; tunnel leads to valve II",
    ];

    #[test]
    fn test_soln() {
        let lines = as_vstrings(&INPUT);
        assert_eq!(Day16_1.solve(&lines), 1651.to_string());
    }

    #[test]
    fn test_soln2() {
        let lines = as_vstrings(&INPUT);
        assert_eq!(Day16_2.solve(&lines), 1707.to_string());
    }

    #[test]
    fn test_tpr() {
        let lines = as_vstrings(&INPUT);
        let valves = lines_to_valves(&lines);
        let total = tpr(
            &valves,
            30,
            State::init(vec![&valves["BB"], &valves["DD"]], "AA"),
        );
        println!("TOTAL = {total}");
    }
    #[test]
    fn test_pressurable_valves() {
        let lines = as_vstrings(&INPUT);
        let valves = lines_to_valves(&lines);
        let pressurables = pressurable_valves(&valves).collect::<HashSet<&Valve>>();
        assert!(pressurables.contains(&valves["HH"]));
        assert!(pressurables.contains(&valves["BB"]));
        assert!(pressurables.contains(&valves["CC"]));
        assert!(pressurables.contains(&valves["DD"]));
        assert!(pressurables.contains(&valves["JJ"]));
        assert!(pressurables.contains(&valves["EE"]));
    }

    #[test]
    fn test_shortest_path() {
        let lines = as_vstrings(&INPUT);
        let valves = lines_to_valves(&lines);

        assert_eq!(shortest_path("AA", "HH", &valves), 6);
        assert_eq!(shortest_path("JJ", "BB", &valves), 4);
        assert_eq!(shortest_path("GG", "CC", &valves), 5);
    }
}
