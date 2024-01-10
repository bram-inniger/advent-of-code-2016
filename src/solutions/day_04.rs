use itertools::Itertools;
use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::str::FromStr;

use regex::Regex;

pub fn solve_1(rooms: &[&str]) -> u32 {
    rooms
        .iter()
        .map(|s| Room::new(s))
        .filter(|r| r.real())
        .map(|r| r.sector)
        .sum()
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^(?<name>[a-z\-]+)-(?<sector>\d+)\[(?<check>[a-z]{5})]$").unwrap();
}

#[derive(Debug)]
struct Room<'a> {
    name: &'a str,
    sector: u32,
    checksum: Vec<char>,
}

impl<'a> Room<'a> {
    fn new(room: &'a str) -> Self {
        let caps = RE.captures(room).unwrap();

        let name = caps.name("name").unwrap().as_str();
        let sector = u32::from_str(caps.name("sector").unwrap().as_str()).unwrap();
        let checksum = caps.name("check").unwrap().as_str().chars().collect();

        Self {
            name,
            sector,
            checksum,
        }
    }

    fn real(&self) -> bool {
        let counts = self
            .name
            .chars()
            .filter(|&c| c != '-')
            .counts()
            .into_iter()
            .sorted_by(|(a_char, a_count), (b_char, b_count)| {
                // Sort by char frequency first, in reverse order
                let frequency_cmp = b_count.cmp(a_count);

                // Sort by alphabetical order second for tie-breaks
                match frequency_cmp {
                    Ordering::Equal => a_char.cmp(b_char),
                    _ => frequency_cmp,
                }
            })
            .map(|(ch, _)| ch)
            .take(self.checksum.len())
            .collect_vec();

        counts == self.checksum
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_04_part_01_sample() {
        let sample = vec![
            "aaaaa-bbb-z-y-x-123[abxyz]",
            "a-b-c-d-e-f-g-h-987[abcde]",
            "not-a-real-room-404[oarel]",
            "totally-real-room-200[decoy]",
        ];

        assert_eq!(1_514, solve_1(&sample));
    }

    #[test]
    fn day_04_part_01_solution() {
        let input = include_str!("../../inputs/day_04.txt")
            .lines()
            .collect_vec();

        assert_eq!(245_102, solve_1(&input));
    }
}
