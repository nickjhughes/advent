use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() -> String {
    format!("{}", total_frequency(None))
}

pub fn part2() -> String {
    format!("{}", first_revisited_frequency(None))
}

fn get_frequency_changes() -> Vec<i32> {
    let file = File::open("inputs/input01").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let mut frequency_changes = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let frequency_change = line.parse::<i32>().expect("Could not parse line to i32");
        frequency_changes.push(frequency_change);
    }
    frequency_changes
}

fn total_frequency(frequency_changes: Option<Vec<i32>>) -> i32 {
    let frequency_changes = frequency_changes.unwrap_or_else(get_frequency_changes);
    let mut frequency: i32 = 0;
    for frequency_change in frequency_changes {
        frequency += frequency_change;
    }
    frequency
}

fn first_revisited_frequency(frequency_changes: Option<Vec<i32>>) -> i32 {
    let frequency_changes = frequency_changes.unwrap_or_else(get_frequency_changes);
    let mut frequency: i32 = 0;
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
    assert_eq!(total_frequency(Some(vec![1, -2, 3, 1])), 3);
    assert_eq!(total_frequency(Some(vec![1, 1, 1])), 3);
    assert_eq!(total_frequency(Some(vec![1, 1, -2])), 0);
    assert_eq!(total_frequency(Some(vec![-1, -2, -3])), -6);
}

#[test]
fn part2_tests() {
    assert_eq!(first_revisited_frequency(Some(vec![1, -2, 3, 1])), 2);
    assert_eq!(first_revisited_frequency(Some(vec![1, -1])), 0);
    assert_eq!(first_revisited_frequency(Some(vec![3, 3, 4, -2, -4])), 10);
    assert_eq!(first_revisited_frequency(Some(vec![-6, 3, 8, 5, -6])), 5);
    assert_eq!(first_revisited_frequency(Some(vec![7, 7, -2, -7, -4])), 14);
}
