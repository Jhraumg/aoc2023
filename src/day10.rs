use ahash::AHashSet;
use eyre::{eyre, Error};
use num::Integer;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Vert,
    Hrz,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}
impl Tile {
    pub fn can_face_north(&self) -> bool {
        matches!(
            self,
            Self::Start | Self::Vert | Self::NorthEast | Self::NorthWest
        )
    }

    pub fn can_face_south(&self) -> bool {
        matches!(
            self,
            Self::Start | Self::Vert | Self::SouthEast | Self::SouthWest
        )
    }

    pub fn can_face_east(&self) -> bool {
        matches!(
            self,
            Self::Start | Self::Hrz | Self::SouthEast | Self::NorthEast
        )
    }
    pub fn can_face_west(&self) -> bool {
        matches!(
            self,
            Self::Start | Self::Hrz | Self::NorthWest | Self::SouthWest
        )
    }
}
impl Tile {
    fn from_char(s: char) -> Result<Self, Error> {
        match s {
            '|' => Ok(Self::Vert),
            '-' => Ok(Self::Hrz),
            'L' => Ok(Self::NorthEast),
            'J' => Ok(Self::NorthWest),
            '7' => Ok(Self::SouthWest),
            'F' => Ok(Self::SouthEast),
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Start),
            _ => Err(eyre!("'{s}' does not match a Tile")),
        }
    }
}
struct Map {
    ground: Vec<Vec<Tile>>,
    hlen: usize,
    vlen: usize,
    start: (usize, usize), // actual_start : Tile
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ground: Result<Vec<Vec<Tile>>, _> = s
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                l.chars().map(Tile::from_char).collect() // this is collected in a Result<Vec<..>,_>
            })
            .collect();
        let mut ground = ground?;
        let hlen = ground[0].len();
        let vlen = ground.len();
        let start = ground
            .iter()
            .enumerate()
            .flat_map(|(j, l)| l.iter().enumerate().map(move |(i, t)| (t, i, j)))
            .filter_map(|(t, x, y)| {
                if *t == Tile::Start {
                    Some((x, y))
                } else {
                    None
                }
            })
            .next()
            .ok_or(eyre!("didn't found start !!"))?;

        ground[start.1][start.0] = Map::get_start_type(start, &ground);
        Ok(Self {
            ground,
            hlen,
            vlen,
            start,
        })
    }
}

impl Map {
    fn are_connected(t1: Tile, pos1: (usize, usize), t2: Tile, pos2: (usize, usize)) -> bool {
        let (x1, y1) = pos1;
        let (x2, y2) = pos2;
        assert_ne!(pos1, pos2);
        assert!(x1 == x2 || y1 == y2);

        if x1 < x2 {
            return t1.can_face_east() && t2.can_face_west();
        }
        if x1 > x2 {
            return t1.can_face_west() && t2.can_face_east();
        }
        if y1 < y2 {
            return t1.can_face_south() && t2.can_face_north();
        }
        if y1 > y2 {
            return t1.can_face_north() && t2.can_face_south();
        }

        unimplemented!()
    }

    fn get_neighbours(x: usize, y: usize, ground: &[Vec<Tile>]) -> Vec<(Tile, (usize, usize))> {
        let hlen = ground[0].len();
        let vlen = ground.len();

        let mut result: Vec<(Tile, (usize, usize))> = vec![];
        if x > 0 {
            result.push((ground[y][x - 1], (x - 1, y)));
        }
        if x + 1 < hlen {
            result.push((ground[y][x + 1], (x + 1, y)));
        }

        if y > 0 {
            result.push((ground[y - 1][x], (x, y - 1)));
        }
        if y + 1 < vlen {
            result.push((ground[y + 1][x], (x, y + 1)));
        }

        result
    }
    fn get_connected_neighbours(&self, pos: (usize, usize)) -> Vec<(Tile, (usize, usize))> {
        let (x, y) = pos;
        let current = &self.ground[y][x];
        Self::get_neighbours(x, y, &self.ground)
            .iter()
            .filter(|(tn, posn)| Map::are_connected(*current, pos, *tn, *posn))
            .copied()
            .collect()
    }
    fn get_start_type(start_pos: (usize, usize), ground: &[Vec<Tile>]) -> Tile {
        let (startx, starty) = start_pos;
        let neighbours = Map::get_neighbours(startx, starty, ground);
        let north_n = neighbours.iter().find(|(t, (_, y))| t.can_face_south() && *y < starty);
        let south_n = neighbours.iter().find(|(t, (_, y))| t.can_face_north() && *y > starty);
        let east_n = neighbours.iter().find(|(t, (x, _))| t.can_face_west() && *x > startx);
        let west_n = neighbours.iter().find(|(t, (x, _))| t.can_face_east() && *x < startx);

        if north_n.is_some() {
            if south_n.is_some() {
                return Tile::Vert;
            }
            if east_n.is_some() {
                return Tile::NorthEast;
            }
            if west_n.is_some() {
                return Tile::NorthWest;
            }
            unreachable!();
        }
        if south_n.is_some() {
            if east_n.is_some() {
                return Tile::SouthEast;
            }
            if west_n.is_some() {
                return Tile::SouthWest;
            }
            unreachable!()
        }

        if west_n.and(east_n).is_some() {
            return Tile::Hrz;
        }

        unreachable!()
    }

