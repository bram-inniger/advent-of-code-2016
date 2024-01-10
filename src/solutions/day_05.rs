use itertools::Itertools;
use md5::{Digest, Md5};

pub fn solve_1(door_id: &str) -> String {
    let mut index = 0u32;
    let mut valid_hashes = vec![];

    while valid_hashes.len() < 8 {
        let mut hasher = Md5::new();
        hasher.update(format!("{}{}", door_id, index));
        let result = hex::encode(hasher.finalize());

        if result.starts_with("00000") {
            valid_hashes.push(result);
        }

        index += 1;
    }

    valid_hashes
        .iter()
        .flat_map(|s| s.chars().dropping(5).take(1))
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
}
