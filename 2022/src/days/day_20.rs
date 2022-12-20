use std::{cmp, fs};

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let numbers = parse_numbers(&contents);
    let mixed_numbers = mix_numbers(&numbers, 1);
    let coordinates_sum = get_coordinates_sum(&mixed_numbers, &[1000, 2000, 3000]);
    format!("{}", coordinates_sum)
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let numbers = parse_numbers(&contents);
    let decrypted_numbers = decrypt_numbers(&numbers);
    let coordinates_sum = get_coordinates_sum(&decrypted_numbers, &[1000, 2000, 3000]);
    format!("{}", coordinates_sum)
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input20").expect("Failed to open input file")
}

fn parse_numbers(contents: &str) -> Vec<i64> {
    let mut numbers = Vec::new();
    for line in contents.lines() {
        numbers.push(line.parse::<i64>().expect("Failed to parse number"));
    }
    numbers
}

// Move the element at the given index one step to the left (wrapping as appropriate)
fn move_left<T>(slice: &mut [T], index: usize) {
    if index == 0 {
        slice.swap(index, slice.len() - 1);
    } else {
        slice.swap(index, index - 1);
    }
}

// Move the element at the given index one step to the right (wrapping as appropriate)
fn move_right<T>(slice: &mut [T], index: usize) {
    if index == slice.len() - 1 {
        slice.swap(index, 0);
    } else {
        slice.swap(index, index + 1);
    }
}

// Move the element at the given index n steps to the left (wrapping as appropriate)
fn move_left_n_times<T>(slice: &mut [T], index: usize, count: usize) {
    let reduced_count = count % (slice.len() - 1);
    let mut current_index = index;
    for _ in 0..reduced_count {
        move_left(slice, current_index);
        if current_index == 0 {
            current_index = slice.len() - 1;
        } else {
            current_index -= 1;
        }
    }
}

// Move the element at the given index n steps to the right (wrapping as appropriate)
fn move_right_n_times<T>(slice: &mut [T], index: usize, count: usize) {
    let reduced_count = count % (slice.len() - 1);
    let mut current_index = index;
    for _ in 0..reduced_count {
        move_right(slice, current_index);
        if current_index == slice.len() - 1 {
            current_index = 0;
        } else {
            current_index += 1;
        }
    }
}

fn mix_numbers(numbers: &[i64], rounds: usize) -> Vec<i64> {
    let mut numbers_with_orig_indices = numbers
        .iter()
        .cloned()
        .enumerate()
        .collect::<Vec<(usize, i64)>>();
    for _ in 0..rounds {
        for i in 0..numbers.len() {
            let current_index = numbers_with_orig_indices
                .iter()
                .position(|(orig_i, _)| *orig_i == i)
                .unwrap();
            let number = numbers_with_orig_indices[current_index].1;
            match number.cmp(&0) {
                cmp::Ordering::Less => {
                    move_left_n_times(
                        &mut numbers_with_orig_indices,
                        current_index,
                        number.abs() as usize,
                    );
                }
                cmp::Ordering::Greater => move_right_n_times(
                    &mut numbers_with_orig_indices,
                    current_index,
                    number as usize,
                ),
                cmp::Ordering::Equal => {}
            }
        }
    }
    numbers_with_orig_indices
        .iter()
        .map(|(_, n)| *n)
        .collect::<Vec<i64>>()
}

fn decrypt_numbers(numbers: &[i64]) -> Vec<i64> {
    const DECRYPTION_KEY: i64 = 811589153;
    const MIX_ROUNDS: usize = 10;

    let multiplied_numbers = numbers
        .iter()
        .map(|n| n * DECRYPTION_KEY)
        .collect::<Vec<i64>>();
    mix_numbers(&multiplied_numbers, MIX_ROUNDS)
}

fn get_coordinate(numbers: &[i64], index: usize) -> i64 {
    let zero_index = numbers
        .iter()
        .position(|n| *n == 0)
        .expect("0 not found in numbers");
    numbers[(zero_index + index) % numbers.len()]
}

fn get_coordinates_sum(numbers: &[i64], indices: &[usize]) -> i64 {
    indices.iter().map(|i| get_coordinate(numbers, *i)).sum()
}

#[test]
fn test_parse_numbers() {
    let contents = "1\n2\n-3\n3\n-2\n0\n4\n";
    let numbers = parse_numbers(contents);
    assert_eq!(numbers.len(), 7);
    assert_eq!(numbers, vec![1, 2, -3, 3, -2, 0, 4]);
}

#[test]
fn test_mix_numbers() {
    let numbers = vec![1, 2, -3, 3, -2, 0, 4];
    let result = mix_numbers(&numbers, 1);
    assert_eq!(result, vec![-2, 1, 2, -3, 4, 0, 3]);
}

#[test]
fn test_mix_big_numbers() {
    let numbers = vec![
        811589153,
        1623178306,
        -2434767459,
        2434767459,
        -1623178306,
        0,
        3246356612,
    ];
    let result = mix_numbers(&numbers, 1);
    assert_eq!(
        result,
        vec![
            811589153,
            0,
            -2434767459,
            3246356612,
            -1623178306,
            2434767459,
            1623178306,
        ]
    );
}

#[test]
fn test_decrypt_numbers() {
    let numbers = vec![1, 2, -3, 3, -2, 0, 4];
    let result = decrypt_numbers(&numbers);
    assert_eq!(
        result,
        vec![
            3246356612,
            -1623178306,
            2434767459,
            811589153,
            0,
            -2434767459,
            1623178306,
        ]
    );
}

#[test]
fn test_get_coordinate() {
    let numbers = &[1, 2, -3, 4, 0, 3, -2];
    assert_eq!(get_coordinate(numbers, 1000), 4);
    assert_eq!(get_coordinate(numbers, 2000), -3);
    assert_eq!(get_coordinate(numbers, 3000), 2);
}

#[test]
fn test_get_coordinates_sum() {
    let numbers = &[1, 2, -3, 4, 0, 3, -2];
    assert_eq!(get_coordinates_sum(numbers, &[1000, 2000, 3000]), 3);
}

#[test]
fn test_move_left_and_right() {
    {
        let mut vec = vec![-2, 4, 5, 6, 7, 8, 9];
        move_left(&mut vec, 0);
        assert_eq!(vec, vec![9, 4, 5, 6, 7, 8, -2]);
    }

    {
        let mut vec = vec![4, 5, 6, 1, 7, 8, 9];
        move_right(&mut vec, 3);
        assert_eq!(vec, vec![4, 5, 6, 7, 1, 8, 9]);
    }
}

#[test]
fn test_move_left_and_right_n_times() {
    {
        let mut vec = vec![1, -3, 2, 3, -2, 0, 4];
        move_left_n_times(&mut vec, 1, 3);
        assert_eq!(vec, vec![4, 1, 2, 3, -2, -3, 0]);
    }
    {
        let mut vec = vec![1, 2, -2, -3, 0, 3, 4];
        move_left_n_times(&mut vec, 2, 2);
        assert_eq!(vec, vec![-2, 1, 2, -3, 0, 3, 4]);
    }

    {
        let mut vec = vec![2, 1, -3, 3, -2, 0, 4];
        move_right_n_times(&mut vec, 0, 2);
        assert_eq!(vec, vec![1, -3, 2, 3, -2, 0, 4]);
    }
}

// #[test]
// fn test_reduced_move_count() {
//     let mut vec = vec![0, 1, 2, 3, 4, 5, 6];
//     move_left_n_times(&mut vec, 2, 6);
//     assert_eq!(vec, vec![0, 1, 2, 3, 4, 5, 6]);
// }
