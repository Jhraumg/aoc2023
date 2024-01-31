use ahash::AHashMap;
use itertools::Itertools;
use std::cmp::min;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Dir {
    North,
    West,
    South,
    East,
    StartStop,
}
struct HeatLossMap(Vec<Vec<usize>>);

impl FromStr for HeatLossMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .filter(|l| !l.is_empty())
                .map(|l| l.chars().filter_map(|c| c.to_digit(10).map(|d| d as usize)).collect())
                .collect(),
        ))
    }
}
impl Display for HeatLossMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for l in &self.0 {
            for v in l {
                f.write_fmt(format_args!("{v}"))?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Move {
    x: usize,
    y: usize,
    dir: Dir,
    l: usize,
}

// fn get_next_moves(heatloss_map: &[Vec<usize>], current:Move) -> [Option<Move>;3]{
//     let maxx = heatloss_map[0].len();
//     let maxy= heatloss_map.len();
//     let Move{x,y,dir,l}=current;
//     match dir {
//         Dir::North => {[if l < 3 && y >0 {Some(Move{x,y:y-1,dir:Dir::North,l:l+1})}else{None},
//                         if x >0 {Some(Move{x:x-1,y,dir:Dir::West,l:1})}else{None},
//                         if x+1<maxx {Some(Move{x:x+1,y,dir:Dir::East,l:1})}else { None }]}
//         Dir::West => {
//             [if l < 3 && x >0 {Some(Move{x:x-1,y,dir:Dir::West,l:l+1})}else{None},
//                 if y >0 {Some(Move{x,y:y-1,dir:Dir::North,l:1})}else{None},
//                 if y+1<maxy {Some(Move{x,y:y+1,dir:Dir::South,l:1})}else { None }]
//         }
//         Dir::South => {[if l < 3 && y+1 <maxy {Some(Move{x,y:y+1,dir:Dir::South,l:l+1})}else{None},
//                         if x >0 {Some(Move{x:x-1,y,dir:Dir::West,l:1})}else{None},
//                         if x+1<maxx {Some(Move{x:x+1,y,dir:Dir::East,l:1})}else { None }]}
//         Dir::East => {
//             [if l < 3 && x+1 < maxx {Some(Move{x:x+1,y,dir:Dir::East,l:l+1})}else{None},
//                 if y >0 {Some(Move{x,y:y-1,dir:Dir::North,l:1})}else{None},
//                 if y+1<maxy {Some(Move{x,y:y+1,dir:Dir::South,l:1})}else { None }]
//
//
//         }
//         Dir::StartStop => {if current.x==0 && current.y==0 {[Some(Move{x:1,y:0,dir:Dir::East,l:1}),Some(Move{x:0,y:1,dir:Dir::South,l:1}),None]}else { panic!("None move on {current:?}") }}
//     }
// }

fn get_next_moves(
    heatloss_map: &[Vec<usize>],
    current: Move,
    min_len: usize,
    max_len: usize,
) -> [Option<Move>; 3] {
    let maxx = heatloss_map[0].len();
    let maxy = heatloss_map.len();
    let Move { x, y, dir, l } = current;
    match dir {
        Dir::North => [
            (l < max_len && y > 0).then_some(Move {
                x,
                y: y - 1,
                dir: Dir::North,
                l: l + 1,
            }),
            (l >= min_len && x > 0).then_some(Move {
                x: x - 1,
                y,
                dir: Dir::West,
                l: 1,
            }),
            (l >= min_len && x + 1 < maxx).then_some(Move {
                x: x + 1,
                y,
                dir: Dir::East,
                l: 1,
            }),
        ],
        Dir::West => [
            (l < max_len && x > 0).then_some(Move {
                x: x - 1,
                y,
                dir: Dir::West,
                l: l + 1,
            }),
            (l >= min_len && y > 0).then_some(Move {
                x,
                y: y - 1,
                dir: Dir::North,
                l: 1,
            }),
            (l >= min_len && y + 1 < maxy).then_some(Move {
                x,
                y: y + 1,
                dir: Dir::South,
                l: 1,
            }),
        ],
        Dir::South => [
            (l < max_len && y + 1 < maxy).then_some(Move {
                x,
                y: y + 1,
                dir: Dir::South,
                l: l + 1,
            }),
            (l >= min_len && x > 0).then_some(Move {
                x: x - 1,
                y,
                dir: Dir::West,
                l: 1,
            }),
            (l >= min_len && x + 1 < maxx).then_some(Move {
                x: x + 1,
                y,
                dir: Dir::East,
                l: 1,
            }),
        ],
        Dir::East => [
            (l < max_len && x + 1 < maxx).then_some(Move {
                x: x + 1,
                y,
                dir: Dir::East,
                l: l + 1,
            }),
            (l >= min_len && y > 0).then_some(Move {
                x,
                y: y - 1,
                dir: Dir::North,
                l: 1,
            }),
            (l >= min_len && y + 1 < maxy).then_some(Move {
                x,
                y: y + 1,
                dir: Dir::South,
                l: 1,
            }),
        ],
        Dir::StartStop => {
            if current.x == 0 && current.y == 0 {
                [
                    Some(Move {
                        x: 1,
                        y: 0,
                        dir: Dir::East,
                        l: 1,
                    }),
                    Some(Move {
                        x: 0,
                        y: 1,
                        dir: Dir::South,
                        l: 1,
                    }),
                    None,
                ]
            } else {
                panic!("None move on {current:?}")
            }
        }
    }
}

fn get_minimal_heat_loss(heatloss_map: &[Vec<usize>], min_len: usize, max_len: usize) -> usize {
    let maxx = heatloss_map[0].len();
    let maxy = heatloss_map.len();
    let end: Move = Move {
        x: maxx - 1,
        y: maxy - 1,
        dir: Dir::StartStop,
        l: 42,
    };

    let mut moves_to_minimal_loss: AHashMap<Move, usize> = AHashMap::with_capacity(maxx * maxy * 4);

    let mut current_moves: Vec<_> = get_next_moves(
        heatloss_map,
        Move {
            x: 0,
            y: 0,
            dir: Dir::StartStop,
            l: 42,
        },
        min_len,
        max_len,
    )
    .into_iter()
    .flatten()
    .collect();
    for m in &current_moves {
        moves_to_minimal_loss.insert(*m, heatloss_map[m.y][m.x]);
    }
    while !current_moves.is_empty() {
        let mut new_moves = Vec::with_capacity(current_moves.len());
        for current_m in &current_moves {
            let current_loss = moves_to_minimal_loss.get(current_m).copied().unwrap();
            for next_m in
                get_next_moves(heatloss_map, *current_m, min_len, max_len).iter().flatten()
            {
                let loss = current_loss + heatloss_map[next_m.y][next_m.x];
                if next_m.x == maxx - 1 && next_m.y == maxy - 1 {
                    if next_m.l < min_len {
                        continue;
                    }
                    moves_to_minimal_loss
                        .entry(end)
                        .and_modify(|l| *l = min(*l, loss))
                        .or_insert(loss);
                    // goal reached , no need to get next step
                } else {
                    if let Some(l) = moves_to_minimal_loss.get(next_m) {
                        if *l <= loss {
                            // a better move is already registered
                            continue;
                        }
                    }
                    moves_to_minimal_loss
                        .entry(*next_m)
                        .and_modify(|l| *l = min(*l, loss))
                        .or_insert(loss);
                    new_moves.push(*next_m);
                }
            }
        }
        current_moves = new_moves.into_iter().unique().collect();
    }

    *moves_to_minimal_loss.get(&end).unwrap()
}
pub fn carry_lava() {
    let heatloss_map: HeatLossMap =
        include_str!("../resources/day17_heatloss.txt").parse().unwrap();
    let minimum_loss = get_minimal_heat_loss(&heatloss_map.0, 1, 3);
    println!("minimum los {minimum_loss}");
    let minimum_loss_ultra = get_minimal_heat_loss(&heatloss_map.0, 4, 10);
    println!("minimum los {minimum_loss_ultra}");
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    #[test]
    fn aoc_examples_works() {
        let heat_map: HeatLossMap = indoc! {"
            1111
            1111
            1111
        "}
        .parse()
        .unwrap();
        println!("{heat_map}\n");
        assert_eq!(5, get_minimal_heat_loss(&heat_map.0, 1, 3));

        let heat_map: HeatLossMap = indoc! {"
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533
        "}
        .parse()
        .unwrap();
        println!("{heat_map}\n");
        assert_eq!(102, get_minimal_heat_loss(&heat_map.0, 1, 3));
        assert_eq!(94, get_minimal_heat_loss(&heat_map.0, 4, 10));
        let heat_map: HeatLossMap = indoc! {"
            111111111111
            999999999991
            999999999991
            999999999991
            999999999991
        "}
        .parse()
        .unwrap();
        assert_eq!(71, get_minimal_heat_loss(&heat_map.0, 4, 10));
    }
}
