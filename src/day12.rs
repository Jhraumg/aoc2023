use std::cmp::min;
use itertools::{repeat_n, Itertools};
use rayon::prelude::*;

use num::integer::binomial;

fn count_unfolded_matches(
    record: &str,
    damaged: &[usize],
) -> usize {

    let record = repeat_n(record.to_string(), 5).join("?");
    let damaged: Vec<_> = (0..5).flat_map(|_| damaged.iter()).copied().collect();
    // let mut sorted_consecutive_damaged_groups=_get_consecutive_damaged_groups(&record);
    // sorted_consecutive_damaged_groups.sort_unstable_by(|(_,l1),(_,l2)|l2.cmp(l1));
    // // println!("{sorted_consecutive_damaged_groups:?}");
    // _compute_matches_damaged_first(&record, &damaged, &sorted_consecutive_damaged_groups,0)
    compute_matches(&record, &damaged)
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
    for d in damaged.iter() {
        sum_with_gaps += *d + if result > 0 { 1 } else { 0 };
        if sum_with_gaps > gap_len {
            break;
        }
        result += 1;
    }
    result
}

#[allow(dead_code)]
fn get_consecutive_damaged_groups(record: &str) -> Vec<(usize, usize)> {
    record.chars().enumerate()
        .group_by(|(_,c)|*c=='#').into_iter()
        .filter_map(|(damaged, group)|if damaged{
            group.map(|(i,_)|(i,1usize))
                .reduce(|(i1,j1),(i2,j2)|(min(i1,i2),j1+j2))

        }else{None}).collect()
}

/// Compute number of possible matches by focusing first on known damaged spots
///
/// placing damaged_group on mandatory spots should converge faster, but alas, it does not
///
/// Arguments:
/// -----------
///
/// * `record` : search zone, as `str`
/// * `damaged`: damage spots to place on record, described by length
/// * `sorted_consecutives_damaged`: slice of (consecutive spot index, number of consecutive spots)
///     <br/>**index is from original record, thus must be corrected by `record_offset`**
/// * `record_offset` : offset of the current record in the original one
#[allow(dead_code)]
fn compute_matches_damaged_first(record: &str,
                                  damaged: &[usize],
                                  sorted_consecutives_damaged:&[(usize, usize)],
                                  record_offset:usize) -> usize {
    let len =record.len();
    let longest_damaged = sorted_consecutives_damaged.iter().find(|(i,_)|*i>=record_offset && *i-record_offset<len).copied();
    if let Some((d_idx,d_len)) = longest_damaged {
        let d_idx=d_idx-record_offset;

        damaged.iter().enumerate().filter(|(_,d)|**d >= d_len).map(|(i,d)|{
            let current=*d;
            (0..=current-d_len).filter(|offset|*offset<=d_idx).map(|offset| {

                let current_record=&record[d_idx-offset..];
                if current_record.len()<current {return 0;}// not enough space
                if current_record[..current].chars().any(|c|c=='.'){return  0;} // some OK on current spot

                if d_idx-offset > 0 && record[d_idx-offset-1..].starts_with('#'){return 0;} // '#' just before
                if current_record[current..].starts_with('#') {return 0;}// '#' just after

                let solutions_before = if d_idx-offset > 0 {
                    compute_matches_damaged_first(&record[..d_idx-offset-1], &damaged[..i],sorted_consecutives_damaged, record_offset)
                }else{1};
                let solutions_after= if current_record.len()>current {
                    compute_matches_damaged_first(&current_record[current+1..], &damaged[i+1..],sorted_consecutives_damaged, record_offset+d_idx-offset+current+1)}else{1};

                solutions_before*solutions_after
            }).sum::<usize>()
        }).sum()
    }else{
        compute_matches(record,damaged)
    }
}

