use ahash::AHashSet;
use eyre::{eyre, Error};
use std::cmp::min;
use std::str::FromStr;

struct CardsGame {
    wins: Vec<Vec<u32>>,
    values: Vec<AHashSet<u32>>,
}

impl FromStr for CardsGame {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut wins: Vec<Vec<u32>> = vec![];
        let mut values: Vec<AHashSet<u32>> = vec![];

        for l in s.lines() {
            let (_card, str_values) =
                l.split_once(':').ok_or_else(|| eyre!("no card Id separator in {l}"))?;
            let (win, ours) = str_values
                .split_once('|')
                .ok_or_else(|| eyre!("no win|self separator in {str_values}"))?;

            wins.push(win.split(' ').filter_map(|v| v.parse().ok()).collect());
            values.push(ours.split(' ').filter_map(|v| v.parse::<u32>().ok()).collect());
        }
        Ok(Self { wins, values })
    }
}

fn sum_cards_scores(input: &str) -> u32 {
    let game: CardsGame = input.parse().unwrap();

    game.wins
        .iter()
        .zip(game.values.iter())
        .map(|(w, o)| w.iter().map(|v| if o.contains(v) { 2 } else { 1 }).product::<u32>() / 2)
        .sum()
}

pub fn count_cards(input: &str) -> u32 {
    let game: CardsGame = input.parse().unwrap();

    let mut cards_count = vec![1; game.wins.len()];
    for (i, (w, o)) in game.wins.iter().zip(game.values.iter()).enumerate() {
        let count = w.iter().filter(|c| o.contains(c)).count();
        for j in i + 1..min(game.wins.len(), i + 1 + count) {
            cards_count[j] += cards_count[i];
        }
    }

    cards_count.into_iter().sum()
}

pub fn play_cards() {
    let input = include_str!("../resources/day4_cards.txt");
    let score = sum_cards_scores(input);
    println!("score : {score}");
    let count = count_cards(input);
    println!("card counts : {count}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn aoc_example_works() {
        let input = indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};
        let score = sum_cards_scores(input);
        assert_eq!(13, score);

        assert_eq!(30, count_cards(input));
    }
}
