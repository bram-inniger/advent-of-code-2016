// Formula worked out on paper, found the sequence going:
// 1  1 3  1 3 5 7  1 3 5 7 9 11 13 15  1 ...
// i.e. a row of rows off all odd numbers, increasing in length by power of 2
//
// The solution finds iteratively which group contains the elf, then finds its position inside the group
pub fn solve_1(nr_elves: u32) -> u32 {
    let mut idx = 1;
    let mut seen = 2u32.pow(idx) - 1;

    while seen < nr_elves {
        idx += 1;
        seen = 2u32.pow(idx) - 1;
    }
    idx -= 1;
    seen = 2u32.pow(idx) - 1;

    2 * (nr_elves - seen) - 1
}

// Formula worked out on paper, found the sequence going:
// 1  1 3  1 2 3 5 7 9  1 2 3 4 5 6 7 8 9 11 13 15 17 19 21 23 25 27  1 2 ...
// The pattern is as follows, for every group, iterate until the next power of 3.
// If the index is lower than the previous power of 3, iterate all numbers (1, 2, 3, ..)
// If the index is higher than the previous power of 3, iterate all odd numbers (5, 7, 9, ...)
// The solution is then first finding where in this off sequence we are, and mapping
//
// The solution finds iteratively which group contains the elf, then finds its position inside the group
pub fn solve_2(nr_elves: u32) -> u32 {
    let mut idx = 0;
    let mut seen = 1;

    while seen < nr_elves {
        seen += 2 * 3u32.pow(idx);
        idx += 1;
    }
    idx -= 1;
    seen -= 2 * 3u32.pow(idx);

    let elf = nr_elves - seen;
    let prev_pow = 3u32.pow(idx);
    if elf <= prev_pow {
        elf
    } else {
        2 * elf - prev_pow
    }
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

    #[test]
    fn day_19_part_02_sample() {
        assert_eq!(2, solve_2(5));
    }

    #[test]
    fn day_19_part_02_solution() {
        let input = u32::from_str(include_str!("../../inputs/day_19.txt").trim()).unwrap();

        assert_eq!(1_410_967, solve_2(input));
    }
}
