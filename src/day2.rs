use eyre::{eyre, Error};
use std::str::FromStr;

#[derive(Debug)]
enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}
const COLOR_VARIANT_COUNT: usize = 3;

#[derive(Debug)]
struct CubesSet([u32; COLOR_VARIANT_COUNT]);

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<CubesSet>,
}

impl FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(eyre!("'{s}' is not a known color")),
        }
    }
}

impl FromStr for CubesSet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cube_counts: Result<Vec<(Color, u32)>, Self::Err> = s
            .split(',')
            .map(|count_str| {
                let mut item_itr = count_str.trim().split(' ');
                let count: u32 = item_itr
                    .next()
                    .ok_or_else(|| eyre!("cannot parse count from '{count_str}'"))?
                    .parse()?;
                let color: Color = item_itr
                    .next()
                    .ok_or_else(|| eyre!("cannot parse color from '{count_str}'"))?
                    .parse()?;
                Ok((color, count))
            })
            .collect();

        let mut result = [0; COLOR_VARIANT_COUNT];
        for (color, count) in cube_counts? {
            result[color as usize] += count;
        }
        Ok(CubesSet(result))
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rounds) = s
            .split_once(':')
            .ok_or_else(|| eyre!("no game id delimiter in {s}"))?;
        let id: u32 = id.trim().replace("Game ", "").parse()?;
        let rounds: Result<Vec<CubesSet>, _> =
            rounds.split(';').map(|round| round.parse()).collect();
        let rounds = rounds?;

        Ok(Self { id, rounds })
    }
}
fn is_game_possible(game: &Game, cubes_set: &CubesSet) -> bool {
    for draw in &game.rounds {
        for i in 0..COLOR_VARIANT_COUNT {
            if draw.0[i] > cubes_set.0[i] {
                return false;
            }
        }
    }
    true
}

fn sum_possible_games(games: &[Game]) -> u32 {
    const CUBES_SET: CubesSet = CubesSet([12, 13, 14]);
    games
        .iter()
        .filter(|g| is_game_possible(g, &CUBES_SET))
        .map(|g| g.id)
        .sum()
}

fn get_minimum_cub_set(game: &Game) -> CubesSet {
    game.rounds
        .iter()
        .fold(CubesSet([0; COLOR_VARIANT_COUNT]), |mut current, round| {
            for i in 0..COLOR_VARIANT_COUNT {
                if current.0[i] < round.0[i] {
                    current.0[i] = round.0[i]
                };
            }
            current
        })
}

fn parse_games(input: &str) -> Result<Vec<Game>, Error> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.parse::<Game>())
        .collect()
}

fn power(cubes_set: &CubesSet) -> u32 {
    cubes_set.0.iter().product()
}

fn sum_games_power(games: &[Game]) -> u32 {
    games.iter().map(|g| power(&get_minimum_cub_set(g))).sum()
}

pub fn play_with_cubes() {
    let input = include_str!("../resources/day2_cubes_games.txt");
    let games = parse_games(input).unwrap();
    let valid_games_sum = sum_possible_games(&games);

    println!("sum of valid games Id is {valid_games_sum}");

    let games_power = sum_games_power(&games);
    println!("sum of games power is {games_power}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn possible_games_can_be_summed() {
        let input = indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};
        let games = parse_games(input).unwrap();

        assert_eq!(8, sum_possible_games(&games));
        assert_eq!(2286, sum_games_power(&games));
    }
}
