use std::fs;

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let elf_food = parse_elf_food(&contents);
    let most_food = get_most_food(&elf_food);
    format!("{}", most_food)
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let mut elf_food = parse_elf_food(&contents);
    let top_three_food = get_total_top_three_food(&mut elf_food);
    format!("{}", top_three_food)
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input01").expect("Failed to open input file")
}

fn parse_elf_food(contents: &str) -> Vec<u32> {
    let mut elf_food = Vec::new();
    let mut current_elf_food = 0;
    for line in contents.split('\n') {
        if line.is_empty() {
            if current_elf_food > 0 {
                elf_food.push(current_elf_food);
                current_elf_food = 0;
            }
        } else {
            current_elf_food += line.parse::<u32>().expect("Failed to parse food line");
        }
    }
    if current_elf_food > 0 {
        elf_food.push(current_elf_food);
    }
    elf_food
}

fn get_most_food(elf_food: &[u32]) -> u32 {
    *elf_food.iter().max().unwrap_or(&0)
}

fn get_total_top_three_food(elf_food: &mut [u32]) -> u32 {
    elf_food.sort();
    elf_food.reverse();
    elf_food[0..3].iter().sum()
}

#[test]
fn test_parse() {
    let contents = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
    let elf_food = parse_elf_food(&contents);
    assert_eq!(
        elf_food,
        vec![
            1000 + 2000 + 3000,
            4000,
            5000 + 6000,
            7000 + 8000 + 9000,
            10000
        ]
    );

    let contents = "1\n2\n3\n\n\n4\n5\n";
    let elf_food = parse_elf_food(&contents);
    assert_eq!(elf_food, vec![1 + 2 + 3, 4 + 5]);

    let contents = "";
    let elf_food = parse_elf_food(&contents);
    dbg!(&elf_food);
    assert!(elf_food.is_empty());
}

#[test]
fn test_most_food() {
    let elf_food = vec![
        1000 + 2000 + 3000,
        4000,
        5000 + 6000,
        7000 + 8000 + 9000,
        10000,
    ];
    assert_eq!(get_most_food(&elf_food), 24000)
}

#[test]
fn test_top_three() {
    let mut elf_food = vec![
        1000 + 2000 + 3000,
        4000,
        5000 + 6000,
        7000 + 8000 + 9000,
        10000,
    ];
    assert_eq!(get_total_top_three_food(&mut elf_food), 45000)
}
