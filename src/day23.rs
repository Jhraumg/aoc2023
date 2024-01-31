use ahash::AHashMap;
use eyre::{eyre, Error};
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::ops::Index;
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
    row_len: usize,
    row_count: usize,
    tiles: Vec<Tile>,
}

#[derive(Debug)]
struct OrientedGraph {
    distances: AHashMap<(usize, usize), Vec<((usize, usize), usize)>>,
}
impl Display for OrientedGraph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for ((from_x, from_y), tos) in &self.distances {
            f.write_fmt(format_args!("({from_x},{from_y}):\n"))?;
            for ((to_x, to_y), d) in tos {
                f.write_fmt(format_args!("  => ({to_x},{to_y}): {d}\n"))?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Alternative {
    src: (usize, usize),
    alts: Vec<((usize, usize), usize)>,
}
impl<const SLIPPERY: bool> FromStr for Map<SLIPPERY> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<Vec<_>> = s
            .lines()
            .map(|l| l.chars().filter_map(|c| Tile::from_char(c).ok()).collect())
            .collect();

        let row_len = tiles[0].len();
        let start = (
            tiles[0].iter().enumerate().find(|(_, t)| **t == Tile::Path).unwrap().0,
            0,
        );
        let endy = tiles.len() - 1;
        let end = (
            tiles[endy].iter().enumerate().find(|(_, t)| **t == Tile::Path).unwrap().0,
            endy,
        );

        let tiles = tiles.into_iter().flat_map(|r| r.into_iter()).collect();
        if SLIPPERY {
            Ok(Self {
                start,
                end,
                tiles,
                row_len,
                row_count: endy + 1,
            })
        } else {
            let tiles =
                tiles.into_iter().map(|t| if Tile::Forest == t { t } else { Tile::Path }).collect();
            Ok(Self {
                start,
                end,
                tiles,
                row_len,
                row_count: endy + 1,
            })
        }
    }
}

impl<const SLIPPERY: bool> Index<(usize, usize)> for Map<SLIPPERY> {
    type Output = Tile;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (col, row) = index;
        &self.tiles[row * self.row_len + col]
    }
}

impl<const SLIPPERY: bool> Map<SLIPPERY> {
    fn follow_path(
        &self,
        path_start: (usize, usize),
        previous: (usize, usize),
    ) -> Option<((usize, usize), usize)> {
        let mut current = Some(path_start);
        let mut previous = previous;
        let mut d = 0;
        while let Some(c) = current {
            if c == self.end {
                break;
            }
            d += 1;
            let next = self
                .raw_step(c)
                .into_iter()
                .filter(|n| *n != previous && self.walkable(*n))
                .collect_vec();
            if next.len() != 1 {
                break;
            }
            previous = c;
            current = next.first().copied();
        }
        current.map(|c| (c, d))
    }

    fn build_oriented_graph(&self) -> OrientedGraph {
        let crosses: Vec<_> = (0..self.row_count)
            .flat_map(|y| (0..self.row_len).map(move |x| (x, y)))
            .filter(|(x, y)| {
                self.raw_step((*x, *y)).into_iter().filter(|p| self.walkable(*p)).count() > 2
            })
            .collect();

        let distances = crosses
            .iter()
            .map(|c| {
                (
                    *c,
                    self.raw_step(*c)
                        .into_iter()
                        .filter_map(|n| self.follow_path(n, *c))
                        .collect_vec(),
                )
            })
            .collect();

        OrientedGraph { distances }
    }

    fn walkable(&self, pos: (usize, usize)) -> bool {
        let (x, y) = pos;
        x < self.row_len && y < self.row_count && self[pos] != Tile::Forest
    }
    fn raw_step(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        if pos == self.start {
            vec![(self.start.0, 1)]
        } else {
            // there's a wall of forest, hence no need to check for limits (except start)
            let (x, y) = pos;
            match self[pos] {
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

    fn longest_path_depth_first(&self) -> usize {
        // println!("self.start {:?}", self.start);
        let (first_step, init_d) = self.follow_path(self.start, self.start).unwrap();
        let graph = self.build_oriented_graph();
        let mut max_path = 0;
        let mut current = first_step;
        let mut alternatives: Vec<Alternative> = Default::default();
        let mut current_len = init_d;

        const EMPTY: Vec<&((usize, usize), usize)> = vec![];
        loop {
            // TODO : buffer in 3 or 4 alternatives, and then process them concurrently though rayon
            if current == self.end && current_len > max_path {
                // println!(
                //     "better path {current_len},still {} alternatives depth",
                //     alternatives.len()
                // );
                max_path = current_len;
            }

            // TODO : mutates visited on backtrack to avoid rebuilding it each time ?
            let visited: Vec<(usize, usize)> = alternatives.iter().map(|alt| alt.src).collect();
            // println!("CURRENT {current:?}, visited {}, alternatives {}", visited.len(), alternatives.len());

            let next = graph
                .distances
                .get(&current)
                .map(|v| v.iter().filter(|(n, _)| !visited.contains(n)).collect_vec())
                .unwrap_or(EMPTY);

            if !next.is_empty() {
                let alt = Alternative {
                    src: current,
                    alts: next[1..].iter().map(|(n, d)| (*n, current_len + d)).collect(),
                };
                current_len += next[0].1;
                current = next[0].0;

                alternatives.push(alt);
            } else {
                // backtrack
                while let Some(cross) = alternatives.last() {
                    if cross.alts.is_empty() {
                        alternatives.pop();
                    } else {
                        break;
                    }
                }
                if let Some(cross) = alternatives.pop() {
                    (current, current_len) = *cross.alts.first().unwrap();
                    // println!("backtracking to {current:?}");

                    alternatives.push(Alternative {
                        alts: cross.alts[1..].to_vec(),
                        ..cross
                    })
                } else {
                    break;
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
        println!("\ntrue :\n{:}", map.build_oriented_graph());
        assert_eq!(94, map.longest_path_depth_first());
        let map: Map<false> = input.parse().unwrap();
        println!("\nfalse :\n{:}", map.build_oriented_graph());
        assert_eq!(154, map.longest_path_depth_first());
    }
}
