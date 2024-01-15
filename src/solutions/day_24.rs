use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve_1(blueprint: &[&str]) -> u32 {
    Ducts::new(blueprint).total_shortest_path()
}

#[derive(Debug)]
struct Ducts {
    locations: Vec<Coordinate>,
    neighbours: HashMap<Coordinate, Vec<Coordinate>>,
}

impl Ducts {
    fn new(blueprint: &[&str]) -> Self {
        let blueprint = blueprint.iter().map(|s| s.as_bytes()).collect_vec();
        let mut passages: HashSet<Coordinate> = HashSet::default();
        let mut locations: HashMap<Coordinate, u8> = HashMap::default();

        for y in 0..blueprint.len() {
            for x in 0..blueprint[0].len() {
                let c = Coordinate {
                    x: x as i32,
                    y: y as i32,
                };
                match blueprint[y][x] {
                    b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                        passages.insert(c);
                        locations.insert(c, blueprint[y][x]);
                    }
                    b'.' => {
                        passages.insert(c);
                    }
                    b'#' => {}
                    _ => unreachable!(),
                }
            }
        }

        let locations = locations
            .into_iter()
            .sorted_by_key(|(_, v)| *v)
            .map(|(k, _)| k)
            .collect();
        let neighbours = passages
            .iter()
            .map(|p| {
                (
                    *p,
                    p.neighbours()
                        .into_iter()
                        .filter(|n| passages.contains(n))
                        .collect_vec(),
                )
            })
            .collect();

        Self {
            locations,
            neighbours,
        }
    }

    fn total_shortest_path(&self) -> u32 {
        let shortest_paths: HashMap<(&Coordinate, &Coordinate), u32> = self
            .locations
            .iter()
            .permutations(2)
            .map(|p| (p[0], p[1]))
            .map(|(start, end)| ((start, end), self.shortest_path(start, end)))
            .collect();

        let visit_orderings = self.locations[1..]
            .iter()
            .permutations(self.locations.len() - 1)
            .collect_vec();

        let mut min_distance = u32::MAX;

        for ordering in visit_orderings {
            let mut distance = 0;
            let mut current = &self.locations[0];

            for next in ordering {
                distance += shortest_paths[&(current, next)];
                current = next;
            }

            min_distance = min_distance.min(distance);
        }

        min_distance
    }

    fn shortest_path(&self, start: &Coordinate, end: &Coordinate) -> u32 {
        let mut visited: HashSet<&Coordinate> = HashSet::default();
        let mut to_visit = VecDeque::new();
        to_visit.push_back((start, 0u32));

        while let Some((c, steps)) = to_visit.pop_front() {
            if visited.contains(&c) {
                continue;
            }
            if c == end {
                return steps;
            }

            visited.insert(c);

            self.neighbours[c]
                .iter()
                .for_each(|n| to_visit.push_back((n, steps + 1)));
        }

        unreachable!()
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn neighbours(&self) -> Vec<Coordinate> {
        [(0, -1), (1, 0), (0, 1), (-1, 0)]
            .into_iter()
            .map(|c| Coordinate {
                x: self.x + c.0,
                y: self.y + c.1,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_24_part_01_sample() {
        let sample = vec![
            "###########",
            "#0.1.....2#",
            "#.#######.#",
            "#4.......3#",
            "###########",
        ];

        assert_eq!(14, solve_1(&sample));
    }

    #[test]
    fn day_24_part_01_solution() {
        let input = include_str!("../../inputs/day_24.txt")
            .lines()
            .collect_vec();

        assert_eq!(474, solve_1(&input));
    }
}
