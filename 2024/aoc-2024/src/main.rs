use std::collections::HashMap;

fn solve(input: &str, blink_count: usize) -> usize {
    let mut stones: HashMap<u64, usize> = HashMap::new();
    for number_str in input.trim_end().split_ascii_whitespace() {
        let number = number_str
            .parse::<u64>()
            .expect("input contains valid u64s");
        stones.insert(number, 1);
    }

    for _ in 0..blink_count {
        let mut next_stones = HashMap::new();
        for (stone, count) in stones.iter() {
            if *stone == 0 {
                *next_stones.entry(1).or_default() += count;
            } else if stone.to_string().len() % 2 == 0 {
                let stone_string = stone.to_string();
                let chars_array = stone_string.bytes().collect::<Vec<u8>>();
                let new_stone_left = std::str::from_utf8(&chars_array[0..stone_string.len() / 2])
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                let new_stone_right = std::str::from_utf8(&chars_array[stone_string.len() / 2..])
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                *next_stones.entry(new_stone_left).or_default() += count;
                *next_stones.entry(new_stone_right).or_default() += count;
            } else {
                *next_stones.entry(stone * 2024).or_default() += count;
            }
        }
        stones = next_stones;
    }

    stones.values().sum()
}

fn main() {
    let result = solve("5178527 8525 22 376299 3 69312 0 275\n", 75);
    println!("{result}");
}
