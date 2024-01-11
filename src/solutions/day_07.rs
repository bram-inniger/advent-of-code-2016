pub fn solve_1(ips: &[&str]) -> usize {
    ips.iter().filter(|ip| supports_tls(ip)).count()
}

fn supports_tls(ip: &str) -> bool {
    let bytes = ip.as_bytes();
    let mut supports = false;
    let mut inside_hypernet = false;

    for idx in 0..bytes.len() - 3 {
        match bytes[idx] {
            b'[' => inside_hypernet = true,
            b']' => inside_hypernet = false,
            _ => {
                if bytes[idx] == bytes[idx + 3]
                    && bytes[idx + 1] == bytes[idx + 2]
                    && bytes[idx] != bytes[idx + 1]
                {
                    if inside_hypernet {
                        return false;
                    } else {
                        supports = true
                    }
                }
            }
        }
    }

    supports
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_07_part_01_sample() {
        let sample = vec![
            "abba[mnop]qrst",
            "abcd[bddb]xyyx",
            "aaaa[qwer]tyui",
            "ioxxoj[asdfgh]zxcvbn",
        ];

        assert_eq!(2, solve_1(&sample));
    }

    #[test]
    fn day_07_part_01_solution() {
        let input = include_str!("../../inputs/day_07.txt")
            .lines()
            .collect_vec();

        assert_eq!(110, solve_1(&input));
    }
}
