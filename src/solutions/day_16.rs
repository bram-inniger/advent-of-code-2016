use itertools::Itertools;

pub fn solve(data: &str, disk_size: usize) -> String {
    DragonCurve::new(data).enlarge(disk_size).checksum()
}

#[derive(Debug)]
struct DragonCurve {
    data: String,
}

impl DragonCurve {
    fn new(data: &str) -> Self {
        Self {
            data: data.to_string(),
        }
    }

    fn enlarge(&self, size: usize) -> Self {
        let mut a = self.data.clone();

        while a.len() < size {
            let b: String = a
                .chars()
                .rev()
                .map(|c| match c {
                    '0' => '1',
                    '1' => '0',
                    _ => unreachable!(),
                })
                .collect();

            a = format!("{}0{}", a, b);
        }

        Self {
            data: a.chars().take(size).collect(),
        }
    }

    fn checksum(&self) -> String {
        let mut checksum = self.data.clone();

        while checksum.len() % 2 == 0 {
            checksum = checksum
                .chars()
                .chunks(2)
                .into_iter()
                .map(|c| {
                    let pair: String = c.collect();
                    match pair.as_str() {
                        "00" | "11" => '1',
                        "01" | "10" => '0',
                        _ => unreachable!(),
                    }
                })
                .collect()
        }

        checksum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_16_part_01_sample() {
        assert_eq!("100", solve("110010110100", 12));
        assert_eq!("01100", solve("10000", 20));
    }

    #[test]
    fn day_16_part_01_solution() {
        let input = include_str!("../../inputs/day_16.txt").trim();

        assert_eq!("10101001010100001", solve(input, 272));
    }

    #[test]
    fn day_16_part_02_sample() {
        // No sample inputs for part 2
    }

    #[ignore = "slow brute force test"]
    #[test]
    fn day_16_part_02_solution() {
        let input = include_str!("../../inputs/day_16.txt").trim();

        assert_eq!("10100001110101001", solve(input, 35_651_584));
    }
}
