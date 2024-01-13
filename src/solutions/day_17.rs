use md5::{Digest, Md5};
use std::collections::VecDeque;

pub fn solve_1(passcode: &str) -> String {
    let destination = Coordinate { x: 3, y: 3 };
    let start = Room::new(passcode);

    let mut to_visit: VecDeque<Room> = VecDeque::new();
    to_visit.push_back(start);

    while let Some(room) = to_visit.pop_front() {
        if room.coordinate == destination {
            return room.data.chars().skip(passcode.len()).collect();
        }

        room.neighbours()
            .into_iter()
            .for_each(|n| to_visit.push_back(n))
    }

    unreachable!()
}

#[derive(Debug)]
struct Room {
    data: String,
    coordinate: Coordinate,
}

impl Room {
    fn new(passcode: &str) -> Self {
        Self {
            data: passcode.to_string(),
            coordinate: Coordinate { x: 0, y: 0 },
        }
    }

    fn neighbours(&self) -> Vec<Self> {
        let mut hasher = Md5::new();
        hasher.update(&self.data);
        let result: Vec<Door> = hex::encode(hasher.finalize())
            .chars()
            .take(4)
            .map(Door::new)
            .collect();

        let up = if result[0] == Door::Open && self.coordinate.y > 0 {
            Some(Self {
                data: format!("{}{}", self.data, 'U'),
                coordinate: Coordinate {
                    x: self.coordinate.x,
                    y: self.coordinate.y - 1,
                },
            })
        } else {
            None
        };
        let down = if result[1] == Door::Open && self.coordinate.y < 3 {
            Some(Self {
                data: format!("{}{}", self.data, 'D'),
                coordinate: Coordinate {
                    x: self.coordinate.x,
                    y: self.coordinate.y + 1,
                },
            })
        } else {
            None
        };
        let left = if result[2] == Door::Open && self.coordinate.x > 0 {
            Some(Self {
                data: format!("{}{}", self.data, 'L'),
                coordinate: Coordinate {
                    x: self.coordinate.x - 1,
                    y: self.coordinate.y,
                },
            })
        } else {
            None
        };
        let right = if result[3] == Door::Open && self.coordinate.x < 3 {
            Some(Self {
                data: format!("{}{}", self.data, 'R'),
                coordinate: Coordinate {
                    x: self.coordinate.x + 1,
                    y: self.coordinate.y,
                },
            })
        } else {
            None
        };

        [up, down, left, right].into_iter().flatten().collect()
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Door {
    Open,
    Closed,
}

impl Door {
    fn new(door: char) -> Self {
        match door {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'a' => Door::Closed,
            'b' | 'c' | 'd' | 'e' | 'f' => Door::Open,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Coordinate {
    x: u8,
    y: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_17_part_01_sample() {
        assert_eq!("DDRRRD", solve_1("ihgpwlah"));
        assert_eq!("DDUDRLRRUDRD", solve_1("kglvqrro"));
        assert_eq!("DRURDRUDDLLDLUURRDULRLDUUDDDRR", solve_1("ulqzkmiv"));
    }

    #[test]
    fn day_17_part_01_solution() {
        let input = include_str!("../../inputs/day_17.txt").trim();

        assert_eq!("RDRDUDLRDR", solve_1(input));
    }
}
