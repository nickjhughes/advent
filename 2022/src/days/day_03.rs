use std::collections::HashSet;
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Item(char);

impl Item {
    pub fn priority(&self) -> u32 {
        match self.0 {
            'a'..='z' => ((self.0 as u8) - 96) as u32,
            'A'..='Z' => ((self.0 as u8) - 65 + 27) as u32,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Rucksack {
    items: Vec<Item>,
}

impl Rucksack {
    fn compartment_size(&self) -> usize {
        self.items.len() / 2
    }

    fn first_compartment_items(&self) -> &[Item] {
        &self.items[0..self.compartment_size()]
    }

    fn second_compartment_items(&self) -> &[Item] {
        &self.items[self.compartment_size()..]
    }

    pub fn shared_item(&self) -> &Item {
        for item1 in self.first_compartment_items() {
            for item2 in self.second_compartment_items() {
                if item1 == item2 {
                    return item1;
                }
            }
        }
        panic!("Could not find any shared items in both rucksack compartments");
    }

    pub fn unique_items(&self) -> HashSet<&Item> {
        HashSet::from_iter(self.items.iter())
    }
}

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let rucksacks = parse_rucksacks(&contents);
    let shared_priorities_sum = shared_comparment_item_priorities_sum(rucksacks);
    format!("{}", shared_priorities_sum).to_string()
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let rucksacks = parse_rucksacks(&contents);
    let group_priorities_sum = shared_group_item_priorities_sum(rucksacks);
    format!("{}", group_priorities_sum).to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input03").expect("Failed to open input file")
}

fn parse_rucksacks(contents: &str) -> Vec<Rucksack> {
    let mut rucksacks = Vec::new();
    for line in contents.split("\n") {
        if line.is_empty() {
            continue;
        }
        let items = line.chars().map(|c| Item(c)).collect::<Vec<Item>>();
        rucksacks.push(Rucksack { items });
    }
    rucksacks
}

fn shared_comparment_item_priorities_sum(rucksacks: Vec<Rucksack>) -> u32 {
    let mut sum = 0;
    for rucksack in &rucksacks {
        sum += rucksack.shared_item().priority();
    }
    sum
}

fn shared_group_item_priorities_sum(rucksacks: Vec<Rucksack>) -> u32 {
    let mut sum = 0;
    for i in (0..rucksacks.len()).step_by(3) {
        let mut rucksack_unique_items = Vec::new();
        for rucksack in &rucksacks[i..i + 3] {
            rucksack_unique_items.push(rucksack.unique_items());
        }
        let group_items = rucksack_unique_items
            .iter()
            .skip(1)
            .fold(rucksack_unique_items[0].clone(), |set1, set2| {
                set1.intersection(set2).cloned().collect()
            });
        assert_eq!(group_items.len(), 1);
        sum += group_items.iter().next().unwrap().priority();
    }
    sum
}

#[test]
fn test_parse_rucksacks() {
    let contents = "vJrwpW\njqHR\n";
    let rucksacks = parse_rucksacks(&contents);
    assert_eq!(rucksacks[0].compartment_size(), 3);
    assert_eq!(
        rucksacks[0].first_compartment_items(),
        vec![Item('v'), Item('J'), Item('r')]
    );
    assert_eq!(
        rucksacks[0].second_compartment_items(),
        vec![Item('w'), Item('p'), Item('W')]
    );

    assert_eq!(rucksacks[1].compartment_size(), 2);
    assert_eq!(
        rucksacks[1].first_compartment_items(),
        vec![Item('j'), Item('q')]
    );
    assert_eq!(
        rucksacks[1].second_compartment_items(),
        vec![Item('H'), Item('R')]
    );
}

#[test]
fn test_item_priority() {
    let item = Item('p');
    assert_eq!(item.priority(), 16);

    let item = Item('L');
    assert_eq!(item.priority(), 38);
}

#[test]
fn test_shared_comparment_item_priorities_sum() {
    let contents = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw\n";
    let rucksacks = parse_rucksacks(&contents);
    assert_eq!(shared_comparment_item_priorities_sum(rucksacks), 157);
}

#[test]
fn test_shared_group_item_priorities_sum() {
    let contents = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw\n";
    let rucksacks = parse_rucksacks(&contents);
    assert_eq!(shared_group_item_priorities_sum(rucksacks), 70);
}
