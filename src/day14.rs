use crate::day14::Axis::{Horizontal, Vertical};
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Scene {
    maxx: usize,
    maxy: usize,
    rounded: Vec<[usize; 2]>,
    cubed: Vec<[usize; 2]>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl From<Axis> for usize {
    fn from(value: Axis) -> Self {
        match value {
            Axis::Horizontal => 0,
            Axis::Vertical => 1,
        }
    }
}

impl FromStr for Scene {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<Vec<char>> = s
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().collect())
            .collect();

        let chars = &chars;

        let maxy = chars.len();
        let maxx = chars[0].len();
        let rounded = (0..maxx)
            .flat_map(|x| {
                (0..maxy).filter_map(move |y| {
                    if chars[y][x] == 'O' {
                        Some([x, y])
                    } else {
                        None
                    }
                })
            })
            .collect();
        let cubed = (0..maxx)
            .flat_map(|x| {
                (0..maxy).filter_map(move |y| {
                    if chars[y][x] == '#' {
                        Some([x, y])
                    } else {
                        None
                    }
                })
            })
            .collect();

        Ok(Self {
            maxx,
            maxy,
            rounded,
            cubed,
        })
    }
}
impl Scene {
    fn weight(&self) -> usize {
        self.rounded.iter().map(|[_, y]| self.maxy - y).sum()
    }

    fn move_to_origin(&self, round: &[usize; 2], axis: Axis) -> [usize; 2] {
        let m_axis = axis as usize;
        let f_axis = 1 - m_axis;
        let start = round[m_axis];
        let f_col_row = round[f_axis];
        let next_cubed_pos = self
            .cubed
            .iter()
            .filter(|c| c[f_axis] == f_col_row && c[m_axis] < start)
            .map(|c| c[m_axis])
            .max();
        let number_of_rounded_before = self
            .rounded
            .iter()
            .filter(|r| {
                r[f_axis] == f_col_row
                    && r[m_axis] < start
                    && if let Some(next_cubed_pos) = next_cubed_pos {
                        r[m_axis] > next_cubed_pos
                    } else {
                        true
                    }
            })
            .count();

        let mut result = *round;
        result[m_axis] = if let Some(next_cubed_pos) = next_cubed_pos {
            next_cubed_pos + 1
        } else {
            0
        } + number_of_rounded_before;
        result
    }
    fn move_away_origin(&self, round: &[usize; 2], axis: Axis) -> [usize; 2] {
        let m_axis = axis as usize;
        let f_axis = 1 - m_axis;
        let start = round[m_axis];
        let f_col_row = round[f_axis];
        let next_cubed_pos = self
            .cubed
            .iter()
            .filter(|c| c[f_axis] == f_col_row && c[m_axis] > start)
            .map(|c| c[m_axis])
            .min();
        let number_of_rounded_after = self
            .rounded
            .iter()
            .filter(|r| {
                r[f_axis] == f_col_row
                    && r[m_axis] > start
                    && if let Some(next_cubed_pos) = next_cubed_pos {
                        r[m_axis] < next_cubed_pos
                    } else {
                        true
                    }
            })
            .count();

        let mut result = *round;
        result[m_axis] = next_cubed_pos.unwrap_or(match axis {
            Axis::Horizontal => self.maxx,
            Axis::Vertical => self.maxy,
        }) - 1
            - number_of_rounded_after;

        result
    }

    fn tilt(&self, round: &[[usize; 2]], direction: Direction) -> Vec<[usize; 2]> {
        let mut result: Vec<[usize; 2]> = round
            .iter()
            .map(|r| match direction {
                Direction::North => self.move_to_origin(r, Vertical),
                Direction::West => self.move_to_origin(r, Horizontal),
                Direction::South => self.move_away_origin(r, Vertical),
                Direction::East => self.move_away_origin(r, Horizontal),
            })
            .collect();
        result.sort();
        result
    }
    fn tune(&mut self) {
        let mut previous: HashMap<Vec<[usize; 2]>, usize> = HashMap::new();

        previous.insert(self.rounded.to_vec(), 0);

        let mut i = 0usize;

        while i < 1000000000 {
            for dir in [
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
            ] {
                self.rounded = self.tilt(&self.rounded, dir);
                // self.rounded.sort();

                // println!("{:?}",self.rounded);
                // println!("after {dir:?}");
                // self.print();
                // println!("\n");
            }
            i += 1;
            if previous.contains_key(&self.rounded) {
                let period = i - previous.get(&self.rounded).unwrap();
                i += ((1000000000 - i) / period) * period;
            }
            previous.insert(self.rounded.clone(), i);
        }
    }

    fn print(&self) {
        println!("---print--");
        println!("{:?}", self.rounded);
        for y in 0..self.maxy {
            // for x in 0..self.maxx {
            //     println!("[{x},{y}] rounded : {:?}", self.rounded.contains(&[x, y]));
            //     println!("[{x},{y}] dubed : {:?}", self.cubed.contains(&[x, y]));
            // }
            println!(
                "{}",
                (0..self.maxx)
                    .map(|x| if self.cubed.contains(&[x, y]) {
                        '#'
                    } else if self.rounded.contains(&[x, y]) {
                        'O'
                    } else {
                        '.'
                    })
                    .join("")
            );
        }
    }
}
pub fn tune_parabol() {
    let input = include_str!("../resources/day14_scene.txt");
    let scene: Scene = input.parse().unwrap();

    let mut scene1 = scene.clone();
    scene1.rounded = scene1.tilt(&scene1.rounded, Direction::North);
    let weight = scene1.weight();
    println!("weight, {weight}");

    let mut scene2 = scene.clone();
    scene2.tune();
    let weight_tuned = scene2.weight();
    println!("weight_tuned, {weight_tuned}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    #[test]
    fn tst_aoc_example() {
        let input = indoc! {"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "};
        let mut scene: Scene = input.parse().unwrap();
        scene.rounded = scene.tilt(&scene.rounded, Direction::North);
        assert_eq!(136, scene.weight());

        scene.print();
        let mut scenenorth_west = scene.clone();
        scenenorth_west.rounded = scenenorth_west.tilt(&scenenorth_west.rounded, Direction::West);
        scenenorth_west.rounded.sort();

        println!("***scenenorth_west");

        scenenorth_west.print();
        println!("***scenenorth_west");

        let mut scene6: Scene = input.parse().unwrap();
        scene6.tune();
        assert_eq!(64, scene6.weight());
    }
}
