use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

struct Sky<const F: usize> {
    galaxies: HashSet<(usize, usize)>,
}
impl<const F: usize> FromStr for Sky<F> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut galaxies: HashSet<_> = s
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

        let maxx = galaxies.iter().map(|(x, _)| *x).max().unwrap_or(0);
        let maxy = galaxies.iter().map(|(_, y)| *y).max().unwrap_or(0);
        for x in (1..maxx).rev() {
            if !galaxies.iter().any(|(i, _)| *i == x) {
                galaxies = galaxies
                    .into_iter()
                    .map(|(i, j)| if i > x { (i + F - 1, j) } else { (i, j) })
                    .collect()
            }
        }

        for y in (1..maxy).rev() {
            if !galaxies.iter().any(|(_, j)| *j == y) {
                galaxies = galaxies
                    .into_iter()
                    .map(|(i, j)| if j > y { (i, j + F - 1) } else { (i, j) })
                    .collect()
            }
        }
        Ok(Self { galaxies })
    }
}

fn distance(g1: (usize, usize), g2: (usize, usize)) -> usize {
    let (x1, y1) = g1;
    let (x2, y2) = g2;

    x1.abs_diff(x2) + y1.abs_diff(y2)
}
impl<const N: usize> Sky<N> {
    pub fn sum_distance(&self) -> usize {
        self.galaxies.iter().combinations(2).map(|g| distance(*g[0], *g[1])).sum()
    }
}

pub fn observe_space() {
    let sky: Sky<1> = include_str!("../resources/day11_space_observation.txt").parse().unwrap();
    let sum_distance = sky.sum_distance();
    println!("sum distance {sum_distance}");

    let sky1000000: Sky<1000000> =
        include_str!("../resources/day11_space_observation.txt").parse().unwrap();
    let sum_distance1000000 = sky1000000.sum_distance();
    println!("sum distance1000000 {sum_distance1000000}");
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    #[test]
    fn aoc_example_works() {
        let input = indoc! {"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "};
        let sky: Sky<2> = input.parse().unwrap();

        assert_eq!(374, sky.sum_distance());

        let sky10: Sky<10> = input.parse().unwrap();
        assert_eq!(1030, sky10.sum_distance());

        let sky100: Sky<100> = input.parse().unwrap();
        assert_eq!(8410, sky100.sum_distance());
    }
}
