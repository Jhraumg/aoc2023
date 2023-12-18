use crate::day16::Direction::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug)]
struct Contraption {
    mirrors: HashMap<(usize, usize), char>,
    maxx: usize,
    maxy: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    West,
    South,
    East,
}
impl FromStr for Contraption {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mirrors = s
            .lines()
            .enumerate()
            .flat_map(|(j, l)| l.trim().chars().enumerate().map(move |(i, c)| ((i, j), c)))
            .filter(|(_, c)| *c != '.')
            .collect();
        let maxy = s.lines().count();
        let maxx = s.lines().next().unwrap().len();
        Ok(Self {
            mirrors,
            maxx,
            maxy,
        })
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Beam {
    x: usize,
    y: usize,
    dir: Option<Direction>,
}

impl Contraption {
    fn propagate_step(&self, beam: Beam, inplace: bool) -> Vec<Beam> {
        let Beam { x, y, dir } = beam;
        if dir.is_none() {
            return vec![];
        }
        if !inplace {
            if let Some(c) = self.mirrors.get(&(x, y)) {
                let beams = match c {
                    '-' => match dir.unwrap() {
                        North | South => {
                            vec![
                                Beam {
                                    x,
                                    y,
                                    dir: Some(West),
                                },
                                Beam {
                                    x,
                                    y,
                                    dir: Some(East),
                                },
                            ]
                        }
                        West | East => {
                            vec![]
                        }
                    },
                    '/' => match dir.unwrap() {
                        North => vec![Beam {
                            x,
                            y,
                            dir: Some(East),
                        }],
                        West => vec![Beam {
                            x,
                            y,
                            dir: Some(South),
                        }],
                        South => vec![Beam {
                            x,
                            y,
                            dir: Some(West),
                        }],
                        East => vec![Beam {
                            x,
                            y,
                            dir: Some(North),
                        }],
                    },
                    '\\' => match dir.unwrap() {
                        North => vec![Beam {
                            x,
                            y,
                            dir: Some(West),
                        }],
                        West => vec![Beam {
                            x,
                            y,
                            dir: Some(North),
                        }],
                        South => vec![Beam {
                            x,
                            y,
                            dir: Some(East),
                        }],
                        East => vec![Beam {
                            x,
                            y,
                            dir: Some(South),
                        }],
                    },
                    '|' => match dir.unwrap() {
                        North | South => {
                            vec![]
                        }
                        West | East => {
                            vec![
                                Beam {
                                    x,
                                    y,
                                    dir: Some(North),
                                },
                                Beam {
                                    x,
                                    y,
                                    dir: Some(South),
                                },
                            ]
                        }
                    },
                    _ => panic!(),
                };
                if !beams.is_empty() {
                    return beams
                        .into_iter()
                        .flat_map(|b| self.propagate_step(b, true).into_iter())
                        .collect();
                }
            }
        }

        let mut result = match dir.unwrap() {
            North => {
                if y == 0 {
                    vec![]
                } else {
                    let mut result = match self.mirrors.get(&(x, y - 1)) {
                        None => vec![Beam { x, y: y - 1, dir }],
                        Some('|') => {
                            if y > 1 {
                                vec![Beam { x, y: y - 2, dir }]
                            } else {
                                vec![]
                            }
                        }
                        Some('\\') => {
                            if x > 0 {
                                vec![Beam {
                                    x: x - 1,
                                    y: y - 1,
                                    dir: Some(West),
                                }]
                            } else {
                                vec![]
                            }
                        }
                        Some('/') => {
                            if x + 1 < self.maxx {
                                vec![Beam {
                                    x: x + 1,
                                    y: y - 1,
                                    dir: Some(East),
                                }]
                            } else {
                                vec![]
                            }
                        }

                        Some('-') => [
                            if x > 0 {
                                Some(Beam {
                                    x: x - 1,
                                    y: y - 1,
                                    dir: Some(West),
                                })
                            } else {
                                None
                            },
                            if x + 1 < self.maxx {
                                Some(Beam {
                                    x: x + 1,
                                    y: y - 1,
                                    dir: Some(East),
                                })
                            } else {
                                None
                            },
                        ]
                        .into_iter()
                        .flatten()
                        .collect(),
                        Some(c) => panic!("unknown mirror {c} at ({x},{y})"),
                    };
                    result.push(Beam {
                        x,
                        y: y - 1,
                        dir: None,
                    });
                    result
                }
            }
            West => {
                if x == 0 {
                    vec![]
                } else {
                    let mut result = match self.mirrors.get(&(x - 1, y)) {
                        None => vec![Beam { x: x - 1, y, dir }],
                        Some('-') => {
                            if x > 1 {
                                vec![Beam { x: x - 2, y, dir }]
                            } else {
                                vec![]
                            }
                        }
                        Some('\\') => {
                            if y > 0 {
                                vec![Beam {
                                    x: x - 1,
                                    y: y - 1,
                                    dir: Some(North),
                                }]
                            } else {
                                vec![]
                            }
                        }
                        Some('/') => {
                            if y + 1 < self.maxy {
                                vec![Beam {
                                    x: x - 1,
                                    y: y + 1,
                                    dir: Some(South),
                                }]
                            } else {
                                vec![]
                            }
                        }
                        Some('|') => [
                            if y > 0 {
                                Some(Beam {
                                    x: x - 1,
                                    y: y - 1,
                                    dir: Some(North),
                                })
                            } else {
                                None
                            },
                            if y + 1 < self.maxy {
                                Some(Beam {
                                    x: x - 1,
                                    y: y + 1,
                                    dir: Some(South),
                                })
                            } else {
                                None
                            },
                        ]
                        .into_iter()
                        .flatten()
                        .collect(),
                        Some(c) => panic!("unexpected mirror {c} at ({x},{y})"),
                    };
                    result.push(Beam {
                        x: x - 1,
                        y,
                        dir: None,
                    });
                    result
                }
            }
            South => {
                if y >= self.maxy - 1 {
                    vec![]
                } else {
                    let mut result = match self.mirrors.get(&(x, y + 1)) {
                        None => vec![Beam { x, y: y + 1, dir }],
                        Some('|') => {
                            if y + 2 < self.maxy {
                                vec![Beam { x, y: y + 2, dir }]
                            } else {
                                vec![]
                            }
                        }
                        Some('/') => {
                            if x > 0 {
                                vec![Beam {
                                    x: x - 1,
                                    y: y + 1,
                                    dir: Some(West),
                                }]
                            } else {
                                vec![]
                            }
                        }
                        Some('\\') => {
                            if x + 1 < self.maxx {
                                vec![Beam {
                                    x: x + 1,
                                    y: y + 1,
                                    dir: Some(East),
                                }]
                            } else {
                                vec![]
                            }
                        }

                        Some('-') => [
                            if x > 0 {
                                Some(Beam {
                                    x: x - 1,
                                    y: y + 1,
                                    dir: Some(West),
                                })
                            } else {
                                None
                            },
                            if x + 1 < self.maxx {
                                Some(Beam {
                                    x: x + 1,
                                    y: y + 1,
                                    dir: Some(East),
                                })
                            } else {
                                None
                            },
                        ]
                        .into_iter()
                        .flatten()
                        .collect(),
                        Some(c) => panic!("unexpected mirror {c} at ({x},{y})"),
                    };
                    result.push(Beam {
                        x,
                        y: y + 1,
                        dir: None,
                    });
                    result
                }
            }
            East => {
                if x >= self.maxx - 1 {
                    vec![]
                } else {
                    let mut result = match self.mirrors.get(&(x + 1, y)) {
                        None => vec![Beam { x: x + 1, y, dir }],
                        Some('-') => {
                            if x + 2 < self.maxx {
                                vec![Beam { x: x + 2, y, dir }]
                            } else {
                                vec![]
                            }
                        }
                        Some('/') => {
                            if y > 0 {
                                vec![Beam {
                                    x: x + 1,
                                    y: y - 1,
                                    dir: Some(North),
                                }]
                            } else {
                                vec![]
                            }
                        }
                        Some('\\') => {
                            if y + 1 < self.maxy {
                                vec![Beam {
                                    x: x + 1,
                                    y: y + 1,
                                    dir: Some(South),
                                }]
                            } else {
                                vec![]
                            }
                        }
                        Some('|') => [
                            if y > 0 {
                                Some(Beam {
                                    x: x + 1,
                                    y: y - 1,
                                    dir: Some(North),
                                })
                            } else {
                                None
                            },
                            if y + 1 < self.maxy {
                                Some(Beam {
                                    x: x + 1,
                                    y: y + 1,
                                    dir: Some(South),
                                })
                            } else {
                                None
                            },
                        ]
                        .into_iter()
                        .flatten()
                        .collect(),
                        Some(c) => panic!("unexpected mirror {c} at ({x},{y})"),
                    };
                    result.push(Beam {
                        x: x + 1,
                        y,
                        dir: None,
                    });
                    result
                }
            }
        };
        for b in result.iter().filter(|b| b.x >= self.maxx || b.y >= self.maxy) {
            println!("Error on {b:?} from {x},{y},{dir:?}");
        }

        result.push(Beam { x, y, dir: None });
        result
    }

