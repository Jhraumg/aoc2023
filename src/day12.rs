/// given (a,b,c,d) we need
/// (\.)*#{a}(\.)+#{b}(\.)+#{c}(\.)+#{d}(\.)*
use itertools::{repeat_n, Itertools};
use num::Integer;
use std::cmp::{max, min};
use rayon::prelude::*;
fn get_next_max_gap(record: &str, damaged: &[usize]) -> usize {
    let max_alea = record.len() + 1 - damaged.iter().sum::<usize>() - damaged.len();

    // were looking for the first #
    let mut first_damage =         record
        .as_bytes()
        .iter()
        .enumerate()
        .find(|(_, c)| **c as char == '#')
        .map(|(i, _)| i).unwrap_or_else(||record.len());


    if damaged.is_empty() && first_damage!=record.len() {
        // no solution anyway
        return 0;
    }
    if damaged.is_empty(){
        // should not happen
        return record.len();
    }

    // and correct it by considering damaged[0] and space after this first #
    let next_ok= first_damage+ record[first_damage..]
        .as_bytes()
        .iter()
        .enumerate()
        .find(|(_, c)| **c as char == '.')
        .map(|(i, _)| i)
        .unwrap_or_else(||record[first_damage..].len());
    if next_ok+1 < damaged[0]{

        _= first_damage.saturating_sub(damaged[0]-next_ok-1);
    }


    min(
        max_alea,
        first_damage
    )
}
fn count_unfolded_matches(record: &str, damaged: &[usize], is_init: bool) -> usize {
    if is_init {
        let record = repeat_n(record.to_string(), 5).join("?");
        let damaged: Vec<_> = (0..5).flat_map(|_| damaged.iter()).copied().collect();
        count_matches(&record, &damaged, true)
    } else {
        count_matches(&record, &damaged, is_init)
    }
}
fn count_matches(record: &str, damaged: &[usize], is_start: bool) -> usize {
    // if is_start {println!("-------------- {record} {damaged:?}");}
    let next_max_gap = get_next_max_gap(record, damaged);
    // println!("count matches({record},{damaged:?},{is_start:?}), {next_max_gap}");
    if damaged.len() == 0 {
        return if next_max_gap == record.len() { 1 } else { 0 };
    }
    (0..next_max_gap + 1)
        .map(|next_gap| {
            let next_gap = if is_start { next_gap } else { next_gap + 1 };
            if record.len() < next_gap + damaged[0] {
                return 0;
            }
            for j in 0..next_gap {
                if record.as_bytes()[j] != '.' as u8 && record.as_bytes()[j] != '?' as u8 {
                    // println!(". {record}[{i}] is not compatible with {gaps:?}/{damaged:?}");
                    return 0;
                }
            }
            for j in next_gap..next_gap + damaged[0] {
                if record.as_bytes()[j] != '#' as u8 && record.as_bytes()[j] != '?' as u8 {
                    // println!(". {record}[{i}] is not compatible with {gaps:?}/{damaged:?}");
                    return 0;
                }
            }
            //let gaps :Vec<usize>=gaps.iter().copied().chain(once(next_gap)).collect();
            count_matches(&record[next_gap + damaged[0]..], &damaged[1..], false)
            //, &gaps)
        })
        .sum()
}

fn sum_arrangments(inputs: &'static str, counter: impl Fn(&str, &[usize], bool) -> usize + std::marker::Send + std::marker::Sync) -> usize {
    inputs
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .collect_vec()
        .par_iter()
        .map(|(i,l)| {
            if i %10 == 0 {println!("{i} : {l}");}
            let (record, damaged) = l.split_once(" ").unwrap();
            let damaged: Vec<usize> = damaged.split(',').filter_map(|g| g.parse().ok()).collect();
            counter(record, &damaged, true)
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
