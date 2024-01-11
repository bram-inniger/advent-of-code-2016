use itertools::Itertools;

pub fn solve_1(messages: &[&str]) -> String {
    let bytes = messages.iter().map(|s| s.as_bytes()).collect_vec();

    (0..messages[0].len())
        .map(|x| {
            (0..messages.len())
                .map(|y| bytes[y][x])
                .counts()
                .iter()
                .max_by_key(|(_, &count)| count)
                .map(|(c, _)| *c as char)
                .unwrap()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_06_part_01_sample() {
        let sample = vec![
            "eedadn", "drvtee", "eandsr", "raavrd", "atevrs", "tsrnev", "sdttsa", "rasrtv",
            "nssdts", "ntnada", "svetve", "tesnvt", "vntsnd", "vrdear", "dvrsen", "enarar",
        ];

        assert_eq!("easter", solve_1(&sample));
    }

    #[test]
    fn day_06_part_01_solution() {
        let input = include_str!("../../inputs/day_06.txt")
            .lines()
            .collect_vec();

        assert_eq!("gyvwpxaz", solve_1(&input));
    }
}
