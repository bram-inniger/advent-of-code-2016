use std::str::FromStr;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

pub fn solve_1(instructions: &[&str], seek_chips: [u32; 2]) -> u32 {
    let (bots, chips) = parse_instructions(instructions);

    Factory::new(bots, chips, seek_chips).run().0
}

pub fn solve_2(instructions: &[&str], seek_chips: [u32; 2]) -> u32 {
    let (bots, chips) = parse_instructions(instructions);

    let out = Factory::new(bots, chips, seek_chips).run().1;
    out[&0] * out[&1] * out[&2]
}

lazy_static! {
    static ref RE_BOT: Regex = Regex::new(r"^bot (?<number>\d+) gives low to (?<dest_low>(:?bot|output) \d+) and high to (?<dest_high>(:?bot|output) \d+)$").unwrap();
    static ref RE_CHIP: Regex = Regex::new(r"^value (?<value>\d+) goes to (?<dest>(:?bot|output) \d+)$").unwrap();
}

fn parse_instructions(instructions: &[&str]) -> (FxHashMap<u32, Bot>, VecDeque<Chip>) {
    let bots = instructions
        .iter()
        .filter(|s| s.starts_with("bot"))
        .map(|s| Bot::new(s))
        .map(|b| (b.number, b))
        .collect();
    let chips = instructions
        .iter()
        .filter(|s| s.starts_with("value"))
        .map(|s| Chip::new(s))
        .collect();

    (bots, chips)
}

#[derive(Debug, Clone)]
struct Factory {
    bots: FxHashMap<u32, Bot>,
    chips: VecDeque<Chip>,
    seek_chips: [u32; 2],
}

impl Factory {
    fn new(bots: FxHashMap<u32, Bot>, chips: VecDeque<Chip>, seek_chips: [u32; 2]) -> Self {
        Self {
            bots,
            chips,
            seek_chips,
        }
    }

    fn run(&self) -> (u32, FxHashMap<u32, u32>) {
        let mut fact = self.clone();
        let mut chips_found = 0;
        let mut outputs: FxHashMap<u32, u32> = FxHashMap::default();

        while let Some(chip) = fact.chips.pop_front() {
            match chip.destination {
                Destination::DBot { number } => {
                    let bot = fact.bots.get_mut(&number).unwrap();
                    bot.chips.push(chip.value);

                    if bot.chips.len() == 2 {
                        let sorted_chips = bot.chips.iter().copied().sorted().collect_vec();

                        if sorted_chips == fact.seek_chips {
                            chips_found = bot.number;
                        }

                        bot.chips.clear();
                        fact.chips.push_back(Chip {
                            value: sorted_chips[0],
                            destination: bot.dest_low,
                        });
                        fact.chips.push_back(Chip {
                            value: sorted_chips[1],
                            destination: bot.dest_high,
                        });
                    }
                }
                Destination::Output { number } => {
                    outputs.insert(number, chip.value);
                }
            }
        }

        (chips_found, outputs)
    }
}

#[derive(Debug, Clone)]
struct Bot {
    number: u32,
    dest_low: Destination,
    dest_high: Destination,
    chips: Vec<u32>,
}

impl Bot {
    fn new(instruction: &str) -> Self {
        let caps = RE_BOT.captures(instruction).unwrap();

        let number = u32::from_str(caps.name("number").unwrap().as_str()).unwrap();
        let dest_low = Destination::new(caps.name("dest_low").unwrap().as_str());
        let dest_high = Destination::new(caps.name("dest_high").unwrap().as_str());

        Self {
            number,
            dest_low,
            dest_high,
            chips: vec![],
        }
    }
}

#[derive(Debug, Clone)]
struct Chip {
    value: u32,
    destination: Destination,
}

impl Chip {
    fn new(instruction: &str) -> Self {
        let caps = RE_CHIP.captures(instruction).unwrap();

        let value = u32::from_str(caps.name("value").unwrap().as_str()).unwrap();
        let destination = Destination::new(caps.name("dest").unwrap().as_str());

        Self { value, destination }
    }
}

#[derive(Debug, Copy, Clone)]
enum Destination {
    DBot { number: u32 },
    Output { number: u32 },
}

impl Destination {
    fn new(destination: &str) -> Self {
        match &destination[0..3] {
            "bot" => Destination::DBot {
                number: u32::from_str(&destination[4..]).unwrap(),
            },
            "out" => Destination::Output {
                number: u32::from_str(&destination[7..]).unwrap(),
            },
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_10_part_01_sample() {
        let sample = vec![
            "value 5 goes to bot 2",
            "bot 2 gives low to bot 1 and high to bot 0",
            "value 3 goes to bot 1",
            "bot 1 gives low to output 1 and high to bot 0",
            "bot 0 gives low to output 2 and high to output 0",
            "value 2 goes to bot 2",
        ];

        assert_eq!(2, solve_1(&sample, [2, 5]));
    }

    #[test]
    fn day_10_part_01_solution() {
        let input = include_str!("../../inputs/day_10.txt")
            .lines()
            .collect_vec();

        assert_eq!(161, solve_1(&input, [17, 61]));
    }

    #[test]
    fn day_10_part_02_sample() {
        // No sample inputs for part 2
    }

    #[test]
    fn day_10_part_02_solution() {
        let input = include_str!("../../inputs/day_10.txt")
            .lines()
            .collect_vec();

        assert_eq!(133_163, solve_2(&input, [17, 61]));
    }
}
