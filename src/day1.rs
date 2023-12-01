fn calibrate(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let digits: Vec<_> = l.chars().filter(char::is_ascii_digit).collect();
            if digits.is_empty() {
                0
            } else {
                digits[0].to_digit(10).unwrap_or(0) * 10
                    + digits[digits.len() - 1].to_digit(10).unwrap_or(0)
            }
        })
        .sum()
}

const LITERALS: [(&[u8], u8); 9] = [
    ("one".as_bytes(), b'1'),
    ("two".as_bytes(), b'2'),
    ("three".as_bytes(), b'3'),
    ("four".as_bytes(), b'4'),
    ("five".as_bytes(), b'5'),
    ("six".as_bytes(), b'6'),
    ("seven".as_bytes(), b'7'),
    ("eight".as_bytes(), b'8'),
    ("nine".as_bytes(), b'9'),
];

fn calibrate_literals(input: &str) -> u32 {
    assert!(input.is_ascii(), "input {input} is not ascii only");
    let mut input = input.to_lowercase();

    // justification : input is ascii only, hence lowercase input is ascii only too
    let bytes = unsafe { input.as_bytes_mut() };

    // let's replace literals from left to right
    let mut i = 0usize;
    let total_len = bytes.len();
    'main: while i < total_len {
        for (lit, digit) in &LITERALS {
            let len = lit.len();
            if i + len <= total_len && &bytes[i..i + len] == *lit {
                bytes[i] = *digit;
                // ⚠️ overlapping literals must all be considered ==> only override the first char
                // for b in &mut bytes[i..i+len] {
                //     *b =*digit;
                // }
                // i+=len;
                // continue 'main;
                break;
            }
        }
        i += 1;
    }

    for (lit, _) in &crate::day1::LITERALS {
        let lit = String::from_utf8_lossy(lit);
        assert!(!input.contains(&*lit));
    }

    calibrate(&input)
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
