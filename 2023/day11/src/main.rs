use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

type Map = HashMap<(i64, i64), usize>;
type Expansion = HashSet<i64>;

#[derive(Debug, Default)]
struct Universe {
    map: Map,
    size: (usize, usize),
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                if let Some(galaxy) = self.map.get(&(x as i64, y as i64)) {
                    write!(f, "{galaxy}")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "{}", self.map.len())
    }
}

fn parse(input: &str) -> Universe {
    let mut map = HashMap::new();
    let mut galaxy = 1;
    let size = (input.lines().next().unwrap().len(), input.lines().count());
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert((x as i64, y as i64), galaxy);
                galaxy += 1;
            }
        }
    }
    Universe { map, size }
}

fn expansion(ex: &Expansion, distance: i64) -> HashMap<i64, i64> {
    let mut tmp = ex.iter().map(|v| (*v, 0)).collect::<Vec<_>>();
    tmp.sort();

    let mut prev: Option<(i64, i64)> = None;
    for v in tmp.iter_mut() {
        if let Some(last) = prev {
            v.1 = last.1 + (v.0 - last.0 - 1) * (distance - 1);
        } else {
            v.1 = 0;
        }
        prev = Some(*v);
    }

    tmp.iter().fold(Default::default(), |mut acc, v| {
        acc.insert(v.0, v.1);
        acc
    })
}

fn galaxy_locations(universe: &Universe) -> (Expansion, Expansion) {
    universe
        .map
        .iter()
        .fold((Expansion::new(), Expansion::new()), |mut acc, (pos, _)| {
            acc.0.insert(pos.0);
            acc.1.insert(pos.1);
            acc
        })
}

fn expand(universe: &Universe, dist: i64) -> Universe {
    let ex = galaxy_locations(universe);

    let hex = expansion(&ex.0, dist);
    let vex = expansion(&ex.1, dist);

    universe
        .map
        .iter()
        .fold(Universe::default(), |mut universe, galaxy| {
            let dx = hex.get(&galaxy.0 .0).unwrap_or(&0);
            let dy = vex.get(&galaxy.0 .1).unwrap_or(&0);
            let pos = (galaxy.0 .0 + dx, galaxy.0 .1 + dy);
            universe.map.insert(pos, *galaxy.1);
            universe.size = (
                universe.size.0.max(pos.0 as usize + 1),
                universe.size.1.max(pos.1 as usize + 1),
            );
            universe
        })
}

fn distances(universe: &Universe) -> i64 {
    let mut q = universe
        .map
        .iter()
        .map(|(pos, g)| (*pos, *g))
        .collect::<Vec<_>>();
    let mut sum = 0;
    let mut total = 0;
    while let Some(start) = q.pop() {
        sum += q
            .iter()
            .map(|galaxy| {
                total += 1;
                let dist = (galaxy.0 .0 - start.0 .0).abs() + (galaxy.0 .1 - start.0 .1).abs();
                dist
            })
            .sum::<i64>()
    }
    sum
}

fn solve_part1(input: &str) -> i64 {
    let before = parse(input);
    let universe = expand(&before, 2);
    distances(&universe)
}

fn solve_part2(input: &str) -> i64 {
    let before = parse(input);
    let universe = expand(&before, 1000000);
    distances(&universe)
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
        assert_eq!(solve_part1(TEST_INPUT), 374);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_2), 467835);
    }

    #[test]
    fn test_part2_10times() {
        let before = parse(TEST_INPUT_2);
        let universe = expand(&before, 10);
        println!("{universe}");
        let d = distances(&universe);
        assert_eq!(d, 1030);
    }

    #[test]
    fn test_part2_100times() {
        let before = parse(TEST_INPUT_2);
        let universe = expand(&before, 100);
        let d = distances(&universe);
        assert_eq!(d, 8410);
    }

    #[test]
    fn test_parse() {
        let universe = parse(TEST_INPUT);
        println!("{universe}");
        assert_eq!(universe.size, (10, 10));
        assert_eq!(universe.map.len(), 9);
    }

    #[test]
    fn test_expansion() {
        let universe = parse(TEST_INPUT);
        println!("{universe}");
        let locs = galaxy_locations(&universe);
        let hex = expansion(&locs.0, 1);
        let vex = expansion(&locs.0, 2);
        println!("{:?}", hex);
        println!("{:?}", vex);
        assert_eq!(universe.size, (13, 12));
        assert_eq!(universe.map.len(), 9);
    }

    #[test]
    fn test_expand() {
        let before = parse(TEST_INPUT);
        println!("{before}");
        let universe = expand(&before, 1);
        println!("{universe}");
        assert_eq!(universe.size, (13, 12));
        assert_eq!(universe.map.len(), 9);

        let universe = expand(&before, 2);
        println!("{universe}");
        assert_eq!(universe.size, (13, 12));
        assert_eq!(universe.map.len(), 9);
    }
}
