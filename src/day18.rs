use eyre::{eyre, Error};
use itertools::Itertools;
use num::Integer;
use std::cmp::{max, min};
use ahash::AHashSet;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}
impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().ok_or_else(|| eyre!("empty direction !"))? {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            c => Err(eyre!("unknown direction {c}")),
        }
    }
}
#[derive(Debug)]
struct Trench<const COLOR_FIRST: bool> {
    dug_edges: Vec<((usize, usize), (usize, usize))>,
}

impl<const COLOR_FIRST: bool> FromStr for Trench<COLOR_FIRST> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dug: Result<Vec<(Direction, usize)>, Self::Err> = s
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                let mut parts = l.split(' ');
                if !COLOR_FIRST {
                    let dir = parts
                        .next()
                        .ok_or_else(|| eyre!("No direction provided in {l}"))
                        .and_then(str::parse::<Direction>)
                        .map_err(|e| eyre!("parse error {e} on {l}"))?;
                    let len: usize = parts
                        .next()
                        .ok_or_else(|| eyre!("No len provided in {l}"))
                        .and_then(|len| len.parse().map_err(|e| eyre!("len read error {e}")))?;
                    //let color :&str = parts.next().ok_or_else(||eyre!("No color provided in {l}"))?;
                    Ok((dir, len))
                } else {
                    let info = parts.nth(2).unwrap();
                    let l = info.len();
                    let info = &info[2..l - 1];
                    let len = info[0..5]
                        .chars()
                        .map(|c| c.to_digit(16).unwrap())
                        .reduce(|acc, m| acc * 16 + m)
                        .unwrap() as usize;
                    let dir = match info.chars().nth(5).unwrap() {
                        '0' => Direction::Right,
                        '1' => Direction::Down,
                        '2' => Direction::Left,
                        '3' => Direction::Up,
                        _other => panic!("unexpected direction {_other}"),
                    };
                    Ok((dir, len))
                }
            })
            .collect();
        let dug = dug?;

        let points = dug.into_iter().fold(vec![(0isize, 0isize)], |mut acc, (dir, len)| {
            let (x, y) = acc.last().copied().unwrap();
            match dir {
                Direction::Up => {
                    acc.push((x, y - len as isize));
                }
                Direction::Left => {
                    acc.push((x - len as isize, y));
                }
                Direction::Down => {
                    acc.push((x, y + len as isize));
                }
                Direction::Right => {
                    acc.push((x + len as isize, y));
                }
            }
            acc
        });

        let minx = points
            .iter()
            .map(|(x, _)| x)
            .min()
            .copied()
            .ok_or_else(|| eyre!("no points in Trench"))?;
        let miny = points
            .iter()
            .map(|(_, y)| y)
            .min()
            .copied()
            .ok_or_else(|| eyre!("no points in Trench"))?;
        println!("minx {minx}, miny  {miny}");
        let dug_edges: Vec<_> = points
            .into_iter()
            .tuple_windows::<(_, _)>()
            .map(|((x1, y1), (x2, y2))| {
                (
                    ((x1 - minx + 1) as usize, (y1 - miny + 1) as usize),
                    ((x2 - minx + 1) as usize, (y2 - miny + 1) as usize),
                )
            })
            .collect();
        Ok(Self {
            dug_edges,
            // maxx,
            // maxy,
        })
    }
}

impl<const COLOR_FIRST: bool> Trench<COLOR_FIRST> {
    fn compute_area(&self) -> usize {
        let lines_y = self
            .dug_edges
            .iter()
            .filter_map(|((_, y1), (_, y2))| if *y1 == *y2 { Some(*y1) } else { None })
            .sorted()
            .unique()
            .collect_vec();
        let column_x = self
            .dug_edges
            .iter()
            .filter_map(|((x1, _), (x2, _))| if *x1 == *x2 { Some(*x1) } else { None })
            .sorted()
            .unique()
            .collect_vec();

        let mut area = 0;
        let mut rectangles_in: AHashSet<(usize, usize)> = AHashSet::new();
        for (j, (y1, y2)) in lines_y.iter().copied().tuple_windows().enumerate() {
            for (i, (x1, x2)) in column_x.iter().copied().tuple_windows().enumerate() {
                let column_before = self
                    .dug_edges
                    .iter()
                    .filter_map(|((x1, y1), (x2, y2))| {
                        if *x1 == *x2 {
                            Some((*x1, min(*y1, *y2), max(*y1, *y2)))
                        } else {
                            None
                        }
                    })
                    .filter(|(x, ymin, ymax)| *x <= x1 && *ymin <= y1 && *ymax >= y2)
                    .count();
                if column_before.is_odd() {
                    rectangles_in.insert((i, j));
                    area += (x2 + 1 - x1) * (y2 + 1 - y1);
                    let left_inside = i > 0 && rectangles_in.contains(&(i - 1, j));
                    if left_inside {
                        //rectangle on the left is also in the area, left side as been counted twice
                        area -= y2 + 1 - y1;
                    }
                    let up_inside = j > 0 && rectangles_in.contains(&(i, j - 1));
                    if up_inside {
                        //rectangle above is also in the area, left side as been counted twice
                        area -= x2 + 1 - x1;
                    }
                    if left_inside && up_inside {
                        // edge has been removed twice
                        area += 1;
                    }
                    if j > 0 {
                        let up_right_inside = rectangles_in.contains(&(i + 1, j - 1));
                        if up_right_inside && !up_inside {
                            area -= 1;
                        }
                    }
                    if i > 0 && j > 0 {
                        let upleft_inside = rectangles_in.contains(&(i - 1, j - 1));
                        if upleft_inside && !(up_inside || left_inside) {
                            area -= 1;
                        }
                    }
                }
            }
        }
        area
    }
}

pub fn dig_lagoon() {
    let trench: Trench<false> =
        include_str!("../resources/day18_dig_instructions.txt").parse().unwrap();
    let area = trench.compute_area();
    println!("area: {area}");

    let trench: Trench<true> =
        include_str!("../resources/day18_dig_instructions.txt").parse().unwrap();
    let area = trench.compute_area();
    println!("area: {area}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    #[test]
    fn aoc_example_works() {
        let input = indoc! {"
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
        "};
        let trench: Trench<false> = input.parse().unwrap();
        assert_eq!(62, trench.compute_area());

        let trench: Trench<true> = input.parse().unwrap();
        assert_eq!(952408144115, trench.compute_area());
    }
}
