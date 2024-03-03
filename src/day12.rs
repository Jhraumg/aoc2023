use ahash::{HashMap, HashMapExt};
use itertools::{repeat_n, Itertools};
use std::cmp::min;

use num::integer::binomial;

fn count_unfolded_matches<'a, 'b>(record: &'a str, damaged: &'b [usize]) -> usize {
    let record = repeat_n(record, 5).join("?");
    let damaged: Vec<_> = (0..5).flat_map(|_| damaged.iter()).copied().collect();
    let mut memo = HashMap::new();

    inner_compute_matches(&mut memo, record.as_bytes(), &damaged)
}

fn minimum_len(damaged: &[usize]) -> usize {
    if damaged.is_empty() {
        0
    } else {
        damaged.iter().sum::<usize>() + damaged.len() - 1
    }
}

fn count_max_damaged_seq_fitting(damaged: &[usize], gap_len: usize) -> usize {
    let mut sum_with_gaps = 0;
    let mut result = 0;
    for d in damaged {
        sum_with_gaps += *d + if result > 0 { 1 } else { 0 };
        if sum_with_gaps > gap_len {
            break;
        }
        result += 1;
    }
    result
}

fn compute_matches<'a, 'b>(record: &'a  str, damaged: &'b [usize]) -> usize {
    let mut memo = HashMap::new();
    inner_compute_matches(&mut memo, record.as_bytes(), damaged)
}

const OK: u8 = b'.';
const ANY: u8 = b'?';
const DMG: u8 = b'#';

 fn inner_compute_matches<'a, 'b> (memo : &mut HashMap<(&'a[u8],&'b[usize]),usize>, record: &'a[u8], damaged: &'b[usize]) -> usize {

    if let Some(result) = memo.get(&(record, damaged)) {
        return *result;
    }
    // let's trim record from its OK chars
    let forced_ok = record.iter().take_while(|c| **c == OK).count();
    let record = &record[forced_ok..];
    let forced_ok = record.iter().rev().take_while(|c| **c == OK).count();
    let record = &record[..record.len() - forced_ok];
    if minimum_len(damaged) > record.len() {
        memo.insert((record,damaged), 0);
        return 0;
    }

    if damaged.is_empty() {
        let result =  if record.contains(&DMG) { 0 } else { 1 };
        memo.insert((record,damaged), result);
        return result;

    }

    let leading_choices = record.iter().take_while(|c| **c == ANY).count();
    let next_no_choice = record[leading_choices..].first();
    let result = match next_no_choice {
        None => {
            let margin =record.len()- minimum_len(damaged);
            binomial(damaged.len()+margin,margin)
        },
        Some(&OK) => {
            // let's try to spread the damaged around this ok point
            let nb_damaged_max= count_max_damaged_seq_fitting(damaged, leading_choices);
             (0..=nb_damaged_max).map(|i| {
                let margin = leading_choices - minimum_len(&damaged[0..i]);
                binomial(i+margin,margin)*inner_compute_matches(memo, &record[leading_choices+1..],&damaged[i..])
            }).sum()
        }
        _ /* DaMaGed */=> {
            // there must be one damaged on this spot, the others are on each sides
            let nb_damaged_max= min(count_max_damaged_seq_fitting(damaged, leading_choices), damaged.len()-1);
            (0..=nb_damaged_max).map(|i|{
                // if i >= damaged.len() {return 0;} // there are no damaged left, but still a '#'
                //
                let current=damaged[i];

                let record_len = record.len();
                let min_reserved = if current+leading_choices >= record_len{1+ current+leading_choices - record_len } else {1};

                (min_reserved..=min(current, leading_choices+1)).map(|reserved| {

                    // record.len() - leading_choices -1 + reserved >= current  => reserved >= current +1 + leading_choices - len
                    let current_record=&record[leading_choices+1-reserved..];
                    //if current_record.len()<current {return 0;}//not enough place
                    if current_record[..current].contains(&OK){return 0;} // some OK on damaged[i] place

                    let solutions_before = if i==0 {1} else { inner_compute_matches(memo, &record[..leading_choices.saturating_sub(reserved)], &damaged[..i]) };
                    if current_record.len()==current { // no place after
                         if damaged.len()==i+1 {solutions_before} else { /* no place left for the remaining  damaged */ 0  }
                    }else if current_record[current..].starts_with(&[DMG]) {  0} // cannot have a '#' just after current
                    else {
                        let solutions_after = inner_compute_matches(memo, &current_record[current + 1..], &damaged[i + 1..]);

                        solutions_before * solutions_after
                    }
                }).sum::<usize>()
            }).sum()
        }
    };
    memo.insert((record,damaged), result);
    result
}

fn sum_arrangements(
    inputs: &'static str,
    counter: impl Fn(&str, &[usize]) -> usize + Send + Sync,
) -> usize {
    inputs
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        // .par_bridge()
        .map(|(_i, l)| {
            let (record, damaged) = l.split_once(' ').unwrap();
            let damaged: Vec<usize> = damaged.split(',').filter_map(|g| g.parse().ok()).collect();
            counter(record, &damaged)
        })
        .sum()
}

pub fn arrange_springs() {
    let input = include_str!("../resources/day12_records.txt");
    let sum = sum_arrangements(input, compute_matches);
    println!("sum {sum}");

    let sum_unfold = sum_arrangements(input, count_unfolded_matches);
    println!("sum_unfold {sum_unfold}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn aoc_example_works() {
        assert_eq!(
            150,
            sum_arrangements("?????????###???????? 2,1,3,2,1", compute_matches)
        );
        assert_eq!(1, sum_arrangements("..?##. 3", compute_matches));

        assert_eq!(10, sum_arrangements("?###???????? 3,2,1", compute_matches));
        assert_eq!(4, sum_arrangements(".??..??...?##. 1,1,3", compute_matches));
        assert_eq!(1, sum_arrangements("?### 3", compute_matches));
        assert_eq!(1, sum_arrangements("#???. 3", compute_matches));
        assert_eq!(1, sum_arrangements("???. 3", compute_matches));

        assert_eq!(
            1,
            sum_arrangements("?#?#?#?#?#?#?#? 1,3,1,6", compute_matches)
        );
        assert_eq!(1, sum_arrangements("????.#...#... 4,1,1", compute_matches));
        assert_eq!(
            4,
            sum_arrangements("????.######..#####. 1,6,5", compute_matches)
        );
        let input = indoc! {"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        "};
        assert_eq!(21, sum_arrangements(input, compute_matches));

        assert_eq!(1, sum_arrangements("???.### 1,1,3", compute_matches));
        assert_eq!(
            sum_arrangements("#.?#.?#.?#.?#. 1,1,1,1,1", compute_matches),
            sum_arrangements("#. 1", count_unfolded_matches)
        );
        assert_eq!(
            16384,
            sum_arrangements(".??..??...?##. 1,1,3", count_unfolded_matches)
        );
        assert_eq!(
            1,
            sum_arrangements("?#?#?#?#?#?#?#? 1,3,1,6", count_unfolded_matches)
        );
        assert_eq!(
            16,
            sum_arrangements("????.#...#... 4,1,1", count_unfolded_matches)
        );
        assert_eq!(
            2500,
            sum_arrangements("????.######..#####. 1,6,5", count_unfolded_matches)
        );
        assert_eq!(
            506250,
            sum_arrangements("?###???????? 3,2,1", count_unfolded_matches)
        );
    }
}
