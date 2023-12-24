use eyre::{eyre, Error};
use std::cmp::max;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
impl<const SLIPPERY: bool> FromStr for Map<SLIPPERY> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<Vec<_>> = s
            .lines()
            .map(|l| l.chars().filter_map(|c| Tile::from_char(c).ok()).collect())
            .collect();
        let start = (
            tiles[0].iter().enumerate().filter(|(_, t)| **t == Tile::Path).next().unwrap().0,
            0,
        );
        let endy = tiles.len() - 1;
        let end = (
            tiles[endy].iter().enumerate().filter(|(_, t)| **t == Tile::Path).next().unwrap().0,
            endy,
        );
        println!("Map {start:?}=>{end:?}");
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
    fn longest_path(self) -> usize {
        let mut paths: Vec<(HashSet<(usize, usize)>, (usize, usize))> =
            vec![([self.start].into_iter().collect(), self.start)];
        let mut max_path: usize = 0;

        loop {
            let mut new_paths: Vec<_> = Vec::with_capacity(paths.len());
            for (path, pos) in paths.iter() {
                for p in self
                    .raw_step(*pos)
                    .into_iter()
                    .filter(|p| self.walkable(*p) && !path.contains(p))
                {
                    if p == self.end {
                        max_path = max(max_path, path.len());
                    } else {
                        let mut new_path = path.clone();
                        new_path.insert(p);
                        new_paths.push((new_path, p));
                    }
                }
            }

            if new_paths == paths {
                break;
            }
            paths = new_paths;
        }

        max_path
    }
}
pub fn hike_garden() {
    let garden: Map<true> = include_str!("../resources/day23_garden.txt").parse().unwrap();
    let longest_path = garden.longest_path();
    println!("longuest path (slippery) : {longest_path}");
    let garden: Map<false> = include_str!("../resources/day23_garden.txt").parse().unwrap();
    let longest_path = garden.longest_path();
    println!("longuest path : {longest_path}");
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
        assert_eq!(94, map.longest_path());
        let map: Map<false> = input.parse().unwrap();
        assert_eq!(154, map.longest_path());
    }
}
