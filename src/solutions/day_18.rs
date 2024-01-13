pub fn solve(row: &str, nr_rows: u32) -> usize {
    let mut row = Row::new(row);
    let mut rows = vec![row.clone()];

    for _ in 1..nr_rows {
        row = row.next();
        rows.push(row.clone());
    }

    rows.into_iter()
        .map(|r| r.tiles.into_iter().filter(|&t| t == Tile::Safe).count())
        .sum()
}

#[derive(Debug, Clone)]
struct Row {
    tiles: Vec<Tile>,
}

impl Row {
    fn new(row: &str) -> Self {
        Self {
            tiles: row.chars().map(Tile::new).collect(),
        }
    }

    fn next(&self) -> Self {
        let tiles = (0..self.tiles.len())
            .map(|idx| {
                let left = if idx > 0 {
                    self.tiles[idx - 1]
                } else {
                    Tile::Safe
                };
                let center = self.tiles[idx];
                let right = if idx < self.tiles.len() - 1 {
                    self.tiles[idx + 1]
                } else {
                    Tile::Safe
                };

                if (left == Tile::Trap && right == Tile::Safe)
                    || (left == Tile::Safe && center == Tile::Trap && right == Tile::Trap)
                    || (left == Tile::Safe && center == Tile::Safe && right == Tile::Trap)
                {
                    Tile::Trap
                } else {
                    Tile::Safe
                }
            })
            .collect();
        Self { tiles }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Tile {
    Safe,
    Trap,
}

impl Tile {
    fn new(tile: char) -> Self {
        match tile {
            '.' => Tile::Safe,
            '^' => Tile::Trap,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_18_part_01_sample() {
        assert_eq!(6, solve("..^^.", 3));
        assert_eq!(38, solve(".^^.^.^^^^", 10));
    }

    #[test]
    fn day_18_part_01_solution() {
        let input = include_str!("../../inputs/day_18.txt").trim();

        assert_eq!(1_951, solve(input, 40));
    }

    #[test]
    fn day_18_part_02_sample() {
        // No sample inputs for part 2
    }

    #[test]
    fn day_18_part_02_solution() {
        let input = include_str!("../../inputs/day_18.txt").trim();

        assert_eq!(20_002_936, solve(input, 400_000));
    }
}
