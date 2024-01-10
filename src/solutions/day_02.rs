use itertools::Itertools;

pub fn solve_1(instructions: &[&str]) -> String {
    let instructions = instructions
        .iter()
        .map(|s| s.chars().map(Direction::new).collect_vec())
        .collect_vec();

    let mut code = vec![];
    let mut keypad = SimpleKeypadKey::new();

    for key in &instructions {
        for direction in key {
            keypad = keypad.next(direction)
        }

        code.push(keypad.key.value());
    }

    code.iter().collect()
}

pub fn solve_2(instructions: &[&str]) -> String {
    let instructions = instructions
        .iter()
        .map(|s| s.chars().map(Direction::new).collect_vec())
        .collect_vec();

    let mut code = vec![];
    let mut keypad = TrickyKeypadKey::new();

    for key in &instructions {
        for direction in key {
            keypad = keypad.next(direction)
        }

        code.push(keypad.key.value());
    }

    code.iter().collect()
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
enum Key {
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
    KA,
    KB,
    KC,
    KD,
}

impl Key {
    fn value(&self) -> char {
        match self {
            Key::K1 => '1',
            Key::K2 => '2',
            Key::K3 => '3',
            Key::K4 => '4',
            Key::K5 => '5',
            Key::K6 => '6',
            Key::K7 => '7',
            Key::K8 => '8',
            Key::K9 => '9',
            Key::KA => 'A',
            Key::KB => 'B',
            Key::KC => 'C',
            Key::KD => 'D',
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct SimpleKeypadKey {
    key: Key,
}

impl SimpleKeypadKey {
    fn new() -> Self {
        Self { key: Key::K5 }
    }

    fn next(&self, direction: &Direction) -> Self {
        let key = match self.key {
            Key::K1 => match direction {
                Direction::R => Key::K2,
                Direction::D => Key::K4,
                _ => self.key,
            },
            Key::K2 => match direction {
                Direction::R => Key::K3,
                Direction::D => Key::K5,
                Direction::L => Key::K1,
                _ => self.key,
            },
            Key::K3 => match direction {
                Direction::D => Key::K6,
                Direction::L => Key::K2,
                _ => self.key,
            },
            Key::K4 => match direction {
                Direction::U => Key::K1,
                Direction::R => Key::K5,
                Direction::D => Key::K7,
                _ => self.key,
            },
            Key::K5 => match direction {
                Direction::U => Key::K2,
                Direction::R => Key::K6,
                Direction::D => Key::K8,
                Direction::L => Key::K4,
            },
            Key::K6 => match direction {
                Direction::U => Key::K3,
                Direction::D => Key::K9,
                Direction::L => Key::K5,
                _ => self.key,
            },
            Key::K7 => match direction {
                Direction::U => Key::K4,
                Direction::R => Key::K8,
                _ => self.key,
            },
            Key::K8 => match direction {
                Direction::U => Key::K5,
                Direction::R => Key::K9,
                Direction::L => Key::K7,
                _ => self.key,
            },
            Key::K9 => match direction {
                Direction::U => Key::K6,
                Direction::L => Key::K8,
                _ => self.key,
            },
            _ => unreachable!(),
        };

        Self { key }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct TrickyKeypadKey {
    key: Key,
}

impl TrickyKeypadKey {
    fn new() -> Self {
        Self { key: Key::K5 }
    }

    fn next(&self, direction: &Direction) -> Self {
        let key = match self.key {
            Key::K1 => match direction {
                Direction::D => Key::K3,
                _ => self.key,
            },
            Key::K2 => match direction {
                Direction::R => Key::K3,
                Direction::D => Key::K6,
                _ => self.key,
            },
            Key::K3 => match direction {
                Direction::U => Key::K1,
                Direction::R => Key::K4,
                Direction::D => Key::K7,
                Direction::L => Key::K2,
            },
            Key::K4 => match direction {
                Direction::L => Key::K3,
                Direction::D => Key::K8,
                _ => self.key,
            },
            Key::K5 => match direction {
                Direction::R => Key::K6,
                _ => self.key,
            },
            Key::K6 => match direction {
                Direction::U => Key::K2,
                Direction::R => Key::K7,
                Direction::D => Key::KA,
                Direction::L => Key::K5,
            },
            Key::K7 => match direction {
                Direction::U => Key::K3,
                Direction::R => Key::K8,
                Direction::D => Key::KB,
                Direction::L => Key::K6,
            },
            Key::K8 => match direction {
                Direction::U => Key::K4,
                Direction::R => Key::K9,
                Direction::D => Key::KC,
                Direction::L => Key::K7,
            },
            Key::K9 => match direction {
                Direction::L => Key::K8,
                _ => self.key,
            },
            Key::KA => match direction {
                Direction::U => Key::K6,
                Direction::R => Key::KB,
                _ => self.key,
            },
            Key::KB => match direction {
                Direction::U => Key::K7,
                Direction::R => Key::KC,
                Direction::D => Key::KD,
                Direction::L => Key::KA,
            },
            Key::KC => match direction {
                Direction::U => Key::K8,
                Direction::L => Key::KB,
                _ => self.key,
            },
            Key::KD => match direction {
                Direction::U => Key::KB,
                _ => self.key,
            },
        };

        Self { key }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_02_part_01_sample() {
        let sample = vec!["ULL", "RRDDD", "LURDL", "UUUUD"];

        assert_eq!("1985", solve_1(&sample));
    }

    #[test]
    fn day_02_part_01_solution() {
        let input = include_str!("../../inputs/day_02.txt")
            .lines()
            .collect_vec();

        assert_eq!("69642", solve_1(&input));
    }

    #[test]
    fn day_02_part_02_sample() {
        let sample = vec!["ULL", "RRDDD", "LURDL", "UUUUD"];

        assert_eq!("5DB3", solve_2(&sample));
    }

    #[test]
    fn day_02_part_02_solution() {
        let input = include_str!("../../inputs/day_02.txt")
            .lines()
            .collect_vec();

        assert_eq!("8CB23", solve_2(&input));
    }
}
