use std::cmp::min;

fn to_calibration(l: &str, values: &[(&str, u32)]) -> u32 {
    let total_len = l.len();

    // first digit search
    let mut lvalue_idx = total_len;
    let mut lvalue = 0;

    // last digit search
    let mut rvalue_idx = 0;
    let mut rvalue = 0;

    for (expr, val) in values {
        if let Some(idx) = l[0..min(lvalue_idx + expr.len(), total_len)].find(expr) {
            lvalue_idx = idx;
            lvalue = *val;
        }
        if let Some(idx) = l[rvalue_idx..].rfind(expr) {
            rvalue_idx += idx;
            rvalue = *val;
        }
    }
    10 * lvalue + rvalue
}

fn calibrate(input: &str) -> u32 {
    const DIGITS: [(&str, u32); 10] = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("0", 0),
    ];

    input.lines().map(|l| to_calibration(l, &DIGITS)).sum()
}

fn calibrate_literals(input: &str) -> u32 {
    const DIGITS_AND_LITERALS: [(&str, u32); 19] = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("0", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    input.lines().map(|l| to_calibration(l, &DIGITS_AND_LITERALS)).sum()
}

pub fn calibrate_trebuchet() {
    let input = include_str!("../resources/day1_calibration.txt");

    let calibration = calibrate(input);
    println!("calibration is {calibration}");

    let calibration_literals = calibrate_literals(input);
    println!("calibration with literals is {calibration_literals}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn trebuchet_can_be_calibrated() {
        let input = indoc! {
            "
                1abc2
                pqr3stu8vwx
                a1b2c3d4e5f
                treb7uchet
            "
        };

        assert_eq!(calibrate(input), 142);
        let input = indoc! {
            "
                two1nine
                eightwothree
                abcone2threexyz
                xtwone3four
                4nineeightseven2
                zoneight234
                7pqrstsixteen
            "
        };
        assert_eq!(calibrate_literals(input), 281);
    }
}
