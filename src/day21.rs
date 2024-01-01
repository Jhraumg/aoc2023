use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use num::Integer;
use rayon::prelude::*;
use std::cmp::max;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Garden {
    rocks: FxHashSet<(isize, isize)>,
    min_p: isize,
    max_p: isize,
    period: isize,
}
impl FromStr for Garden {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rocks: Vec<_> = s
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
        let (start_x, start_y) = s
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
        assert_eq!(start_y, start_x);
        let maxx = s.lines().filter(|l| !l.is_empty()).map(|l| l.trim().len()).max().unwrap();
        let maxy = s.lines().filter(|l| !l.is_empty()).count();
        assert_eq!(maxx, maxy);
        let min_p = 0 - start_x as isize;
        let max_p = maxx as isize - start_x as isize;
        let period = max_p - min_p;
        let rocks = rocks
            .into_iter()
            .map(|(i, j)| (i as isize - start_x as isize, j as isize - start_y as isize))
            .collect();
        Ok(Self {
            rocks,
            min_p,
            max_p,
            period,
        })
    }
}

impl Garden {
    fn step(&self, pos: (isize, isize)) -> Vec<(isize, isize)> {
        let (x, y) = pos;
        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .into_iter()
            // .filter(|(x, y)| *x >= self.min_p && *x < self.max_p && *y >= self.min_p && *y <self.max_p && !self.rocks.contains(&(*x, *y)))
            .filter(|p| !self.is_rock(*p))
            .collect()
    }
    fn is_rock(&self, p: (isize, isize)) -> bool {
        let (x, y) = p;
        let x = self.min_p + (self.period + (x - self.min_p) % self.period) % self.period;
        let y = self.min_p + (self.period + (y - self.min_p) % self.period) % self.period;
        // assert!(x>=self.min_p && x<self.max_p,"{p:?} => {x} [{},{}]",self.min_p,self.max_p);
        // assert!(y>=self.min_p && y<self.max_p, "{p:?} => {y} [{},{}]",self.min_p,self.max_p);

        self.rocks.contains(&(x, y))
    }
    fn naive_pos_after_n_steps(&self, n: usize) -> usize {
        let mut rank_reached: HashMap<(isize, isize), usize> = HashMap::new();
        let mut cur_pos: Vec<(isize, isize)> = vec![(0, 0)];
        for k in 0..n {
            cur_pos = cur_pos.into_iter().flat_map(|p| self.step(p).into_iter()).unique().collect();
            for (x, y) in &cur_pos {
                rank_reached.entry((*x, *y)).or_insert(k + 1);
            }
        }


        #[cfg(debug_assertions)]
        {
            let n = n as isize;
            for y in -n..=n {
                if (y - self.max_p) % self.period == 0 {
                    println!();
                }
                for x in -n..=n {
                    if (x - self.max_p) % self.period == 0 {
                        print!("  ");
                    }
                    if self.is_rock((x, y)) {
                        print!("###");
                    } else if rank_reached.contains_key(&(x, y)) {
                        print!("{:3}", rank_reached.get(&(x, y)).unwrap());
                    } else {
                        print!("...");
                    }
                }
                println!()
            }
        }
        cur_pos.len()
    }

    fn get_time_to_reach_base(&self) -> FxHashMap<(isize, isize), isize> {
        let period = self.period;
        let n = 3 * period;

        let mut rank_reached: FxHashMap<(isize, isize), isize> = Default::default();
        let mut cur_pos: Vec<(isize, isize)> = vec![(0, 0)];
        for k in 0..n {
            cur_pos = cur_pos.into_iter().flat_map(|p| self.step(p).into_iter()).unique().collect();
            for (x, y) in &cur_pos {
                rank_reached.entry((*x, *y)).or_insert(k + 1);
            }
        }

        #[cfg(debug_assertions)]
        {
            // no need to loop in release
            let edges_pos = (period - 1) / 2;
            let mut base: FxHashMap<(isize, isize), usize> = Default::default();
            for y in -n..n {
                for x in -n + y..n - y {
                    let k = if x.abs() > edges_pos {
                        (x.abs() - edges_pos - 1) / period
                    } else {
                        0
                    };
                    let p = if y.abs() > edges_pos {
                        (y.abs() - edges_pos - 1) / period
                    } else {
                        0
                    };
                    let modx = x - x.signum() * k * period;
                    let mody = y - y.signum() * p * period;
                    debug_assert_eq!(
                        rank_reached.get(&(x, y)).copied(),
                        rank_reached
                            .get(&(modx, mody))
                            .map(|r| *r + k * period + p * period)
                            .filter(|v| v.abs() <= n)
                    );
                    base.entry((modx, mody)).and_modify(|v| *v += 1).or_insert(1);
                }
            }
        }
        rank_reached
    }

