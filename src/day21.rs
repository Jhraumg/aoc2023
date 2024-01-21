use ahash::{AHashMap, AHashSet};
use num::Integer;
use rayon::prelude::*;
use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug)]
struct Garden {
    rocks: Vec<bool>,
    min_p: isize,
    max_p: isize,
    period: isize,
}
impl FromStr for Garden {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let src_rocks: Vec<_> = s
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
        let mut rocks = vec![false; period as usize * period as usize];
        for (x, y) in src_rocks {
            rocks[y * period as usize + x] = true;
        }
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

        let x = ((self.period + (x - self.min_p) % self.period) % self.period) as usize;
        let y = ((self.period + (y - self.min_p) % self.period) % self.period) as usize;
        // assert!(x>=self.min_p && x<self.max_p,"{p:?} => {x} [{},{}]",self.min_p,self.max_p);
        // assert!(y>=self.min_p && y<self.max_p, "{p:?} => {y} [{},{}]",self.min_p,self.max_p);

        self.rocks[x + y * self.period as usize]
    }
    fn naive_pos_after_n_steps(&self, n: usize) -> usize {
        let mut rank_reached: AHashMap<(isize, isize), usize> = AHashMap::new();
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

    fn count_rocks_by_steps(&self, n: usize) -> usize {
        let period = self.period;
        let n = n as isize;
        let k = n / period;
        if k > 0 {
            println!("n: {n}, period : {period} => k : {k}");
        }

        // squares fully covered can only have 2 counts :
        // * either they're on even coordinates, and only rocks having the same parity as n must be counted
        // * either they're on odd coordinates,and only other rocks must be chosen
        // there are (k+1)*(k+1) square with k parity and k² with the other parity
        if k <= 1 {
            return (-n..=n)
                .map(|i| {
                    (-n + i.abs()..=n - i.abs())
                        .step_by(2)
                        .filter(|j| self.is_rock((i, *j)))
                        .count()
                })
                .sum::<usize>();
        }

        let (even_count, odd_count) =
            self.rocks.iter().enumerate().fold((0usize, 0usize), |(even_c, odd_c), (i, r)| {
                if *r {
                    match i % 2 {
                        0 => (even_c + 1, odd_c),
                        _ => (even_c, odd_c + 1),
                    }
                } else {
                    (even_c, odd_c)
                }
            });

        let full_squares_count = (k as usize)
            * (k as usize)
            * if (k + n).is_even() {
                odd_count
            } else {
                even_count
            }
            + (k as usize - 1)
                * (k as usize - 1)
                * if (k + n).is_even() {
                    even_count
                } else {
                    odd_count
                };

        // println!("k*period - period/2 = {}",k*period - period/2);

        // we're counting spots outside full square
        let outer_count = (-n..=n)
            // .into_par_iter()
            .map(|i| {
                if i.abs() >= k * period - period / 2 {
                    (-n + i.abs()..=n - i.abs())
                        .step_by(2)
                        .filter(|j| self.is_rock((i, *j)))
                        .count()
                } else {
                    // from  [-n + i.abs() ..= n- i..abs()], we need only to cover
                    //  [-n + i.abs().. -l*period -period/2[  and ]l*period+period/2..n-i.abs()]
                    // where l = k - i/period
                    let l = k - (i.abs() + period / 2) / period;
                    let min_r = l * period - period / 2;

                    // println!("i={i} => counting uppon [{},{}] and [{},{}]", -n + i.abs(), -min_r, min_r, n - i.abs());

                    (-n + i.abs()..=-min_r)
                        .filter(|j| (i.abs() + j.abs()) % 2 == n % 2 && self.is_rock((i, *j)))
                        .count()
                        + (min_r..=n - i.abs())
                            .filter(|j| (i.abs() + j.abs()) % 2 == n % 2 && self.is_rock((i, *j)))
                            .count()
                }
            })
            .sum::<usize>();

        // println!("count_rocks_by_steps({n}) : full {full_squares_count} + outer {outer_count}");
        full_squares_count + outer_count
    }

    /// Count reachable points by considering storing unreached values.
    /// * memory wise, if there are no unreachable tiles except rocks, unreached tiles size grows in O(n)
    /// * computing wise, it is a O(n²) process (0..n)*n
    fn count_reachable_after_n_steps(&self, n: usize) -> usize {
        // println!("count_reachable_after_n_steps({n})");
        let mut not_reached: AHashSet<(isize, isize)> = Default::default();
        let mut previous_not_reached: AHashSet<(isize, isize)> = Default::default();

        previous_not_reached.insert((0, 0));

        for i in 0..=n as isize {
            let occulted: AHashSet<_> = (0..=i)
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

            let newly_reached: AHashSet<_> = previous_not_reached
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
}

/// Inner class to compute the time a tile is reached without looping
/// compute here are based on the fact that the garden square :
/// * is surrounded with empty tiles
/// * presents direct empty path from center to all its edges
/// this implies that he time to reach a tile is derivable from the garden size (period)
/// and from the time to reach one off the inner tiles
struct BaseReachCounter {
    period: isize,
    max_offset: isize,
    first_reached: Vec<usize>,
}
impl BaseReachCounter {
    fn new(garden: &Garden) -> Self {
        let period = garden.period;
        let n = 3 * period;
        let offset = n / 2;
        assert_eq!(n, 2 * offset + 1);

        let mut rank_reached: AHashMap<(isize, isize), isize> = Default::default();
        let mut cur_pos: Vec<(isize, isize)> = vec![(0, 0)];
        for k in 0..n {
            cur_pos =
                cur_pos.into_iter().flat_map(|p| garden.step(p).into_iter()).unique().collect();
            for (x, y) in &cur_pos {
                rank_reached.entry((*x, *y)).or_insert(k + 1);
            }
        }

        let mut first_reached = vec![0usize; (n as usize + 1) * (n as usize + 1)];

        for x in -offset..=offset {
            for y in -offset..=offset {
                if let Some(rr) = rank_reached.get(&(x, y)) {
                    let index: usize = ((y + offset) * n + x + offset) as usize;
                    // println!("({x}, {y}) : storing {rr} at {index}");
                    first_reached[index] = *rr as usize;
                }
            }
        }

        Self {
            period,
            max_offset: offset,
            first_reached,
        }
    }

    /// derive time to reach a tile from the time to reach the sibling inner tile
    fn get_first_reached(&self, pos: (isize, isize)) -> Option<usize> {
        let (x, y) = pos;
        let period = self.period;
        let edges_pos = period / 2;
        let n = self.max_offset * 2 + 1;

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

        let index = usize::try_from((mody + self.max_offset) * n + modx + self.max_offset).unwrap();

        let base = self.first_reached[index];
        let period: usize = period as usize;

        if base != 0 {
            Some(base + (o + p) as usize * period)
        } else {
            None
        }
    }
}
impl Garden {
    /// compute the count knowing that it takes `period` to cross a square both horizontally and vertically
    /// this allows to :
    /// * directly count inner square as being full covered, thus directly counting there reachable tiles depending and their coordinates
    /// * use a `BaseReachCounter` to derive time to reach any other tile
    fn opt_count_reachable_after_n_steps(&self, n: usize) -> usize {
        // println!("opt_count_reachable_after_n_steps({n})");

        let time_to_reach_counter = BaseReachCounter::new(self);

        let bttr = &time_to_reach_counter;
        let (full_reach_even, full_reach_odd) = (self.min_p..self.max_p)
            .flat_map(|y| (self.min_p..self.max_p).map(move |x| (x, y)))
            .fold((0, 0), |(even, odd), (x, y)| {
                match bttr.get_first_reached((x, y)) {
                    None => (even, odd),
                    Some(p) if p.is_even() => (even + 1, odd),
                    _ => (even, odd + 1),
                }
            });
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

        let extra = (-n..=n)
            .into_par_iter()
            .map(|y| {
                if y.abs() >= k * period - period / 2 {
                    (-n + y.abs()..=n - y.abs())
                        .step_by(2)
                        .filter(|x| {
                            Self::get_reachable(
                                &time_to_reach_counter,
                                k * period,
                                n as usize,
                                (*x, y),
                            )
                        })
                        .count()
                } else {
                    // from  [-n + i.abs() ..= n- i..abs()], we need only to cover
                    //  [-n + i.abs().. -l*period -period/2[  and ]l*period+period/2..n-i.abs()]
                    // where l = k - i/period
                    let l = k - (y.abs() + period / 2) / period;
                    let min_r = l * period - period / 2;
                    let max_r = n - y.abs();

                    // we can inspect only every other value, but must be carefull about our start index
                    let min_r = if (min_r + y.abs()) % 2 == n % 2 {
                        min_r
                    } else {
                        min_r + 1
                    };
                    (-max_r..=-min_r)
                        .step_by(2)
                        .filter(|x| {
                            Self::get_reachable(
                                &time_to_reach_counter,
                                k * period,
                                n as usize,
                                (*x, y),
                            )
                        })
                        .count()
                        + (min_r..=max_r)
                            .step_by(2)
                            .filter(|x| {
                                Self::get_reachable(
                                    &time_to_reach_counter,
                                    k * period,
                                    n as usize,
                                    (*x, y),
                                )
                            })
                            .count()
                }
            })
            .sum::<usize>();

        extra + full_squares_count as usize
    }
    fn get_reachable(
        time_to_reach_counter: &BaseReachCounter,
        min_distance: isize,
        n: usize,
        pos: (isize, isize),
    ) -> bool {
        let (x, y) = pos;
        let period = time_to_reach_counter.period;
        let edge_pos = period / 2;
        (x.abs() + edge_pos) + (y.abs() + edge_pos) >= min_distance
            && time_to_reach_counter
                .get_first_reached(pos)
                .map(|v| v <= n && v % 2 == n % 2)
                .unwrap_or(false)
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
    use plotters::define_color;

    fn naive_count_rocks(garden: &Garden, n: usize) -> usize {
        let n = n as isize;
        // let's cover the entier square, but only considering tiles which are compatible with n, ie which rack is of same parity
        (-n..=n)
            .map(|i|
                // we need only to cover -n + i.abs() ..= n- i..abs()
                (-n+i.abs()..=n-i.abs()).step_by(2).filter(|j|garden.is_rock((i,*j))).count())
            .sum::<usize>()
    }

    #[test]
    fn can_count_reached_rocks() {
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

        for i in (0..garden.period as usize * 30).step_by(10) {
            assert_eq!(
                naive_count_rocks(&garden, i),
                garden.count_rocks_by_steps(i),
                "different results for {i}"
            );
        }
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
        for l in [1, 2, 3, 10, 19, 200] {
            assert_eq!(
                garden.count_reachable_after_n_steps(l),
                garden.opt_count_reachable_after_n_steps(l),
                "counts don't match for {l}"
            );

            assert_eq!(
                garden.naive_pos_after_n_steps(l),
                garden.opt_count_reachable_after_n_steps(l),
                "counts don't match for {l}"
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
                garden.count_rocks_by_steps(i),
                "different results for {i}"
            );
        }
        assert_eq!(4, garden.count_reachable_after_n_steps(2));
        assert_eq!(6, garden.count_reachable_after_n_steps(3));

        assert_eq!(16, garden.count_reachable_after_n_steps(6));
        assert_eq!(50, garden.count_reachable_after_n_steps(10));

        assert_eq!(167004, garden.count_reachable_after_n_steps(500));
        #[cfg(not(debug_assertions))]
        assert_eq!(668697, garden.count_reachable_after_n_steps(1000));
        #[cfg(not(debug_assertions))]
        assert_eq!(16733044, garden.count_reachable_after_n_steps(5000));

        // checking optimised (not suitable for example) compute against the slower but exact once
        #[cfg(not(debug_assertions))]
        {
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
}

#[cfg(graph)]
use plotters::prelude::*;

/// draw reachable(n) as a series of lines, one for each value %garden.period
/// it indeed appears to be polinomial if values are sampled every garden.period
/// (which was not obvious for me when there was some rocks)
#[cfg(graph)]
#[test]
fn draw_counts() -> eyre::Result<()> {
    #[cfg(debug_assertions)]
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

    let g2: Garden = indoc! {"
            ...........
            .#########.
            .#.........
            .#.#######.
            .#.#.....#.
            .#.#.S##.#.
            .#.#...#.#.
            .#.#####.#.
            .#.......#.
            .#########.
            ...........
        "}
    .parse()
    .unwrap();

    #[cfg(not(debug_assertions))]
    let garden: Garden = include_str!("../resources/day21_garden.txt").parse().unwrap();
    #[cfg(not(debug_assertions))]
    let output_suffix = "release";
    #[cfg(debug_assertions)]
    let output_suffix = "debug";

    let output_bitmap = format!(".day_21_counts.{output_suffix}.png");
    let root_area = BitMapBackend::new(&output_bitmap, (3840, 2160)).into_drawing_area();

    root_area.fill(&WHITE)?;

    let root_area = root_area.titled(
        &format!("reached tile by distance ({output_suffix})"),
        ("sans-serif", 60),
    )?;

    let max_n: usize = garden.period as usize * 7;

    let max_value = garden.count_reachable_after_n_steps(max_n);

    let x_axis = IntoLinspace::step(0..max_n, garden.period as usize);

    let mut cc = ChartBuilder::on(&root_area)
        .margin(5)
        .set_all_label_area_size(50)
        .caption("reached ", ("sans-serif", 40))
        .build_cartesian_2d(0..max_n, 0..max_value)?;

    cc.configure_mesh()
        .x_labels(10)
        .y_labels(20)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{v}"))
        .y_label_formatter(&|v| format!("{v}"))
        .draw()?;

    for i in 1..garden.period as usize {
        // let color = RGBColor(random(),random(),random());
        let g = &garden;
        cc.draw_series(LineSeries::new(
            x_axis.values().map(move |x| (x + i, g.count_reachable_after_n_steps(x + i))),
            RED,
        ))?
        .label(format!("g reacheable tiles ({i}%period)"))
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

        let g2 = &g2;
        cc.draw_series(LineSeries::new(
            x_axis.values().map(move |x| (x + i, g2.count_reachable_after_n_steps(x + i))),
            BLUE,
        ))?
        .label(format!("g2 reacheable tiles ({i}%period)"))
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));
    }

    cc.configure_series_labels().border_style(BLACK).draw()?;

    root_area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    Ok(())
}