    pub fn get_loop(&self) -> (AHashSet<(usize, usize)>, u64) {
        let mut pipe_loop: AHashSet<(usize, usize)> =
            AHashSet::with_capacity(self.hlen * self.vlen);
        let mut new_neighbours: Vec<(usize, usize)> = vec![self.start];
        let mut dist = 0;

        while !new_neighbours.is_empty() {
            for n in &new_neighbours {
                pipe_loop.insert(*n);
            }

            new_neighbours = new_neighbours
                .iter()
                .flat_map(|pos| {
                    self.get_connected_neighbours(*pos)
                        .into_iter()
                        .map(|(_, pos)| pos)
                        .filter(|pos| !pipe_loop.contains(pos))
                })
                .collect();
            dist += 1
        }

        (pipe_loop, dist - 1) // start was counted
    }

    fn inner_area(&self) -> u64 {
        let pipe_loop = self.get_loop().0;

        let mut area = 0;

        // standard surface detection
        // for each point we trace 2 half-line originating from it,
        // and count wether it cross the loop an odd number of times
        // hallf lines are chose vertical&&horizontal
        for x in 0..self.hlen + 1 {
            for y in 0..self.vlen + 1 {
                if x == 0 || x == self.hlen {
                    continue;
                }
                if y == 0 || y == self.vlen {
                    continue;
                }
                if pipe_loop.contains(&(x, y)) {
                    continue;
                };

                let left_cross_count = (0..x)
                    .fold((0, Tile::Ground), |(count, tile), i| {
                        if pipe_loop.contains(&(i, y)) {
                            let new_tile = self.ground[y][i];
                            match new_tile {
                                Tile::Vert => (count + 1, Tile::Ground), // explicit cross
                                Tile::NorthEast | Tile::SouthEast => (count, new_tile), // potential cross start
                                Tile::Hrz => {
                                    // continuation
                                    assert_ne!(tile, Tile::Ground);
                                    (count, tile)
                                }
                                Tile::NorthWest => {
                                    if tile == Tile::SouthEast {
                                        (count + 1, Tile::Ground) // actual cross end
                                    } else {
                                        assert_eq!(Tile::NorthEast, tile);
                                        (count, Tile::Ground) // u-turn => no cross
                                    }
                                }
                                Tile::SouthWest => {
                                    if tile == Tile::NorthEast {
                                        (count + 1, Tile::Ground) // actual cross end
                                    } else {
                                        assert_eq!(Tile::SouthEast, tile);
                                        (count, Tile::Ground) // u-turn => no cross
                                    }
                                }
                                Tile::Start => {
                                    panic!("start tile type is updated at init")
                                }

                                _ => (count, Tile::Ground),
                            }
                        } else {
                            assert_eq!(tile, Tile::Ground);
                            (count, Tile::Ground)
                        }
                    })
                    .0;
                let up_cross_count = (0..y)
                    .fold((0, Tile::Ground), |(count, tile), j| {
                        if pipe_loop.contains(&(x, j)) {
                            let new_tile = self.ground[j][x];
                            match new_tile {
                                Tile::Hrz => (count + 1, Tile::Ground), // explicit cross
                                Tile::SouthWest | Tile::SouthEast => (count, new_tile), // potential cross start
                                Tile::Vert => {
                                    // continuation
                                    assert_ne!(tile, Tile::Ground);
                                    (count, tile)
                                }
                                Tile::NorthWest => {
                                    if tile == Tile::SouthEast {
                                        (count + 1, Tile::Ground) // actual cross end
                                    } else {
                                        assert_eq!(Tile::SouthWest, tile);
                                        (count, Tile::Ground) // u-turn => no cross
                                    }
                                }
                                Tile::NorthEast => {
                                    if tile == Tile::SouthWest {
                                        (count + 1, Tile::Ground) // actual cross end
                                    } else {
                                        assert_eq!(Tile::SouthEast, tile);
                                        (count, Tile::Ground) // u-turn => no cross
                                    }
                                }
                                Tile::Start => {
                                    panic!("start tile type is updated at init")
                                }

                                _ => (count, Tile::Ground),
                            }
                        } else {
                            assert_eq!(tile, Tile::Ground);
                            (count, Tile::Ground)
                        }
                    })
                    .0;

                if left_cross_count.is_odd() || up_cross_count.is_odd() {
                    area += 1;
                }
            }
        }

        area
    }
}

pub fn follow_pipes() {
    let map: Map = include_str!("../resources/day10_pipes.txt").parse().unwrap();
    let farthest = map.get_loop().1;
    println!("fartest : {farthest}");
    let area = map.inner_area();
    println!("loop area : {area}");
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    #[test]
    fn aoc_examples_works() {
        let input = indoc! {"
            -L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF
        "};
        let map: Map = input.parse().unwrap();
        assert_eq!(4, map.get_loop().1);

        let input = indoc! {"
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
        "};
        let map: Map = input.parse().unwrap();
        assert_eq!(8, map.get_loop().1);

        let map: Map = indoc! {"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        "}
        .parse()
        .unwrap();
        assert_eq!(4, map.inner_area());

        let map: Map = indoc! {"
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        "}
        .parse()
        .unwrap();
        assert_eq!(8, map.inner_area());

        let map: Map = indoc! {"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        "}
        .parse()
        .unwrap();
        assert_eq!(10, map.inner_area());
    }
}
