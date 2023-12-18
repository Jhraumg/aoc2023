use itertools::Itertools;
use std::cmp::min;

// enum Tile {
//     Rock,
//     Sand,
// }
struct Pattern {
    tiles: Vec<Vec<char>>,
}
fn read_patterns(input: &str) -> Vec<Pattern> {
    let mut result = vec![];
    for (empty, group) in &input.lines().group_by(|l| l.is_empty()) {
        if !empty {
            let tiles = group.map(|l| l.chars().collect()).collect();
            result.push(Pattern { tiles });
        }
    }
    result
}
fn is_horizontal_reflexion_after(tiles: &[Vec<char>], index: usize) -> bool {
    if index + 1 >= tiles.len() {
        return false;
    };

    let max_len = min(tiles.len() - index - 1, index + 1);
    for j in 0..max_len {
        if tiles[index - j] != tiles[index + 1 + j] {
            // println!("tiles[{}]:'{}'!=tiles[{}]:'{}'",index-j,tiles[index-j].iter().join(""),index+1+j,tiles[index+1+j].iter().join(""));
            return false;
        }
    }
    true
}

fn is_vertical_reflexion_after(tiles: &[Vec<char>], index: usize) -> bool {
    if index + 1 >= tiles[0].len() {
        return false;
    };

    let max_len = min(tiles[0].len() - index - 1, index + 1);
    // println!("max_len {max_len}");
    for line in tiles {
        for j in 0..max_len {
            if line[index - j] != line[index + 1 + j] {
                // println!("line {j} {} [{}]:'{}'!=[{}]:'{}'",line.iter().join(""),index-j,line[index-j],index+1+j,line[index+1+j]);
                return false;
            }
        }
    }
    true
}

fn _print_tiles(tiles: &[Vec<char>]) {
    for l in tiles {
        println!("{}", l.iter().join(""));
    }
}

fn sum_note_with_correction(tiles: &[Vec<char>]) -> usize {
    let maxx = tiles[0].len();
    let maxy = tiles.len();
    (0..maxx)
        .flat_map(|i| (0..maxy).map(move |j| (i, j)))
        .map(|(i, j)| {
            let mut new_tiles: Vec<Vec<char>> = tiles.to_vec();
            new_tiles[j][i] = if tiles[j][i] == '.' { '#' } else { '.' };

            (0..maxy - 1)
                .filter(|vidx| {
                    if *vidx >= j {
                        //reflexion below the spot
                        if *vidx - j + 1 > maxy - *vidx - 1 {
                            false
                        } else {
                            is_horizontal_reflexion_after(&new_tiles, *vidx)
                        }
                    } else {
                        false
                        // if j- *vidx > *vidx+1 {false}else {is_horizontal_reflexion_after(&new_tiles, *vidx)}
                        // reflexion uppon the spo
                    }
                })
                .map(|i| {
                    /*println!("horz refl after {i}");*/
                    (i + 1) * 100
                })
                .sum::<usize>()
                + (0..maxx - 1)
                    .filter(|hidx| {
                        if *hidx >= i {
                            //reflexion below the spot
                            if *hidx - i + 1 > maxx - *hidx - 1 {
                                false
                            } else {
                                is_vertical_reflexion_after(&new_tiles, *hidx)
                            }
                        } else {
                            false
                            // if i- *hidx > *hidx+1 {false}else {is_vertical_reflexion_after(&new_tiles, *hidx)}
                            // reflexion uppon the spo
                        }
                    })
                    .map(|j| {
                        /*println!("vert refl after {j}");*/
                        j + 1
                    })
                    .sum::<usize>()
        })
        .sum()
}

fn sum_note(tiles: &[Vec<char>]) -> usize {
    // println!("----");

    (0..tiles.len() - 1)
        .filter(|i| is_horizontal_reflexion_after(tiles, *i))
        .map(|i| {
            /*println!("horz refl after {i}");*/
            (i + 1) * 100
        })
        .sum::<usize>()
        + (0..tiles[0].len() - 1)
            .filter(|j| is_vertical_reflexion_after(tiles, *j))
            .map(|j| {
                /*println!("vert refl after {j}");*/
                j + 1
            })
            .sum::<usize>()
}

pub fn check_notes() {
    let input = include_str!("../resources/day13_notes.txt");
    let patterns = read_patterns(input);
    assert_eq!(100, patterns.len());
    let sum: usize = patterns.iter().map(|p| sum_note(&p.tiles)).sum();

    println!("sum : {sum}");
    let sum_with_corr: usize = patterns.iter().map(|p| sum_note_with_correction(&p.tiles)).sum();
    println!("sum with correction {sum_with_corr}");
}

// FIXME dont't forget to start at 1
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn aoc_example_works() {
        let patterns = read_patterns(indoc! {"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "});
        assert!(is_vertical_reflexion_after(&patterns[0].tiles, 4));
        assert!(!is_vertical_reflexion_after(&patterns[0].tiles, 7));

        assert!(is_horizontal_reflexion_after(&patterns[1].tiles, 3));

        assert_eq!(
            405,
            sum_note(&patterns[0].tiles) + sum_note(&patterns[1].tiles)
        );

        assert_eq!(300, sum_note_with_correction(&patterns[0].tiles));
        assert_eq!(100, sum_note_with_correction(&patterns[1].tiles));
        assert_eq!(
            400,
            sum_note_with_correction(&patterns[0].tiles)
                + sum_note_with_correction(&patterns[1].tiles)
        );
    }
}
