use std::{num::ParseIntError, str::FromStr};

enum CommandErr {
    BadData,
    Parse(ParseIntError),
}

impl From<ParseIntError> for CommandErr {
    fn from(err: ParseIntError) -> CommandErr {
        CommandErr::Parse(err)
    }
}

struct Pos(u64, u64);
struct Aim(u64, Pos);

enum Command {
    Forward(u64),
    Down(u64),
    Up(u64),
}

impl Command {
    fn update_pos(self: &Self, pos: Pos) -> Pos {
        match self {
            Command::Forward(x) => Pos(pos.0 + x, pos.1),
            Command::Down(y) => Pos(pos.0, pos.1 + y),
            Command::Up(y) => Pos(pos.0, pos.1.saturating_sub(*y)),
        }
    }

    fn update_aim(self: &Self, aim: Aim) -> Aim {
        match self {
            Command::Forward(x) => Aim(aim.0, Pos(aim.1 .0 + x, aim.1 .1 + x * aim.0)),
            Command::Down(y) => Aim(aim.0 + y, aim.1),
            Command::Up(y) => Aim(aim.0.saturating_sub(*y), aim.1),
        }
    }
}

impl FromStr for Command {
    type Err = CommandErr;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut parts = line.split(' ');
        let cmd = parts.next().ok_or(CommandErr::BadData)?;
        let dist = parts.next().ok_or(CommandErr::BadData)?.parse::<u64>()?;
        match cmd {
            "forward" => Ok(Command::Forward(dist)),
            "down" => Ok(Command::Down(dist)),
            "up" => Ok(Command::Up(dist)),
            _ => Err(CommandErr::BadData),
        }
    }
}

fn solve_part1(commands: &[Command]) -> u64 {
    let pos = commands.iter().fold(Pos(0, 0), |p, cmd| cmd.update_pos(p));
    pos.0 * pos.1
}

fn solve_part2(commands: &[Command]) -> u64 {
    let aim = commands
        .iter()
        .fold(Aim(0, Pos(0, 0)), |a, cmd| cmd.update_aim(a));
    aim.1 .0 * aim.1 .1
}

fn parse_input(input: &str) -> Vec<Command> {
    input
        .split('\n')
        .into_iter()
        .flat_map(|line| Command::from_str(line))
        .collect()
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let commands = parse_input(INPUT);
    let part1 = solve_part1(&commands);
    println!("Part1: {part1}");
    let part2 = solve_part2(&commands);
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&parse_input(TEST_INPUT)), 150);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&parse_input(TEST_INPUT)), 900);
    }
}
