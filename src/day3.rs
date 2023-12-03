use eyre::{eyre, Error};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum EngineSymbol {
    Number {
        x: usize,
        y: usize,
        val: u32,
        len: usize,
    },
    Symbol {
        x: usize,
        y: usize,
        val: char,
    },
}
impl EngineSymbol {
    fn get_spot(&self) -> ((usize, usize), usize) {
        match self {
            EngineSymbol::Number { x, len, y, .. } => ((*x, *x + *len - 1), *y),
            EngineSymbol::Symbol { x, y, .. } => ((*x, *x), *y),
        }
    }

    pub fn is_adjacent(&self, other: &Self) -> bool {
        let ((minx, maxx), y) = self.get_spot();
        let ((ominx, omaxx), oy) = other.get_spot();

        if y > oy + 1 || oy > y + 1 {
            return false;
        }
        if maxx + 1 < ominx {
            return false;
        }
        if minx > omaxx + 1 {
            return false;
        }

        true
    }
}

#[derive(Debug)]
struct EngineSchema {
    symbols: Vec<EngineSymbol>,
}

impl FromStr for EngineSchema {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut symbols: Vec<EngineSymbol> = vec![];
        for (y, l) in s.lines().enumerate().filter(|(_, l)| !l.is_empty()) {
            let mut acc: Vec<u32> = Vec::with_capacity(l.len());

            for (x, c) in l.chars().enumerate() {
                match c {
                    d if d.is_ascii_digit() => {
                        acc.push(
                            d.to_digit(10)
                                .ok_or_else(|| eyre!("digit conversion for '{d}'"))?,
                        );
                    }
                    c => {
                        if let Some(val) = acc.iter().copied().reduce(|a, d| a * 10 + d) {
                            symbols.push(EngineSymbol::Number {
                                x: x - acc.len(),
                                y,
                                val,
                                len: acc.len(),
                            });
                            acc.clear()
                        }

                        if c != '.' {
                            symbols.push(EngineSymbol::Symbol { x, y, val: c });
                        }
                    }
                }
            }
            if let Some(val) = acc.iter().copied().reduce(|a, d| a * 10 + d) {
                symbols.push(EngineSymbol::Number {
                    x: l.len() - acc.len(),
                    y,
                    val,
                    len: acc.len(),
                });
                acc.clear()
            }
        }

        Ok(Self { symbols })
    }
}

impl EngineSchema {
    fn get_adjacent_symbol<F>(&self, s: &EngineSymbol, adj_filter: F) -> HashSet<EngineSymbol>
    where
        F: Fn(&EngineSymbol) -> bool,
    {
        self.symbols
            .iter()
            .filter(|o| o.is_adjacent(s))
            .filter(|a| adj_filter(a))
            .copied()
            .collect()
    }
}

fn get_part_numbers_sum(input: &str) -> u32 {
    let schema: EngineSchema = input.parse().unwrap();
    for s in &schema.symbols {
        println!("- {s:?}");
    }

    schema
        .symbols
        .iter()
        .filter_map(| n| match n {
            EngineSymbol::Number { val, .. } => {
                if !dbg!(schema.get_adjacent_symbol(n, |s| matches!(s, EngineSymbol::Symbol { .. }))).is_empty(){
                    Some(*val)
                } else {
                    None
                }
            }
            EngineSymbol::Symbol { .. } => None,
        })
        .sum()
}

fn to_number(symbol: &EngineSymbol) -> Option<u32> {
    match symbol {
        EngineSymbol::Number { val, .. } => Some(*val),
        EngineSymbol::Symbol { .. } => None,
    }
}

fn sum_gear_ratios(input: &str) -> u32 {
    let schema: EngineSchema = input.parse().unwrap();

    schema
        .symbols
        .iter()
        .filter_map(|n| match n {
            EngineSymbol::Symbol { val, .. } if *val == '*' => {
                let ratios =
                    schema.get_adjacent_symbol(n, |s| matches!(s, EngineSymbol::Number { .. }));
                if ratios.len() != 2 {
                    println!("rejected {n:?} : {}", ratios.len());
                    None
                } else {
                    Some(ratios.iter().filter_map(to_number).product::<u32>())
                }
            }
            _ => None,
        })
        .sum()
}

pub fn calibrate_engine() {
    let input = include_str!("../resources/day3_schema.txt");

    let parts_sum = get_part_numbers_sum(input);
    println!("parts number sum : {parts_sum}");

    let gear_ratios = sum_gear_ratios(input);
    println!("gear ratios sum : {gear_ratios}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn aoc_example_works() {
        let input = indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};
        let part_number_sum = get_part_numbers_sum(input);
        assert_eq!(4361, part_number_sum);
        assert_eq!(467835, sum_gear_ratios(input));
    }
}
