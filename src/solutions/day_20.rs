use std::str::FromStr;

use itertools::Itertools;

pub fn solve_1(blacklist: &[&str]) -> u32 {
    let ranges = blacklist
        .iter()
        .map(|s| Range::new(s))
        .sorted()
        .collect_vec();

    if ranges.is_empty() || ranges[0].start > 0 {
        return 0;
    }

    let mut lowest_range = Range { start: 0, end: 0 };

    for range in ranges.iter() {
        if range.start > lowest_range.end + 1 {
            break;
        }

        lowest_range.end = lowest_range.end.max(range.end)
    }

    lowest_range.end + 1
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(range: &str) -> Self {
        let split = range
            .split('-')
            .map(|s| u32::from_str(s).unwrap())
            .collect_vec();
        Self {
            start: split[0],
            end: split[1],
        }
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
}
