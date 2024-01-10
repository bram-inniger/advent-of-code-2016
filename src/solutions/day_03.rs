use itertools::Itertools;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

pub fn solve_1(triangles: &[&str]) -> usize {
    triangles
        .iter()
        .map(|s| Triangle::new(s))
        .filter(|t| t.possible())
        .count()
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^\s*(?<a>\d+)\s*(?<b>\d+)\s*(?<c>\d+)$").unwrap();
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Triangle {
    sides: [u32; 3],
}

impl Triangle {
    fn new(triangle: &str) -> Self {
        let caps = RE.captures(triangle).unwrap();
        let a = u32::from_str(caps.name("a").unwrap().as_str()).unwrap();
        let b = u32::from_str(caps.name("b").unwrap().as_str()).unwrap();
        let c = u32::from_str(caps.name("c").unwrap().as_str()).unwrap();

        Triangle { sides: [a, b, c] }
    }

    fn possible(&self) -> bool {
        let sorted = self.sides.iter().copied().sorted().collect_vec();
        sorted[0] + sorted[1] > sorted[2]
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_03_part_01_sample() {
        // No sample inputs for part 1
    }

    #[test]
    fn day_03_part_01_solution() {
        let input = include_str!("../../inputs/day_03.txt")
            .lines()
            .collect_vec();

        assert_eq!(862, solve_1(&input));
    }
}
