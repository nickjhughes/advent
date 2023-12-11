use std::{collections::HashMap, fs};

pub fn part1() -> String {
    let input = get_input_file_contents();
    let mut hands = parse_hands(&input);
    total_winnings(&mut hands).to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let mut hands = joker_parse_hands(&input);
    joker_total_winnings(&mut hands).to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input07").expect("Failed to open input file")
}

fn parse_hands(input: &str) -> Vec<Hand> {
    input.lines().map(Hand::parse).collect()
}

fn joker_parse_hands(input: &str) -> Vec<JokerHand> {
    input.lines().map(JokerHand::parse).collect()
}

fn total_winnings(hands: &mut [Hand]) -> u64 {
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            let rank = (i + 1) as u64;
            rank * hand.bid
        })
        .sum()
}

fn joker_total_winnings(hands: &mut [JokerHand]) -> u64 {
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            let rank = (i + 1) as u64;
            rank * hand.bid
        })
        .sum()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(u8)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(u8)]
enum JokerCard {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl TryFrom<u8> for Card {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'2' => Ok(Card::Two),
            b'3' => Ok(Card::Three),
            b'4' => Ok(Card::Four),
            b'5' => Ok(Card::Five),
            b'6' => Ok(Card::Six),
            b'7' => Ok(Card::Seven),
            b'8' => Ok(Card::Eight),
            b'9' => Ok(Card::Nine),
            b'T' => Ok(Card::Ten),
            b'J' => Ok(Card::Jack),
            b'Q' => Ok(Card::Queen),
            b'K' => Ok(Card::King),
            b'A' => Ok(Card::Ace),
            _ => anyhow::bail!("invalid card {value}"),
        }
    }
}

impl TryFrom<u8> for JokerCard {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'J' => Ok(JokerCard::Joker),
            b'2' => Ok(JokerCard::Two),
            b'3' => Ok(JokerCard::Three),
            b'4' => Ok(JokerCard::Four),
            b'5' => Ok(JokerCard::Five),
            b'6' => Ok(JokerCard::Six),
            b'7' => Ok(JokerCard::Seven),
            b'8' => Ok(JokerCard::Eight),
            b'9' => Ok(JokerCard::Nine),
            b'T' => Ok(JokerCard::Ten),
            b'Q' => Ok(JokerCard::Queen),
            b'K' => Ok(JokerCard::King),
            b'A' => Ok(JokerCard::Ace),
            _ => anyhow::bail!("invalid card {value}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct JokerHand {
    cards: [JokerCard; 5],
    bid: u64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn parse(line: &str) -> Self {
        let (cards_str, bid_str) = line.split_once(' ').unwrap();
        let bid = bid_str.parse::<u64>().unwrap();
        let cards = cards_str
            .bytes()
            .map(|b| Card::try_from(b).unwrap())
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();
        Hand { cards, bid }
    }

    fn ty(&self) -> HandType {
        let mut card_counts: HashMap<Card, u8> = HashMap::new();
        for card in self.cards.iter() {
            let count = card_counts.entry(*card).or_insert(0);
            *count += 1;
        }

        match card_counts.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if card_counts.values().any(|c| *c == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if card_counts.values().any(|c| *c == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.ty() != other.ty() {
            // Type comparison sets order
            self.ty().cmp(&other.ty())
        } else {
            // Type is the same, need to check cards
            for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                if c1 != c2 {
                    return c1.cmp(c2);
                }
            }
            std::cmp::Ordering::Equal
        }
    }
}

impl JokerHand {
    fn parse(line: &str) -> Self {
        let (cards_str, bid_str) = line.split_once(' ').unwrap();
        let bid = bid_str.parse::<u64>().unwrap();
        let cards = cards_str
            .bytes()
            .map(|b| JokerCard::try_from(b).unwrap())
            .collect::<Vec<JokerCard>>()
            .try_into()
            .unwrap();
        JokerHand { cards, bid }
    }

    fn ty(&self) -> HandType {
        let mut card_counts: HashMap<JokerCard, u8> = HashMap::new();
        for card in self.cards.iter() {
            let count = card_counts.entry(*card).or_insert(0);
            *count += 1;
        }

        match card_counts.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if card_counts.values().any(|c| *c == 4) {
                    if self.cards.contains(&JokerCard::Joker) {
                        // JJJJ? or ????J
                        HandType::FiveOfAKind
                    } else {
                        HandType::FourOfAKind
                    }
                } else if self.cards.contains(&JokerCard::Joker) {
                    // JJJ?? or ???JJ
                    HandType::FiveOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if card_counts.values().any(|c| *c == 3) {
                    if self.cards.contains(&JokerCard::Joker) {
                        // JJJ?_ or ???J_
                        HandType::FourOfAKind
                    } else {
                        HandType::ThreeOfAKind
                    }
                } else if self.cards.contains(&JokerCard::Joker) {
                    if *card_counts.get(&JokerCard::Joker).unwrap() == 1 {
                        // J??__
                        HandType::FullHouse
                    } else {
                        // JJ??_
                        HandType::FourOfAKind
                    }
                } else {
                    HandType::TwoPair
                }
            }
            4 => {
                if self.cards.contains(&JokerCard::Joker) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::OnePair
                }
            }
            5 => {
                if self.cards.contains(&JokerCard::Joker) {
                    HandType::OnePair
                } else {
                    HandType::HighCard
                }
            }
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.ty() != other.ty() {
            // Type comparison sets order
            self.ty().cmp(&other.ty())
        } else {
            // Type is the same, need to check cards
            for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                if c1 != c2 {
                    return c1.cmp(c2);
                }
            }
            std::cmp::Ordering::Equal
        }
    }
}

#[test]
fn test_parse() {
    let hand = Hand::parse("32T3K 765");
    assert_eq!(
        hand,
        Hand {
            cards: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
            bid: 765
        }
    );
}

#[test]
fn test_hand_type() {
    assert_eq!(Hand::parse("AAAAA 0").ty(), HandType::FiveOfAKind);
    assert_eq!(Hand::parse("AA8AA 0").ty(), HandType::FourOfAKind);
    assert_eq!(Hand::parse("23332 0").ty(), HandType::FullHouse);
    assert_eq!(Hand::parse("TTT98 0").ty(), HandType::ThreeOfAKind);
    assert_eq!(Hand::parse("23432 0").ty(), HandType::TwoPair);
    assert_eq!(Hand::parse("A23A4 0").ty(), HandType::OnePair);
    assert_eq!(Hand::parse("23456 0").ty(), HandType::HighCard);
}

#[test]
fn test_total_winnings() {
    let mut hands = parse_hands("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483\n");
    assert_eq!(total_winnings(&mut hands), 6440);
}

#[test]
fn test_joker_total_winngs() {
    let mut hands = joker_parse_hands("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483\n");
    assert_eq!(joker_total_winnings(&mut hands), 5905);
}
