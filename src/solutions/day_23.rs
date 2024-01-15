use crate::util::{Computer, Register};

pub fn solve_1(code: &[&str]) -> i32 {
    Computer::new(code, vec![(Register::A, 7)]).run()[&Register::A]
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_23_part_01_sample() {
        let sample = vec![
            "cpy 2 a", "tgl a", "tgl a", "tgl a", "cpy 1 a", "dec a", "dec a",
        ];

        assert_eq!(3, solve_1(&sample));
    }

    #[test]
    fn day_23_part_01_solution() {
        let input = include_str!("../../inputs/day_23.txt")
            .lines()
            .collect_vec();

        assert_eq!(11_893, solve_1(&input));
    }
}
