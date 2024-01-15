use crate::computer::{Computer, Register};

pub fn solve_1(code: &[&str]) -> i32 {
    Computer::new(code, vec![]).run(false, 0).registers[&Register::A]
}

pub fn solve_2(code: &[&str]) -> i32 {
    Computer::new(code, vec![(Register::C, 1)])
        .run(false, 0)
        .registers[&Register::A]
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_12_part_01_sample() {
        let sample = vec!["cpy 41 a", "inc a", "inc a", "dec a", "jnz a 2", "dec a"];

        assert_eq!(42, solve_1(&sample));
    }

    #[test]
    fn day_12_part_01_solution() {
        let input = include_str!("../../inputs/day_12.txt")
            .lines()
            .collect_vec();

        assert_eq!(318_009, solve_1(&input));
    }

    #[test]
    fn day_12_part_02_sample() {
        // No sample inputs for part 2
    }

    #[test]
    fn day_12_part_02_solution() {
        let input = include_str!("../../inputs/day_12.txt")
            .lines()
            .collect_vec();

        assert_eq!(9_227_663, solve_2(&input));
    }
}
