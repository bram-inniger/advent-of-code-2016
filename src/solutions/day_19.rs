pub fn solve_1(nr_elves: u32) -> u32 {
    let mut idx = 1;
    let mut pow = 2u32.pow(idx) - 1;

    while pow < nr_elves {
        idx += 1;
        pow = 2u32.pow(idx) - 1;
    }

    // Formula worked out on paper, found the sequence going:
    // 1  1 3  1 3 5 7  1 3 5 7 9 11 13 15  1 ...
    // i.e. a row of rows off all odd numbers, increasing in length by power of 2
    // The solution is then first finding where in this off sequence we are, and mapping
    2 * (nr_elves - (2u32.pow(idx - 1) - 1)) - 1
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn day_19_part_01_sample() {
        assert_eq!(3, solve_1(5));
    }

    #[test]
    fn day_19_part_01_solution() {
        let input = u32::from_str(include_str!("../../inputs/day_19.txt").trim()).unwrap();

        assert_eq!(1_816_277, solve_1(input));
    }
}
