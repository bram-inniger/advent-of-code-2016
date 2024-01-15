use itertools::Itertools;
use lazy_static::lazy_static;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use regex::Regex;

pub fn solve_1(instructions: &[&str], password: &str) -> String {
    let instructions = instructions
        .iter()
        .map(|s| Instruction::new(s))
        .collect_vec();
    let mut password = Password::new(password);

    for ins in &instructions {
        ins.run(&mut password);
    }

    password.to_string()
}

pub fn solve_2(instructions: &[&str], to_find: &str) -> String {
    let permutations = to_find.chars().permutations(to_find.len()).collect_vec();
    let instructions = instructions
        .iter()
        .map(|s| Instruction::new(s))
        .collect_vec();

    for p in permutations {
        let mut password = Password { chars: p.clone() };
        for ins in &instructions {
            ins.run(&mut password);
        }

        if password.to_string() == to_find {
            return p.iter().collect();
        }
    }

    unreachable!()
}

#[derive(Debug)]
enum Instruction {
    SwapPosition { x: usize, y: usize },
    SwapLetter { x: char, y: char },
    RotateSteps { rotation: Rotation, steps: usize },
    RotateLetter { x: char },
    ReversePosition { x: usize, y: usize },
    MovePosition { x: usize, y: usize },
}

impl Instruction {
    fn new(instruction: &str) -> Self {
        match &instruction[0..8] {
            "swap pos" => {
                let caps = RE_SWAP_POS.captures(instruction).unwrap();
                let x = usize::from_str(caps.name("pos_x").unwrap().as_str()).unwrap();
                let y = usize::from_str(caps.name("pos_y").unwrap().as_str()).unwrap();
                Instruction::SwapPosition { x, y }
            }
            "swap let" => {
                let caps = RE_SWAP_LET.captures(instruction).unwrap();
                let x = caps.name("let_x").unwrap().as_str().chars().next().unwrap();
                let y = caps.name("let_y").unwrap().as_str().chars().next().unwrap();
                Instruction::SwapLetter { x, y }
            }
            "rotate l" | "rotate r" => {
                let caps = RE_ROT_STEPS.captures(instruction).unwrap();
                let rotation = Rotation::new(caps.name("rot").unwrap().as_str());
                let steps = usize::from_str(caps.name("steps").unwrap().as_str()).unwrap();
                Instruction::RotateSteps { rotation, steps }
            }
            "rotate b" => {
                let caps = RE_ROT_LET.captures(instruction).unwrap();
                let x = caps.name("let").unwrap().as_str().chars().next().unwrap();
                Instruction::RotateLetter { x }
            }
            "reverse " => {
                let caps = RE_REV_POS.captures(instruction).unwrap();
                let x = usize::from_str(caps.name("pos_x").unwrap().as_str()).unwrap();
                let y = usize::from_str(caps.name("pos_y").unwrap().as_str()).unwrap();
                Instruction::ReversePosition { x, y }
            }
            "move pos" => {
                let caps = RE_MOV_POS.captures(instruction).unwrap();
                let x = usize::from_str(caps.name("pos_x").unwrap().as_str()).unwrap();
                let y = usize::from_str(caps.name("pos_y").unwrap().as_str()).unwrap();
                Instruction::MovePosition { x, y }
            }
            _ => unreachable!(),
        }
    }

    fn run(&self, password: &mut Password) {
        match self {
            Instruction::SwapPosition { x, y } => password.swap(*x, *y),
            Instruction::SwapLetter { x, y } => {
                let pos_x = password.position(x);
                let pos_y = password.position(y);
                password.swap(pos_x, pos_y);
            }
            Instruction::RotateSteps { rotation, steps } => password.rotate(rotation, *steps),
            Instruction::RotateLetter { x } => {
                let idx = password.position(x);
                let steps = 1 + idx + if idx >= 4 { 1 } else { 0 };
                password.rotate(&Rotation::Right, steps)
            }
            Instruction::ReversePosition { x, y } => password.rev_substring(*x, *y),
            Instruction::MovePosition { x, y } => password.move_char(*x, *y),
        }
    }
}

