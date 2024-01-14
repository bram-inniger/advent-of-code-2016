use std::str::FromStr;

use itertools::Itertools;

pub fn solve_1(blacklist: &[&str]) -> u64 {
    let ranges = blacklist
        .iter()
        .map(|s| Range::new(s))
        .sorted()
        .collect_vec();

    let mut lowest_range = Range { start: 0, end: 0 };

    for range in ranges.iter() {
        if range.start > lowest_range.end + 1 {
            return lowest_range.end + 1;
        }

        lowest_range.end = lowest_range.end.max(range.end)
    }

    unreachable!()
}

pub fn solve_2(blacklist: &[&str]) -> u64 {
    let ranges = blacklist
        .iter()
        .map(|s| Range::new(s))
        .sorted()
        .collect_vec();

    let mut current = Range { start: 0, end: 0 };
    let mut merged_ranges = vec![];

    for range in ranges.into_iter() {
        if range.start > current.end + 1 {
            merged_ranges.push(current);
            current = range
        } else {
            current.end = current.end.max(range.end)
        }
    }
    merged_ranges.push(current);

    u32::MAX as u64 - merged_ranges.iter().map(|r| r.len()).sum::<u64>() + 1
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(range: &str) -> Self {
        let split = range
            .split('-')
            .map(|s| u64::from_str(s).unwrap())
            .collect_vec();
        Self {
            start: split[0],
            end: split[1],
        }
    }

    fn len(&self) -> u64 {
        self.end - self.start + 1
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_20_part_01_sample() {
        let sample = vec!["5-8", "0-2", "4-7"];

        assert_eq!(3, solve_1(&sample));
    }

    #[test]
    fn day_20_part_01_solution() {
        let input = include_str!("../../inputs/day_20.txt")
            .lines()
            .collect_vec();

        assert_eq!(14_975_795, solve_1(&input));
    }

    #[test]
    fn day_20_part_02_sample() {
        // No sample inputs for part 2
    }

    #[test]
    fn day_20_part_02_solution() {
        let input = include_str!("../../inputs/day_20.txt")
            .lines()
            .collect_vec();

        assert_eq!(101, solve_2(&input));
    }
}
