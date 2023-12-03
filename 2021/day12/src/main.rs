use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

#[derive(Debug, Clone)]
enum Path<'a> {
    None,
    Start(&'a Cave<'a>),
    Step(Rc<Path<'a>>, &'a Cave<'a>, bool),
}

impl<'a> Path<'a> {
    fn cave(&self) -> Option<&Cave<'a>> {
        match self {
            Path::None => None,
            Path::Start(c) => Some(c),
            Path::Step(_, c, _) => Some(c),
        }
    }

    fn visited_twice(&self) -> bool {
        match self {
            Path::Step(_, _, visited) => *visited,
            _ => false,
        }
    }

    fn to_vec(&mut self) -> Vec<&'a str> {
        self.map(|cave| cave.name).collect()
    }
}

struct PathIt<'a> {
    path: Rc<Path<'a>>,
}
impl<'a> Iterator for Path<'a> {
    type Item = &'a Cave<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (n, r) = match self {
            Path::None => (Path::None, None),
            Path::Start(c) => (Path::None, Some(*c)),
            Path::Step(p, c, _) => (p.as_ref().clone(), Some(*c)),
        };

        *self = n.clone();
        r
    }
}

#[derive(Debug, PartialEq)]
enum CaveType {
    Small,
    Big,
}

#[derive(Debug)]
struct Cave<'a> {
    name: &'a str,
    cave: CaveType,
    tunnels: Vec<&'a str>,
}

impl<'a> Cave<'a> {
    fn new(name: &'a str, tunnels: Vec<&'a str>) -> Self {
        let cave = if name.chars().all(|c| c.is_uppercase()) {
            CaveType::Big
        } else {
            CaveType::Small
        };

        Cave {
            name,
            cave,
            tunnels,
        }
    }
}

struct CaveSystem<'a> {
    caves: HashMap<&'a str, Cave<'a>>,
}

impl<'a> CaveSystem<'a> {
    fn new() -> CaveSystem<'a> {
        CaveSystem {
            caves: Default::default(),
        }
    }

    fn traverse(&self, allow_twice: bool) -> Vec<Vec<&str>> {
        let mut all_paths = Vec::<Vec<&str>>::new();
        let start = &self.caves["start"];
        let mut to_visit: Vec<Rc<Path>> = vec![Rc::new(Path::Start(start))];

        while let Some(path) = to_visit.pop() {
            if let Some(cave) = path.cave() {
                if cave.name == "end" {
                    all_paths.push(path.as_ref().clone().to_vec());
                } else {
                    cave.tunnels.iter().for_each(|t| {
                        let next_cave = &self.caves[t];
                        let visited = path.visited_twice();
                        if next_cave.cave == CaveType::Big
                            || !(*path).clone().any(|c: &Cave| c.name == next_cave.name)
                        {
                            to_visit.push(Rc::new(Path::Step(
                                Rc::clone(&path),
                                next_cave,
                                visited,
                            )));
                        } else if allow_twice && !visited && next_cave.name != start.name {
                            to_visit.push(Rc::new(Path::Step(Rc::clone(&path), next_cave, true)));
                        }
                    });
                }
            }
        }
        all_paths
    }
}

fn parse<'a>(input: &'a str) -> CaveSystem<'a> {
    let mut cave_names = HashSet::<&str>::new();
    let mut tunnel_names = HashMap::<&str, Vec<&str>>::new();
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .for_each(|line| {
            let mut lit = line.split('-');
            let cave1_str = lit.next().expect("value");
            let cave2_str = lit.next().expect("value");
            cave_names.insert(cave1_str);
            cave_names.insert(cave2_str);
            tunnel_names
                .entry(cave1_str)
                .and_modify(|x| x.push(cave2_str))
                .or_insert(vec![cave2_str]);
            tunnel_names
                .entry(cave2_str)
                .and_modify(|x| x.push(cave1_str))
                .or_insert(vec![cave1_str]);
        });

    let caves = cave_names
        .iter()
        .fold(HashMap::<&str, Cave>::new(), |mut acc, name| {
            acc.insert(
                name,
                Cave::new(name, tunnel_names.remove(name).unwrap_or_default()),
            );
            acc
        });

    CaveSystem { caves }
}

fn solve_part1(g: &str) -> usize {
    parse(g).traverse(false).len()
}

fn solve_part2(g: &str) -> usize {
    parse(g).traverse(true).len()
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

    const TEST_INPUT_1: &str = include_str!("test_input_1.txt");
    const TEST_INPUT_2: &str = include_str!("test_input_2.txt");
    const TEST_INPUT_3: &str = include_str!("test_input_3.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT_1), 10);
        assert_eq!(solve_part1(TEST_INPUT_2), 19);
        assert_eq!(solve_part1(TEST_INPUT_3), 226);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT_1), 36);
        assert_eq!(solve_part2(TEST_INPUT_2), 103);
        assert_eq!(solve_part2(TEST_INPUT_3), 3509);
    }

    #[test]
    fn test_traverse() {
        let cs = [
            parse(TEST_INPUT_1),
            parse(TEST_INPUT_2),
            parse(TEST_INPUT_3),
        ];
        let t = cs.iter().map(|c| c.traverse(false)).collect::<Vec<_>>();
        //t.iter().for_each(|tr| println!("{:?}", tr));
        assert_eq!(t[0].len(), 10);
        assert_eq!(t[1].len(), 19);
        assert_eq!(t[2].len(), 226);

        let t = cs.iter().map(|c| c.traverse(true)).collect::<Vec<_>>();
        //t.iter().for_each(|tr| println!("{:?}", tr));
        println!("{:?}", t[0]);
        assert_eq!(t[0].len(), 36);
        assert_eq!(t[1].len(), 103);
        assert_eq!(t[2].len(), 3509);
    }
}
