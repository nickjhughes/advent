use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() -> String {
    format!("{}", overlapping_area(None))
}

pub fn part2() -> String {
    format!("{}", standalone_claim(None))
}

struct Claim {
    id: u64,
    left: u64,
    top: u64,
    width: u64,
    height: u64,
}

fn get_claims() -> Vec<Claim> {
    let file = File::open("inputs/input03").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let mut claims = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let clean_line = line
            .replace('#', "")
            .replace("@ ", "")
            .replace(',', " ")
            .replace(':', "")
            .replace('x', " ");
        let parts: Vec<&str> = clean_line.split(' ').collect();
        if parts.len() != 5 {
            panic!("Could not parse claim");
        }
        claims.push(Claim {
            id: parts[0].parse::<u64>().expect("Could not parse claim ID"),
            left: parts[1].parse::<u64>().expect("Could not parse claim left"),
            top: parts[2].parse::<u64>().expect("Could not parse claim top"),
            width: parts[3]
                .parse::<u64>()
                .expect("Could not parse claim width"),
            height: parts[4]
                .parse::<u64>()
                .expect("Could not parse claim height"),
        });
    }
    claims
}

fn squares_with_counts(claims: &Vec<Claim>) -> HashMap<(u64, u64), u64> {
    let mut squares: HashMap<(u64, u64), u64> = HashMap::new();
    for claim in claims {
        for x in claim.left..(claim.left + claim.width) {
            for y in claim.top..(claim.top + claim.height) {
                *squares.entry((x, y)).or_insert(0) += 1;
            }
        }
    }
    squares
}

fn overlapping_area(claims: Option<Vec<Claim>>) -> u64 {
    let claims = claims.unwrap_or_else(get_claims);
    let squares = squares_with_counts(&claims);
    let mut overlapping_count = 0;
    for &count in squares.values() {
        if count > 1 {
            overlapping_count += 1;
        }
    }
    overlapping_count
}

fn standalone_claim(claims: Option<Vec<Claim>>) -> u64 {
    let claims = claims.unwrap_or_else(get_claims);
    let squares = squares_with_counts(&claims);
    for claim in &claims {
        let mut overlapping_area = 0;
        for x in claim.left..(claim.left + claim.width) {
            for y in claim.top..(claim.top + claim.height) {
                if squares[&(x, y)] > 1 {
                    overlapping_area += 1;
                }
            }
        }
        if overlapping_area == 0 {
            return claim.id;
        }
    }
    panic!("Could not find standalone claim");
}

#[test]
fn part1_tests() {
    assert_eq!(
        overlapping_area(Some(vec![
            Claim {
                id: 1,
                left: 1,
                top: 3,
                width: 4,
                height: 4,
            },
            Claim {
                id: 2,
                left: 3,
                top: 1,
                width: 4,
                height: 4,
            },
            Claim {
                id: 3,
                left: 5,
                top: 5,
                width: 2,
                height: 2,
            }
        ])),
        4
    );
}

#[test]
fn part2_tests() {
    assert_eq!(
        standalone_claim(Some(vec![
            Claim {
                id: 1,
                left: 1,
                top: 3,
                width: 4,
                height: 4,
            },
            Claim {
                id: 2,
                left: 3,
                top: 1,
                width: 4,
                height: 4,
            },
            Claim {
                id: 3,
                left: 5,
                top: 5,
                width: 2,
                height: 2,
            }
        ])),
        3
    );
}
