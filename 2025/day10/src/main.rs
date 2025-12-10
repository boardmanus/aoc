use std::collections::{HashMap, VecDeque};

type Lights = usize;
type Button = Vec<usize>;
type Joltage = [u16; 10];

#[derive(Debug)]
struct Machine {
    lights: Lights,
    buttons: Vec<Button>,
    joltage: Joltage,
    jnum: usize,
}

impl Machine {
    fn map_to_nums(csv_str: &str) -> Vec<usize> {
        csv_str
            .split(',')
            .map(|t_str| {
                t_str
                    .chars()
                    .filter(|c| c.is_numeric())
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap()
            })
            .collect()
    }
    fn parse_goal(g_str: &str) -> Lights {
        g_str
            .chars()
            .skip(1)
            .enumerate()
            .map(|(i, c)| match c {
                '.' => 0,
                '#' => 1 << i,
                _ => panic!("bad char!"),
            })
            .sum()
    }
    fn parse_buttons(b_str: &str) -> Vec<Button> {
        b_str
            .split_ascii_whitespace()
            .map(|button_str| Machine::map_to_nums(button_str))
            .collect()
    }

    fn parse_joltage(j_str: &str) -> (Joltage, usize) {
        let nums = Machine::map_to_nums(j_str);
        (
            nums.iter().enumerate().fold([0; 10], |mut ja, (i, &j)| {
                ja[i] = j as u16;
                ja
            }),
            nums.len(),
        )
    }
    fn parse(line: &str) -> Machine {
        let (goal_str, rest_str) = line.split_once(']').unwrap();
        let (buttons_str, joltage_str) = rest_str.split_once('{').unwrap();
        let lights = Machine::parse_goal(goal_str);
        let buttons = Machine::parse_buttons(buttons_str);
        let (joltage, jnum) = Machine::parse_joltage(joltage_str);
        Machine {
            lights,
            buttons,
            joltage,
            jnum,
        }
    }
    fn light_mask(button: &Button) -> Lights {
        button.iter().map(|light| 1 << light).sum()
    }
    fn shortest_num_presses(&self) -> usize {
        let mut visited = HashMap::<Lights, usize>::new();
        let mut q = self
            .buttons
            .iter()
            .map(|b| (b, 0, 0))
            .collect::<VecDeque<_>>();
        while let Some((button, lights, presses)) = q.pop_front() {
            if lights == self.lights {
                return presses;
            }

            let new_lights = lights ^ Machine::light_mask(button);
            if let Some(first_visit) = visited.get(&new_lights) {
                //println!("found {new_lights} at presses {first_visit} (now at {presses})");
                continue;
            }
            visited.insert(new_lights, presses);
            self.buttons
                .iter()
                .for_each(|b| q.push_back((b, new_lights, presses + 1)));
        }
        0
    }

    fn update_joltage(button: &Button, joltage: Joltage) -> Joltage {
        button.iter().fold(joltage, |mut j, &b| {
            j[b] += 1;
            j
        })
    }

    fn d_joltage(&self, joltage: &Joltage) -> Joltage {
        let min = joltage[0..self.jnum].iter().min().unwrap();
        joltage[0..self.jnum]
            .iter()
            .enumerate()
            .fold([0; 10], |mut dj, (i, &j)| {
                dj[i] = joltage[i] - min;
                dj
            })
    }

    fn shortest_joltage_presses(&self) -> usize {
        println!("finding joltage for {:?}", self.joltage);
        let mut visited = HashMap::<(Joltage, Joltage), usize>::new();
        let mut q = self
            .buttons
            .iter()
            .map(|b| (b, [0; 10], 0))
            .collect::<VecDeque<_>>();
        let mut last_presses = 0;
        let mut last_joltage = [0; 10];
        while let Some((button, joltage, presses)) = q.pop_front() {
            last_presses = presses;
            last_joltage = joltage;
            if (joltage == self.joltage) {
                println!("Joltage presses={presses}");
                return presses;
            }
            let new_joltage = Machine::update_joltage(button, joltage);
            println!("Applying {:?}/{:?} => {:?}", button, joltage, new_joltage);
            if new_joltage[0..self.jnum]
                .iter()
                .enumerate()
                .any(|(i, &j)| j > self.joltage[i])
            {
                println!("Toobig: {presses}/{:?}/{:?}", new_joltage, self.joltage);

                continue;
            }

            self.buttons.iter().for_each(|b| {
                println!("Adding {:?}/{:?}/{}", b, new_joltage, presses + 1);
                q.push_back((b, new_joltage, presses + 1))
            });
            if q.len() > 10000000 {
                panic!("not good enough!");
            }
        }
        panic!(
            "should always find an answer! presses={last_presses}, joltage={:?}",
            last_joltage
        );
        0
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    input.lines().map(|line| Machine::parse(line)).collect()
}
pub fn part1(input: &str) -> usize {
    let machines = parse_input(input);
    machines.iter().map(|m| m.shortest_num_presses()).sum()
}

pub fn part2(input: &str) -> usize {
    let machines = parse_input(input);
    machines.iter().map(|m| m.shortest_joltage_presses()).sum()
}

const INPUT: &str = include_str!("data/input");
fn main() {
    aoc_utils::run::main(INPUT, part1, part2);
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 7;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 33;

    #[test]
    fn test_machine_parts() {
        let machines = parse_input(INPUT);
        machines.iter().for_each(|m| {
            let mut buttons = m.buttons.clone();
            let j = buttons
                .iter()
                .fold([0; 10], |j, b| Machine::update_joltage(b, j));
            println!("{:?} => {:?}", j, m.joltage);
        });
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
