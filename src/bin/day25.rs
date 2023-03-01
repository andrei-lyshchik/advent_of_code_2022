use aoc2022::input::read_lines;

fn main() {
    let sum = read_lines("inputs/day25.txt")
        .map(|l| parse_snafu_number(&l))
        .collect::<Result<Vec<Vec<Digit>>, String>>()
        .unwrap()
        .iter()
        .map(|s| snafu_number_to_decimal(&s))
        .sum::<isize>();

    println!("Part 1: {}", to_snafu_string(sum));
}

fn parse_snafu_number(str: &str) -> Result<Vec<Digit>, String> {
    str.chars().map(|c| parse_snafu_digit(c)).rev().collect()
}

fn parse_snafu_digit(c: char) -> Result<Digit, String> {
    match c {
        '=' => Ok(Digit::MinusTwo),
        '-' => Ok(Digit::MinusOne),
        '0' => Ok(Digit::Zero),
        '1' => Ok(Digit::One),
        '2' => Ok(Digit::Two),
        unexpected @ _ => Err(format!("Unexpected char for snafu digit: '{}'", unexpected)),
    }
}

enum Digit {
    MinusTwo,
    MinusOne,
    Zero,
    One,
    Two,
}

impl Digit {
    fn decimal(&self) -> isize {
        match self {
            Digit::MinusTwo => -2,
            Digit::MinusOne => -1,
            Digit::Zero => 0,
            Digit::One => 1,
            Digit::Two => 2,
        }
    }

    fn char(&self) -> char {
        match self {
            Digit::MinusTwo => '=',
            Digit::MinusOne => '-',
            Digit::Zero => '0',
            Digit::One => '1',
            Digit::Two => '2',
        }
    }

    fn negate(&self) -> Digit {
        match self {
            Digit::MinusTwo => Digit::Two,
            Digit::MinusOne => Digit::One,
            Digit::Zero => Digit::Zero,
            Digit::One => Digit::MinusOne,
            Digit::Two => Digit::MinusTwo,
        }
    }
}

fn snafu_number_to_decimal(snafu: &Vec<Digit>) -> isize {
    let mut power = 1;
    let mut result = 0;
    for digit in snafu {
        result += power * digit.decimal();
        power *= 5;
    }
    result
}

fn decimal_to_snafu(number: isize) -> Vec<Digit> {
    if number < 0 {
        return decimal_to_snafu(-number)
            .iter()
            .map(|d| d.negate())
            .collect();
    }

    match number {
        0 => return vec![Digit::Zero],
        1 => return vec![Digit::One],
        2 => return vec![Digit::Two],
        _ => {}
    };

    let mut highest_digit_power = 5;
    let mut max_number_with_digit_count = 12;
    let mut digit_count = 2;
    while number > max_number_with_digit_count {
        highest_digit_power *= 5;
        max_number_with_digit_count += 2 * highest_digit_power;
        digit_count += 1;
    }

    let highest_digit_decimal = (1..=2)
        .min_by_key(|hd| (hd * highest_digit_power - number).abs())
        .unwrap();
    let highest_digit_snafu = if highest_digit_decimal == 1 {
        Digit::One
    } else {
        Digit::Two
    };
    let mut result = decimal_to_snafu(number - highest_digit_decimal * highest_digit_power);
    for _ in 0..(digit_count - 1 - result.len()) {
        result.push(Digit::Zero);
    }
    result.push(highest_digit_snafu);

    result
}

fn to_snafu_string(number: isize) -> String {
    decimal_to_snafu(number)
        .iter()
        .map(|d| d.char())
        .rev()
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::to_snafu_string;

    #[test]
    fn tests() {
        assert_eq!("1", to_snafu_string(1));
        assert_eq!("1=", to_snafu_string(3));
        assert_eq!("12", to_snafu_string(7));
        assert_eq!("1=0", to_snafu_string(15));
        assert_eq!("10-", to_snafu_string(24));
        assert_eq!("100", to_snafu_string(25));
    }
}
