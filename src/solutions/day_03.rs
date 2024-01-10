use std::str::FromStr;

use itertools::Itertools;
use regex::Regex;

pub fn solve_1(sides: &[&str]) -> usize {
    let sides = parse_sides(sides);

    (0..sides.len())
        .flat_map(|y| {
            (0..sides[0].len())
                .chunks(3)
                .into_iter()
                .map(|c| {
                    let xs = c.collect_vec();
                    Triangle::new(sides[y][xs[0]], sides[y][xs[1]], sides[y][xs[2]])
                })
                .collect_vec()
        })
        .filter(|t| t.possible())
        .count()
}

pub fn solve_2(sides: &[&str]) -> usize {
    let sides = parse_sides(sides);

    (0..sides[0].len())
        .flat_map(|x| {
            (0..sides.len())
                .chunks(3)
                .into_iter()
                .map(|c| {
                    let ys = c.collect_vec();
                    Triangle::new(sides[ys[0]][x], sides[ys[1]][x], sides[ys[2]][x])
                })
                .collect_vec()
        })
        .filter(|t| t.possible())
        .count()
}

fn parse_sides(sides: &[&str]) -> Vec<Vec<usize>> {
    let re = Regex::new(r"(\d+)").unwrap();
    sides
        .iter()
        .map(|l| {
            re.find_iter(l)
                .map(|s| usize::from_str(s.as_str()).unwrap())
                .collect()
        })
        .collect()
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Triangle {
    sides: Vec<usize>,
}

impl Triangle {
    fn new(a: usize, b: usize, c: usize) -> Self {
        Self {
            sides: [a, b, c].into_iter().sorted().collect_vec(),
        }
    }

    fn possible(&self) -> bool {
        self.sides[0] + self.sides[1] > self.sides[2]
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

    #[test]
    fn day_03_part_02_sample() {
        // No sample inputs for part 2
    }

    #[test]
    fn day_03_part_02_solution() {
        let input = include_str!("../../inputs/day_03.txt")
            .lines()
            .collect_vec();

        assert_eq!(1_577, solve_2(&input));
    }
}
