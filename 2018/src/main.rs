mod days;
mod inputs;

use crate::inputs::get_day_input;
use days::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Please specify a day");
        return;
    }
    let day = args[1].parse::<u8>().expect("Failed to parse input day");

    get_day_input(day);
    let fns = get_day_functions(day);

    println!("Day {}", day);
    let result1 = (fns.part1)();
    println!("Part 1: {}", result1);
    let result2 = (fns.part2)();
    println!("Part 2: {}", result2);
}
