use std::ops::Shl;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Registers {
    ip: u64,
    a: u64,
    b: u64,
    c: u64,
}

impl Registers {
    fn new(ip: u64, a: u64, b: u64, c: u64) -> Registers {
        Registers { ip, a, b, c }
    }

    fn run(&mut self, opcode: OpCode) -> Option<u8> {
        match opcode {
            OpCode::Adv(v) => self.adv(v),
            OpCode::Bxl(v) => self.bxl(v),
            OpCode::Bst(v) => self.bst(v),
            OpCode::Jnz(v) => self.jnz(v),
            OpCode::Bxc(v) => self.bxc(v),
            OpCode::Out(v) => self.out(v),
            OpCode::Bdv(v) => self.bdv(v),
            OpCode::Cdv(v) => self.cdv(v),
        }
    }

    fn combo(&self, val: u8) -> u64 {
        match val {
            x if x < 4 => x as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!(),
        }
    }

    fn adv(&mut self, val: u8) -> Option<u8> {
        self.a = self.a / 1u64.shl(self.combo(val));
        self.ip += 2;
        None
    }

    fn bxl(&mut self, val: u8) -> Option<u8> {
        self.b = (self.b ^ (val as u64));
        self.ip += 2;
        None
    }

    fn bst(&mut self, val: u8) -> Option<u8> {
        self.b = self.combo(val) % 8;
        self.ip += 2;
        None
    }

    fn jnz(&mut self, val: u8) -> Option<u8> {
        if self.a == 0 {
            self.ip += 2;
        } else {
            self.ip = val as u64;
        }
        None
    }

    fn bxc(&mut self, _val: u8) -> Option<u8> {
        self.b = self.b ^ self.c;
        self.ip += 2;
        None
    }

    fn out(&mut self, val: u8) -> Option<u8> {
        self.ip += 2;
        Some((self.combo(val) % 8) as u8)
    }

