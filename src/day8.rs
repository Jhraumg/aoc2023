use num::integer::lcm;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Node {
    name: &'static str,
    left: &'static str,
    right: &'static str,
}

// TODO : nodes could be sorted => nodes id, left and right could be stored as usize
// starts and end indexes would be indexes too
struct Map {
    directions: Vec<Direction>,
    nodes: HashMap<&'static str, Node>,
}
impl Map {
    pub fn new(input: &'static str) -> Self {
        let mut lines = input.lines();
        let directions: Vec<Direction> = lines
            .next()
            .unwrap()
            .chars()
            .filter_map(|c| match c {
                'L' => Some(Direction::Left),
                'R' => Some(Direction::Right),
                _ => None,
            })
            .collect();

        let nodes: HashMap<&'static str, Node> = lines
            .filter(|l| !l.is_empty())
            .map(to_node)
            .map(|n| (n.name, n))
            .collect();

        Self { directions, nodes }
    }
}

fn to_node(s: &'static str) -> Node {
    let (name, nexts) = s.split_once('=').unwrap();
    let name = name.trim();
    let (left, right) = nexts.split_once(',').unwrap();
    let (_, left) = left.trim().split_once('(').unwrap();
    let (right, _) = right.trim().split_once(')').unwrap();

    Node { name, left, right }
}

fn path_len(input: &'static str) -> u64 {
    let Map { directions, nodes } = Map::new(input);
    let dir_len = directions.len();

    let mut node = nodes.get("AAA").unwrap();
    let mut len: u64 = 0;
    while node.name != "ZZZ" {
        node = match directions[len as usize % dir_len] {
            Direction::Left => nodes.get(&node.left).unwrap(),
            Direction::Right => nodes.get(&node.right).unwrap(),
        };
        len += 1;
    }
    len
}

fn ghost_path_len(input: &'static str) -> u64 {
    let Map { directions, nodes } = Map::new(input);
    let dir_len = directions.len();

    let start_nodes = nodes
        .iter()
        .filter_map(|(_, n)| if n.name.ends_with('A') { Some(n) } else { None });

    let revolving_paths: Vec<(Vec<&'static str>, usize)> = start_nodes
        .map(|n| {
            let mut current_node = n;
            let mut steps_by_node: HashMap<&'static str, Vec<usize>> = HashMap::new();
            let mut steps = vec![];
            for len in 0usize.. {
                let dir = directions[len % dir_len];
                current_node = match dir {
                    Direction::Left => nodes.get(&current_node.left).unwrap(),
                    Direction::Right => nodes.get(&current_node.right).unwrap(),
                };
                steps.push(current_node.name);

                let lens = steps_by_node.entry(current_node.name).or_default();
                if let Some(l) = lens.iter().find(|l| *l % dir_len == len % dir_len) {
                    // println!("period found {} at {l} and {len} (mod {dir_len})",current_node.name);
                    return (steps, len - l);
                }
                lens.push(len);
            }
            unreachable!()
        })
        .collect();

    #[derive(Debug, Clone)]
    struct Path {
        end_indexes: Vec<usize>,
        period: usize,
    }

    let paths: Vec<Path> = revolving_paths
        .into_iter()
        .map(|(steps, period)| {
            let end_indexes = steps
                .into_iter()
                .enumerate()
                .filter_map(|(i, n)| if n.ends_with('Z') { Some(i) } else { None })
                .collect();
            Path {
                period,
                end_indexes,
            }
        })
        .collect();

    // for each path, let's get the next point where there is an end match with the previous path
    // from there, next match must be on the LCM(paths until current  periods), so we move to next path
    // some path may have several end in their periods, so we must follow the road for each one of them
    // we can stop anyway once we have gone farther than LCM(current  period, current path period) : there won't be a match ever, since we're on a periodic path, now
    paths
        .iter()
        .fold(vec![(0, 1)], |acc, p| {
            acc.into_iter()
                .flat_map(|(current, period)| {
                    p.end_indexes.iter().filter_map(move |idx| {
                        let next_period = lcm(period, p.period);
                        let mut next_match = current;

                        while next_match % p.period != idx % p.period {
                            if next_match - current > idx + next_period {
                                // there won't be any matching end on this path !
                                return None;
                            }
                            next_match += period;
                        }
                        Some((next_match, next_period))
                    })
                })
                .collect()
        }) // Then, we can chose the shortest remaining one, and add the initial step
        .into_iter()
        .map(|(len, _)| len + 1)
        .min()
        .unwrap() as u64
}

pub fn cross_desert() {
    let input = include_str!("../resources/day8_maps.txt");
    let len = path_len(input);
    println!("path len {len}");
    let ghost_len = ghost_path_len(input);

    println!("ghost len {ghost_len}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn aoc_example_works() {
        let input = indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
            "
        };
        assert_eq!(6, path_len(input));
        let input = indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
            "
        };
        assert_eq!(6, ghost_path_len(input));
    }
}
