use std::fs;

pub fn part1() -> String {
    let input = get_input_file_contents();
    let (pub_key1, pub_key2) = parse_public_keys(&input);
    find_encryption_key(pub_key1, pub_key2).to_string()
}

pub fn part2() -> String {
    let _input = get_input_file_contents();
    "".into()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input25").expect("Failed to open input file")
}

fn parse_public_keys(input: &str) -> (u64, u64) {
    let key1 = input.lines().next().unwrap().parse::<u64>().unwrap();
    let key2 = input.lines().nth(1).unwrap().parse::<u64>().unwrap();
    (key1, key2)
}

fn find_encryption_key(pub_key1: u64, pub_key2: u64) -> u64 {
    let loop_size1 = find_loop_size(pub_key1);
    let loop_size2 = find_loop_size(pub_key2);

    let enc_key = transform(pub_key2, loop_size1);
    assert_eq!(enc_key, transform(pub_key1, loop_size2));
    enc_key
}

fn find_loop_size(pub_key: u64) -> u64 {
    let mut loops = 0;
    let mut value = 1;
    loop {
        value *= 7;
        value %= 20201227;

        loops += 1;
        if value == pub_key {
            return loops;
        }
    }
}

fn transform(subject_number: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= subject_number;
        value %= 20201227;
    }
    value
}

#[test]
fn test_parse_public_keys() {
    let input = "5764801\n17807724\n";
    let (pub_key1, pub_key2) = parse_public_keys(input);
    assert_eq!(pub_key1, 5764801);
    assert_eq!(pub_key2, 17807724);
}

#[test]
fn test_transform() {
    assert_eq!(transform(7, 8), 5764801);
    assert_eq!(transform(7, 11), 17807724);

    assert_eq!(transform(17807724, 8), 14897079);
    assert_eq!(transform(5764801, 11), 14897079);
}

#[test]
fn test_find_loop_size() {
    assert_eq!(find_loop_size(5764801), 8);
    assert_eq!(find_loop_size(17807724), 11);
}

#[test]
fn test_find_encryption_key() {
    let input = "5764801\n17807724\n";
    let (pub_key1, pub_key2) = parse_public_keys(input);
    assert_eq!(find_encryption_key(pub_key1, pub_key2), 14897079);
}
