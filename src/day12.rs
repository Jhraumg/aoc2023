/// given (a,b,c,d) we need
/// (\.)*#{a}(\.)+#{b}(\.)+#{c}(\.)+#{d}(\.)*
use itertools::{repeat_n, Itertools};
use num::Integer;
use rayon::prelude::*;
use std::cmp::{max, min};
use std::iter::once;
fn get_next_max_gap(record: &str, damaged: &[usize]) -> (usize, usize) {
    if damaged.is_empty() {
        return if record.as_bytes().iter().all(|u| *u as char != '#') {
            (record.len(), record.len() + 1)
        } else {
            (0, 0)
        };
    }

    let garanted_ending_ok = record
        .as_bytes()
        .iter()
        .rev()
        .take_while(|u| **u as char == '.')
        .count();
    let remaining_minimum_len =
        garanted_ending_ok + damaged.iter().sum::<usize>() + damaged.len() - 1;
    let number_of_ok_min = record
        .as_bytes()
        .iter()
        .take_while(|u| **u as char == '.')
        .count();
    if number_of_ok_min + remaining_minimum_len > record.len() {
        return (0, 0);
    }
    let max_alea = record.len() - remaining_minimum_len;

    let mut number_of_ok_max = record
        .as_bytes()
        .iter()
        .take_while(|u| **u as char != '#')
        .count();

    // were looking for the first #
    let mut first_damage = record
        .as_bytes()
        .iter()
        .find_position(|c| **c as char == '#')
        .map(|(i, _)| i);

    // and correct it by considering damaged[0] and space after this first #
    let number_of_damaged_max = first_damage
        .map(|fd| {
            record[fd..]
                .as_bytes()
                .iter()
                .take_while(|u| **u as char != '.')
                .count()
        })
        .unwrap_or(0);
    if number_of_damaged_max < damaged[0] {
        _ = number_of_ok_max.saturating_sub(damaged[0] - number_of_damaged_max);
    }

    // println!("{record} => ({}..{})",number_of_ok_min,min(max_alea, number_of_ok_max)+1);
    (number_of_ok_min, min(max_alea, number_of_ok_max) + 1)
}
fn count_unfolded_matches(record: &str, damaged: &[usize], previous: &[usize]) -> usize {
    if previous.is_empty() {
        let record = repeat_n(record.to_string(), 5).join("?");
        let damaged: Vec<_> = (0..5).flat_map(|_| damaged.iter()).copied().collect();
        count_matches(&record, &damaged, previous)
    } else {
        count_matches(&record, &damaged, previous)
    }
}
fn count_matches(record: &str, damaged: &[usize], previous: &[usize]) -> usize {
    // if is_start {println!("-------------- {record} {damaged:?}");}
    let (min_next_gap, max_next_gap) = get_next_max_gap(record, damaged);
    // println!("count matches({record},{damaged:?},{is_start:?}), {next_max_gap}");
    if min_next_gap >= max_next_gap {
        return 0;
    }
    let min_next_gap = if !previous.is_empty() {
        max(1, min_next_gap)
    } else {
        min_next_gap
    };
    if damaged.len() == 0 {
        return if min_next_gap <= record.len() && record.len() <= max_next_gap {
            1
        } else {
            0
        };
    }
    (min_next_gap..max_next_gap)
        .map(|next_gap| {
            if record.len() < next_gap + damaged[0] {
                // should not be possible
                return 0;
            }
            if record[next_gap..next_gap + damaged[0]]
                .as_bytes()
                .iter()
                .any(|u| *u as char == '.')
            {
                return 0;
            }
            if record.len() == next_gap + damaged[0] {
                return if damaged.len() == 1 { 1 } else { 0 };
            }
            if record.as_bytes()[next_gap + damaged[0]] as char == '#' {
                return 0;
            }
            let current: Vec<_> = previous
                .iter()
                .chain(once(&next_gap))
                .chain(once(&damaged[0]))
                .copied()
                .collect();
            count_matches(&record[next_gap + damaged[0]..], &damaged[1..], &current)
        })
        .sum()
}

fn sum_arrangments(
    inputs: &'static str,
    counter: impl Fn(&str, &[usize], &[usize]) -> usize + std::marker::Send + std::marker::Sync,
) -> usize {
    inputs
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .collect_vec()
        .par_iter()
        .map(|(i, l)| {
            let (record, damaged) = l.split_once(" ").unwrap();
            let damaged: Vec<usize> = damaged.split(',').filter_map(|g| g.parse().ok()).collect();
            let result = counter(record, &damaged, &[]);
            println!("{i}  {record} => {result}");
            result
        })
        .sum()
}
pub fn arrange_springs() {
    let input = include_str!("../resources/day12_records.txt");
    let sum = sum_arrangments(input, count_matches);
    println!("sum {sum}");

    let sum_unfold = sum_arrangments(input, count_unfolded_matches);
    println!("sum_unfold {sum_unfold}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn aoc_example_works() {
        // assert_eq!(1, sum_arrangments("???.### 1,1,3", count_matches));
        assert_eq!(4, sum_arrangments(".??..??...?##. 1,1,3", count_matches));
        assert_eq!(1, sum_arrangments("?#?#?#?#?#?#?#? 1,3,1,6", count_matches));
        assert_eq!(1, sum_arrangments("????.#...#... 4,1,1", count_matches));
        assert_eq!(
            4,
            sum_arrangments("????.######..#####. 1,6,5", count_matches)
        );
        assert_eq!(10, sum_arrangments("?###???????? 3,2,1", count_matches));
        let input = indoc! {"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        "};
        assert_eq!(21, sum_arrangments(input, count_matches));

        assert_eq!(1, sum_arrangments("???.### 1,1,3", count_unfolded_matches));
        assert_eq!(
            16384,
            sum_arrangments(".??..??...?##. 1,1,3", count_unfolded_matches)
        );
        assert_eq!(
            1,
            sum_arrangments("?#?#?#?#?#?#?#? 1,3,1,6", count_unfolded_matches)
        );
        assert_eq!(
            16,
            sum_arrangments("????.#...#... 4,1,1", count_unfolded_matches)
        );
        assert_eq!(
            2500,
            sum_arrangments("????.######..#####. 1,6,5", count_unfolded_matches)
        );
        assert_eq!(
            506250,
            sum_arrangments("?###???????? 3,2,1", count_unfolded_matches)
        );
    }
}
//4382 too low
//