    fn propagate(&self) -> HashSet<Beam> {
        self.propagate_from_edge(Beam {
            x: 0,
            y: 0,
            dir: Some(East),
        })
    }
    fn propagate_from_edge(&self, beam: Beam) -> HashSet<Beam> {
        let mut result: HashSet<Beam> = Default::default();
        result.insert(beam);

        let mut current_beams: Vec<Beam> = vec![beam];

        while !current_beams.is_empty() {
            // println!("current_beams {current_beams:?}");
            current_beams = current_beams
                .into_iter()
                .flat_map(|b| self.propagate_step(b, false).into_iter())
                .filter(|b| !result.contains(b))
                .collect();
            for b in &current_beams {
                result.insert(*b);
            }
        }

        // self._display_beam(&result);
        result
    }
    fn _display_beam(&self, beams: &HashSet<Beam>) {
        let beam_points: HashSet<(usize, usize)> = beams.iter().map(|b| (b.x, b.y)).collect();
        assert!(beam_points.iter().all(|(x, y)| *x < self.maxx && *y < self.maxy));
        for y in 0..self.maxy {
            println!(
                "{}",
                (0..self.maxx)
                    .map(|x| if beam_points.contains(&(x, y)) {
                        &'#'
                    } else {
                        self.mirrors.get(&(x, y)).unwrap_or(&'.')
                    })
                    .join("")
            );
        }
    }
    fn _print(&self) {
        for y in 0..self.maxy {
            println!(
                "{}",
                (0..self.maxx).map(|x| self.mirrors.get(&(x, y)).unwrap_or(&'.')).join("")
            );
        }
    }
    fn tune(&self) -> usize {
        max(
            [0, self.maxy - 1]
                .into_iter()
                .flat_map(|y| {
                    (0..self.maxx).flat_map(move |x| {
                        [North, South].into_iter().map(move |dir| Beam {
                            x,
                            y,
                            dir: Some(dir),
                        })
                    })
                })
                .collect_vec()
                .into_par_iter()
                .map(|b| count_energized(self.propagate_from_edge(b)))
                .max()
                .unwrap(),
            [0, self.maxx - 1]
                .into_iter()
                .flat_map(|x| {
                    (0..self.maxy).flat_map(move |y| {
                        [West, East].into_iter().map(move |dir| Beam {
                            x,
                            y,
                            dir: Some(dir),
                        })
                    })
                })
                .map(|b| count_energized(self.propagate_from_edge(b)))
                .max()
                .unwrap(),
        )
    }
}

fn count_energized(beams: HashSet<Beam>) -> usize {
    beams.into_iter().map(|b| (b.x, b.y)).unique().count()
}

pub fn fix_contraption() {
    let contraption: Contraption =
        include_str!("../resources/day16_contraption.txt").parse().unwrap();
    let energized = count_energized(contraption.propagate());
    println!("energized cells : {energized}");
    let max_energized = contraption.tune();
    println!("tuned contraption : {max_energized} energized cells");
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    #[test]
    fn aoc_examples_works() {
        let input = indoc! {r"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
        "};
        let contraption: Contraption = input.parse().unwrap();
        contraption._print();
        println!();
        assert_eq!(46, count_energized(contraption.propagate()));
        assert_eq!(51, contraption.tune());
    }
}
