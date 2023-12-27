use itertools::Itertools;
use std::cmp::{max, min};
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Brick {
    base: [(usize, usize); 2],
    height: usize,
    init_z: usize,
}
impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p1, p2) = s.trim().split_once('~').unwrap();
        let (x1, y1, z1) =
            p1.split(',').filter_map(|v| v.parse::<usize>().ok()).collect_tuple().unwrap();
        let (x2, y2, z2) =
            p2.split(',').filter_map(|v| v.parse::<usize>().ok()).collect_tuple().unwrap();
        let init_z = min(z1, z2);
        let base = [(min(x1, x2), min(y1, y2)), (max(x1, x2), max(y1, y2))];
        let height = 1 + z1.abs_diff(z2);

        Ok(Self {
            base,
            height,
            init_z,
        })
    }
}
impl Brick {
    fn intersect(&self, other: &Self) -> bool {
        let [(x1, y1), (x2, y2)] = self.base;
        let [(ox1, oy1), (ox2, oy2)] = other.base;
        max(x1, ox1) <= min(x2, ox2) && max(y1, oy1) <= min(y2, oy2)
    }
}

#[derive(Debug, Default)]
struct Stack {
    bricks: Vec<(usize, Brick)>,
}

impl FromStr for Stack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let falling: Vec<Brick> = s
            .lines()
            .filter_map(|l| l.parse::<Brick>().ok())
            .sorted_by(|b1, b2| b1.init_z.cmp(&b2.init_z))
            .collect();
        let mut bricks = vec![];

        for b in falling {
            Self::stack(&mut bricks, b);
        }
        Ok(Self { bricks })
    }
}
impl Stack {
    fn stack(bricks: &mut Vec<(usize, Brick)>, b: Brick) {
        let height = bricks
            .iter()
            .filter_map(|(h, stacked)| {
                if stacked.intersect(&b) {
                    Some(*h + stacked.height)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(1);
        bricks.push((height, b));
    }

    fn count_removeable(&self) -> usize {
        self.bricks
            .iter()
            .filter(|(h, b)| {
                let level = *h + b.height;
                let supported: Vec<_> = self
                    .bricks
                    .iter()
                    .filter_map(|(stacked_h, stacked_b)| {
                        if *stacked_h == level && b.intersect(stacked_b) {
                            Some(stacked_b)
                        } else {
                            None
                        }
                    })
                    .collect();
                supported.into_iter().all(|sb| {
                    self.bricks
                        .iter()
                        .filter(|(bh, base)| *bh + base.height == level && base.intersect(sb))
                        .count()
                        > 1
                })
            })
            .count()
    }

    fn sum_falling(&self) -> usize {
        self.bricks
            .iter()
            .enumerate()
            .map(|(i, init)| {
                let mut falling = vec![false; self.bricks.len()];
                falling[i] = true;

                let mut current_base = init.0;
                loop {
                    let removed: Vec<_> = self.bricks[i + 1..]
                        .iter()
                        .enumerate()
                        .filter(|(_, (df_h, _))| *df_h > current_base)
                        .filter(|(j, _)| !falling[*j])
                        .filter(|(_, (df_h, df))| {
                            self.bricks
                                .iter()
                                .enumerate()
                                .filter(|(_, (bh, bb))| {
                                    *bh + bb.height == *df_h && bb.intersect(df)
                                })
                                .all(|(k, _)| falling[k])
                        })
                        .map(|(j, (h, _))| (j, h))
                        .collect();
                    if removed.is_empty() {
                        break;
                    }
                    current_base = removed.iter().map(|(_, h)| **h).min().unwrap();
                    for (j, _) in removed {
                        // println!("removing {start_idx} => {df:?} falling");
                        falling[j] = true;
                    }
                }
                falling.len() - 1
            })
            .sum()
    }
}

pub fn dispatch_sand() {
    let input = include_str!("../resources/day22_bricks.txt");
    let stack: Stack = input.parse().unwrap();
    let removeable_count = stack.count_removeable();

    println!("removeable bricks {removeable_count}");
    let sum_falling = stack.sum_falling();
    println!("sum falling {sum_falling}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    #[test]
    fn aoc_example_works() {
        let input = indoc! {"
            1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9
        "};
        let stack: Stack = input.parse().unwrap();
        assert_eq!(5, stack.count_removeable());
        assert_eq!(7, stack.sum_falling());
    }
}
