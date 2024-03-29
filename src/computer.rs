use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::ops::Not;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Computer {
    pub registers: FxHashMap<Register, i32>,
    pub output: Vec<i32>,
    instructions: Vec<Instruction>,
}

impl Computer {
    pub fn new(code: &[&str], overrides: Vec<(Register, i32)>) -> Self {
        let registers = [Register::A, Register::B, Register::C, Register::D]
            .into_iter()
            .map(|r| (r, 0))
            .chain(overrides)
            .collect();
        let instructions = code.iter().map(|i| Instruction::new(i)).collect();

        Self {
            registers,
            output: vec![],
            instructions,
        }
    }

    pub fn run(&self, unwrap_loop: bool, output_len: usize) -> Self {
        let mut computer = self.clone();
        let mut ip = 0;

        while ip < computer.instructions.len() && computer.output.len() <= output_len {
            ip = computer.step(ip, unwrap_loop);
        }

        computer
    }

    fn step(&mut self, ip: usize, unwrap_loop: bool) -> usize {
        let instruction = &self.instructions[ip];
        let mut ip = ip;

        // Manually unrolled the assembunny for Day 23 Part 2 to replace the loops with the direct code below
        if ip == 5 && unwrap_loop {
            self.registers = [
                (
                    Register::A,
                    self.registers[&Register::A]
                        + self.registers[&Register::B] * self.registers[&Register::D],
                ),
                (Register::B, self.registers[&Register::B]),
                (Register::C, 0),
                (Register::D, 1),
            ]
            .into_iter()
            .collect();
            return 8;
        }
        ip += 1;

        if let Instruction::Jnz { valid: false, .. } = instruction {
            return ip;
        }

        match instruction {
            Instruction::Cpy { from, to, .. } => match from {
                Value::R { register } => {
                    *self.registers.get_mut(to).unwrap() = self.registers[register]
                }
                Value::C { constant } => *self.registers.get_mut(to).unwrap() = *constant,
            },
            Instruction::Inc { register, .. } => *self.registers.get_mut(register).unwrap() += 1,
            Instruction::Dec { register, .. } => *self.registers.get_mut(register).unwrap() -= 1,
            Instruction::Jnz { value, offset, .. } => {
                let i_value = match value {
                    Value::R { register } => self.registers[register],
                    Value::C { constant } => *constant,
                };
                let offset_value = match offset {
                    Value::R { register } => self.registers[register],
                    Value::C { constant } => *constant,
                };

                if i_value != 0 {
                    ip = (ip as i32 + offset_value) as usize - 1;
                }
            }
            Instruction::Tgl { register, .. } => {
                let idx = ip as i32 - 1 + self.registers[register];

                if idx >= 0 && idx < self.instructions.len() as i32 {
                    let idx = idx as usize;
                    self.instructions[idx] = self.instructions[idx].toggle();
                }
            }
            Instruction::Out { value } => {
                let out_value = match value {
                    Value::R { register } => self.registers[register],
                    Value::C { constant } => *constant,
                };
                self.output.push(out_value);
            }
        }
        ip
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Cpy {
        from: Value,
        to: Register,
    },
    Inc {
        register: Register,
    },
    Dec {
        register: Register,
    },
    Jnz {
        value: Value,
        offset: Value,
        valid: bool,
    },
    Tgl {
        register: Register,
    },
    Out {
        value: Value,
    },
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
                    offset: Value::new(split[1]),
                    valid: true,
                }
            }
            "tgl" => Instruction::Tgl {
                register: Register::new(&ins[4..]),
            },
            "out" => Instruction::Out {
                value: Value::new(&ins[4..]),
            },
            _ => unreachable!(),
        }
    }

    fn toggle(&self) -> Self {
        match self.clone() {
            Instruction::Cpy { from, to } => Instruction::Jnz {
                value: from,
                offset: Value::R { register: to },
                valid: true,
            },
            Instruction::Inc { register } => Instruction::Dec { register },
            Instruction::Dec { register } => Instruction::Inc { register },
            Instruction::Jnz {
                value,
                offset,
                valid,
            } => {
                if valid.not() {
                    Instruction::Jnz {
                        value,
                        offset,
                        valid: true,
                    }
                } else {
                    match offset {
                        Value::R { register } => Instruction::Cpy {
                            from: value,
                            to: register,
                        },
                        Value::C { .. } => Instruction::Jnz {
                            value,
                            offset,
                            valid: false,
                        },
                    }
                }
            }
            Instruction::Tgl { register } => Instruction::Inc { register },
            Instruction::Out { .. } => unimplemented!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
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
pub enum Register {
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
