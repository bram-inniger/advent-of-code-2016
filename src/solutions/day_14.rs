use itertools::Itertools;
use md5::{Digest, Md5};
use rustc_hash::{FxHashMap, FxHashSet};

pub fn solve_1(salt: &str) -> usize {
    solve(salt, &Hash::single)
}

pub fn solve_2(salt: &str) -> usize {
    solve(salt, &Hash::stretched)
}

fn solve(salt: &str, hasher: &dyn Fn(&str, usize) -> Hash) -> usize {
    let mut idx = 0;
    let mut hashes = (0..=1_000).map(|idx| hasher(salt, idx)).collect_vec();
    let mut next_quints: FxHashMap<char, usize> = (1..=1_000)
        .flat_map(|idx| &hashes[idx].quintuplets)
        .group_by(|&c| c)
        .into_iter()
        .map(|(&c, g)| (c, g.collect_vec().len()))
        .collect();
    let mut key_count = 0;

    loop {
        let hash = &hashes[idx];
        if let Some(triple) = hash.triples {
            if next_quints.contains_key(&triple) && next_quints[&triple] > 0 {
                key_count += 1;

                if key_count == 64 {
                    return idx;
                }
            }
        }

        idx += 1;
        hashes.push(hasher(salt, idx + 1_000));

        hashes[idx].quintuplets.iter().for_each(|q| {
            *next_quints.get_mut(q).unwrap() -= 1;
        });
        hashes[idx + 1_000].quintuplets.iter().for_each(|q| {
            *next_quints.entry(*q).or_insert(0) += 1;
        });
    }
}

#[derive(Debug)]
struct Hash {
    triples: Option<char>,
    quintuplets: FxHashSet<char>,
}

impl Hash {
    fn single(salt: &str, idx: usize) -> Self {
        let mut hasher = Md5::new();
        hasher.update(format!("{}{}", salt, idx));
        let hash = hasher.finalize();

        Self::from_hash(hex::encode(hash))
    }

    fn stretched(salt: &str, idx: usize) -> Self {
        let mut hasher = Md5::new();
        hasher.update(format!("{}{}", salt, idx));
        let mut hash = hex::encode(hasher.finalize());

        for _ in 0..2_016 {
            let mut hasher = Md5::new();
            hasher.update(hash);
            hash = hex::encode(hasher.finalize());
        }

        Self::from_hash(hash)
    }

    fn from_hash(hash: String) -> Hash {
        let result = hash.chars().collect_vec();

        let triples = (0..result.len() - 2)
            .find(|&c| (c + 1..c + 3).all(|n| result[c] == result[n]))
            .map(|c| result[c]);
        let quintuplets = (0..result.len() - 4)
            .filter(|&c| (c + 1..c + 5).all(|n| result[c] == result[n]))
            .map(|c| result[c])
            .collect();

        Self {
            triples,
            quintuplets,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_14_part_01_sample() {
        let salt = "abc";

        assert_eq!(22_728, solve_1(salt));
    }

    #[test]
    fn day_14_part_01_solution() {
        let input = include_str!("../../inputs/day_14.txt").trim();

        assert_eq!(15_168, solve_1(input));
    }

    #[ignore = "slow brute force test"]
    #[test]
    fn day_14_part_02_sample() {
        let salt = "abc";

        assert_eq!(22_551, solve_2(salt));
    }

    #[ignore = "slow brute force test"]
    #[test]
    fn day_14_part_02_solution() {
        let input = include_str!("../../inputs/day_14.txt").trim();

        assert_eq!(20_864, solve_2(input));
    }
}
