use itertools::Itertools;
use md5::{Digest, Md5};
use rustc_hash::{FxHashMap, FxHashSet};

pub fn solve_1(salt: &str) -> usize {
    let mut idx = 0;
    let mut hashes = (0..=1_000).map(|idx| Hash::new(salt, idx)).collect_vec();
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
        hashes.push(Hash::new(salt, idx + 1_000));

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
    fn new(salt: &str, idx: usize) -> Self {
        let mut hasher = Md5::new();
        hasher.update(format!("{}{}", salt, idx));
        let result = hex::encode(hasher.finalize()).chars().collect_vec();

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
}