    fn bdv(&mut self, val: u8) -> Option<u8> {
        self.b = self.a / 1u64.shl(self.combo(val));
        self.ip += 2;
        None
    }
    fn cdv(&mut self, val: u8) -> Option<u8> {
        self.c = self.a / 1u64.shl(self.combo(val));
        self.ip += 2;
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpCode {
    Adv(u8),
    Bxl(u8),
    Bst(u8),
    Jnz(u8),
    Bxc(u8),
    Out(u8),
    Bdv(u8),
    Cdv(u8),
}

impl OpCode {
    fn from(a: u8, b: u8) -> OpCode {
        match a {
            0 => OpCode::Adv(b),
            1 => OpCode::Bxl(b),
            2 => OpCode::Bst(b),
            3 => OpCode::Jnz(b),
            4 => OpCode::Bxc(b),
            5 => OpCode::Out(b),
            6 => OpCode::Bdv(b),
            7 => OpCode::Cdv(b),
            _ => panic!(),
        }
    }

    fn run(&self, registers: &mut Registers) {}
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Computer {
    registers: Registers,
    memory: Vec<u8>,
}

impl Computer {
    fn new(registers: Registers, memory: Vec<u8>) -> Computer {
        Computer { registers, memory }
    }

    fn opcode(&self) -> OpCode {
        OpCode::from(
            self.memory[self.registers.ip as usize],
            self.memory[self.registers.ip as usize + 1],
        )
    }

    fn run(&mut self) -> (Vec<u8>, usize) {
        let mut output = Vec::new();
        let mut num_instructions = 0usize;
        while self.registers.ip < (self.memory.len() - 1) as u64 {
            if let Some(out) = self.registers.run(self.opcode()) {
                output.push(out);
            }
            num_instructions += 1;
        }
        (output, num_instructions)
    }
    fn parse(input: &str) -> Computer {
        let mut sections = input.split("\n\n");
        let mut initial_vals = sections
            .next()
            .unwrap()
            .lines()
            .map(|line| line.split(": ").last().unwrap().parse::<u64>().unwrap())
            .map(|value| value)
            .collect::<Vec<_>>();
        // IP
        let registers = Registers {
            ip: 0,
            a: initial_vals[0],
            b: initial_vals[1],
            c: initial_vals[2],
        };

        let memory = input
            .split(": ")
            .last()
            .unwrap()
            .trim()
            .split(",")
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<_>>();

        Computer { registers, memory }
    }
}

pub fn part1(input: &str) -> String {
    let mut computer = Computer::parse(input);
    let (output, _num_instructions) = computer.run();
    output
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn part2(input: &str) -> u64 {
    let mut computer = Computer::parse(input);
    let num_digits = computer.memory.len();
    let mut valid = vec![0u64];
    for length in (0..num_digits).rev() {
        let old_valid = valid.clone();
        valid.clear();
        for a in old_valid {
            for offset in 0..8 {
                let new_a = 8 * a + offset;
                computer.registers = Registers::new(0, new_a, 0, 0);

                let (output, _) = computer.run();
                println!(
                    "num={a}, newnum={new_a}, {:?}, {:?}",
                    output,
                    &computer.memory[length..]
                );
                if output == computer.memory[length..] {
                    println!("match: {:?}", output);
                    valid.push(new_a)
                }
            }
        }
    }
    *valid.iter().min().unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = include_str!("data/input");
    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: &str = "4,6,3,5,6,3,5,2,1,0";
    pub const TEST_INPUT_2: &str = include_str!("data/input_example_2");
    pub const TEST_ANSWER_2: u64 = 117440;

    #[test]
    fn test_parse_input() {
        let computer = Computer::parse(TEST_INPUT);
        assert_eq!(
            computer.registers,
            Registers {
                ip: 0,
                a: 729,
                b: 0,
                c: 0
            }
        );
        assert_eq!(computer.memory, vec![0, 1, 5, 4, 3, 0]);
    }

    #[test]
    fn test_computer() {
        let mut computer = Computer::new(Registers::new(0, 0, 0, 9), vec![2, 6]);
        let _output = computer.run();
        assert_eq!(computer.registers, Registers::new(2, 0, 1, 9));

        let mut computer = Computer::new(Registers::new(0, 10, 0, 0), vec![5, 0, 5, 1, 5, 4]);
        let (output, _) = computer.run();
        assert_eq!(computer.registers, Registers::new(6, 10, 0, 0));
        assert_eq!(output, vec![0, 1, 2]);

        let mut computer = Computer::new(Registers::new(0, 2024, 0, 0), vec![0, 1, 5, 4, 3, 0]);
        let (output, _) = computer.run();
        assert_eq!(computer.registers, Registers::new(6, 0, 0, 0));
        assert_eq!(output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    }

    #[test]
    fn test_assumption() {
        let mut computer = Computer::parse(INPUT);
        let num_digits = computer.memory.len();
        let min_a = 8u64.pow(num_digits as u32 - 1);
        let a = min_a;
        let mut offset = 1;
        let mut last_digit = 0;
        for j in 0..num_digits / 2 {
            for i in 0..100 {
                computer.registers = Registers::new(0, a + i * offset, 0, 0);
                let (output, _) = computer.run();
                println!(
                    "j={j},i={i},offset={offset},a={},{:?}",
                    a + i * offset,
                    output
                );
                if i != 0 {
                    //assert_ne!(last_digit, output[j]);
                }
                last_digit = output[j];
            }
            offset *= 8;
        }
    }

    #[test]
    fn test_sequence() {
        let mut computer = Computer::parse(INPUT);
        let num_digits = computer.memory.len();
        let mut sequence = Vec::<u8>::new();
        let digit = 10;
        let min = 8u64.pow(digit);
        for a in min..min + 100000 {
            computer.registers = Registers::new(0, a * min, 0, 0);
            let (output, _) = computer.run();
            sequence.push(output[digit as usize]);
        }

        let test_len = 100;
        for m in 1..sequence.len() - test_len {
            if (0..test_len).all(|i| sequence[i] == sequence[m + i]) {
                println!("{test_len} sequence repeats at {m}");
            }
        }
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
