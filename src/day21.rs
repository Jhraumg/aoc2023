use itertools::Itertools;
use num::Integer;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug)]
struct Garden {
    rocks: HashSet<(usize, usize)>,
    maxx: usize,
    maxy: usize,
    start: (usize, usize),
}
impl FromStr for Garden {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rocks: HashSet<_> = s
            .lines()
            .filter(|l| !l.is_empty())
            .enumerate()
            .flat_map(|(j, l)| {
                l.trim()
                    .chars()
                    .enumerate()
                    .filter_map(move |(i, c)| if c == '#' { Some((i, j)) } else { None })
            })
            .collect();
        let start = s
            .lines()
            .filter(|l| !l.is_empty())
            .enumerate()
            .flat_map(|(j, l)| {
                l.trim()
                    .chars()
                    .enumerate()
                    .filter_map(move |(i, c)| if c == 'S' { Some((i, j)) } else { None })
            })
            .next()
            .unwrap();
        let maxx = s.lines().filter(|l| !l.is_empty()).map(|l| l.trim().len()).max().unwrap();
        let maxy = s.lines().filter(|l| !l.is_empty()).count();
        Ok(Self {
            rocks,
            maxx,
            maxy,
            start,
        })
    }
}

impl Garden {
    fn step(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = pos;
        let west = if x > 0 { Some((x - 1, y)) } else { None };
        let north = if y > 0 { Some((x, y - 1)) } else { None };
        let east = if x + 1 < self.maxx {
            Some((x + 1, y))
        } else {
            None
        };
        let south = if y + 1 < self.maxy {
            Some((x, y + 1))
        } else {
            None
        };
        [north, west, south, east]
            .into_iter()
            .flatten()
            .filter(|(x, y)| !self.rocks.contains(&(*x, *y)))
            .collect()
    }
    fn naive_pos_after_n_steps(&self, n: usize) -> usize {
        let mut cur_pos: Vec<(usize, usize)> = vec![self.start];
        for _ in 0..n {
            cur_pos = cur_pos
                .into_iter()
                .flat_map(|p| self.step(p).into_iter())
                .filter(|(x, y)| !self.rocks.contains(&(*x, *y)))
                .unique()
                .collect();
        }
        cur_pos.len()
    }
    fn modulo_step(
        &self,
        pos: (usize, usize),
        garden_pos: (isize, isize),
    ) -> [((usize, usize), (isize, isize)); 4] {
        let (x, y) = pos;
        let (gx, gy) = garden_pos;
        let west = if x == 0 {
            ((self.maxx - 1, y), (gx - 1, gy))
        } else {
            ((x - 1, y), (gx, gy))
        };
        let north = if y == 0 {
            ((x, self.maxy - 1), (gx, gy - 1))
        } else {
            ((x, y - 1), (gx, gy))
        };
        let east = if x != self.maxx - 1 {
            ((x + 1, y), (gx, gy))
        } else {
            ((0, y), (gx + 1, gy))
        };
        let south = if y != self.maxy - 1 {
            ((x, y + 1), (gx, gy))
        } else {
            ((x, 0), (gx, gy + 1))
        };
        [north, west, south, east]
    }
    fn infinite_pos_after_n_steps(&self, n: usize) -> usize {
        self.inner_infinite_pos_after_n_steps(self.start, 1, n)
    }
    fn inner_infinite_pos_after_n_steps(
        &self,
        start: (usize, usize),
        start_idx: usize,
        n: usize,
    ) -> usize {
        let mut first_on: Vec<Vec<usize>> = vec![vec![0; self.maxx]; self.maxy];
        let mut cur_pos: Vec<((usize, usize), (isize, isize))> = vec![(start, (0, 0))];

        for i in start_idx..self.maxx * self.maxy {
            let mut changed = false;
            cur_pos = cur_pos
                .into_iter()
                .flat_map(|(p, g)| self.modulo_step(p, g).into_iter())
                .filter(|((x, y), _)| !self.rocks.contains(&(*x, *y)))
                .unique()
                .collect();
            for ((x, y), (gx, gy)) in &cur_pos {
                if *gy == 0 && *gx == 0 {
                    if first_on[*y][*x] == 0 {
                        first_on[*y][*x] = i;
                        changed = true;
                    } else {
                        assert_eq!(first_on[*y][*x] % 2, i % 2);
                    }
                }
            }
            if !changed {
                // println!("mapped all garden at {i}");
                //
                for l in &first_on {
                    println!("{}", l.iter().map(|on| format!("{on:3}")).join(" "));
                }

                break;
            }
        }
        let north = first_on[0]
            .iter()
            .enumerate()
            .map(|(x, on)| (x, 0, *on))
            .min_by(|(_, _, v1), (_, _, v2)| v1.cmp(v2))
            .unwrap();
        let south = first_on[self.maxy - 1]
            .iter()
            .enumerate()
            .map(|(x, on)| (x, self.maxy - 1, *on))
            .min_by(|(_, _, v1), (_, _, v2)| v1.cmp(v2))
            .unwrap();
        let west = first_on
            .iter()
            .enumerate()
            .map(|(y, l)| (0, y, l[0]))
            .min_by(|(_, _, v1), (_, _, v2)| v1.cmp(v2))
            .unwrap();
        let east = first_on
            .iter()
            .enumerate()
            .map(|(y, l)| (self.maxx - 1, y, l[self.maxx - 1]))
            .min_by(|(_, _, v1), (_, _, v2)| v1.cmp(v2))
            .unwrap();

        println!("north {north:?}, west {west:?}, south {south:?}, east {east:?}");

        if start_idx == 1 {
            for (x, y, v) in &[north, west, south, east] {
                dbg!((x, y, v));
                self.inner_infinite_pos_after_n_steps(
                    (
                        if *x == 0 { self.maxx - 1 } else { *x },
                        if *y == 0 { self.maxy - 1 } else { *y },
                    ),
                    dbg!(2 + *v),
                    n,
                );
            }
        }

        0
    }
}

pub fn walk_exercise() {
    let garden: Garden = include_str!("../resources/day21_garden.txt").parse().unwrap();
    let max_pos = garden.naive_pos_after_n_steps(64);
    println!("pos after 64 steps : {max_pos}");
    let max_pos = garden.infinite_pos_after_n_steps(26501365);
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    #[test]
    fn aoc_example_works() {
        let garden: Garden = indoc! {"
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........
        "}
        .parse()
        .unwrap();
        assert_eq!(16, garden.naive_pos_after_n_steps(6));
        assert_eq!(16, garden.infinite_pos_after_n_steps(6));
        assert_eq!(50, garden.infinite_pos_after_n_steps(10));
        assert_eq!(167004, garden.infinite_pos_after_n_steps(500));
        assert_eq!(668697, garden.infinite_pos_after_n_steps(1000));
        assert_eq!(16733044, garden.infinite_pos_after_n_steps(5000));
    }
}
// 7154 too high
