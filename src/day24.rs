use eyre::{eyre, Error};
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Hail {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl FromStr for Hail {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, v) = s.split_once('@').ok_or(eyre!("no separator in {s}"))?;
        let (x, y, z) = pos
            .split(',')
            .filter_map(|v| v.trim().parse().ok())
            .collect_tuple()
            .ok_or(eyre!("wrong pos {pos}"))?;
        let (vx, vy, vz) = v
            .split(',')
            .filter_map(|v| v.trim().parse().ok())
            .collect_tuple()
            .ok_or(eyre!("wrong speed {v}"))?;
        Ok(Self {
            x,
            y,
            z,
            vx,
            vy,
            vz,
        })
    }
}
impl Hail {
    fn may_cross(hail1: &Hail, hail2: &Hail) -> Option<(f64, f64)> {
        ///
        /// x1+t1vx1 =x2+t2vx2 && y1+t1vy1=y2+t2vy2
        ///t1vx1-t2vx2 = x2-X1 && t1vy1-t2vy2=y2-y1
        ///   *vy2                      * -vx2
        /// t1(vx1*vy2-vy1*vx2)=vy2(x2-x1)-vx2(y2-y1)
        ///t1 =  vy2(x2-x1)-vx2(y2-y1) /(vx1*vy2-vy1*vx2)
        ///
        let discr = hail1.vx * hail2.vy - hail1.vy * hail2.vx;
        if discr == 0.0 {
            return None;
        }

        let t1 = (hail2.vy * (hail2.x - hail1.x) - hail2.vx * (hail2.y - hail1.y)) / (discr);
        let t2 = if hail2.vx != 0.0 {
            (hail1.x + t1 * hail1.vx - hail2.x) / hail2.vx
        } else {
            (hail1.y + t1 * hail1.vy - hail2.y) / hail2.vy
        };
        if t1 < 0.0 || t2 < 0.0 {
            return None;
        }
        Some((hail1.x + t1 * hail1.vx, hail1.y + t1 * hail1.vy))
    }
    fn may_cross_in_testzone(hail1: &Hail, hail2: &Hail, testzone: (f64, f64)) -> bool {
        Self::may_cross(hail1, hail2)
            .map(|(x, y)| x >= testzone.0 && x <= testzone.1 && y >= testzone.0 && y <= testzone.1)
            .unwrap_or(false)
    }
}

/// Part 2
/// were looking for t such as AtaBtb // AtaCtc
///


pub fn split_snow() {
    let hails: Vec<Hail> = include_str!("../resources/day24_hails.txt")
        .lines()
        .filter_map(|l| l.parse().ok())
        .collect();

    let cross_count = hails
        .iter()
        .combinations(2)
        .filter(|v| Hail::may_cross_in_testzone(v[0], v[1], (200000000000000.0, 400000000000000.0)))
        .count();

    println!("potential cross count {cross_count}");
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    #[test]
    fn aoc_examples_work() {
        let hails: Vec<Hail> = indoc! {"
            19, 13, 30 @ -2,  1, -2
            18, 19, 22 @ -1, -1, -2
            20, 25, 34 @ -2, -2, -4
            12, 31, 28 @ -1, -2, -1
            20, 19, 15 @  1, -5, -3
        "}
        .lines()
        .filter_map(|l| l.parse().ok())
        .collect();

        assert_eq!(
            2,
            hails
                .iter()
                .combinations(2)
                .filter(|v| {
                    let cross = Hail::may_cross_in_testzone(v[0], v[1], (7.0, 27.0));
                    if cross {
                        println!(
                            "{:?} and {:?} cross at {:?}",
                            v[0],
                            v[1],
                            Hail::may_cross(v[0], v[1])
                        );
                    }
                    cross
                })
                .count()
        );
    }
}
