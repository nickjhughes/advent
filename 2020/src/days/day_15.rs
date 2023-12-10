use std::{collections::HashMap, fs};

pub fn part1() -> String {
    let input = get_input_file_contents();
    let starting_numbers = parse(&input);
    nth_number(&starting_numbers, 2020 - 1).to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let starting_numbers = parse(&input);
    nth_number(&starting_numbers, 30000000 - 1).to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input15").expect("Failed to open input file")
}

fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

fn nth_number(starting_numbers: &[u64], n: usize) -> u64 {
    let mut last_spoken: HashMap<u64, usize> = HashMap::new();

    let mut last_n = 0;
    let mut last_number = starting_numbers[last_n];
    last_spoken.insert(last_number, last_n);

    while last_n != n {
        let next_number = if last_n < starting_numbers.len() - 1 {
            starting_numbers[last_n + 1]
        } else if let Some(last_spoken_n) = last_spoken.get(&last_number) {
            (last_n - last_spoken_n) as u64
        } else {
            0
        };
        last_spoken.insert(last_number, last_n);
        last_number = next_number;
        last_n += 1;
    }

    last_number
}

#[test]
fn test_parse() {
    let input = "0,3,6\n";
    assert_eq!(parse(input), vec![0, 3, 6]);
}

#[test]
fn test_nth_number() {
    let input = "0,3,6\n";
    let starting_numbers = parse(&input);
    assert_eq!(nth_number(&starting_numbers, 0), 0);
    assert_eq!(nth_number(&starting_numbers, 1), 3);
    assert_eq!(nth_number(&starting_numbers, 2), 6);
    assert_eq!(nth_number(&starting_numbers, 3), 0);
    assert_eq!(nth_number(&starting_numbers, 4), 3);
    assert_eq!(nth_number(&starting_numbers, 5), 3);
    assert_eq!(nth_number(&starting_numbers, 6), 1);
    assert_eq!(nth_number(&starting_numbers, 7), 0);
    assert_eq!(nth_number(&starting_numbers, 8), 4);
    assert_eq!(nth_number(&starting_numbers, 9), 0);
}

#[test]
fn test_2020th_number() {
    assert_eq!(nth_number(&parse("0,3,6"), 2019), 436);
    assert_eq!(nth_number(&parse("1,3,2"), 2019), 1);
    assert_eq!(nth_number(&parse("2,1,3"), 2019), 10);
    assert_eq!(nth_number(&parse("1,2,3"), 2019), 27);
    assert_eq!(nth_number(&parse("2,3,1"), 2019), 78);
    assert_eq!(nth_number(&parse("3,2,1"), 2019), 438);
    assert_eq!(nth_number(&parse("3,1,2"), 2019), 1836);
}

#[test]
#[ignore]
fn test_30000000th_number() {
    assert_eq!(nth_number(&parse("0,3,6"), 30000000 - 1), 175594);
    assert_eq!(nth_number(&parse("1,3,2"), 30000000 - 1), 2578);
    assert_eq!(nth_number(&parse("2,1,3"), 30000000 - 1), 3544142);
    assert_eq!(nth_number(&parse("1,2,3"), 30000000 - 1), 261214);
    assert_eq!(nth_number(&parse("2,3,1"), 30000000 - 1), 6895259);
    assert_eq!(nth_number(&parse("3,2,1"), 30000000 - 1), 18);
    assert_eq!(nth_number(&parse("3,1,2"), 30000000 - 1), 362);
}
