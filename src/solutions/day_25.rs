use crate::computer::{Computer, Register};

pub fn solve_1(code: &[&str]) -> i32 {
    (0..i32::MAX)
        .find(|signal| {
            Computer::new(code, vec![(Register::A, *signal)])
                .run_limited()
                .into_iter()
                .enumerate()
                .all(|(idx, out)| (idx % 2) as i32 == out)
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_25_part_01_sample() {
        // No sample inputs for part 1
    }

    #[test]
    fn day_25_part_01_solution() {
        let input = include_str!("../../inputs/day_25.txt")
            .lines()
            .collect_vec();

        assert_eq!(175, solve_1(&input));
    }
}
