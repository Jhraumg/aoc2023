use ahash::AHashMap;
use itertools::Itertools;
use std::cmp::Ordering;

// Fixme : derive PartialOrd instead
#[repr(u8)]
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Card {
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    N9 = 9,
    N8 = 8,
    N7 = 7,
    N6 = 6,
    N5 = 5,
    N4 = 4,
    N3 = 3,
    N2 = 2,
}
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum CardType {
    FiveK = 6,
    FourK = 5,
    FullHouse = 4,
    ThreeK = 3,
    TwoPairs = 2,
    OnePair = 1,
    HighCard = 0,
}
struct Hand {
    cards: [Card; 5],
    ctype: CardType,
}

fn to_card(card: char) -> Card {
    match card {
        'A' => Card::A,
        'K' => Card::K,
        'Q' => Card::Q,
        'J' => Card::J,
        'T' => Card::T,
        '9' => Card::N9,
        '8' => Card::N8,
        '7' => Card::N7,
        '6' => Card::N6,
        '5' => Card::N5,
        '4' => Card::N4,
        '3' => Card::N3,
        '2' => Card::N2,
        _ => panic!("no card type {card}"),
    }
}
fn to_cards(input: &str) -> [Card; 5] {
    let mut result = [Card::N2; 5];
    for (i, c) in input.trim().chars().take(5).enumerate() {
        result[i] = to_card(c);
    }
    result
}
fn get_type(cards: &[Card; 5]) -> CardType {
    let mut counts: AHashMap<Card, usize> = AHashMap::new();
    for c in cards {
        *counts.entry(*c).or_default() += 1;
    }
    let values: Vec<_> = counts.values().copied().collect();
    if *values.iter().max().unwrap() == 5 {
        return CardType::FiveK;
    }
    if *values.iter().max().unwrap() == 4 {
        return CardType::FourK;
    }
    if *values.iter().max().unwrap() == 3 {
        for v in &values {
            if *v == 2 {
                return CardType::FullHouse;
            }
        }
        return CardType::ThreeK;
    }
    match values.iter().filter(|v| **v == 2).count() {
        2 => CardType::TwoPairs,
        1 => CardType::OnePair,
        _ => CardType::HighCard,
    }
}

fn get_type_with_jokers(cards: &[Card; 5]) -> CardType {
    let mut counts: AHashMap<Card, usize> = AHashMap::new();
    for c in cards {
        *counts.entry(*c).or_default() += 1;
    }
    let values: Vec<_> = counts.values().copied().collect();
    if *values.iter().max().unwrap() == 5 {
        return CardType::FiveK;
    }
    if *values.iter().max().unwrap() == 4 {
        return if *counts.get(&Card::J).unwrap_or(&0) > 0 {
            CardType::FiveK
        } else {
            CardType::FourK
        };
    }
    if *values.iter().max().unwrap() == 3 {
        match *counts.get(&Card::J).unwrap_or(&0) {
            2 => {
                return CardType::FiveK;
            }
            1 => {
                return CardType::FourK;
            }
            3 => {
                // counting non J cards
                return if counts.values().contains(&2) {
                    CardType::FiveK
                } else {
                    CardType::FourK
                };
            }
            _ => {
                // 0 actually
                for v in &values {
                    if *v == 2 {
                        return CardType::FullHouse;
                    }
                }
                return CardType::ThreeK;
            }
        }
    }
    match values.iter().filter(|v| **v == 2).count() {
        2 => match *counts.get(&Card::J).unwrap_or(&0) {
            2 => CardType::FourK,
            1 => CardType::FullHouse,
            _ => CardType::TwoPairs,
        },
        1 => {
            if counts.contains_key(&Card::J) {
                // either there 2 J, and they'll bon ith any other card, or there one other pair with wich this J will bond
                CardType::ThreeK
            } else {
                CardType::OnePair
            }
        }

        _ => {
            if counts.contains_key(&Card::J) {
                CardType::OnePair
            } else {
                CardType::HighCard
            }
        }
    }
}

fn sum_winnings(input: &str) -> u64 {
    let game: Vec<(Hand, u64)> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (cards, bid) = l.trim().split_once(' ').unwrap();
            let bid: u64 = bid.parse().unwrap();
            let cards = to_cards(cards);
            let ctype = get_type(&cards);
            let hand = Hand { cards, ctype };
            (hand, bid)
        })
        .sorted_by(
            |(h1, _), (h2, _)| match (h1.ctype as isize).cmp(&(h2.ctype as isize)) {
                Ordering::Equal => {
                    for i in 0..5 {
                        if (h1.cards[i] as isize).cmp(&(h2.cards[i] as isize)) != Ordering::Equal {
                            return (h1.cards[i] as isize).cmp(&(h2.cards[i] as isize));
                        }
                    }
                    Ordering::Equal
                }
                _ => (h1.ctype as isize).cmp(&(h2.ctype as isize)),
            },
        )
        .collect();
    game.into_iter().enumerate().map(|(i, (_, bid))| (i as u64 + 1) * bid).sum()
}

fn cmp_hands_with_j(h1: &Hand, h2: &Hand) -> std::cmp::Ordering {
    match (h1.ctype as isize).cmp(&(h2.ctype as isize)) {
        Ordering::Equal => {
            for i in 0..5 {
                let h1_value = if h1.cards[i] == Card::J {
                    0
                } else {
                    h1.cards[i] as isize
                };
                let h2_value = if h2.cards[i] == Card::J {
                    0
                } else {
                    h2.cards[i] as isize
                };

                if h1_value.cmp(&h2_value) != Ordering::Equal {
                    return h1_value.cmp(&h2_value);
                }
            }
            Ordering::Equal
        }
        _ => (h1.ctype as isize).cmp(&(h2.ctype as isize)),
    }
}

fn to_hand_with_j(cards: &str) -> Hand {
    let cards = to_cards(cards);
    let ctype = get_type_with_jokers(&cards);
    Hand { cards, ctype }
}
fn sum_winnings_with_j(input: &str) -> u64 {
    let game: Vec<(Hand, u64)> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (cards, bid) = l.trim().split_once(' ').unwrap();
            let bid: u64 = bid.parse().unwrap();

            let hand = to_hand_with_j(cards);
            (hand, bid)
        })
        .sorted_by(|(h1, _), (h2, _)| cmp_hands_with_j(h1, h2))
        .collect();
    game.into_iter().enumerate().map(|(i, (_, bid))| (i as u64 + 1) * bid).sum()
}
pub fn play_camel_cards() {
    let input = include_str!("../resources/day7_camel_cards.txt");
    let winnings = sum_winnings(input);
    println!("winnings {winnings}");
    let jwinnings = sum_winnings_with_j(input);
    println!("jwinnings {jwinnings}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn aoc_example_works() {
        let input = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};
        let winnings = sum_winnings(input);
        assert_eq!(6440, winnings);
        assert_eq!(5905, sum_winnings_with_j(input));
        let h1 = to_hand_with_j("JKKK2");
        let h2 = to_hand_with_j("QQQQ2");
        assert_eq!(Ordering::Less, cmp_hands_with_j(&h1, &h2));

        assert_eq!(
            Ordering::Equal,
            cmp_hands_with_j(&to_hand_with_j("JKKK2"), &to_hand_with_j("JKKK2"))
        );
        assert_eq!(
            Ordering::Greater,
            cmp_hands_with_j(&to_hand_with_j("JKKK2"), &to_hand_with_j("QKKK2"))
        );

        let card = to_cards("A234J");
        let _card_type = get_type_with_jokers(&card);
        assert!(matches!(CardType::OnePair, _card_type));
    }
}
