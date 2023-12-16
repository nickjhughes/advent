use std::{collections::HashMap, fs};

pub fn part1() -> String {
    let input = get_input_file_contents();
    let orbits = parse_orbits(&input);
    total_orbits(&orbits).to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let orbits = parse_orbits(&input);
    transfers_required(&orbits).to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input06").expect("Failed to open input file")
}

fn parse_orbits(input: &str) -> HashMap<&str, &str> {
    input
        .lines()
        .map(|line| {
            let (parent, object) = line.split_once(')').unwrap();
            (object, parent)
        })
        .collect()
}

fn total_orbits(orbits: &HashMap<&str, &str>) -> usize {
    let mut total_orbits = 0;
    for mut object in orbits.keys() {
        let mut object_orbits = 0;
        while let Some(parent) = orbits.get(*object) {
            object_orbits += 1;
            object = parent;
        }
        total_orbits += object_orbits;
    }
    total_orbits
}

fn transfers_required(orbits: &HashMap<&str, &str>) -> usize {
    let mut you_parents = Vec::new();
    let mut object = orbits.get("YOU").unwrap();
    while let Some(parent) = orbits.get(*object) {
        you_parents.push(object);
        object = parent;
    }

    let mut san_parents = Vec::new();
    let mut object = orbits.get("SAN").unwrap();
    while let Some(parent) = orbits.get(*object) {
        san_parents.push(object);
        object = parent;
    }

    let mut shared_parent = 0;
    for (you, san) in you_parents.iter().rev().zip(san_parents.iter().rev()) {
        shared_parent += 1;
        if you != san {
            break;
        }
    }
    you_parents.iter().rev().skip(shared_parent).count()
        + 1
        + san_parents.iter().rev().skip(shared_parent).count()
        + 1
}

#[test]
fn test_parse() {
    let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\n";
    let orbits = parse_orbits(input);
    let expected_orbits = {
        let mut map = HashMap::new();
        map.insert("B", "COM");
        map.insert("C", "B");
        map.insert("D", "C");
        map.insert("E", "D");
        map.insert("F", "E");
        map.insert("G", "B");
        map.insert("H", "G");
        map.insert("I", "D");
        map.insert("J", "E");
        map.insert("K", "J");
        map.insert("L", "K");
        map
    };
    assert_eq!(orbits, expected_orbits);
}

#[test]
fn test_total_orbits() {
    let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\n";
    let orbits = parse_orbits(input);
    assert_eq!(total_orbits(&orbits), 42);
}

#[test]
fn test_transfers_required() {
    let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN\n";
    let orbits = parse_orbits(input);
    assert_eq!(transfers_required(&orbits), 4);
}
