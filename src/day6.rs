use indoc::indoc;
use itertools::Itertools;

/// distance parcourue :
/// duréee totale : d
/// tacc : durée acc
/// tmv : durée mouvement = T - tacc
///Distance parcourue = tacc*(T-tacc) = T*tacc-tacc²
/// si on n'est pas bourrin on résoud  tacc²-T*tacc +Drace == 0, et on se place entre les racines
/// delta = T²-4*Drace
/// (T+sqrt(delta))/2 et (T-sqrt(delta))/2  nombre de valeur : sqrt(delta)

fn compute_margins(time: u64, distance: u64) -> u64 {
    let delta: f64 = (time * time - 4 * distance) as f64;
    assert!(delta > 0.0);
    let ftime = time as f64;

    let min_val = ((ftime - delta.sqrt()) / 2.0).ceil() as u64;
    let max_val = ((ftime + delta.sqrt()) / 2.0).floor() as u64;

    // remove extremities in case of exact match
    1 + (if time * max_val - max_val * max_val > distance {
        max_val
    } else {
        max_val - 1
    }) - (if time * min_val - min_val * min_val > distance {
        min_val
    } else {
        min_val + 1
    })
}

fn multiply_race_margins(input: &str) -> u64 {
    let mut lines = input.lines().filter(|l| !l.is_empty());
    let (_, times) = lines.next().unwrap().split_once(':').unwrap();
    let times: Vec<u64> = times.split(' ').filter_map(|w| w.parse().ok()).collect();

    let (_, distances) = lines.next().unwrap().split_once(':').unwrap();
    let distances: Vec<u64> = distances
        .split(' ')
        .filter_map(|w| w.parse().ok())
        .collect();

    times
        .into_iter()
        .zip(distances)
        .map(|(t, d)| compute_margins(t, d))
        .product()
}

fn compute_single_race_margins(input: &str) -> u64 {
    let mut lines = input.lines().filter(|l| !l.is_empty());
    let (_, times) = lines.next().unwrap().split_once(':').unwrap();
    let time: u64 = times
        .split(' ')
        .filter(|w| !w.is_empty())
        .join("")
        .parse()
        .unwrap();

    let (_, distances) = lines.next().unwrap().split_once(':').unwrap();
    let distance: u64 = distances
        .split(' ')
        .filter(|w| !w.is_empty())
        .join("")
        .parse()
        .unwrap();

    compute_margins(time, distance)
}

pub fn race_boat() {
    let input = indoc! {"
        Time:        40     81     77     72
        Distance:   219   1012   1365   1089
    "};
    let margins_product = multiply_race_margins(input);
    println!("margins product {margins_product}");

    let single_race_margin = compute_single_race_margins(input);
    println!("single race margin {single_race_margin}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_aoc_works() {
        let input = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};
        assert_eq!(288, multiply_race_margins(input));
        assert_eq!(71503, compute_single_race_margins(input));
    }
}
