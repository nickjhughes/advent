use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};
use std::{fmt, fs};

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let snafu_numbers = parse_numbers(&contents);
    let decimal_numbers = snafu_numbers
        .iter()
        .map(|n| n.to_decimal())
        .collect::<Vec<i64>>();
    let decimal_sum = decimal_numbers.iter().sum();
    let snafu_sum = SNAFUNumber::from_decimal(decimal_sum);
    format!("{}", snafu_sum)
}

pub fn part2() -> String {
    "".into()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input25").expect("Failed to open input file")
}

#[derive(Debug, PartialEq, Eq)]
enum SNAFUDigit {
    Two,
    One,
    Zero,
    Minus,
    DoubleMinus,
}

impl SNAFUDigit {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            alt((tag("2"), tag("1"), tag("0"), tag("-"), tag("="))),
            |str| match str {
                "2" => Self::Two,
                "1" => Self::One,
                "0" => Self::Zero,
                "-" => Self::Minus,
                "=" => Self::DoubleMinus,
                _ => unreachable!(),
            },
        )(input)
    }

    fn from_value(value: i64) -> Self {
        match value {
            2 => Self::Two,
            1 => Self::One,
            0 => Self::Zero,
            -1 => Self::Minus,
            -2 => Self::DoubleMinus,
            _ => panic!("invalid digit value: {}", value),
        }
    }

    fn value(&self) -> i64 {
        match self {
            Self::Two => 2,
            Self::One => 1,
            Self::Zero => 0,
            Self::Minus => -1,
            Self::DoubleMinus => -2,
        }
    }
}

impl fmt::Display for SNAFUDigit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SNAFUDigit::Two => write!(f, "2"),
            SNAFUDigit::One => write!(f, "1"),
            SNAFUDigit::Zero => write!(f, "0"),
            SNAFUDigit::Minus => write!(f, "-"),
            SNAFUDigit::DoubleMinus => write!(f, "="),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SNAFUNumber {
    digits: Vec<SNAFUDigit>,
}

impl SNAFUNumber {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(many1(SNAFUDigit::parse), |digits| Self { digits })(input)
    }

    fn from_decimal(number: i64) -> Self {
        let mut snafu = Self { digits: Vec::new() };
        let mut power = 0i32;
        while number > 2 * 5i64.pow(power as u32) {
            power += 1;
        }
        let mut max_diff = 0;
        for i in 0..power {
            max_diff += 2 * 5i64.pow(i as u32);
        }
        let coefficient = if 2 * 5i64.pow(power as u32) - max_diff > number {
            1
        } else {
            2
        };
        snafu.digits.push(SNAFUDigit::from_value(coefficient));
        let mut remainder = number - coefficient * 5i64.pow(power as u32);
        power -= 1;

        while power >= 0 {
            let mut max_rest = 0;
            for i in 0..power {
                max_rest += 2 * 5i64.pow(i as u32);
            }
            let mut coefficient = -2;
            while (remainder - coefficient * 5i64.pow(power as u32)).abs() > max_rest {
                coefficient += 1;
            }
            snafu.digits.push(SNAFUDigit::from_value(coefficient));
            remainder -= coefficient * 5i64.pow(power as u32);
            power -= 1;
        }

        snafu
    }

    fn to_decimal(&self) -> i64 {
        let mut result = 0;
        for i in (0..self.digits.len()).rev() {
            let value = 5i64.pow(i as u32);
            let coefficient = self.digits[self.digits.len() - 1 - i].value();
            result += coefficient * value;
        }
        result
    }
}

impl fmt::Display for SNAFUNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for digit in &self.digits {
            write!(f, "{}", digit)?;
        }
        Ok(())
    }
}

fn parse_numbers(contents: &str) -> Vec<SNAFUNumber> {
    let (rest, numbers) =
        terminated(separated_list1(newline, SNAFUNumber::parse), opt(newline))(contents)
            .expect("Failed to parse numbers");
    assert!(rest.is_empty());
    numbers
}

#[test]
fn test_parse_numbers() {
    let contents = "1=-0-2\n12111\n2=0=\n";
    let numbers = parse_numbers(contents);
    assert_eq!(numbers.len(), 3);

    assert_eq!(
        numbers[0],
        SNAFUNumber {
            digits: vec![
                SNAFUDigit::One,
                SNAFUDigit::DoubleMinus,
                SNAFUDigit::Minus,
                SNAFUDigit::Zero,
                SNAFUDigit::Minus,
                SNAFUDigit::Two
            ]
        }
    );
    assert_eq!(
        numbers[1],
        SNAFUNumber {
            digits: vec![
                SNAFUDigit::One,
                SNAFUDigit::Two,
                SNAFUDigit::One,
                SNAFUDigit::One,
                SNAFUDigit::One,
            ]
        }
    );
    assert_eq!(
        numbers[2],
        SNAFUNumber {
            digits: vec![
                SNAFUDigit::Two,
                SNAFUDigit::DoubleMinus,
                SNAFUDigit::Zero,
                SNAFUDigit::DoubleMinus,
            ]
        }
    );
}

#[test]
fn test_to_decimal() {
    let contents = "1=-0-2\n12111\n2=0=\n21\n2=01\n111\n20012\n112\n1=-1=\n1-12\n12\n1=\n122\n";
    let numbers = parse_numbers(contents);
    assert_eq!(numbers.len(), 13);
    assert_eq!(
        numbers.iter().map(|n| n.to_decimal()).collect::<Vec<i64>>(),
        vec![1747, 906, 198, 11, 201, 31, 1257, 32, 353, 107, 7, 3, 37]
    );
}

#[test]
fn test_from_decimal() {
    let decimal_numbers = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 20, 2022, 12345, 314159265,
    ];
    let snafu_numbers = decimal_numbers
        .iter()
        .map(|n| SNAFUNumber::from_decimal(*n))
        .collect::<Vec<SNAFUNumber>>();
    assert_eq!(
        snafu_numbers
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>(),
        vec![
            "1",
            "2",
            "1=",
            "1-",
            "10",
            "11",
            "12",
            "2=",
            "2-",
            "20",
            "1=0",
            "1-0",
            "1=11-2",
            "1-0---0",
            "1121-1110-1=0",
        ]
    );
}
