use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_frequency_changes() -> Vec<i64> {
    let file = File::open("inputs/input01").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let mut frequency_changes = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let frequency_change = line.parse::<i64>().expect("Could not parse line to i64");
        frequency_changes.push(frequency_change);
    }
    frequency_changes
}

pub fn part1(frequency_changes: Option<Vec<i64>>) -> i64 {
    let frequency_changes = frequency_changes.unwrap_or(get_frequency_changes());
    let mut frequency: i64 = 0;
    for frequency_change in frequency_changes {
        frequency += frequency_change;
    }
    frequency
}

pub fn part2(frequency_changes: Option<Vec<i64>>) -> i64 {
    let frequency_changes = frequency_changes.unwrap_or(get_frequency_changes());
    let mut frequency: i64 = 0;
    let mut frequencies_seen = HashSet::from([0]);

    let mut i = 0;
    loop {
        frequency += frequency_changes[i];
        if frequencies_seen.contains(&frequency) {
            return frequency;
        } else {
            frequencies_seen.insert(frequency);
        }

        i += 1;
        if i == frequency_changes.len() {
            i = 0;
        }
    }
}

#[test]
fn part1_tests() {
    assert_eq!(part1(Some(vec![1, -2, 3, 1])), 3);
    assert_eq!(part1(Some(vec![1, 1, 1])), 3);
    assert_eq!(part1(Some(vec![1, 1, -2])), 0);
    assert_eq!(part1(Some(vec![-1, -2, -3])), -6);
}

#[test]
fn part2_tests() {
    assert_eq!(part2(Some(vec![1, -2, 3, 1])), 2);
    assert_eq!(part2(Some(vec![1, -1])), 0);
    assert_eq!(part2(Some(vec![3, 3, 4, -2, -4])), 10);
    assert_eq!(part2(Some(vec![-6, 3, 8, 5, -6])), 5);
    assert_eq!(part2(Some(vec![7, 7, -2, -7, -4])), 14);
}
