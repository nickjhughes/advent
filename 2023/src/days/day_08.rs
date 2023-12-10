use num::Integer;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn part1() -> String {
    let input = get_input_file_contents();
    let map = Map::parse(&input);
    map.path_length("AAA", "ZZZ").unwrap().to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let map = Map::parse(&input);
    map.ghost_path_length().to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input08").expect("Failed to open input file")
}

#[derive(Debug)]
struct Map {
    directions: Vec<Dir>,
    nodes: Vec<String>,
    mapping: HashMap<usize, (usize, usize)>,
}

#[derive(Debug, PartialEq)]
enum Dir {
    Left,
    Right,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();

        let directions = lines
            .next()
            .unwrap()
            .chars()
            .map(|ch| match ch {
                'L' => Dir::Left,
                'R' => Dir::Right,
                _ => panic!("invalid direction {ch}"),
            })
            .collect();

        lines.next();

        let mut nodes = Vec::new();
        let mut mapping = HashMap::new();
        for line in lines {
            let (start_str, ends) = line.split_once(" = ").unwrap();
            let (left_str, right_str) = ends
                .trim_matches(|ch| ch == ')' || ch == '(')
                .split_once(", ")
                .unwrap();
            let start_id = nodes
                .iter()
                .position(|n| *n == start_str)
                .unwrap_or_else(|| {
                    nodes.push(start_str.to_owned());
                    nodes.len() - 1
                });
            let left_id = nodes
                .iter()
                .position(|n| *n == left_str)
                .unwrap_or_else(|| {
                    nodes.push(left_str.to_owned());
                    nodes.len() - 1
                });
            let right_id = nodes
                .iter()
                .position(|n| *n == right_str)
                .unwrap_or_else(|| {
                    nodes.push(right_str.to_owned());
                    nodes.len() - 1
                });
            mapping.insert(start_id, (left_id, right_id));
        }

        Map {
            directions,
            nodes,
            mapping,
        }
    }

    fn path_length(&self, start: &str, end: &str) -> Option<usize> {
        let end_id = self.nodes.iter().position(|s| s == end).unwrap();

        let mut visited_states = HashSet::new();

        let mut steps = 0;
        let mut cur_id = self.nodes.iter().position(|s| s == start).unwrap();
        let mut cur_dir = 0;
        visited_states.insert((cur_id, cur_dir));
        while cur_id != end_id {
            let (left_id, right_id) = self.mapping.get(&cur_id).unwrap();
            cur_id = match self.directions[cur_dir] {
                Dir::Left => *left_id,
                Dir::Right => *right_id,
            };
            steps += 1;

            cur_dir += 1;
            if cur_dir == self.directions.len() {
                cur_dir = 0;
            }

            if !visited_states.insert((cur_id, cur_dir)) {
                return None;
            }
        }

        Some(steps)
    }

    fn ghost_path_length(&self) -> usize {
        let mut loops = Vec::new();
        for n1 in self.nodes.iter().filter(|n| n.ends_with('A')) {
            for n2 in self.nodes.iter().filter(|n| n.ends_with('Z')) {
                let steps = self.path_length(n1, n2);
                if let Some(steps) = steps {
                    loops.push(steps);
                }
            }
        }

        loops.into_iter().reduce(|acc, e| acc.lcm(&e)).unwrap()
    }
}

#[test]
fn test_parse() {
    let input = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)\n";
    let map = Map::parse(input);
    assert_eq!(map.directions, vec![Dir::Left, Dir::Left, Dir::Right]);
    assert_eq!(
        map.nodes,
        vec!["AAA".to_string(), "BBB".to_string(), "ZZZ".to_string()]
    );
    assert_eq!(map.mapping, {
        let mut mapping = HashMap::new();
        mapping.insert(0, (1, 1));
        mapping.insert(1, (0, 2));
        mapping.insert(2, (2, 2));
        mapping
    });
}

#[test]
fn test_path_length() {
    let input = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)\n";
    let map = Map::parse(input);
    assert_eq!(map.path_length("AAA", "ZZZ"), Some(6));
}

#[test]
fn test_ghost_path_length() {
    let input = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)\n";
    let map = Map::parse(input);
    assert_eq!(map.ghost_path_length(), 6);
}
