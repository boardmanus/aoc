use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Cube(i64, i64, i64);

impl Cube {
    fn in_bounds(&self, bounds: &(Cube, Cube)) -> bool {
        let (min, max) = bounds;
        self.0 >= min.0
            && self.0 <= max.0
            && self.1 >= min.1
            && self.1 <= max.1
            && self.2 >= min.2
            && self.2 <= max.2
    }
}

#[derive(Debug, PartialEq)]
struct Droplet {
    cubes: HashSet<Cube>,
}

static ADJACENCY: [(i64, i64, i64); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

impl Droplet {
    fn parse(input: &str) -> Droplet {
        let re = regex::Regex::new(r"^(\d+),(\d+),(\d+)$").unwrap();
        let cubes = input
            .split("\n")
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

    fn num_adjacent(&self, cube: &Cube, contained_cubes: &HashSet<Cube>) -> i64 {
        let mut adjacent: Vec<&Cube> = Default::default();
        ADJACENCY.iter().fold(0, |a, d| {
            let test_cube = Cube(cube.0 + d.0, cube.1 + d.1, cube.2 + d.2);
            if self.cubes.contains(&test_cube) || contained_cubes.contains(&test_cube) {
                a + 1
            } else {
                a
            }
        })
    }

    fn unconnected_faces(&self, contained_cubes: &HashSet<Cube>) -> i64 {
        self.cubes
            .iter()
            .map(|cube| 6 - self.num_adjacent(cube, contained_cubes))
            .sum()
    }

    fn bounds(&self) -> (Cube, Cube) {
        let mut min = Cube(i64::MAX, i64::MAX, i64::MAX);
        let mut max = Cube(0, 0, 0);
        self.cubes.iter().for_each(|cube| {
            min = Cube(min.0.min(cube.0), min.1.min(cube.1), min.2.min(cube.2));
            max = Cube(max.0.max(cube.0), max.1.max(cube.1), max.2.max(cube.2));
        });
        (min, max)
    }

    fn adjacent<F: FnMut(&Cube) -> bool>(&self, cube: &Cube, pred: F) -> Vec<Cube> {
        ADJACENCY
            .iter()
            .map(|adj| Cube(cube.0 + adj.0, cube.1 + adj.1, cube.2 + adj.2))
            .filter(pred)
            .collect::<Vec<Cube>>()
    }
    fn adjacent_empty(&self, cube: &Cube) -> Vec<Cube> {
        //impl Iterator<Item = &'_ Cube> + '_ {
        self.adjacent(cube, |c| !self.cubes.contains(c))
    }

    fn floodfill<'a>(&'a self, start_cube: &'a Cube) -> HashSet<Cube> {
        let (imin, imax) = self.bounds();
        // extend bounds...
        let bounds = (
            Cube(imin.0 - 1, imin.1 - 1, imin.2 - 1),
            Cube(imax.0 + 1, imax.1 + 1, imax.2 + 1),
        );
        let mut q: Vec<Cube> = vec![start_cube.clone()];
        let mut res: HashSet<Cube> = Default::default();

        while let Some(n) = q.pop() {
            res.insert(n.clone());
            let adj = self.adjacent(&n, |cube| {
                !self.cubes.contains(cube) && !res.contains(cube) && cube.in_bounds(&bounds)
            });

            adj.iter().for_each(|cube| q.push(cube.clone()));
        }
        res
    }
}

fn solve_part1(input: &str) -> String {
    let contained: HashSet<Cube> = Default::default();
    Droplet::parse(input)
        .unconnected_faces(&contained)
        .to_string()
}

fn solve_part2(input: &str) -> String {
    let droplet = Droplet::parse(input);
    let ff = droplet.floodfill(&Cube(1, 1, 2));
    let (min, max) = droplet.bounds();
    let mut contained_cubes: HashSet<Cube> = Default::default();
    for x in min.0..=max.0 {
        for y in min.1..=max.1 {
            for z in min.2..=max.2 {
                let c = Cube(x, y, z);
                if !droplet.cubes.contains(&c) && !ff.contains(&c) {
                    contained_cubes.insert(c);
                }
            }
        }
    }
    droplet.unconnected_faces(&contained_cubes).to_string()
}

fn main() {
    let res = solve_part1(include_str!("input.txt"));
    println!("Part1: {res}");
    let res = solve_part2(include_str!("input.txt"));
    println!("Part2: {res}");
}

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
        let start_cube = Cube(0, 0, 0);
        let ff = droplet.floodfill(&start_cube);
        droplet
            .cubes
            .iter()
            .for_each(|cube| assert!(!ff.contains(cube)));
        assert!(!ff.contains(&Cube(2, 2, 5)));
        assert!(ff.contains(&Cube(0, 0, 0)));
        println!("{:?}", ff);
    }
    #[test]
    fn test_input_bounds() {
        let droplet = Droplet::parse(include_str!("input.txt"));
        println!("Input bounds = {:?}", droplet.bounds());
        assert!(droplet.cubes.contains(&Cube(0, 9, 8)));
    }

    #[test]
    fn test_in_bounds() {
        let bounds = (Cube(10, 10, 10), Cube(20, 20, 20));
        assert!(Cube(15, 15, 15).in_bounds(&bounds));
        assert!(Cube(19, 19, 19).in_bounds(&bounds));
        assert!(Cube(11, 11, 11).in_bounds(&bounds));
        assert!(!Cube(10, 11, 11).in_bounds(&bounds));
    }
    #[test]
    fn test_num_adjacent() {
        let droplet = Droplet::parse("1,2,3\n2,3,3\n");
        let contained_cubes: HashSet<Cube> = Default::default();
        assert_eq!(droplet.num_adjacent(&Cube(1, 2, 4), &contained_cubes), 1);
        assert_eq!(droplet.num_adjacent(&Cube(1, 3, 3), &contained_cubes), 2);
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
