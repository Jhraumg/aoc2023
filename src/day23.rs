use eyre::{eyre, Error};
use fxhash::FxHashSet;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Dir {
    N,
    W,
    S,
    E,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    Slide(Dir),
}

impl Tile {
    fn from_char(c: char) -> Result<Self, Error> {
        match c {
            '.' => Ok(Self::Path),
            '#' => Ok(Self::Forest),
            '<' => Ok(Self::Slide(Dir::W)),
            '>' => Ok(Self::Slide(Dir::E)),
            '^' => Ok(Self::Slide(Dir::N)),
            'v' => Ok(Self::Slide(Dir::S)),
            c => Err(eyre!("unknown Tile '{c}'")),
        }
    }
}

struct Map<const SLIPPERY: bool> {
    start: (usize, usize),
    end: (usize, usize),
    tiles: Vec<Vec<Tile>>,
}
#[derive(Debug, Clone)]
struct Alternative {
    src: (usize, usize),
    alts: Vec<(usize, usize)>,
}
impl<const SLIPPERY: bool> FromStr for Map<SLIPPERY> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<Vec<_>> = s
            .lines()
            .map(|l| l.chars().filter_map(|c| Tile::from_char(c).ok()).collect())
            .collect();
        let start = (
            tiles[0].iter().enumerate().find(|(_, t)| **t == Tile::Path).unwrap().0,
            0,
        );
        let endy = tiles.len() - 1;
        let end = (
            tiles[endy].iter().enumerate().find(|(_, t)| **t == Tile::Path).unwrap().0,
            endy,
        );
        if SLIPPERY {
            Ok(Self { start, end, tiles })
        } else {
            let tiles = tiles
                .into_iter()
                .map(|l| {
                    l.into_iter().map(|t| if Tile::Forest == t { t } else { Tile::Path }).collect()
                })
                .collect();
            Ok(Self { start, end, tiles })
        }
    }
}

impl<const SLIPPERY: bool> Map<SLIPPERY> {
    fn walkable(&self, pos: (usize, usize)) -> bool {
        let (x, y) = pos;
        x < self.tiles[0].len() && y < self.tiles.len() && self.tiles[y][x] != Tile::Forest
    }
    fn raw_step(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        if pos == self.start {
            vec![(self.start.0, 1)]
        } else {
            // there's a wall of forest, hence no need to check for limits (except start)
            let (x, y) = pos;
            match self.tiles[y][x] {
                Tile::Path => {
                    vec![(x - 1, y), (x, y + 1), (x + 1, y), (x, y - 1)]
                }
                Tile::Forest => {
                    vec![]
                }
                Tile::Slide(d) => match d {
                    Dir::N => {
                        vec![(x, y - 1)]
                    }
                    Dir::W => {
                        vec![(x - 1, y)]
                    }
                    Dir::S => {
                        vec![(x, y + 1)]
                    }
                    Dir::E => {
                        vec![(x + 1, y)]
                    }
                },
            }
        }
    }

    // FIXME : store only cross + distance between cross

    fn longest_path_depth_first(&self) -> usize {
        let mut max_path = 0;
        let mut current = self.start;
        let mut previous = self.start;
        let mut alternatives_and_len: Vec<(Alternative, usize)> = Default::default();
        let mut current_len = 0;
        loop {
            if current == self.end && current_len > max_path {
                println!(
                    "better path {current_len},still {} alternatives depth",
                    alternatives_and_len.len()
                );
                max_path = current_len;
            }
            let crosses: FxHashSet<(usize, usize)> =
                alternatives_and_len.iter().map(|(alt, _)| alt.src).collect();

            let next = self
                .raw_step(current)
                .into_iter()
                .filter(|p| self.walkable(*p) && *p != previous && !crosses.contains(p))
                .collect_vec();
            match next.len() {
                0 => {
                    while let Some((cross, _)) = alternatives_and_len.last() {
                        if cross.alts.is_empty() {
                            alternatives_and_len.pop();
                        } else {
                            break;
                        }
                    }
                    // backtrack
                    if let Some((cross, l)) = alternatives_and_len.pop() {
                        previous = cross.src;
                        current = *cross.alts.first().unwrap();
                        current_len = l;
                        alternatives_and_len.push((
                            Alternative {
                                alts: cross.alts[1..].to_vec(),
                                ..cross
                            },
                            l,
                        ))
                    } else {
                        break;
                    }
                }
                1 => {
                    // regular path
                    previous = current;
                    current_len += 1;
                    current = next[0];
                }

                _ => {
                    // branching (cross)
                    let alt = Alternative {
                        src: current,
                        alts: next[1..].to_vec(),
                    };
                    previous = current;
                    current_len += 1;
                    current = next[0];

                    alternatives_and_len.push((alt, current_len));
                }
            }
        }

        max_path
    }
}
pub fn hike_garden() {
    let garden: Map<true> = include_str!("../resources/day23_garden.txt").parse().unwrap();
    let longest_path = garden.longest_path_depth_first();
    println!("longest path (slippery) : {longest_path}");
    let garden: Map<false> = include_str!("../resources/day23_garden.txt").parse().unwrap();
    let longest_path = garden.longest_path_depth_first();
    println!("longest path : {longest_path}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn aoc_examples_works() {
        let input = indoc! {"
            #.#####################
            #.......#########...###
            #######.#########.#.###
            ###.....#.>.>.###.#.###
            ###v#####.#v#.###.#.###
            ###.>...#.#.#.....#...#
            ###v###.#.#.#########.#
            ###...#.#.#.......#...#
            #####.#.#.#######.#.###
            #.....#.#.#.......#...#
            #.#####.#.#.#########v#
            #.#...#...#...###...>.#
            #.#.#v#######v###.###v#
            #...#.>.#...>.>.#.###.#
            #####v#.#.###v#.#.###.#
            #.....#...#...#.#.#...#
            #.#########.###.#.#.###
            #...###...#...#...#.###
            ###.###.#.###v#####v###
            #...#...#.#.>.>.#.>.###
            #.###.###.#.###.#.#v###
            #.....###...###...#...#
            #####################.#
        "};
        let map: Map<true> = input.parse().unwrap();
        assert_eq!(94, map.longest_path_depth_first());
        let map: Map<false> = input.parse().unwrap();
        assert_eq!(154, map.longest_path_depth_first());
    }
}
