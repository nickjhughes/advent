use std::{collections::HashMap, fs};

pub fn part1() -> String {
    let input = get_input_file_contents();
    let adaptors = parse_adaptors(&input);
    let ordered_joltages = ordered_joltages(&adaptors);
    let differences = differences(&ordered_joltages);
    (differences.0 * differences.1).to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let adaptors = parse_adaptors(&input);
    total_arrangements(&adaptors).to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input10").expect("Failed to open input file")
}

fn parse_adaptors(input: &str) -> Vec<u8> {
    input
        .lines()
        .map(|line| line.parse::<u8>().unwrap())
        .collect()
}

fn ordered_joltages(adaptors: &[u8]) -> Vec<u8> {
    let mut sorted = adaptors.to_vec();
    sorted.push(0);
    sorted.push(sorted.iter().max().unwrap() + 3);
    sorted.sort_unstable();
    sorted
}

/// 1- and 3-jolt differences in the ordered list.
fn differences(joltages: &[u8]) -> (usize, usize) {
    let mut differences: HashMap<u8, usize> = HashMap::new();
    for (j1, j2) in joltages.iter().zip(joltages.iter().skip(1)) {
        *differences.entry(j2 - j1).or_default() += 1;
    }
    (
        *differences.get(&1).unwrap_or(&0),
        *differences.get(&3).unwrap_or(&0),
    )
}

fn total_arrangements(adaptors: &[u8]) -> usize {
    let adaptors = {
        let mut a = adaptors.to_vec();
        a.push(0);
        a.push(a.iter().max().unwrap() + 3);
        a.sort_unstable();
        a
    };
    arrangements(&adaptors, 0, &mut HashMap::new())
}

fn arrangements(adaptors: &[u8], start: usize, states: &mut HashMap<usize, usize>) -> usize {
    if start == adaptors.len() - 1 {
        return 1;
    }

    let next_options = adaptors[start..]
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(_, a)| **a <= adaptors[start] + 3)
        .map(|(i, _)| start + i)
        .collect::<Vec<usize>>();
    let mut count = 0;
    for next in next_options {
        if let Some(next_count) = states.get(&next) {
            count += next_count;
        } else {
            let next_count = arrangements(adaptors, next, states);
            states.insert(next, next_count);
            count += next_count;
        }
    }
    count
}

#[test]
fn test_parse() {
    let input = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4\n";
    let adaptors = parse_adaptors(input);
    assert_eq!(adaptors, vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]);
}

#[test]
fn test_ordered_differences() {
    {
        let input = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4\n";
        let adaptors = parse_adaptors(input);
        let ordered_joltages = ordered_joltages(&adaptors);
        let differences = differences(&ordered_joltages);
        assert_eq!(differences.0, 7);
        assert_eq!(differences.1, 5);
    }

    {
        let input = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3\n";
        let adaptors = parse_adaptors(input);
        let ordered_joltages = ordered_joltages(&adaptors);
        let differences = differences(&ordered_joltages);
        assert_eq!(differences.0, 22);
        assert_eq!(differences.1, 10);
    }
}

#[test]
fn test_arrangements() {
    {
        let input = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4\n";
        let adaptors = parse_adaptors(input);
        assert_eq!(total_arrangements(&adaptors), 8);
    }

    {
        let input = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3\n";
        let adaptors = parse_adaptors(input);
        assert_eq!(total_arrangements(&adaptors), 19208);
    }
}
