use std::cmp::{max, min};
use eyre::{eyre, Error};
use itertools::Itertools;
use num::Integer;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use std::iter::once;
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
struct Trench<const COLOR_FIRST:bool> {
    dug_edges: Vec<((usize, usize),(usize,usize))>,
    maxx: usize,
    maxy: usize,
}

impl<const COLOR_FIRST:bool> FromStr for Trench<COLOR_FIRST> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dug: Result<Vec<(Direction, usize)>, Self::Err> = s
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                let mut parts = l.split(' ');
                if ! COLOR_FIRST {
                let dir = parts
                    .next()
                    .ok_or_else(|| eyre!("No direction provided in {l}"))
                    .and_then(str::parse::<Direction>)
                    .map_err(|e| eyre!("parse error on {l}"))?;
                let len: usize = parts
                    .next()
                    .ok_or_else(|| eyre!("No len provided in {l}"))
                    .and_then(|len| len.parse().map_err(|e| eyre!("len read error")))?;
                //let color :&str = parts.next().ok_or_else(||eyre!("No color provided in {l}"))?;
                Ok((dir, len))}else{
                    let info=parts.nth(2).unwrap();
                    let l = info.len();
                    let info = &info[2..l-1];
                    let len = info[0..5].chars().map(|c|c.to_digit(16).unwrap()).rev().reduce(|acc,m|acc*16+m).unwrap() as usize;
                    let dir= match info.chars().nth(5).unwrap() {
                        '0' => Direction::Right,
                        '1' => Direction::Down,
                        '2' => Direction::Left,
                        '3' => Direction::Up,
                        other => panic!("unexpected direction {other}")
                    };
                    Ok((dir, len))
                }
            })
            .collect();
        let dug = dug?;

        let points = dug.into_iter().fold(vec![(0usize,0usize)], |mut acc, (dir, len)| {
            let (x, y) = acc.last().copied().unwrap();

            match dir {
                Direction::Up => {
                    acc.push((x, y - len));
                }
                Direction::Left => {
                    acc.push((x - len, y));
                }
                Direction::Down => {
                    acc.push((x, y + len));
                }
                Direction::Right => {
                    acc.push((x + len, y));
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
            .map(|((x1, y1),(x2, y2))| (((x1 - minx + 1) as usize, (y1 - miny + 1) as usize),((x2 - minx + 1) as usize, (y2 - miny + 1) as usize)))
            .collect();
        let maxx = 1 + dug_edges.iter().map(|((x1,_),(x2,_) )| max(x1,x2)).max().copied().unwrap();
        let maxy = 1 + dug_edges.iter().map(|((_, y1),(_,y2))| max(y1,y2)).max().copied().unwrap();

        assert_eq!(dug_edges[0].0,dug_edges[dug_edges.len()-1].1); // just to check


        Ok(Self { dug_edges, maxx, maxy })
    }
}

impl<const COLOR_FIRST:bool> Display for Trench<COLOR_FIRST> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.maxy {
            for x in 0..self.maxx {
                f.write_char(if self.get_cross((x, y))!=Cross::NoCross { // FIXME
                    '#'
                } else if self.is_inside((x, y)) {
                    'O'
                } else {
                    '.'
                })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Cross {
    Full,
    Left,
    Right,
    Up,
    Down,
    NoCross,
}

impl<const COLOR_FIRST:bool> Trench<COLOR_FIRST> {
    fn get_cross(&self, p:(usize, usize)) -> Cross {
        self.dug_edges.iter().copied().map(|((x1,y1),(x2,y2))| {
            let (x,y)=p;
            if x1 == x2 && x1 == x && min(y1, y2) <= y && max(y1, y2) >= y{
                if min(y1, y2) == y {Cross::Down} else if max(y1,y2)==y{Cross::Up}else  { Cross::Full }
            }else if y1 == y2 && y1 == y && min(x1, x2) <= x && max(x1, x2) >= x{
                if min(x1,x2)==x {Cross::Right} else if max(x1,x2) ==y{Cross::Left}else  { Cross::Full }
            }
             else { Cross::NoCross }
        }).find(|c|*c != Cross::NoCross).unwrap_or(Cross::NoCross)
    }

    fn is_inside(&self, p: (usize, usize)) -> bool {
        let (x, y) = p;
        if self.get_cross((x, y))!=Cross::NoCross {
            return true;
        }
        let vertical_cross = (0..=y)
            .fold((0, Cross::NoCross), |(cross_count, cross), j| {
                let current_cross = match self.get_cross((x, j)) {
                    Cross::Up | Cross::Right => cross,
                    c => c
                };
                let new_cross_count = cross_count + {
                    if current_cross == Cross::Full {
                        1
                    } else if current_cross == Cross::NoCross {0}else{
                        if cross != Cross::NoCross && cross != current_cross {
                            1
                        } else {
                            0
                        }
                    }
                };
                // if new_cross_count != cross_count {
                //     println!("({x},{j}):crossing from {cross:?} to {current_cross:?}");
                // }
                (new_cross_count, current_cross)
            })
            .0;

        let horizontal_cross = (0..=x)
            .fold((0, Cross::NoCross), |(cross_count, cross), i| {
                let current_cross = match self.get_cross((i, y)) {
                    Cross::Left |Cross::Right => cross,
                    c => c
                };
                let new_cross_count = cross_count + {
                    if current_cross == Cross::Full {
                        1
                    } else if current_cross == Cross::NoCross {0}else{
                        if cross != Cross::NoCross && cross != current_cross {
                            1
                        } else {
                            0
                        }
                    }
                };
                // if new_cross_count != cross_count {
                //     println!("({i},{y}):crossing from {cross:?} to {current_cross:?}");
                // }
                (new_cross_count, current_cross)
            })
            .0;

        // println!("({x},{y}) => {vertical_cross},{horizontal_cross}");
        vertical_cross>0 && vertical_cross.is_odd() && horizontal_cross>0 && horizontal_cross.is_odd()
    }
    fn compute_area(&self) -> usize {
        (0..self.maxx)
            .flat_map(|x| (0..self.maxy).map(move |y| (x, y)))
            .map(|(x, y)| if self.is_inside((x, y)) { 1 } else { 0 })
            .sum()
    }
}


pub fn dig_lagoon(){
    let trench:Trench<false> = include_str!("../resources/day18_dig_instructions.txt").parse().unwrap();
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
        println!("{trench:?}");
        assert!(!trench.is_inside((0, 1)));

        println!("{trench}");
        assert!(trench.is_inside((3, 1)));
        assert!(trench.is_inside((3, 2)));
        assert!(!trench.is_inside((2, 4)));

        assert_eq!(62, trench.compute_area());
        let trench: Trench<true> = input.parse().unwrap();
        // assert_eq!(952408144115, trench.compute_area());
    }
}
