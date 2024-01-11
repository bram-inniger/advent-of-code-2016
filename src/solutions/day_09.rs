use crate::util::BASE_10;

pub fn solve_1(data: &str) -> u128 {
    decompress(data, false)
}

pub fn solve_2(data: &str) -> u128 {
    decompress(data, true)
}

fn decompress(data: &str, rec: bool) -> u128 {
    let bytes = data.as_bytes();
    let mut length = 0;
    let mut idx = 0;
    let mut reading_marker = false;
    let mut reading_first = true;
    let mut nr_to_repeat = 0;
    let mut repeats = 0;

    while idx < bytes.len() {
        match bytes[idx] {
            b'(' => {
                reading_marker = true;
                idx += 1;
            }
            b')' => {
                let to_add = repeats as u128
                    * if rec {
                        decompress(&data[idx + 1..idx + 1 + nr_to_repeat], true)
                    } else {
                        nr_to_repeat as u128
                    };

                length += to_add;
                idx += nr_to_repeat + 1;

                reading_marker = false;
                reading_first = true;
                nr_to_repeat = 0;
                repeats = 0;
            }
            _ => {
                if reading_marker {
                    if bytes[idx] == b'x' {
                        reading_first = false;
                    } else {
                        let digit = (bytes[idx] as char).to_digit(BASE_10).unwrap() as usize;

                        match reading_first {
                            true => nr_to_repeat = nr_to_repeat * 10 + digit,
                            false => repeats = repeats * 10 + digit,
                        }
                    }
                } else {
                    length += 1;
                }

                idx += 1;
            }
        }
    }

    length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_09_part_01_sample() {
        assert_eq!(6, solve_1("ADVENT"));
        assert_eq!(7, solve_1("A(1x5)BC"));
        assert_eq!(9, solve_1("(3x3)XYZ"));
        assert_eq!(11, solve_1("A(2x2)BCD(2x2)EFG"));
        assert_eq!(6, solve_1("(6x1)(1x3)A"));
        assert_eq!(18, solve_1("X(8x2)(3x3)ABCY"));
    }

    #[test]
    fn day_09_part_01_solution() {
        let input = include_str!("../../inputs/day_09.txt").trim();

        assert_eq!(70_186, solve_1(input));
    }

    #[test]
    fn day_09_part_02_sample() {
        assert_eq!(9, solve_2("(3x3)XYZ"));
        assert_eq!(20, solve_2("X(8x2)(3x3)ABCY"));
        assert_eq!(241_920, solve_2("(27x12)(20x12)(13x14)(7x10)(1x12)A"));
        assert_eq!(
            445,
            solve_2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")
        );
    }

    #[test]
    fn day_09_part_02_solution() {
        let input = include_str!("../../inputs/day_09.txt").trim();

        assert_eq!(10_915_059_201, solve_2(input));
    }
}
