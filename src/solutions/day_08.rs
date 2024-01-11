use std::str::FromStr;

use itertools::Itertools;

pub fn solve_1(instructions: &[&str], width: usize, height: usize) -> usize {
    display(instructions, width, height).voltage()
}

pub fn solve_2(instructions: &[&str], width: usize, height: usize) -> String {
    display(instructions, width, height)
        .pixels
        .iter()
        .map(|row| {
            row.iter()
                .map(|&p| match p {
                    true => '#',
                    false => '.',
                })
                .collect::<String>()
        })
        .join("\n")
}

fn display(instructions: &[&str], width: usize, height: usize) -> Screen {
    let instructions = instructions
        .iter()
        .map(|s| Instruction::new(s))
        .collect_vec();

    Screen::new(width, height).display(&instructions)
}

#[derive(Debug)]
struct Screen {
    pixels: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![vec![false; width]; height],
            width,
            height,
        }
    }

    fn display(&self, instructions: &[Instruction]) -> Self {
        let mut pixels = self.pixels.clone();
        let width = self.width;
        let height = self.height;

        #[allow(clippy::needless_range_loop)]
        for i in instructions {
            match i {
                Instruction::Rect { width, height } => {
                    for x in 0..*width {
                        for y in 0..*height {
                            pixels[y][x] = true
                        }
                    }
                }
                Instruction::RotateRow { row, shift } => {
                    let mut new_pixels = pixels.clone();
                    for x in 0..width {
                        new_pixels[*row][(x + shift) % width] = pixels[*row][x];
                    }
                    pixels = new_pixels;
                }
                Instruction::RotateCol { col, shift } => {
                    let mut new_pixels = pixels.clone();
                    for y in 0..height {
                        new_pixels[(y + shift) % height][*col] = pixels[y][*col];
                    }
                    pixels = new_pixels;
                }
            }
        }

        Self {
            pixels,
            width,
            height,
        }
    }

    fn voltage(&self) -> usize {
        self.pixels
            .iter()
            .map(|row| row.iter().filter(|&&p| p).count())
            .sum()
    }
}

#[derive(Debug)]
enum Instruction {
    Rect { width: usize, height: usize },
    RotateRow { row: usize, shift: usize },
    RotateCol { col: usize, shift: usize },
}

impl Instruction {
    fn new(instruction: &str) -> Self {
        match instruction {
            i if instruction.starts_with("rect ") => {
                let split = &i[5..]
                    .split('x')
                    .map(|s| usize::from_str(s).unwrap())
                    .collect_vec();

                Instruction::Rect {
                    width: split[0],
                    height: split[1],
                }
            }
            i if i.starts_with("rotate row y=") => {
                let split = &i[13..]
                    .split(" by ")
                    .map(|s| usize::from_str(s).unwrap())
                    .collect_vec();

                Instruction::RotateRow {
                    row: split[0],
                    shift: split[1],
                }
            }
            i if i.starts_with("rotate column x=") => {
                let split = &i[16..]
                    .split(" by ")
                    .map(|s| usize::from_str(s).unwrap())
                    .collect_vec();

                Instruction::RotateCol {
                    col: split[0],
                    shift: split[1],
                }
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_08_part_01_sample() {
        let sample = vec![
            "rect 3x2",
            "rotate column x=1 by 1",
            "rotate row y=0 by 4",
            "rotate column x=1 by 1",
        ];

        assert_eq!(6, solve_1(&sample, 7, 3));
    }

    #[test]
    fn day_08_part_01_solution() {
        let input = include_str!("../../inputs/day_08.txt")
            .lines()
            .collect_vec();

        assert_eq!(110, solve_1(&input, 50, 6));
    }

    #[test]
    fn day_08_part_02_sample() {
        // No sample inputs for part 2
    }

    #[test]
    fn day_08_part_02_solution() {
        let input = include_str!("../../inputs/day_08.txt")
            .lines()
            .collect_vec();
        let expected = "\
            ####...##.#..#.###..#..#..##..###..#....#...#..##.\n\
            ...#....#.#..#.#..#.#.#..#..#.#..#.#....#...#...#.\n\
            ..#.....#.####.#..#.##...#....#..#.#.....#.#....#.\n\
            .#......#.#..#.###..#.#..#....###..#......#.....#.\n\
            #....#..#.#..#.#.#..#.#..#..#.#....#......#..#..#.\n\
            ####..##..#..#.#..#.#..#..##..#....####...#...##..\
            ";

        assert_eq!(expected, solve_2(&input, 50, 6));
    }
}
