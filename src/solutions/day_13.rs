use rustc_hash::FxHashSet;
use std::collections::VecDeque;
use std::str::FromStr;

pub fn solve_1(favourite: &str, dest_x: i32, dest_y: i32) -> u32 {
    let favourite = i32::from_str(favourite).unwrap();

    let start = Coordinate { x: 1, y: 1 };
    let destination = Coordinate {
        x: dest_x,
        y: dest_y,
    };

    let mut visited: FxHashSet<Coordinate> = FxHashSet::default();
    let mut to_visit: VecDeque<(Coordinate, u32)> = VecDeque::new();
    to_visit.push_back((start, 0));

    while let Some((coord, steps)) = to_visit.pop_front() {
        if visited.contains(&coord) {
            continue;
        }

        if coord == destination {
            return steps;
        }

        visited.insert(coord);
        coord
            .neighbours(favourite)
            .into_iter()
            .for_each(|c| to_visit.push_back((c, steps + 1)));
    }

    unreachable!()
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn neighbours(&self, favourite: i32) -> Vec<Self> {
        [
            Coordinate {
                x: self.x + 1,
                y: self.y,
            },
            Coordinate {
                x: self.x,
                y: self.y + 1,
            },
            Coordinate {
                x: self.x - 1,
                y: self.y,
            },
            Coordinate {
                x: self.x,
                y: self.y - 1,
            },
        ]
        .into_iter()
        .filter(|c| c.x >= 0 && c.y >= 0)
        .filter(|c| c.open(favourite))
        .collect()
    }

    fn open(&self, favourite: i32) -> bool {
        let sum = self.x * self.x
            + 3 * self.x
            + 2 * self.x * self.y
            + self.y
            + self.y * self.y
            + favourite;
        sum.count_ones() % 2 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_13_part_01_sample() {
        let sample = "10";

        assert_eq!(11, solve_1(sample, 7, 4));
    }

    #[test]
    fn day_13_part_01_solution() {
        let input = include_str!("../../inputs/day_13.txt").trim();

        assert_eq!(86, solve_1(input, 31, 39));
    }
}