    fn count_rocks_by_steps(&self, n: usize) -> usize {
        let n = n as isize;

        let k = n / self.period;

        let direct_count = (self.period * k..=n)
            .map(|i| {
                if n % 2 != i % 2 {
                    0
                } else {
                    (0..=i)
                        .flat_map(|p| {
                            [(p, i - p), (p, p - i), (-p, i - p), (-p, p - i)]
                                .into_iter()
                                .unique()
                                .filter(|p| self.is_rock(*p))
                            // .map(|c|{println!("  {c:?}");c})
                        })
                        .count()
                }
            })
            .sum::<usize>();
        let full_squares = (0..k)
            .map(|i| {
                (0..=i)
                    .flat_map(|p| {
                        [(p, i - p), (p, p - i), (-p, i - p), (-p, p - i)].into_iter().unique()
                    })
                    .map(|(i, j)| {
                        let offset = (i.abs() + j.abs()) * self.period;
                        // println!("{i},{j} => offset {offset}");
                        self.rocks
                            .iter()
                            .filter(|(i, j)| (i.abs() + j.abs()) % 2 == (n + offset) % 2)
                            .count()
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();

        let part_squares = if k == 0 {
            0
        } else {
            (2 * k as usize - 1)
                * self
                    .rocks
                    .iter()
                    .filter(|(i, j)| (i.abs() + j.abs()) % 2 == (n + k * self.period) % 2)
                    .count()
                - if (self.period * k) % 2 != n % 2 {
                    0
                } else {
                    (0..self.period * k)
                        .flat_map(|p| {
                            [(p, self.period * k - p), (p, p - self.period * k)]
                                .into_iter()
                                .unique()
                                .filter(|p| self.is_rock(*p))
                        })
                        .count()
                }
        };

        // println!("count_rocks_by_steps({n}) : direct {direct_count} + full {full_squares} + parts {part_squares}");
        direct_count + full_squares + part_squares
    }

    fn count_reachable_after_n_steps(&self, n: usize) -> usize {
        // println!("count_reachable_after_n_steps({n})");
        let mut not_reached: FxHashSet<(isize, isize)> = Default::default();
        let mut previous_not_reached: FxHashSet<(isize, isize)> = Default::default();

        previous_not_reached.insert((0, 0));

        for i in 0..=n as isize {
            let occulted: FxHashSet<_> = (0..=i)
                .flat_map(|k| [(k, i - k), (k, k - i), (-k, i - k), (-k, k - i)].into_iter())
                .filter(|p| !self.is_rock(*p))
                .filter(|(x, y)| {
                    [(*x + 1, *y), (*x - 1, *y), (*x, *y + 1), (*x, *y - 1)].into_iter().all(|p| {
                        let (x, y) = p;
                        x.abs() + y.abs() >= i || not_reached.contains(&p) || self.is_rock(p)
                    })
                })
                .collect();
            // println!("{i:2} occulted {occulted:?}");

            let newly_reached: FxHashSet<_> = previous_not_reached
                .iter()
                .filter(|(x, y)| {
                    !self.is_rock((*x, *y))
                        && [(*x + 1, *y), (*x - 1, *y), (*x, *y + 1), (*x, *y - 1)]
                            .into_iter()
                            .any(|p| !(self.is_rock(p) || not_reached.contains(&p)))
                })
                .copied()
                .collect();

            let save = not_reached.clone();

            not_reached = occulted
                .union(&previous_not_reached)
                .filter(|nr| !newly_reached.contains(*nr))
                .copied()
                .collect();
            previous_not_reached = save;
            // println!("{i:2} => not_reached {not_reached:?}");

        }


        (n + 1) * (n + 1) - self.count_rocks_by_steps(n) - not_reached.len()
    }

    /// compute the count knowing that it takes `pediod` to cross a square both horizontally and vertically
    fn opt_count_reachable_after_n_steps(&self, n: usize) -> usize {
        // println!("opt_count_reachable_after_n_steps({n})");

        let base_time_to_reach = self.get_time_to_reach_base();
        let bttr = &base_time_to_reach;
        let full_reach_even = (self.min_p..self.max_p)
            .flat_map(|y| {
                (self.min_p..self.max_p).filter(move |x| {
                    let v = bttr.get(&(*x, y)).copied().unwrap_or(0);
                    v > 0 && v % 2 == 0
                })
            })
            .count() as isize;

        let full_reach_odd = (self.min_p..self.max_p)
            .flat_map(|y| {
                (self.min_p..self.max_p).filter(move |x| {
                    let v = bttr.get(&(*x, y)).copied().unwrap_or(0);
                    v > 0 && v % 2 == 1
                })
            })
            .count() as isize;
        println!("full_reach_even {full_reach_even}, full_reach_odd {full_reach_odd}");
        let n = n as isize;
        let period = self.period;
        let edges_pos = (period - 1) / 2;
        let k = (n - edges_pos) / self.period;
        if k < 0 {
            panic!("not handled here");
        }

        let full_squares_count = if k > 0 {
            k * k
                * if (n + k).is_even() {
                    full_reach_odd
                } else {
                    full_reach_even
                }
                + (k - 1)
                    * (k - 1)
                    * if (n + k).is_even() {
                        full_reach_even
                    } else {
                        full_reach_odd
                    }
        } else {
            0
        };
        println!("full squares count : {full_squares_count}");

        println!("considering {k} square around origin sq");

        let extra = (0..=n)
            .into_par_iter()
            .map(|y| {
                let get_reachable: &dyn Fn(isize, isize) -> bool = &|x, y| {
                    (x.abs() + edges_pos) / period + (y.abs() + edges_pos) / period >= k && {
                        let o = if x.abs() > edges_pos {
                            (x.abs() - edges_pos - 1) / period
                        } else {
                            0
                        };
                        let p = if y.abs() > edges_pos {
                            (y.abs() - edges_pos - 1) / period
                        } else {
                            0
                        };
                        let modx = x - x.signum() * o * period;
                        let mody = y - y.signum() * p * period;
                        base_time_to_reach
                            .get(&(modx, mody))
                            .map(|r| *r + o * period + p * period)
                            .map(|v| v.abs() <= n && v.abs() % 2 == n % 2)
                            .unwrap_or(false)
                    }
                };

                (max(0isize, (k - 1) * period - y)..=n - y)
                    .map(|x| {
                        [(x, y), (-x, y), (x, -y), (-x, -y)]
                            .into_iter()
                            .unique()
                            .filter(|(x, y)| get_reachable(*x, *y))
                            .count()
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();

        extra + full_squares_count as usize
    }
}
pub fn walk_exercise() {
    let garden: Garden = include_str!("../resources/day21_garden.txt").parse().unwrap();
    garden.naive_pos_after_n_steps(garden.period as usize);

    let max_pos = garden.count_reachable_after_n_steps(64);
    println!("pos after 64 steps : {max_pos}");

    let max_pos = garden.opt_count_reachable_after_n_steps(26501365);
    println!("pos after 26501365 steps : {max_pos}");
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn naive_count_rocks(garden: &Garden, n: usize) -> usize {
        println!("naive_count_rocks({n})");

        let n = n as isize;
        (0..=n)
            .map(|i| {
                if n % 2 != i % 2 {
                    0
                } else {
                    (0..=i)
                        .flat_map(|p| {
                            [(p, i - p), (p, p - i), (-p, i - p), (-p, p - i)]
                                .into_iter()
                                .unique()
                                .filter(|p| garden.is_rock(*p))
                            // .map(|c|{println!("  {c:?}");c})
                        })
                        .count()
                }
            })
            .sum::<usize>()
    }
    #[test]
    fn aoc_example_works() {
        let garden: Garden = indoc! {"
            .......
            ..#..#.
            .#..##.
            ...S...
            .....#.
            .##.##
            .......
        "}
        .parse()
        .unwrap();
        for l in [10, 19, 200] {
            assert_eq!(
                garden.naive_pos_after_n_steps(l),
                garden.opt_count_reachable_after_n_steps(l)
            );
        }

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

        for i in [5, 10, 11, 12, 13] {
            assert_eq!(
                naive_count_rocks(&garden, i),
                garden.count_rocks_by_steps(i)
            );
        }
        assert_eq!(4, garden.count_reachable_after_n_steps(2));
        assert_eq!(6, garden.count_reachable_after_n_steps(3));

        assert_eq!(16, garden.count_reachable_after_n_steps(6));
        assert_eq!(50, garden.count_reachable_after_n_steps(10));

        assert_eq!(167004, garden.count_reachable_after_n_steps(500));
        assert_eq!(668697, garden.count_reachable_after_n_steps(1000));
        assert_eq!(16733044, garden.count_reachable_after_n_steps(5000));

        // checking optimised (not suitable for example) compute against the slower but exact once

        let garden: Garden = include_str!("../resources/day21_garden.txt").parse().unwrap();
        assert_eq!(
            garden.opt_count_reachable_after_n_steps(130),
            garden.count_reachable_after_n_steps(130)
        );
        assert_eq!(
            garden.opt_count_reachable_after_n_steps(1000),
            garden.count_reachable_after_n_steps(1000)
        );
    }
}
