use std::str::FromStr;

use regex::Regex;
use rustc_hash::FxHashMap;

pub fn solve_1(nodes: &[&str]) -> usize {
    let grid = Grid::new(nodes);
    grid.nodes
        .values()
        .filter(|n_cur| n_cur.used > 0)
        .map(|n_cur| {
            grid.nodes
                .values()
                .filter(|&n| n_cur != n)
                .filter(|&n| n_cur.used <= n.avail)
                .count()
        })
        .sum()
}

#[derive(Debug)]
struct Grid {
    nodes: FxHashMap<Coordinate, Node>,
    _max_x: i32,
    _max_y: i32,
}

impl Grid {
    fn new(nodes: &[&str]) -> Self {
        let re = Regex::new(
            r"^/dev/grid/node-x(?<x>\d+)-y(?<y>\d+) +\d+T +(?<used>\d+)+T +(?<avail>\d+)T +\d+%$",
        )
        .unwrap();
        let nodes: FxHashMap<Coordinate, Node> = nodes
            .iter()
            .skip(2)
            .map(|s| Node::new(s, &re))
            .map(|n| (n.coord, n))
            .collect();
        let _max_x = nodes.keys().max_by_key(|c| c.x).unwrap().x;
        let _max_y = nodes.keys().max_by_key(|c| c.y).unwrap().y;

        Self {
            nodes,
            _max_x,
            _max_y,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Node {
    coord: Coordinate,
    used: u32,
    avail: u32,
}

impl Node {
    fn new(node: &str, re: &Regex) -> Self {
        let caps = re.captures(node).unwrap();

        let x = i32::from_str(caps.name("x").unwrap().as_str()).unwrap();
        let y = i32::from_str(caps.name("y").unwrap().as_str()).unwrap();
        let used = u32::from_str(caps.name("used").unwrap().as_str()).unwrap();
        let avail = u32::from_str(caps.name("avail").unwrap().as_str()).unwrap();

        Self {
            coord: Coordinate { x, y },
            used,
            avail,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_22_part_01_sample() {
        // No sample inputs for part 1
    }

    #[test]
    fn day_22_part_01_solution() {
        let input = include_str!("../../inputs/day_22.txt")
            .lines()
            .collect_vec();

        assert_eq!(976, solve_1(&input));
    }
}
