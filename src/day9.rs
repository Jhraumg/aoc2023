use itertools::Itertools;
fn build_history(vals: &[i64]) -> Vec<Vec<i64>> {
    let mut history: Vec<Vec<i64>> = vec![vals.into()];
    loop {
        let vals = &history[history.len() - 1];
        let next: Vec<_> = vals
            .iter()
            .tuple_windows::<(&i64, &i64)>()
            .map(|(n1, n2)| *n2 - *n1)
            .collect();
        let all_zeroes = next.iter().all(|n| *n == 0);
        history.push(next);
        if all_zeroes {
            break;
        };
    }
    history
}
fn next_value(vals: &[i64]) -> i64 {
    build_history(vals)
        .iter()
        .rev()
        .map(|vals| vals.last().unwrap())
        .fold(0, |prev, next| *next + prev)
}
fn sum_next_values(current_values: &[Vec<i64>]) -> i64 {
    current_values
        .iter()
        .filter_map(|vals| {
            if !vals.is_empty() {
                Some(next_value(vals))
            } else {
                None
            }
        })
        .sum()
}

fn previous_value(vals: &[i64]) -> i64 {
    build_history(vals)
        .iter()
        .rev()
        .map(|vals| vals.first().unwrap())
        .fold(0, |prev, next| *next - prev)
}

fn sum_previous_values(current_values: &[Vec<i64>]) -> i64 {
    current_values
        .iter()
        .filter_map(|vals| {
            if !vals.is_empty() {
                Some(previous_value(vals))
            } else {
                None
            }
        })
        .sum()
}

fn read_obervations(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| l.split(' ').filter_map(|n| n.parse().ok()).collect())
        .filter(|v: &Vec<i64>| !v.is_empty())
        .collect()
}
pub fn observe_oasis() {
    let input = include_str!("../resources/day9_oasis_obervations.txt");
    let current_values = read_obervations(input);

    let next_val = sum_next_values(&current_values);
    println!("next_val {next_val}");

    let prev_val = sum_previous_values(&current_values);
    println!("prev_val {prev_val}");
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    #[test]
    fn test_aoc_example_works() {
        let input = indoc! {"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
        "};
        let current_values = read_obervations(input);
        assert_eq!(114, sum_next_values(&current_values));
        assert_eq!(2, sum_previous_values(&current_values));
    }
}
