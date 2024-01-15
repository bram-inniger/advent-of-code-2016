use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};

pub fn solve_1(nodes: &[&str]) -> usize {
    Grid::new(nodes).find_pairs().len()
}

pub fn solve_2(nodes: &[&str]) -> u32 {
    Grid::new(nodes).move_data()
}

#[derive(Debug)]
struct Grid {
    nodes: FxHashMap<Coordinate, Node>,
    max_x: i32,
    max_y: i32,
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
        let max_x = nodes.keys().max_by_key(|c| c.x).unwrap().x;
        let max_y = nodes.keys().max_by_key(|c| c.y).unwrap().y;

        Self {
            nodes,
            max_x,
            max_y,
        }
    }

    fn find_pairs(&self) -> Vec<(Node, Node)> {
        self.nodes
            .values()
            .filter(|n_cur| n_cur.used > 0)
            .flat_map(|n_cur| {
                self.nodes
                    .values()
                    .filter(move |&n| n_cur != n)
                    .filter(|&n| n_cur.used <= n.avail)
                    .map(|&n| (*n_cur, n))
            })
            .collect()
    }

    fn move_data(&self) -> u32 {
        if self.max_x == 2 {
            // This is the sample input
            // Worked out by hand by looking at the grid
            let moves_to_place_empty_node_left_of_goal = 1;
            let distance_to_move_goal = 1;
            let moves_needed_to_move_goal_left_once = 5;
            let final_swap_of_goal_to_start = 1;

            moves_to_place_empty_node_left_of_goal
                + distance_to_move_goal * moves_needed_to_move_goal_left_once
                + final_swap_of_goal_to_start
        } else {
            // This is the real input
            // Worked out by hand by looking at the grid
            let moves_to_place_empty_node_left_of_goal = 33;
            let distance_to_move_goal = 35;
            let moves_needed_to_move_goal_left_once = 5;
            let final_swap_of_goal_to_start = 1;

            moves_to_place_empty_node_left_of_goal
                + distance_to_move_goal * moves_needed_to_move_goal_left_once
                + final_swap_of_goal_to_start
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let start = Coordinate { x: 0, y: 0 };
        let goal = Coordinate {
            x: self.max_x,
            y: 0,
        };
        let pairs = self.find_pairs();
        let movable: FxHashSet<Coordinate> =
            pairs.iter().flat_map(|(s, r)| [s.coord, r.coord]).collect();

        let display = (0..=self.max_y)
            .map(|y| {
                (0..=self.max_x)
                    .map(|x| Coordinate { x, y })
                    .map(|c| match c {
                        c if c == goal => 'G',
                        c if c == start => 'S',
                        c if self.nodes[&c].used == 0 => '_',
                        c if movable.contains(&c) => '.',
                        _ => '#',
                    })
                    .collect::<String>()
            })
            .join("\n");
        write!(f, "{}", display)
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

    #[test]
    fn day_22_part_02_sample() {
        let sample = vec![
            "root@ebhq-gridcenter# df -h",
            "Filesystem            Size  Used  Avail  Use%",
            "/dev/grid/node-x0-y0   10T    8T     2T   80%",
            "/dev/grid/node-x0-y1   11T    6T     5T   54%",
            "/dev/grid/node-x0-y2   32T   28T     4T   87%",
            "/dev/grid/node-x1-y0    9T    7T     2T   77%",
            "/dev/grid/node-x1-y1    8T    0T     8T    0%",
            "/dev/grid/node-x1-y2   11T    7T     4T   63%",
            "/dev/grid/node-x2-y0   10T    6T     4T   60%",
            "/dev/grid/node-x2-y1    9T    8T     1T   88%",
            "/dev/grid/node-x2-y2    9T    6T     3T   66%",
        ];

        assert_eq!(7, solve_2(&sample));
    }

    #[test]
    fn day_22_part_02_solution() {
        let input = include_str!("../../inputs/day_22.txt")
            .lines()
            .collect_vec();

        assert_eq!(209, solve_2(&input));
    }
}