fn compute_matches(record: &str, damaged: &[usize]) -> usize {
    let forced_ok = record.chars().take_while(|c| *c == '.').count();
    let record = &record[forced_ok..];
    let forced_ok = record.chars().rev().take_while(|c| *c == '.').count();
    let record = &record[..record.len() - forced_ok];
    // println!("computed_matches('{record}', {damaged:?}, {previous:?})");
    if minimum_len(damaged) > record.len() {
        return 0;
    }

    if damaged.is_empty() {
        return if record.chars().any(|c| c == '#') {
            0
        } else {
            1
        };
    }

    let leading_choices = record.chars().take_while(|c| *c == '?').count();
    let next_no_choice = record.chars().nth(leading_choices);
    match next_no_choice {
        None => {
            let margin =record.len()- minimum_len(damaged);
            binomial(damaged.len()+margin,margin)
        },
        Some('.') /* ok */=> {
            // let's try to spread the damaged around this ok point
            let nb_damaged_max= count_max_damaged_seq_fitting(damaged, leading_choices);
             (0..=nb_damaged_max).map(|i| {
                let margin = leading_choices - minimum_len(&damaged[0..i]);
                binomial(i+margin,margin)*compute_matches(&record[leading_choices+1..],&damaged[i..])
            }).sum()
        }
        _ /*damaged*/=> {
            // there must be one damaged on this spot, the others are on each sides
            let nb_damaged_max= count_max_damaged_seq_fitting(damaged, leading_choices);
            (0..=nb_damaged_max).map(|i|{
                if i == damaged.len() {return 0;} // there are not damaged left, but a #
                //
                let current=damaged[i];

                (1..=current).filter(|offset|*offset<=leading_choices+1).map(|reserved| {

                    let current_record=&record[leading_choices+1-reserved..];
                    // println!("current {current} reserved {reserved}");
                    if current_record.len()<current {/*println!("{current_record} : not_enough place for {current}");*/return 0;}//not enough place
                    if current_record[..current].chars().any(|c|c=='.'){/*println!("{current_record} : some . before  {current}");*/return 0;} // some ok on damage[i] place

                    let solutions_before = if i==0 {1} else { compute_matches(&record[..leading_choices.saturating_sub(reserved)], &damaged[..i]) };
                    if current_record.len()==current { // no place after
                        return  if damaged.len()==i+1 {solutions_before} else { /*println!("still {} but no more spot",damaged.len()-i -1);*/0 };
                    }
                    if current_record[current..].starts_with('#') {/*println!("{current_record} : a # just after  {current}");*/return  0;} // cannot have a # just after current

                    let solutions_after= compute_matches(&current_record[current+1..],&damaged[i+1..]);
                    // println!("@@@ compute_matches({},{:?})=> {solutions_before}  compute_matches({},{:?})=> {solutions_after} ",
                    //          &record[..leading_choices.saturating_sub(reserved)],&damaged[..i],
                    //          &current_record[current+1..],&damaged[i+1..]);
                    solutions_before*solutions_after
                }).sum::<usize>()
            }).sum()
        }
    }
}

fn sum_arrangments(
    inputs: &'static str,
    counter: impl Fn(&str, &[usize]) -> usize + Send + Sync,
) -> usize {
    inputs
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .collect_vec()
        .par_iter()
        .map(|(_i, l)| {
            let (record, damaged) = l.split_once(' ').unwrap();
            let damaged: Vec<usize> = damaged.split(',').filter_map(|g| g.parse().ok()).collect();
            counter(record, &damaged)
        })
        .sum()
}

#[allow(dead_code)]
pub fn arrange_springs() {
    let input = include_str!("../resources/day12_records.txt");
    let sum = sum_arrangments(input, compute_matches);
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
        assert_eq!(
            150,
            sum_arrangments("?????????###???????? 2,1,3,2,1", compute_matches)
        );
        assert_eq!(1, sum_arrangments("..?##. 3", compute_matches));

        assert_eq!(10, sum_arrangments("?###???????? 3,2,1", compute_matches));
        assert_eq!(4, sum_arrangments(".??..??...?##. 1,1,3", compute_matches));
        assert_eq!(1, sum_arrangments("?### 3", compute_matches));
        assert_eq!(1, sum_arrangments("#???. 3", compute_matches));
        assert_eq!(1, sum_arrangments("???. 3", compute_matches));

        assert_eq!(
            1,
            sum_arrangments("?#?#?#?#?#?#?#? 1,3,1,6", compute_matches)
        );
        assert_eq!(1, sum_arrangments("????.#...#... 4,1,1", compute_matches));
        assert_eq!(
            4,
            sum_arrangments("????.######..#####. 1,6,5", compute_matches)
        );
        let input = indoc! {"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        "};
        assert_eq!(21, sum_arrangments(input, compute_matches));

        assert_eq!(1, sum_arrangments("???.### 1,1,3", compute_matches));
        assert_eq!(
            sum_arrangments("#.?#.?#.?#.?#. 1,1,1,1,1", compute_matches),
            sum_arrangments("#. 1", count_unfolded_matches)
        );
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
