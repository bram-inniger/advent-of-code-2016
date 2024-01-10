use std::str::FromStr;

pub fn solve_1(instructions: &str) -> i32 {
    let mut direction = Direction::N;
    let mut coordinate = Coordinate { x: 0, y: 0 };

    instructions
        .split(", ")
        .map(Instruction::new)
        .for_each(|i| {
            direction = direction.turn(&i.turn);
            coordinate = coordinate.go(&direction, i.distance)
        });

    coordinate.distance()
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Instruction {
    turn: Turn,
    distance: i32,
}

impl Instruction {
    fn new(instruction: &str) -> Self {
        let turn = match &instruction[0..1] {
            "R" => Turn::R,
            "L" => Turn::L,
            _ => unreachable!(),
        };

        let distance = i32::from_str(&instruction[1..]).unwrap();

        Self { turn, distance }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn go(&self, direction: &Direction, distance: i32) -> Self {
        match direction {
            Direction::N => Coordinate {
                x: self.x,
                y: self.y - distance,
            },
            Direction::E => Coordinate {
                x: self.x + distance,
                y: self.y,
            },
            Direction::S => Coordinate {
                x: self.x,
                y: self.y + distance,
            },
            Direction::W => Coordinate {
                x: self.x - distance,
                y: self.y,
            },
        }
    }

    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Turn {
    L,
    R,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn turn(&self, turn: &Turn) -> Self {
        match self {
            Direction::N => match turn {
                Turn::L => Direction::W,
                Turn::R => Direction::E,
            },
            Direction::E => match turn {
                Turn::L => Direction::N,
                Turn::R => Direction::S,
            },
            Direction::S => match turn {
                Turn::L => Direction::E,
                Turn::R => Direction::W,
            },
            Direction::W => match turn {
                Turn::L => Direction::S,
                Turn::R => Direction::N,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_01_part_01_sample() {
        assert_eq!(5, solve_1("R2, L3"));
        assert_eq!(2, solve_1("R2, R2, R2"));
        assert_eq!(12, solve_1("R5, L5, R5, R3"));
    }

    #[test]
    fn day_01_part_01_solution() {
        let input = include_str!("../../inputs/day_01.txt").trim();

        assert_eq!(250, solve_1(input));
    }
}
