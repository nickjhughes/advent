use std::fs;

pub fn part1() -> String {
    let input = get_input_file_contents();
    let sum = first_and_last_ascii_digits(&input);
    sum.to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let sum = first_and_last_ascii_and_written_digits(&input);
    sum.to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input01").expect("Failed to open input file")
}

fn first_and_last_ascii_digits(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter(|ch| ch.is_ascii_digit());
            let first_digit = digits.next().unwrap();
            let last_digit = digits.last().unwrap_or(first_digit);
            ((first_digit as u8 - b'0') * 10 + (last_digit as u8 - b'0')) as u64
        })
        .sum::<u64>()
}

fn first_and_last_ascii_and_written_digits(input: &str) -> u64 {
    const DIGITS: [&str; 19] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3",
        "4", "5", "6", "7", "8", "9",
    ];
    const DIGIT_VALUES: [u64; 19] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    input
        .lines()
        .map(|line| {
            let first_positions = DIGITS
                .iter()
                .map(|digit| line.find(digit))
                .collect::<Vec<Option<usize>>>();
            let first_digit = first_positions
                .iter()
                .enumerate()
                .filter(|(_, value)| value.is_some())
                .min_by(|(_, value0), (_, value1)| value0.cmp(value1))
                .map(|(idx, _)| DIGIT_VALUES[idx])
                .unwrap();

            let last_positions: Vec<Option<usize>> = DIGITS
                .iter()
                .map(|digit| line.rfind(digit))
                .collect::<Vec<Option<usize>>>();
            let last_digit = last_positions
                .iter()
                .enumerate()
                .filter(|(_, value)| value.is_some())
                .max_by(|(_, value0), (_, value1)| value0.cmp(value1))
                .map(|(idx, _)| DIGIT_VALUES[idx])
                .unwrap();

            first_digit * 10 + last_digit
        })
        .sum::<u64>()
}

#[test]
fn test_ascii_digits() {
    let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n";
    let result = first_and_last_ascii_digits(input);
    assert_eq!(result, 142);
}

#[test]
fn test_first_and_last_ascii_and_written_digits() {
    let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen\n";
    let result = first_and_last_ascii_and_written_digits(input);
    assert_eq!(result, 281);
}

#[test]
fn edge_cases() {
    let input = "3fiveeightoneightg\n";
    let result = first_and_last_ascii_and_written_digits(input);
    assert_eq!(result, 38);
}
