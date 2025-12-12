use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
};

type Lights = usize;
type Button = Vec<usize>;
type Joltage = Vec<usize>;

#[derive(Debug)]
struct Machine {
    lights: Lights,
    buttons: Vec<Button>,
    joltage: Joltage,
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

    fn parse_joltage(j_str: &str) -> Joltage {
        Machine::map_to_nums(j_str)
    }
    fn parse(line: &str) -> Machine {
        let (goal_str, rest_str) = line.split_once(']').unwrap();
        let (buttons_str, joltage_str) = rest_str.split_once('{').unwrap();
        let lights = Machine::parse_goal(goal_str);
        let buttons = Machine::parse_buttons(buttons_str);
        let joltage = Machine::parse_joltage(joltage_str);
        Machine {
            lights,
            buttons,
            joltage,
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

    fn update_joltage(button: &Button, joltage: &Joltage) -> Joltage {
        button.iter().fold(joltage.clone(), |mut j, &b| {
            j[b] += 1;
            j
        })
    }

    fn d_joltage(&self, joltage: &Joltage) -> Joltage {
        let min = joltage.iter().min().unwrap();
        joltage.iter().enumerate().fold(vec![], |mut dj, (i, &j)| {
            dj.push(joltage[i] - min);
            dj
        })
    }

    fn gen_equations(&self) -> Vec<(usize, Vec<usize>)> {
        self.joltage
            .iter()
            .enumerate()
            .map(|(i, &j)| {
                let e = self
                    .buttons
                    .iter()
                    .enumerate()
                    .filter(|b| b.1.contains(&i))
                    .map(|b| b.0)
                    .collect::<Vec<_>>();
                (j, e)
            })
            .collect()
    }

    fn equations_valid(&self, equations: &[(usize, Vec<usize>)], presses: &[usize]) -> bool {
        equations.iter().all(|e| {
            let res = e.0 >= e.1.iter().map(|&b| presses[b]).sum();
            if !res {
                //println!("Equation failed: {:?} - {:?}", e, presses);
            }
            res
        })
    }

    fn old_shortest_joltage_presses(&self) -> usize {
        println!("finding joltage for {:?}", self.joltage);
        let mut q = self
            .buttons
            .iter()
            .map(|b| (b, vec![0; self.joltage.len()], vec![0; self.buttons.len()]))
            .collect::<VecDeque<_>>();

        let equations = self.gen_equations();

        while let Some((button, joltage, presses)) = q.pop_front() {
            if joltage == self.joltage {
                println!("Joltage presses={:?}", presses);
                return presses.iter().sum();
            }
            let new_joltage = Machine::update_joltage(button, &joltage);
            //println!("Applying {:?}/{:?} => {:?}", button, joltage, new_joltage);
            if new_joltage
                .iter()
                .enumerate()
                .any(|(i, &j)| j > self.joltage[i])
            {
                //println!("Toobig: {:?}/{:?}/{:?}", presses, new_joltage, self.joltage);

                continue;
            }

            self.buttons.iter().enumerate().for_each(|b| {
                let mut new_presses = presses.clone();
                new_presses[b.0] += 1;
                if self.equations_valid(&equations, &presses) {
                    //println!("Adding {:?}/{:?}/{:?}", b, new_joltage, presses);
                    q.push_back((b.1, new_joltage.clone(), new_presses))
                } else {
                    println!("Equations not satisfied! {:?} => {:?}", equations, presses);
                }
            });
            if q.len() > 10000000 {
                panic!("not good enough!");
            }
        }
        panic!("should always find an answer!");
        0
    }

    fn solve_joltage(&self) -> f64 {
        use good_lp::*;
        let mut vars = variables!();
        let press_vars = self
            .buttons
            .iter()
            .map(|_b| vars.add(variable().min(0).integer()))
            .collect::<Vec<_>>();

        let objective: Expression = press_vars.iter().sum();
        let mut problem = vars.minimise(objective).using(default_solver);

        let mut exprs = vec![0.into_expression(); self.joltage.len()];
        for button_idx in 0..self.buttons.len() {
            for &j_idx in &self.buttons[button_idx] {
                exprs[j_idx] += press_vars[button_idx];
            }
        }

        for (e, &j) in exprs.into_iter().zip(self.joltage.iter()) {
            problem = problem.with(e.eq(j as u32));
        }
        let sol = problem.solve().unwrap();
        press_vars.iter().map(|&v| sol.value(v)).sum::<f64>()
    }

    fn joltage_exceeded(&self, joltage: &Joltage) -> bool {
        joltage
            .iter()
            .enumerate()
            .any(|(i, &j)| j > self.joltage[i])
    }

    fn shortest_joltage_presses(&self) -> usize {
        println!("finding joltage for {:?}", self.joltage);
        let mut q = self
            .buttons
            .iter()
            .map(|b| (b, vec![0; self.joltage.len()], vec![0; self.buttons.len()]))
            .collect::<VecDeque<_>>();

        let equations = self.gen_equations();
        let bmap = self
            .buttons
            .iter()
            .map(|b| {
                let mut max_p = 0;
                let mut j = vec![0; self.joltage.len()];
                while self.joltage_exceeded(&j) {
                    max_p += 1;
                    Machine::update_joltage(b, &j);
                }
                (b, max_p - 1)
            })
            .collect::<Vec<_>>();

        while let Some((button, joltage, presses)) = q.pop_front() {
            if joltage == self.joltage {
                println!("Joltage presses={:?}", presses);
                return presses.iter().sum();
            }
            let new_joltage = Machine::update_joltage(button, &joltage);
            //println!("Applying {:?}/{:?} => {:?}", button, joltage, new_joltage);
            if new_joltage
                .iter()
                .enumerate()
                .any(|(i, &j)| j > self.joltage[i])
            {
                //println!("Toobig: {:?}/{:?}/{:?}", presses, new_joltage, self.joltage);

                continue;
            }

            self.buttons.iter().enumerate().for_each(|b| {
                let mut new_presses = presses.clone();
                new_presses[b.0] += 1;
                if self.equations_valid(&equations, &presses) {
                    //println!("Adding {:?}/{:?}/{:?}", b, new_joltage, presses);
                    q.push_back((b.1, new_joltage.clone(), new_presses))
                } else {
                    println!("Equations not satisfied! {:?} => {:?}", equations, presses);
                }
            });
            if q.len() > 10000000 {
                panic!("not good enough!");
            }
        }
        panic!("should always find an answer!");
        0
    }

    fn try_r(&self, buttons: &[(&Button, usize)], d_joltage: &Joltage) -> Option<usize> {
        if buttons.len() == 0 {
            return None;
        }

        let b = buttons[0].0;
        let max_presses = b.iter().map(|&f_i| d_joltage[f_i]).min().unwrap();

        (0..=max_presses).rev().find_map(|presses| {
            let mut d_joltage = d_joltage.clone();
            b.iter().for_each(|&j| d_joltage[j] -= presses);
            //println!(
            //    "trying {:?}: presses={presses} (max={max_presses}), d_joltage={:?}",
            //    b, d_joltage
            //);

            if d_joltage.iter().all(|&j| j == 0) {
                // We've found a match
                return Some(presses);
            }

            if let Some(sub_presses) = self.try_r(&buttons[1..buttons.len()], &d_joltage) {
                Some(presses + sub_presses)
            } else {
                None
            }
        })
    }

    fn try_idea(&self) -> usize {
        println!("trying for {:?} - {:?}", self.buttons, self.joltage);
        let mut button_order = self
            .buttons
            .iter()
            .map(|b| (b, b.iter().map(|&f_i| self.joltage[f_i]).min().unwrap()))
            .collect::<Vec<_>>();
        button_order.sort_by(|a, b| match a.0.len().cmp(&b.0.len()).reverse() {
            Ordering::Equal => a.1.cmp(&b.1),
            c => c,
        });

        let p = self.try_r(&button_order, &self.joltage).unwrap();
        println!("Presses={p}");
        p
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
    //let s: f64 = machines.iter().map(|m| m.solve_joltage()).sum();
    let s: usize = machines.iter().map(|m| m.try_idea()).sum();
    println!("sum={s}");
    s as usize
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
                .fold(vec![], |j, b| Machine::update_joltage(b, &j));
            println!("{:?} => {:?}", j, m.joltage);
        });
    }

    #[test]
    fn test_buttons() {
        let machines = parse_input(INPUT);
        let n = machines.iter().map(|m| m.buttons.len()).max().unwrap();
        println!("max buttons {n}");
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
