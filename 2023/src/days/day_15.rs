use std::{
    collections::{HashMap, VecDeque},
    fs,
};

pub fn part1() -> String {
    let input = get_input_file_contents();
    sum_hash_seq(&input).to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let init_seq = parse_init_seq(&input);
    let mut lens_hash_map = LensHashMap::default();
    for init_step in init_seq.iter() {
        lens_hash_map.run_init_step(init_step);
    }
    lens_hash_map.focusing_power().to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input15").expect("Failed to open input file")
}

fn sum_hash_seq(input: &str) -> u64 {
    input
        .trim_end()
        .split(',')
        .map(|s| hash(s) as u64)
        .sum::<u64>()
}

fn hash(input: &str) -> u8 {
    let mut hash: u8 = 0;
    for ch in input.chars() {
        hash = hash.wrapping_add(ch as u8);
        hash = hash.wrapping_mul(17);
    }
    hash
}

#[derive(Debug, PartialEq)]
enum InitStep<'input> {
    Insert(&'input str, u8, u8),
    Remove(&'input str, u8),
}

impl<'input> InitStep<'input> {
    fn parse(input: &'input str) -> Self {
        if let Some((label, lens)) = input.split_once('=') {
            InitStep::Insert(label, hash(label), lens.parse::<u8>().unwrap())
        } else {
            let label = input.trim_end_matches('-');
            InitStep::Remove(label, hash(label))
        }
    }
}

fn parse_init_seq(input: &str) -> Vec<InitStep> {
    input.trim_end().split(',').map(InitStep::parse).collect()
}

#[derive(Debug, Default)]
struct LensHashMap<'input> {
    boxes: HashMap<u8, VecDeque<(&'input str, u8)>>,
}

impl<'input> LensHashMap<'input> {
    fn run_init_step(&mut self, init_step: &'input InitStep) {
        match init_step {
            InitStep::Insert(label, label_hash, lens) => {
                let entry = self.boxes.entry(*label_hash).or_default();
                if let Some(lens_idx) = entry.iter().position(|(l, _)| l == label) {
                    // Replace
                    entry.get_mut(lens_idx).unwrap().1 = *lens;
                } else {
                    // Insert
                    entry.push_back((label, *lens));
                }
            }
            InitStep::Remove(label, label_hash) => {
                let entry = self.boxes.entry(*label_hash).or_default();
                if let Some(lens_idx) = entry.iter().position(|(l, _)| l == label) {
                    // Remove
                    entry.remove(lens_idx);
                }
            }
        }
    }

    fn focusing_power(&self) -> u64 {
        let mut total_power = 0;
        for box_idx in 0..=255 {
            if let Some(lenses) = self.boxes.get(&box_idx) {
                for (lens_idx, (_, lens)) in lenses.iter().enumerate() {
                    let power = (box_idx as u64 + 1) * (lens_idx as u64 + 1) * (*lens as u64);
                    total_power += power;
                }
            }
        }
        total_power
    }
}

#[test]
fn test_hash() {
    assert_eq!(hash("HASH"), 52);
}

#[test]
fn test_sum_hash_seq() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";
    assert_eq!(sum_hash_seq(input), 1320);
}

#[test]
fn test_parse_init_seq() {
    let input = "rn=1,cm-,qp=3,cm=2\n";
    let init_seq = parse_init_seq(input);

    assert_eq!(init_seq[0], InitStep::Insert("rn", hash("rn"), 1));
    assert_eq!(init_seq[1], InitStep::Remove("cm", hash("cm")));
    assert_eq!(init_seq[2], InitStep::Insert("qp", hash("qp"), 3));
    assert_eq!(init_seq[3], InitStep::Insert("cm", hash("cm"), 2));
}

#[test]
fn test_run_init_seq() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";
    let init_seq = parse_init_seq(input);
    let mut lens_hash_map = LensHashMap::default();
    for init_step in init_seq.iter() {
        lens_hash_map.run_init_step(init_step);
    }

    let box_0 = lens_hash_map.boxes.get(&0).unwrap();
    assert_eq!(box_0.len(), 2);

    let box_3 = lens_hash_map.boxes.get(&3).unwrap();
    assert_eq!(box_3.len(), 3);
}

#[test]
fn test_focusing_power() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";
    let init_seq = parse_init_seq(input);
    let mut lens_hash_map = LensHashMap::default();
    for init_step in init_seq.iter() {
        lens_hash_map.run_init_step(init_step);
    }
    assert_eq!(lens_hash_map.focusing_power(), 145);
}
