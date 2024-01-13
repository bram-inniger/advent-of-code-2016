use itertools::Itertools;
use std::str::FromStr;
use std::{i32, usize};

use rustc_hash::FxHashMap;

pub fn solve_1(code: &[&str]) -> i32 {
    Computer::new(code, 0).run()[&Register::A]
}

pub fn solve_2(code: &[&str]) -> i32 {
    Computer::new(code, 1).run()[&Register::A]
}

#[derive(Debug)]
struct Computer {
    registers: FxHashMap<Register, i32>,
    instructions: Vec<Instruction>,
}

impl Computer {
    fn new(code: &[&str], c: i32) -> Self {
        let registers = [
            (Register::A, 0),
            (Register::B, 0),
            (Register::C, c),
            (Register::D, 0),
        ]
        .into_iter()
        .collect();
        let instructions = code.iter().map(|i| Instruction::new(i)).collect();

        Self {
            registers,
            instructions,
        }
    }

    fn run(&self) -> FxHashMap<Register, i32> {
        let mut registers = self.registers.clone();
        let mut ip = 0;

        while ip < self.instructions.len() {
            match &self.instructions[ip] {
                Instruction::Cpy { from, to } => {
                    match from {
                        Value::R { register } => {
                            *registers.get_mut(to).unwrap() = registers[register]
                        }
                        Value::C { constant } => *registers.get_mut(to).unwrap() = *constant,
                    };
                    ip += 1;
                }
                Instruction::Inc { register } => {
                    *registers.get_mut(register).unwrap() += 1;
                    ip += 1;
                }
                Instruction::Dec { register } => {
                    *registers.get_mut(register).unwrap() -= 1;
                    ip += 1;
                }
                Instruction::Jnz { value, offset } => {
                    let i_value = match value {
                        Value::R { register } => registers[register],
                        Value::C { constant } => *constant,
                    };

                    if i_value != 0 {
                        ip = (ip as i32 + offset) as usize;
                    } else {
                        ip += 1;
                    }
                }
            }
        }

        registers
    }
}

#[derive(Debug)]
enum Instruction {
    Cpy { from: Value, to: Register },
    Inc { register: Register },
    Dec { register: Register },
    Jnz { value: Value, offset: i32 },
}

impl Instruction {
    fn new(ins: &str) -> Self {
        match &ins[0..3] {
            "cpy" => Instruction::Cpy {
                from: Value::new(&ins[4..ins.len() - 2]),
                to: Register::new(&ins[ins.len() - 1..]),
            },
            "inc" => Instruction::Inc {
                register: Register::new(&ins[4..]),
            },
            "dec" => Instruction::Dec {
                register: Register::new(&ins[4..]),
            },
            "jnz" => {
                let split = &ins[4..].split(' ').collect_vec();
                Instruction::Jnz {
                    value: Value::new(split[0]),
                    offset: i32::from_str(split[1]).unwrap(),
                }
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Value {
    R { register: Register },
    C { constant: i32 },
}

impl Value {
    fn new(value: &str) -> Self {
        match value {
            "a" | "b" | "c" | "d" => Self::R {
                register: Register::new(value),
            },
            _ => Self::C {
                constant: i32::from_str(value).unwrap(),
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Register {
    A,
    B,
    C,
    D,
}

impl Register {
    fn new(register: &str) -> Self {
        match register {
            "a" => Register::A,
            "b" => Register::B,
            "c" => Register::C,
            "d" => Register::D,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_12_part_01_sample() {
        let sample = vec!["cpy 41 a", "inc a", "inc a", "dec a", "jnz a 2", "dec a"];

        assert_eq!(42, solve_1(&sample));
    }

    #[test]
    fn day_12_part_01_solution() {
        let input = include_str!("../../inputs/day_12.txt")
            .lines()
            .collect_vec();

        assert_eq!(318_009, solve_1(&input));
    }

    #[test]
    fn day_12_part_02_sample() {
        // No sample inputs for part 2
    }

    #[test]
    fn day_12_part_02_solution() {
        let input = include_str!("../../inputs/day_12.txt")
            .lines()
            .collect_vec();

        assert_eq!(9_227_663, solve_2(&input));
    }
}
