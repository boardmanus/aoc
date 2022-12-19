#[macro_use]
extern crate impl_ops;
use impl_ops::impl_op_ex;
use std::collections::HashSet;
use std::ops;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Cube(i64, i64, i64);
type Offset = (i64, i64, i64);

impl Cube {
    const ADJACENCY: [Offset; 6] = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];

    fn in_bounds(&self, bounds: &(Cube, Cube)) -> bool {
        let (min, max) = bounds;
        self.0 >= min.0
            && self.0 <= max.0
            && self.1 >= min.1
            && self.1 <= max.1
            && self.2 >= min.2
            && self.2 <= max.2
    }

    fn adjacent(&self) -> impl Iterator<Item = Cube> + '_ {
        Cube::ADJACENCY.iter().map(move |adj| self + adj)
    }

    fn min(&self, c: &Cube) -> Cube {
        Cube(self.0.min(c.0), self.1.min(c.1), self.2.min(c.2))
    }

    fn max(&self, c: &Cube) -> Cube {
        Cube(self.0.max(c.0), self.1.max(c.1), self.2.max(c.2))
    }
}

impl_op_ex!(+ |a: &Cube, b: &Offset| -> Cube {
    Cube(a.0 + b.0, a.1 + b.1, a.2 + b.2)
});

#[derive(Debug, PartialEq)]
struct Droplet {
    cubes: HashSet<Cube>,
}

impl Droplet {
    fn parse(input: &str) -> Droplet {
        let re = regex::Regex::new(r"^(\d+),(\d+),(\d+)$").unwrap();
        let cubes = input
            .split('\n')
            .flat_map(|s| {
                if !s.is_empty() {
                    let captures = re.captures(s).unwrap();
                    Some(Cube(
                        captures[1].parse().unwrap(),
                        captures[2].parse().unwrap(),
                        captures[3].parse().unwrap(),
                    ))
                } else {
                    None
                }
            })
            .collect::<HashSet<Cube>>();
        Droplet { cubes }
    }

    fn map_cubes<'a, F, T>(&'a self, f: F) -> impl Iterator<Item = T> + 'a
    where
        F: Fn(&Cube) -> T + 'a,
    {
        self.cubes.iter().map(f)
    }

    fn disconnected_faces(&self) -> usize {
        self.map_cubes(|cube| {
            cube.adjacent()
                .filter(|adj| !self.cubes.contains(adj))
                .count()
        })
        .sum()
    }

    fn external_faces(&self) -> usize {
        let air = self.floodfill();
        self.map_cubes(|cube| cube.adjacent().filter(|adj| air.contains(adj)).count())
            .sum()
    }

    fn bounds(&self) -> (Cube, Cube) {
        let mut min = Cube(i64::MAX, i64::MAX, i64::MAX);
        let mut max = Cube(0, 0, 0);
        self.cubes.iter().for_each(|cube| {
            min = min.min(cube);
            max = max.max(cube);
        });
        (min + (-1, -1, -1), max + (1, 1, 1))
    }

    fn floodfill(&self) -> HashSet<Cube> {
        let bounds = self.bounds();
        let mut q: Vec<Cube> = vec![bounds.0.clone()];
        let mut visited: HashSet<Cube> = Default::default();

        // Depth first search of air-space
        while let Some(n) = q.pop() {
            n.adjacent()
                .filter(|cube| {
                    !self.cubes.contains(cube) && !visited.contains(cube) && cube.in_bounds(&bounds)
                })
                .for_each(|cube| q.push(cube));
            visited.insert(n);
        }
        visited
    }
}

fn solve_part1(input: &str) -> String {
    Droplet::parse(input).disconnected_faces().to_string()
}

fn solve_part2(input: &str) -> String {
    Droplet::parse(input).external_faces().to_string()
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

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT), 64.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT), 58.to_string());
    }

    #[test]
    fn test_flood_fill() {
        let droplet = Droplet::parse(INPUT);
        let ff = droplet.floodfill();
        droplet
            .cubes
            .iter()
            .for_each(|cube| assert!(!ff.contains(cube)));
        assert!(!ff.contains(&Cube(2, 2, 5)));
        assert!(ff.contains(&Cube(0, 0, 0)));
    }
    #[test]
    fn test_input_bounds() {
        let droplet = Droplet::parse(include_str!("input.txt"));
        assert!(droplet.cubes.contains(&Cube(0, 9, 8)));
    }

    #[test]
    fn test_in_bounds() {
        let bounds = (Cube(10, 10, 10), Cube(20, 20, 20));
        assert!(Cube(15, 15, 15).in_bounds(&bounds));
        assert!(Cube(19, 19, 19).in_bounds(&bounds));
        assert!(Cube(11, 11, 11).in_bounds(&bounds));
        assert!(!Cube(9, 11, 11).in_bounds(&bounds));
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            Droplet::parse("1,2,3\n4,5,6\n"),
            Droplet {
                cubes: vec![Cube(1, 2, 3), Cube(4, 5, 6)].into_iter().collect()
            }
        );
    }
}
