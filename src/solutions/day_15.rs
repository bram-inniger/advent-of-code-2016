use itertools::Itertools;
use std::str::FromStr;

use regex::Regex;

pub fn solve_1(disks: &[&str]) -> u32 {
    solve(disks, false)
}

pub fn solve_2(disks: &[&str]) -> u32 {
    solve(disks, true)
}

fn solve(disks: &[&str], extra_disk: bool) -> u32 {
    let mut disks = disks.iter().map(|d| Disk::new(d)).collect_vec();
    if extra_disk {
        disks.push(Disk {
            positions: 11,
            offset: 0,
        });
    }
    let mut time = 0;

    loop {
        let all_zero = disks
            .iter()
            .enumerate()
            .all(|(idx, d)| d.position(time + idx as u32 + 1) == 0);

        if all_zero {
            return time;
        }

        time += 1
    }
}

#[derive(Debug)]
struct Disk {
    positions: u32,
    offset: u32,
}

impl Disk {
    fn position(&self, time: u32) -> u32 {
        (time + self.offset) % self.positions
    }
}

impl Disk {
    fn new(disk: &str) -> Self {
        let re = Regex::new(r"^Disc #(:?\d+) has (?<positions>\d+) positions; at time=0, it is at position (?<offset>\d+)\.$").unwrap();
        let caps = re.captures(disk).unwrap();

        let positions = u32::from_str(caps.name("positions").unwrap().as_str()).unwrap();
        let offset = u32::from_str(caps.name("offset").unwrap().as_str()).unwrap();

        Self { positions, offset }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_15_part_01_sample() {
        let sample = vec![
            "Disc #1 has 5 positions; at time=0, it is at position 4.",
            "Disc #2 has 2 positions; at time=0, it is at position 1.",
        ];

        assert_eq!(5, solve_1(&sample));
    }

    #[test]
    fn day_15_part_01_solution() {
        let input = include_str!("../../inputs/day_15.txt")
            .lines()
            .collect_vec();

        assert_eq!(122_318, solve_1(&input));
    }

    #[test]
    fn day_15_part_02_sample() {
        // No sample inputs for part 2
    }

    #[test]
    fn day_15_part_02_solution() {
        let input = include_str!("../../inputs/day_15.txt")
            .lines()
            .collect_vec();

        assert_eq!(3_208_583, solve_2(&input));
    }
}
