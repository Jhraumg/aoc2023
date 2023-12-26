use eyre::{eyre, Error};
use itertools::Itertools;
use std::str::FromStr;
use indoc::indoc;
use num::Integer;

#[derive(Debug, Copy, Clone, PartialEq,Eq,Hash )]
struct Hail {
    x: isize,
    y: isize,
    z: isize,
    vx: isize,
    vy: isize,
    vz: isize,
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
    fn pos(&self, t:isize)->(isize,isize,isize){
        (self.x+t*self.vx,self.y+t*self.vy,self.z+t*self.vz)
    }
    fn may_cross(hail1: &Hail, hail2: &Hail) -> Option<(f64, f64)> {
        ///
        /// x1+t1vx1 =x2+t2vx2 && y1+t1vy1=y2+t2vy2
        ///t1vx1-t2vx2 = x2-X1 && t1vy1-t2vy2=y2-y1
        ///   *vy2                      * -vx2
        /// t1(vx1*vy2-vy1*vx2)=vy2(x2-x1)-vx2(y2-y1)
        ///t1 =  vy2(x2-x1)-vx2(y2-y1) /(vx1*vy2-vy1*vx2)
        ///
        let &Hail{x:x1, y:y1,z:z1, vx:vx1,vy:vy1,vz:vz1}=hail1;
        let &Hail{x:x2, y:y2,z:z2, vx:vx2,vy:vy2,vz:vz2}=hail2;
        let (x1,y1,vx1,vy1)=(x1 as f64, y1 as f64, vx1 as f64, vy1 as f64);
        let (x2,y2,vx2,vy2)=(x2 as f64, y2 as f64, vx2 as f64, vy2 as f64);
        let discr = vx1 * vy2 - vy1 * vx2;
        if discr == 0.0 {
            return None;
        }

        let t1 = (vy2 * (x2 - x1) - vx2 * (y2 - y1)) / (discr);
        let t2 = if vx2 != 0.0 {
            (x1 + t1 * vx1 - x2) / vx2
        } else {
            (y1 + t1 * vy1 - y2) / vy2
        };
        if t1 < 0.0 || t2 < 0.0 {
            return None;
        }
        Some((x1 + t1 * vx1, y1 + t1 * vy1))
    }
    fn may_cross_in_testzone(hail1: &Hail, hail2: &Hail, testzone: (f64, f64)) -> bool {
        Self::may_cross(hail1, hail2)
            .map(|(x, y)| x >= testzone.0 && x <= testzone.1 && y >= testzone.0 && y <= testzone.1)
            .unwrap_or(false)
    }
}

fn split_all(hails:&[Hail])->isize{
    // for any point i, Z+Ti.VZ = Zi+Ti.VZi => Z = Zi +(VZi-VZ) => Z = Zi % (VZi-VZ)
    // we've already done that this year !
    let mut vz =0;
    let mut z = 0;

    'main: loop {
        vz += 1;
        z=0;

        let mut md = 1;
        'inner: for h in hails.iter().filter(|h| h.vz < vz) {
            let dv = vz -h.vz;
            for _ in 0..dv  {
                if (h.z-z ) %  dv == 0 {
                    md = md.lcm(&dv);
                    continue 'inner;
                }
                if z > isize::MAX -md {
                    continue 'main;
                }
                z += md;
            }
            continue 'main;
        }
        break;
    }

    let h0 = hails.first().unwrap();
    let h1 = hails.iter().nth(1).unwrap();
    let t0 = (z-h0.z)/(h0.vz- vz);
    let t1 = (z-h1.z)/(h1.vz- vz);

    // x+ti*vx = hi.x + ti*h.vx => vx*(t0-t1) = h0.x+t0*h0.vx -h1.x+t1*h1.vx
    let vx = (h0.x+t0* h0.vx -h1.x -t1*h1.vx)/(t0-t1);
    let vy = (h0.y+t0* h0.vy -h1.y -t1*h1.vy)/(t0-t1);
    let x= h0.x+t0* h0.vx -t0*vx;
    let y= h0.y+t0* h0.vy -t0*vy;

    x+y+z
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


    let sum=split_all(&hails);
    println!("x+y+z ={sum}");

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
        assert_eq!(47, split_all(&hails));
    }
}
