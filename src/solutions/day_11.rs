use itertools::Itertools;
use std::collections::VecDeque;
use std::hash::Hash;

use regex::Regex;
use rustc_hash::FxHashSet;

pub fn solve_1(arrangement: &[&str]) -> u16 {
    solve(arrangement, false)
}

pub fn solve_2(arrangement: &[&str]) -> u16 {
    solve(arrangement, true)
}

fn solve(arrangement: &[&str], extras: bool) -> u16 {
    let state = State::new(arrangement, extras);

    let mut visited: FxHashSet<SimpleState> = FxHashSet::default();
    let mut to_visit: VecDeque<State> = VecDeque::new();
    to_visit.push_back(state);

    while let Some(state) = to_visit.pop_front() {
        let simple_state = state.simple();

        if visited.contains(&simple_state) {
            continue;
        }

        if state.finished() {
            return state.nr_steps;
        }

        visited.insert(simple_state);
        state
            .next()
            .into_iter()
            .for_each(|next| to_visit.push_back(next));
    }

    unreachable!()
}

#[derive(Debug, Clone)]
struct State<'a> {
    floor_nr: FloorNr,
    floors: Vec<Floor<'a>>,
    nr_steps: u16,
}

impl<'a> State<'a> {
    fn new(arrangement: &[&'a str], extras: bool) -> Self {
        let re = Regex::new(r"(?<name>\w+)(?<kind> generator|-compatible microchip)").unwrap();
        let mut floors = arrangement
            .iter()
            .map(|floor| {
                re.captures_iter(floor)
                    .map(|caps| {
                        let name = caps.name("name").unwrap().as_str();
                        let kind = caps.name("kind").unwrap().as_str();
                        let kind = match &kind[kind.len() - 9..] {
                            "generator" => Kind::Generator,
                            "microchip" => Kind::Microchip,
                            _ => unreachable!(),
                        };

                        Item { name, kind }
                    })
                    .collect()
            })
            .map(|items| Floor { items })
            .collect_vec();

        if extras {
            [
                Item {
                    name: "elerium",
                    kind: Kind::Generator,
                },
                Item {
                    name: "elerium",
                    kind: Kind::Microchip,
                },
                Item {
                    name: "dilithium",
                    kind: Kind::Generator,
                },
                Item {
                    name: "dilithium",
                    kind: Kind::Microchip,
                },
            ]
            .into_iter()
            .for_each(|i| {
                floors[0].items.insert(i);
            })
        }

        Self {
            floor_nr: FloorNr::new(),
            floors,
            nr_steps: 0,
        }
    }

    fn finished(&self) -> bool {
        (0..3).all(|idx| self.floors[idx].items.is_empty())
    }

    fn simple(&self) -> SimpleState {
        SimpleState {
            floor_nr: self.floor_nr,
            floors: self
                .floors
                .iter()
                .map(|f| {
                    let chips = f.items.iter().filter(|i| i.kind == Kind::Microchip).count();
                    let generators = f.items.iter().filter(|i| i.kind == Kind::Generator).count();

                    (chips, generators)
                })
                .collect(),
        }
    }

    fn next(&self) -> Vec<Self> {
        let items = &self.floors[self.floor_nr.number].items;

        let all_items_to_move = items
            .iter()
            .permutations(2)
            .chain(items.iter().permutations(1))
            .collect_vec();
        self.floor_nr
            .next()
            .into_iter()
            .flat_map(|f| {
                all_items_to_move
                    .iter()
                    .map(|move_items| {
                        let mut new_state = self.clone();

                        new_state.floor_nr = f;
                        move_items.iter().for_each(|i| {
                            new_state.floors[self.floor_nr.number].items.remove(i);
                            new_state.floors[f.number].items.insert((*i).clone());
                        });
                        new_state.nr_steps += 1;

                        new_state
                    })
                    .collect_vec()
            })
            .filter(|s| s.allowed())
            .collect()
    }

    fn allowed(&self) -> bool {
        self.floors.iter().all(|f| f.allowed())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct SimpleState {
    floor_nr: FloorNr,
    floors: Vec<(usize, usize)>,
}

#[derive(Debug, Clone)]
struct Floor<'a> {
    items: FxHashSet<Item<'a>>,
}

impl<'a> Floor<'a> {
    fn allowed(&self) -> bool {
        self.items.is_empty()
            || self.items.iter().all(|i| i.kind == Kind::Microchip)
            || self.items.iter().all(|i| i.kind == Kind::Generator)
            || self
                .items
                .iter()
                .filter(|i| i.kind == Kind::Microchip)
                .all(|i| {
                    self.items.contains(&Item {
                        name: i.name,
                        kind: Kind::Generator,
                    })
                })
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Item<'a> {
    name: &'a str,
    kind: Kind,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Kind {
    Microchip,
    Generator,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct FloorNr {
    number: usize,
}

impl FloorNr {
    fn new() -> FloorNr {
        FloorNr { number: 0 }
    }

    fn next(&self) -> Vec<FloorNr> {
        match self.number {
            0 => vec![FloorNr { number: 1 }],
            1 => vec![FloorNr { number: 0 }, FloorNr { number: 2 }],
            2 => vec![FloorNr { number: 1 }, FloorNr { number: 3 }],
            3 => vec![FloorNr { number: 2 }],
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_11_part_01_sample() {
        let sample = vec![
            "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.",
            "The second floor contains a hydrogen generator.",
            "The third floor contains a lithium generator.",
            "The fourth floor contains nothing relevant.",
        ];

        assert_eq!(11, solve_1(&sample));
    }

    #[test]
    fn day_11_part_01_solution() {
        let input = include_str!("../../inputs/day_11.txt")
            .lines()
            .collect_vec();

        assert_eq!(33, solve_1(&input));
    }

    #[test]
    fn day_11_part_02_sample() {
        // No sample inputs for part 1
    }

    #[test]
    fn day_11_part_02_solution() {
        let input = include_str!("../../inputs/day_11.txt")
            .lines()
            .collect_vec();

        assert_eq!(57, solve_2(&input));
    }
}