#[derive(Debug)]
struct Password {
    chars: Vec<char>,
}

impl Password {
    fn new(password: &str) -> Self {
        let chars = password.chars().collect();
        Self { chars }
    }

    fn swap(&mut self, pos_x: usize, pos_y: usize) {
        self.chars.swap(pos_x, pos_y);
    }

    fn position(&self, x: &char) -> usize {
        self.chars.iter().position(|c| c == x).unwrap()
    }

    fn rotate(&mut self, rotation: &Rotation, steps: usize) {
        let len = self.chars.len() as i32;
        let steps = steps as i32;

        self.chars = (0..len)
            .map(|idx| match rotation {
                Rotation::Left => idx + steps,
                Rotation::Right => idx - steps,
            })
            .map(|idx| idx.rem_euclid(len))
            .map(|idx| self.chars[idx as usize])
            .collect()
    }

    fn rev_substring(&mut self, start: usize, end: usize) {
        let begin = &self.chars[0..start];
        let middle = (0..=end - start)
            .rev()
            .map(|idx| self.chars[start..=end][idx])
            .collect_vec();
        let finish = if end < self.chars.len() - 1 {
            &self.chars[end + 1..]
        } else {
            &[]
        };

        self.chars = begin
            .iter()
            .chain(middle.iter())
            .chain(finish.iter())
            .copied()
            .collect()
    }

    fn move_char(&mut self, from: usize, to: usize) {
        let to_move = self.chars[from];

        let before = &self.chars[0..from];
        let after = &self.chars[from + 1..];

        let chars = before.iter().chain(after.iter()).copied().collect_vec();

        let before = &chars[0..to];
        let after = &chars[to..];

        self.chars = before
            .iter()
            .chain([to_move].iter())
            .chain(after.iter())
            .copied()
            .collect_vec();
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.chars.iter().collect::<String>())
    }
}

#[derive(Debug)]
enum Rotation {
    Left,
    Right,
}

impl Rotation {
    fn new(rotation: &str) -> Self {
        match rotation {
            "left" => Rotation::Left,
            "right" => Rotation::Right,
            _ => unreachable!(),
        }
    }
}

lazy_static! {
    static ref RE_SWAP_POS: Regex =
        Regex::new(r"^swap position (?<pos_x>\d) with position (?<pos_y>\d)$").unwrap();
    static ref RE_SWAP_LET: Regex =
        Regex::new(r"^swap letter (?<let_x>\w) with letter (?<let_y>\w)$").unwrap();
    static ref RE_ROT_STEPS: Regex =
        Regex::new(r"^rotate (?<rot>left|right) (?<steps>\d+) steps?$").unwrap();
    static ref RE_ROT_LET: Regex =
        Regex::new(r"^rotate based on position of letter (?<let>\w)$").unwrap();
    static ref RE_REV_POS: Regex =
        Regex::new(r"^reverse positions (?<pos_x>\d) through (?<pos_y>\d)$").unwrap();
    static ref RE_MOV_POS: Regex =
        Regex::new(r"^move position (?<pos_x>\d) to position (?<pos_y>\d)$").unwrap();
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_21_part_01_sample() {
        let sample = vec![
            "swap position 4 with position 0",
            "swap letter d with letter b",
            "reverse positions 0 through 4",
            "rotate left 1 step",
            "move position 1 to position 4",
            "move position 3 to position 0",
            "rotate based on position of letter b",
            "rotate based on position of letter d",
        ];

        assert_eq!("decab", solve_1(&sample, "abcde"));
    }

    #[test]
    fn day_21_part_01_solution() {
        let input = include_str!("../../inputs/day_21.txt")
            .lines()
            .collect_vec();

        assert_eq!("bdfhgeca", solve_1(&input, "abcdefgh"));
    }

    #[test]
    fn day_21_part_02_sample() {
        // No sample inputs for part 2
    }

    #[test]
    fn day_21_part_02_solution() {
        let input = include_str!("../../inputs/day_21.txt")
            .lines()
            .collect_vec();

        assert_eq!("gdfcabeh", solve_2(&input, "fbgdceah"));
    }
}
