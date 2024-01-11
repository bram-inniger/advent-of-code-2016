use std::ops::Not;

use itertools::Itertools;
use md5::{Digest, Md5};
use rustc_hash::FxHashMap;

pub fn solve_1(door_id: &str) -> String {
    let mut index = 0u32;
    let mut solution = vec![];

    while solution.len() < 8 {
        let mut hasher = Md5::new();
        hasher.update(format!("{}{}", door_id, index));
        let result = hex::encode(hasher.finalize());

        if result.starts_with("00000") {
            solution.push(result.chars().dropping(5).take(1).collect_vec()[0]);
        }

        index += 1;
    }

    solution.iter().collect()
}

pub fn solve_2(door_id: &str) -> String {
    let mut index = 0u32;
    let mut solution: FxHashMap<u8, u8> = FxHashMap::default();

    while solution.len() < 8 {
        let mut hasher = Md5::new();
        hasher.update(format!("{}{}", door_id, index));
        let result = hex::encode(hasher.finalize());
        let chars = result
            .chars()
            .dropping(5)
            .take(2)
            .map(|c| c as u8)
            .collect_vec();

        if result.starts_with("00000")
            && chars[0] >= b'0'
            && chars[0] < b'8'
            && solution.contains_key(&chars[0]).not()
        {
            solution.insert(chars[0], chars[1]);
        }

        index += 1;
    }

    (b'0'..b'8')
        .map(|idx| solution[&idx])
        .map(|c| c as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "slow brute force test"]
    #[test]
    fn day_05_part_01_sample() {
        let sample = "abc";

        assert_eq!("18f47a30", solve_1(sample));
    }

    #[ignore = "slow brute force test"]
    #[test]
    fn day_05_part_01_solution() {
        let input = include_str!("../../inputs/day_05.txt").trim();

        assert_eq!("2414bc77", solve_1(input));
    }

    #[ignore = "slow brute force test"]
    #[test]
    fn day_05_part_02_sample() {
        let sample = "abc";

        assert_eq!("05ace8e3", solve_2(sample));
    }

    #[ignore = "slow brute force test"]
    #[test]
    fn day_05_part_02_solution() {
        let input = include_str!("../../inputs/day_05.txt").trim();

        assert_eq!("437e60fc", solve_2(input));
    }
}
