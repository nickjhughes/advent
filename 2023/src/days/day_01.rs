use std::fs;

pub fn part1() -> String {
    let _input = get_input_file_contents();
    "".into()
}

pub fn part2() -> String {
    let _input = get_input_file_contents();
    "".into()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input01").expect("Failed to open input file")
}
