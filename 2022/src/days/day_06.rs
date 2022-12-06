use std::{collections::HashSet, fs};

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let packet_start_index = get_packet_start_index(&contents);
    format!("{}", packet_start_index)
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let message_start_index = get_message_start_index(&contents);
    format!("{}", message_start_index)
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input06").expect("Failed to open input file")
}

fn index_of_first_n_unique_chars(data: &str, n: usize) -> Option<usize> {
    let chars = data.chars().collect::<Vec<char>>();
    let mut unique_chars = HashSet::with_capacity(n);
    for (i, window) in chars.windows(n).enumerate() {
        unique_chars.clear();
        for c in window {
            unique_chars.insert(c);
        }
        if unique_chars.len() == n {
            return Some(i + n);
        }
    }
    None
}

fn get_packet_start_index(data: &str) -> usize {
    index_of_first_n_unique_chars(data, 4).expect("Failed to find packet start")
}

fn get_message_start_index(data: &str) -> usize {
    index_of_first_n_unique_chars(data, 14).expect("Failed to find message start")
}

#[test]
fn test_index_of_first_n_unique_chars() {
    assert_eq!(index_of_first_n_unique_chars("abcd", 4), Some(4));
    assert_eq!(index_of_first_n_unique_chars("abcabcabcdefg", 5), Some(11));

    assert_eq!(index_of_first_n_unique_chars("abca", 4), None);
    assert_eq!(index_of_first_n_unique_chars("a", 5), None);
}

#[test]
fn test_packet_start() {
    let contents = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let packet_start_index = get_packet_start_index(&contents);
    assert_eq!(packet_start_index, 7);

    let contents = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let packet_start_index = get_packet_start_index(&contents);
    assert_eq!(packet_start_index, 5);

    let contents = "nppdvjthqldpwncqszvftbrmjlhg";
    let packet_start_index = get_packet_start_index(&contents);
    assert_eq!(packet_start_index, 6);

    let contents = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let packet_start_index = get_packet_start_index(&contents);
    assert_eq!(packet_start_index, 10);

    let contents = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    let packet_start_index = get_packet_start_index(&contents);
    assert_eq!(packet_start_index, 11);
}

#[test]
fn test_message_start() {
    let contents = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let message_start_index = get_message_start_index(&contents);
    assert_eq!(message_start_index, 19);

    let contents = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let message_start_index = get_message_start_index(&contents);
    assert_eq!(message_start_index, 23);

    let contents = "nppdvjthqldpwncqszvftbrmjlhg";
    let message_start_index = get_message_start_index(&contents);
    assert_eq!(message_start_index, 23);

    let contents = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let message_start_index = get_message_start_index(&contents);
    assert_eq!(message_start_index, 29);

    let contents = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    let message_start_index = get_message_start_index(&contents);
    assert_eq!(message_start_index, 26);
}
