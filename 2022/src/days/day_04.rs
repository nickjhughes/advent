use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map, sequence::separated_pair,
    IResult,
};
use std::fs;

#[derive(Debug, PartialEq, Eq)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(digit1, tag::<_, &str, _>("-"), digit1),
            |(start, end)| Self {
                start: start.parse::<u32>().expect("Failed to parse number"),
                end: end.parse::<u32>().expect("Failed to parse number"),
            },
        )(input)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Pair {
    one: Range,
    two: Range,
}

impl Pair {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(Range::parse, tag::<_, &str, _>(","), Range::parse),
            |(one, two)| Self { one, two },
        )(input)
    }

    fn fully_contains(&self) -> bool {
        (self.one.start <= self.two.start && self.one.end >= self.two.end)
            || (self.two.start <= self.one.start && self.two.end >= self.one.end)
    }

    fn overlaps(&self) -> bool {
        (self.one.start <= self.two.end) && (self.one.end >= self.two.start)
    }
}

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let pairs = parse_pairs(&contents);
    let count = fully_contains_count(&pairs);
    format!("{}", count)
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let pairs = parse_pairs(&contents);
    let count = overlaps_count(&pairs);
    format!("{}", count)
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input04").expect("Failed to open input file")
}

fn parse_pairs(contents: &str) -> Vec<Pair> {
    let mut pairs = Vec::new();
    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }
        let (_, pair) = Pair::parse(line).expect("Failed to parse pair");
        pairs.push(pair);
    }
    pairs
}

fn fully_contains_count(pairs: &[Pair]) -> usize {
    pairs.iter().filter(|p| p.fully_contains()).count()
}

fn overlaps_count(pairs: &[Pair]) -> usize {
    pairs.iter().filter(|p| p.overlaps()).count()
}

#[test]
fn test_parse() {
    let contents = "2-4,6-8\n2-3,4-5\n";
    let pairs = parse_pairs(&contents);
    assert_eq!(pairs.len(), 2);
    assert_eq!(
        pairs[0],
        Pair {
            one: Range { start: 2, end: 4 },
            two: Range { start: 6, end: 8 },
        }
    );
    assert_eq!(
        pairs[1],
        Pair {
            one: Range { start: 2, end: 3 },
            two: Range { start: 4, end: 5 },
        }
    );
}

#[test]
fn test_contains_count() {
    let contents = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n";
    let pairs = parse_pairs(&contents);
    let count = fully_contains_count(&pairs);
    assert_eq!(count, 2);
}

#[test]
fn test_overlaps_count() {
    let contents = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n";
    let pairs = parse_pairs(&contents);
    let count = overlaps_count(&pairs);
    assert_eq!(count, 4);
}
