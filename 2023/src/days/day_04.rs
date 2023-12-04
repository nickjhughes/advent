use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn part1() -> String {
    let input = get_input_file_contents();
    let cards = parse_cards(&input);
    let total_points = cards.iter().map(|c| c.points()).sum::<u64>();
    total_points.to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let cards = parse_cards(&input);
    let total_cards_count = total_cards(&cards);
    total_cards_count.to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input04").expect("Failed to open input file")
}

#[derive(Debug, PartialEq)]
struct Card {
    matching_numbers: usize,
}

impl Card {
    fn parse(line: &str) -> Self {
        let (winning_str, your_str) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let winning_numbers: HashSet<u8> = winning_str
            .split_whitespace()
            .map(|n| n.parse::<u8>().unwrap())
            .collect();
        let your_numbers: HashSet<u8> = your_str
            .split_whitespace()
            .map(|n| n.parse::<u8>().unwrap())
            .collect();
        let matching_numbers = winning_numbers.intersection(&your_numbers).count();
        Card { matching_numbers }
    }

    fn points(&self) -> u64 {
        if self.matching_numbers == 0 {
            0
        } else {
            2u64.pow((self.matching_numbers - 1) as u32)
        }
    }
}

fn parse_cards(input: &str) -> Vec<Card> {
    input.lines().map(Card::parse).collect()
}

fn total_cards(cards: &[Card]) -> u64 {
    let mut card_counts: HashMap<usize, u64> = HashMap::with_capacity(cards.len());
    for i in 0..cards.len() {
        card_counts.insert(i, 1);
    }

    for (i, card) in cards.iter().enumerate() {
        let this_card_count = *card_counts.get(&i).unwrap();
        for j in i + 1..i + 1 + card.matching_numbers {
            *card_counts.get_mut(&j).unwrap() += this_card_count;
        }
    }

    card_counts.values().sum()
}

#[test]
fn test_parse_card() {
    let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    let card = Card::parse(line);
    assert_eq!(
        card,
        Card {
            matching_numbers: 4
        }
    );
}

#[test]
fn test_card_points() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n";
    let cards = parse_cards(input);
    assert_eq!(cards[0].points(), 8);
    assert_eq!(cards[1].points(), 2);
    assert_eq!(cards[2].points(), 2);
    assert_eq!(cards[3].points(), 1);
    assert_eq!(cards[4].points(), 0);
    assert_eq!(cards[5].points(), 0);
}

#[test]
fn test_total_cards() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n";
    let cards = parse_cards(input);
    let total_cards_count = total_cards(&cards);
    assert_eq!(total_cards_count, 30);
}
