use crate::computer::{Computer, Register};

pub fn solve_1(code: &[&str]) -> i32 {
    Computer::new(code, vec![(Register::A, 7)])
        .run(false, 0)
        .registers[&Register::A]
}

pub fn solve_2(code: &[&str]) -> i32 {
    Computer::new(code, vec![(Register::A, 12)])
        .run(true, 0)
        .registers[&Register::A]
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

    #[test]
    fn day_23_part_02_sample() {
        // No sample inputs for part 2
    }

    #[test]
    fn day_23_part_02_solution() {
        let input = include_str!("../../inputs/day_23.txt")
            .lines()
            .collect_vec();

        assert_eq!(479_008_453, solve_2(&input));
    }
}
