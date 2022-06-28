use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() -> String {
    format!("{}", multiply_twos_and_threes(None))
}

pub fn part2() -> String {
    closest_box_ids_shared_chars(None)
}

fn get_box_ids() -> Vec<String> {
    let file = File::open("inputs/input02").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let mut box_ids = Vec::new();
    for line in reader.lines() {
        box_ids.push(line.expect("Failed to read line"));
    }
    box_ids
}

fn multiply_twos_and_threes(box_ids: Option<Vec<String>>) -> u64 {
    let box_ids = box_ids.unwrap_or_else(get_box_ids);
    let mut two_count = 0;
    let mut three_count = 0;
    for box_id in &box_ids {
        if n_of_any_letter(box_id, 2) {
            two_count += 1;
        }
        if n_of_any_letter(box_id, 3) {
            three_count += 1;
        }
    }
    two_count * three_count
}

fn n_of_any_letter(box_id: &str, n: u64) -> bool {
    let mut letter_counts: HashMap<char, u64> = HashMap::new();
    let chars: Vec<char> = box_id.chars().collect();
    for c in chars {
        *letter_counts.entry(c).or_insert(0) += 1;
    }
    for &count in letter_counts.values() {
        if count == n {
            return true;
        }
    }
    false
}

fn closest_box_ids(box_ids: Vec<String>) -> (String, String) {
    for id1 in &box_ids {
        for id2 in &box_ids {
            if id1 == id2 {
                continue;
            }
            let mut difference_count = 0;
            for (c1, c2) in id1.chars().zip(id2.chars()) {
                if c1 != c2 {
                    difference_count += 1;
                }
            }
            if difference_count == 1 {
                return (id1.clone(), id2.clone());
            }
        }
    }
    //panic!("Failed to find closest box IDs");
    ("".to_string(), "".to_string())
}

fn closest_box_ids_shared_chars(box_ids: Option<Vec<String>>) -> String {
    let box_ids = box_ids.unwrap_or_else(get_box_ids);
    let (id1, id2) = closest_box_ids(box_ids);
    let mut shared_chars = String::new();
    for (c1, c2) in id1.chars().zip(id2.chars()) {
        if c1 == c2 {
            shared_chars.push(c1);
        }
    }
    shared_chars
}

#[test]
fn part1_tests() {
    assert_eq!(n_of_any_letter("abcdef", 2), false);
    assert_eq!(n_of_any_letter("abcdef", 3), false);
    assert_eq!(n_of_any_letter("bababc", 2), true);
    assert_eq!(n_of_any_letter("bababc", 3), true);
    assert_eq!(n_of_any_letter("abbcde", 2), true);
    assert_eq!(n_of_any_letter("abbcde", 3), false);
    assert_eq!(n_of_any_letter("abcccd", 2), false);
    assert_eq!(n_of_any_letter("abcccd", 3), true);
    assert_eq!(n_of_any_letter("aabcdd", 2), true);
    assert_eq!(n_of_any_letter("aabcdd", 3), false);
    assert_eq!(n_of_any_letter("abcdee", 2), true);
    assert_eq!(n_of_any_letter("abcdee", 3), false);
    assert_eq!(n_of_any_letter("ababab", 2), false);
    assert_eq!(n_of_any_letter("ababab", 3), true);

    assert_eq!(
        multiply_twos_and_threes(Some(vec![
            "abcdef".to_string(),
            "bababc".to_string(),
            "abbcde".to_string(),
            "abcccd".to_string(),
            "aabcdd".to_string(),
            "abcdee".to_string(),
            "ababab".to_string()
        ])),
        12
    );
}

#[test]
fn part2_tests() {
    assert_eq!(
        closest_box_ids(vec![
            "abcde".to_string(),
            "fghij".to_string(),
            "klmno".to_string(),
            "pqrst".to_string(),
            "fguij".to_string(),
            "axcye".to_string(),
            "wvxyz".to_string(),
        ]),
        ("fghij".to_string(), "fguij".to_string())
    );

    assert_eq!(
        closest_box_ids_shared_chars(Some(vec![
            "abcde".to_string(),
            "fghij".to_string(),
            "klmno".to_string(),
            "pqrst".to_string(),
            "fguij".to_string(),
            "axcye".to_string(),
            "wvxyz".to_string(),
        ])),
        "fgij"
    );
}
