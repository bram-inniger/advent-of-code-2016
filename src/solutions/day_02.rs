use itertools::Itertools;

pub fn solve_1(instructions: &[&str]) -> u32 {
    let instructions = instructions
        .iter()
        .map(|s| s.chars().map(Direction::new).collect_vec())
        .collect_vec();

    let mut code = 0;
    let mut coordinate = Coordinate::new();

    for key in &instructions {
        for direction in key {
            coordinate = coordinate.go(direction)
        }

        code = code * 10 + coordinate.value()
    }

    code
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Direction {
    U,
    R,
    D,
    L,
}

impl Direction {
    fn new(direction: char) -> Self {
        match direction {
            'U' => Direction::U,
            'R' => Direction::R,
            'D' => Direction::D,
            'L' => Direction::L,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Coordinate {
    x: u8,
    y: u8,
}

impl Coordinate {
    fn new() -> Self {
        Self { x: 1, y: 1 }
    }

    fn go(&self, direction: &Direction) -> Self {
        match direction {
            Direction::U => {
                if self.y > 0 {
                    Coordinate {
                        x: self.x,
                        y: self.y - 1,
                    }
                } else {
                    *self
                }
            }
            Direction::R => {
                if self.x < 2 {
                    Coordinate {
                        x: self.x + 1,
                        y: self.y,
                    }
                } else {
                    *self
                }
            }
            Direction::D => {
                if self.y < 2 {
                    Coordinate {
                        x: self.x,
                        y: self.y + 1,
                    }
                } else {
                    *self
                }
            }
            Direction::L => {
                if self.x > 0 {
                    Coordinate {
                        x: self.x - 1,
                        y: self.y,
                    }
                } else {
                    *self
                }
            }
        }
    }

    fn value(&self) -> u32 {
        match self {
            Coordinate { x: 0, y: 0 } => 1,
            Coordinate { x: 1, y: 0 } => 2,
            Coordinate { x: 2, y: 0 } => 3,
            Coordinate { x: 0, y: 1 } => 4,
            Coordinate { x: 1, y: 1 } => 5,
            Coordinate { x: 2, y: 1 } => 6,
            Coordinate { x: 0, y: 2 } => 7,
            Coordinate { x: 1, y: 2 } => 8,
            Coordinate { x: 2, y: 2 } => 9,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_02_part_01_sample() {
        let sample = vec!["ULL", "RRDDD", "LURDL", "UUUUD"];

        assert_eq!(1_985, solve_1(&sample));
    }

    #[test]
    fn day_02_part_01_solution() {
        let input = include_str!("../../inputs/day_02.txt")
            .lines()
            .collect_vec();

        assert_eq!(69_642, solve_1(&input));
    }
}
