use ahash::{AHashMap, AHashSet};
use rayon::prelude::*;

fn read_connection<'i>(input: &'i str) -> AHashMap<&'i str, AHashSet<&'i str>> {
    let mut result: AHashMap<_, _> = input
        .lines()
        .map(|l| {
            let (from, tos) = l.split_once(':').unwrap();
            let from = from.trim();
            let tos: AHashSet<_> =
                tos.split(' ').map(|name| name.trim()).filter(|n| !n.is_empty()).collect();
            (from, tos)
        })
        .collect();
    let transpose: AHashMap<&'i str, AHashSet<&'i str>> = result
        .iter()
        .flat_map(|(from, tos)| tos.iter().map(move |to| (*to, *from)))
        .fold(Default::default(), |mut acc, (to, from)| {
            acc.entry(to)
                .and_modify(|froms| {
                    froms.insert(from);
                })
                .or_insert([from].into_iter().collect());
            acc
        });
    for (tk, tvs) in &transpose {
        result
            .entry(*tk)
            .and_modify(|tos| {
                for v in tvs {
                    tos.insert(v);
                }
            })
            .or_insert(tvs.clone());
    }
    result
}
fn prod_groups(connections: &AHashMap<&str, AHashSet<&str>>) -> (usize, usize) {
    let mut count = 0;
    let mut prod = 1;
    let mut keys: AHashSet<_> = connections.keys().copied().collect();
    while !keys.is_empty() {
        let key = keys.iter().next().copied().unwrap();
        let mut group: AHashSet<_> = [key].into_iter().collect();
        loop {
            let new_group: AHashSet<_> = group
                .iter()
                .flat_map(|k| connections.get(k).unwrap().iter().copied())
                .filter(|c| !group.contains(c))
                .collect();
            if new_group.is_empty() {
                break;
            }
            for k in new_group.into_iter() {
                group.insert(k);
            }
        }
        count += 1;
        prod *= group.len();
        keys = keys.difference(&group).copied().collect();
    }
    (count, prod)
}
fn distance(
    connections: &AHashMap<&str, AHashSet<&str>>,
    start: &str,
    stop: &str,
) -> Option<usize> {
    let mut visited: AHashSet<&str> = Default::default();
    visited.reserve(connections.len());
    let mut d = 0usize;
    visited.insert(start);
    let mut current: AHashSet<_> = [start].into_iter().collect();
    loop {
        let next: AHashSet<_> = current
            .iter()
            .flat_map(|c| connections.get(c).unwrap().iter().filter(|o| !visited.contains(*o)))
            .copied()
            .collect();
        if next.is_empty() {
            break;
        }
        d += 1;
        if next.contains(&stop) {
            return Some(d);
        }
        for n in next.iter() {
            visited.insert(n);
        }
        current = next
    }

    None
}

fn distance_without_direct_link(
    connections: &AHashMap<&str, AHashSet<&str>>,
    start: &str,
    stop: &str,
) -> Option<usize> {
    let mut cc = connections.clone();
    cc.get_mut(&start).unwrap().remove(&stop);
    cc.get_mut(&stop).unwrap().remove(&start);
    distance(&cc, start, stop)
}

fn split_in_two(connections: &AHashMap<&str, AHashSet<&str>>) -> usize {
    // targeting segments which egdes are the most far appart when the segment itself is removed
    let mut cc = connections.clone();
    for _ in 0..3 {
        if let Some((k, o)) = cc
            .iter()
            .flat_map(|(k, v)| v.iter().map(|o| (*k, *o)))
            .par_bridge()
            .fold(
                || (0, None),
                |(d, edge), (k, o)| {
                    let new_d = distance_without_direct_link(&cc, k, o).unwrap_or(usize::MAX);
                    if new_d > d {
                        (new_d, Some((k, o)))
                    } else {
                        (d, edge)
                    }
                },
            )
            .reduce(
                || (0, None),
                |(d1, e1), (d2, e2)| if d1 > d2 { (d1, e1) } else { (d2, e2) },
            )
            .1
        {
            cc.get_mut(k).unwrap().remove(o);
            cc.get_mut(o).unwrap().remove(k);
        }
    }

    let (c, p) = prod_groups(&cc);
    assert_eq!(2, c, "should have been split in 2 groups, not {c}");
    p
}

pub fn fix_machine() {
    let connections = read_connection(include_str!("../resources/day25_connections.txt"));

    let prod = split_in_two(&connections);
    println!("product {prod}");
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn aoc_example_works() {
        let connections = read_connection(indoc! {"
            jqt: rhn xhk nvd
            rsh: frs pzl lsr
            xhk: hfx
            cmg: qnr nvd lhk bvb
            rhn: xhk bvb hfx
            bvb: xhk hfx
            pzl: lsr hfx nvd
            qnr: nvd
            ntq: jqt hfx bvb xhk
            nvd: lhk
            lsr: lhk
            rzs: qnr cmg lsr rsh
            frs: qnr lhk lsr
        "});
        assert_eq!((1, connections.len()), prod_groups(&connections));
        assert_eq!(54, split_in_two(&connections));
    }
}
