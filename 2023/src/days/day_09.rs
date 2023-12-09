use std::fs;

pub fn part1() -> String {
    let input = get_input_file_contents();
    let histories = parse_histories(&input);
    histories
        .iter()
        .map(|h| h.extrapolate())
        .sum::<i64>()
        .to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let histories = parse_histories(&input);
    histories
        .iter()
        .map(|h| h.extrapolate_backwards())
        .sum::<i64>()
        .to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input09").expect("Failed to open input file")
}

fn parse_histories(input: &str) -> Vec<History> {
    input
        .lines()
        .map(|line| {
            History(
                line.split_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

#[derive(Debug, PartialEq)]
struct History(Vec<i64>);

impl History {
    fn extrapolate(&self) -> i64 {
        let mut sequences = vec![self.0.clone()];
        loop {
            let seq = sequences.last().unwrap();
            let diffs = seq
                .iter()
                .zip(seq.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect::<Vec<i64>>();
            if diffs.iter().all(|d| *d == 0) {
                break;
            }
            sequences.push(diffs);
        }

        let mut last_value = 0;
        for seq in sequences.iter_mut().rev() {
            let new_value = seq.last().unwrap() + last_value;
            seq.push(new_value);
            last_value = new_value;
        }

        *sequences.first().unwrap().last().unwrap()
    }

    fn extrapolate_backwards(&self) -> i64 {
        let mut sequences = vec![self.0.clone()];
        loop {
            let seq = sequences.last().unwrap();
            let diffs = seq
                .iter()
                .zip(seq.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect::<Vec<i64>>();
            if diffs.iter().all(|d| *d == 0) {
                break;
            }
            sequences.push(diffs);
        }

        let mut first_value = 0;
        for seq in sequences.iter_mut().rev() {
            let new_value = seq.first().unwrap() - first_value;
            seq.insert(0, new_value);
            first_value = new_value;
        }

        *sequences.first().unwrap().first().unwrap()
    }
}

#[test]
fn test_parse() {
    let input = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45\n";
    let histories = parse_histories(input);
    assert_eq!(histories[0], History(vec![0, 3, 6, 9, 12, 15]));
    assert_eq!(histories[1], History(vec![1, 3, 6, 10, 15, 21]));
    assert_eq!(histories[2], History(vec![10, 13, 16, 21, 30, 45]));
}

#[test]
fn test_extrapolate() {
    let input = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45\n";
    let histories = parse_histories(input);
    assert_eq!(histories[0].extrapolate(), 18);
    assert_eq!(histories[1].extrapolate(), 28);
    assert_eq!(histories[2].extrapolate(), 68);
}

#[test]
fn test_extrapolate_backwards() {
    let input = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45\n";
    let histories = parse_histories(input);
    assert_eq!(histories[0].extrapolate_backwards(), -3);
    assert_eq!(histories[1].extrapolate_backwards(), 0);
    assert_eq!(histories[2].extrapolate_backwards(), 5);
}
