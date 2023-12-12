use std::{collections::HashMap, fs};

pub fn part1() -> String {
    let input = get_input_file_contents();
    let rows = parse_rows(&input);
    rows.iter()
        .map(|r| r.arrangements_count())
        .sum::<usize>()
        .to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let rows = parse_rows(&input);
    rows.iter()
        .map(|r| r.unfolded_arrangement_count())
        .sum::<usize>()
        .to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input12").expect("Failed to open input file")
}

fn parse_rows(input: &str) -> Vec<Row> {
    input.lines().map(Row::parse).collect()
}

#[derive(Debug, PartialEq)]
struct Row {
    springs: Vec<Spring>,
    damaged_groups: Vec<usize>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<Spring> for char {
    fn from(value: Spring) -> Self {
        match value {
            Spring::Operational => '.',
            Spring::Damaged => '#',
            Spring::Unknown => '?',
        }
    }
}

impl Row {
    fn parse(input: &str) -> Self {
        let (springs_str, damaged_groups_str) = input.split_once(' ').unwrap();
        let springs = springs_str.chars().map(Spring::parse).collect();
        let damaged_groups = damaged_groups_str
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        Row {
            springs,
            damaged_groups,
        }
    }

    fn arrangements_count(&self) -> usize {
        arrangements_count(
            &self.springs,
            &self.damaged_groups,
            0,
            0,
            &mut HashMap::new(),
        )
    }

    fn unfold(&self) -> Row {
        let mut unfolded_springs = Vec::with_capacity(self.springs.len() * 5 + self.springs.len());
        for i in 0..5 {
            unfolded_springs.extend(&self.springs);
            if i < 4 {
                unfolded_springs.push(Spring::Unknown);
            }
        }
        Row {
            springs: unfolded_springs,
            damaged_groups: self.damaged_groups.repeat(5),
        }
    }

    fn unfolded_arrangement_count(&self) -> usize {
        let unfolded_row = self.unfold();
        unfolded_row.arrangements_count()
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for spring in self.springs.iter() {
            write!(f, "{}", char::from(*spring))?;
        }
        write!(f, " ")?;
        write!(
            f,
            "{}",
            self.damaged_groups
                .iter()
                .map(|g| g.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

fn arrangements_count(
    springs: &[Spring],
    groups: &[usize],
    si: usize,
    gi: usize,
    states: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(count) = states.get(&(si, gi)) {
        *count
    } else {
        if gi >= groups.len() {
            // No more groups, so this is a final valid arrangement
            return 1;
        }
        let group = groups[gi];
        let remaining_springs = springs.len() - si;
        if remaining_springs < group {
            // No room for group, so this is not a valid arrangement
            return 0;
        }
        // Find all options for the start of the next group
        let mut count = 0;
        for i in si..(si + remaining_springs - group + 1) {
            // If the group:
            // - Fits in unknown/damaged springs,
            // - Is not immediately followed by a damaged spring,
            // - If is the last group, is not followed by any damaged springs,
            // - And is not preceded by any damaged strings,
            // then it's a valid starting place for the group.
            if springs[i..i + group]
                .iter()
                .all(|s| matches!(s, Spring::Unknown | Spring::Damaged))
                && (i + group == springs.len() || !matches!(springs[i + group], Spring::Damaged))
                && !springs[si..i].iter().any(|s| matches!(s, Spring::Damaged))
                && (gi < groups.len() - 1
                    || !springs[i + group..]
                        .iter()
                        .any(|s| matches!(s, Spring::Damaged)))
            {
                if i + group == springs.len() {
                    // This group completes the springs
                    if gi == groups.len() - 1 {
                        // It's also the last group, so this is a valid arrangement
                        count += 1;
                    } else {
                        // There's more groups, but no more springs, so this is not a valid arrangement
                        count += 0;
                    }
                } else {
                    count += arrangements_count(springs, groups, i + group + 1, gi + 1, states);
                }
            }
        }
        states.insert((si, gi), count);
        count
    }
}

impl Spring {
    fn parse(ch: char) -> Self {
        match ch {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => panic!("invalid spring state {:?}", ch),
        }
    }
}

#[test]
fn test_parse() {
    let input = "???.### 1,1,3";
    let row = Row::parse(input);
    assert_eq!(
        row,
        Row {
            springs: vec![
                Spring::Unknown,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Operational,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
            ],
            damaged_groups: vec![1, 1, 3]
        }
    );
}

#[test]
fn test_arrangements_count() {
    assert_eq!(Row::parse("???.### 1,1,3").arrangements_count(), 1);
    assert_eq!(Row::parse(".??..??...?##. 1,1,3").arrangements_count(), 4);
    assert_eq!(
        Row::parse("?#?#?#?#?#?#?#? 1,3,1,6",).arrangements_count(),
        1
    );
    assert_eq!(Row::parse("????.#...#... 4,1,1").arrangements_count(), 1);
    assert_eq!(
        Row::parse("????.######..#####. 1,6,5").arrangements_count(),
        4
    );
    assert_eq!(Row::parse("?###???????? 3,2,1",).arrangements_count(), 10);

    assert_eq!(Row::parse(".???..??##.. 2,4").arrangements_count(), 2);
    assert_eq!(Row::parse("??##.?#?.?#?# 4,3,3").arrangements_count(), 1);
    assert_eq!(
        Row::parse(".???##?.???.?.??#??? 4,3,1,1,4").arrangements_count(),
        2
    );
    assert_eq!(
        Row::parse("?????.?.?????##?#.?? 3,5").arrangements_count(),
        4
    );
}

#[test]
fn test_arrangements_count_unfolded() {
    assert_eq!(Row::parse("???.### 1,1,3").unfolded_arrangement_count(), 1);
    assert_eq!(
        Row::parse(".??..??...?##. 1,1,3").unfolded_arrangement_count(),
        16384
    );
    assert_eq!(
        Row::parse("?#?#?#?#?#?#?#? 1,3,1,6").unfolded_arrangement_count(),
        1
    );
    assert_eq!(
        Row::parse("????.#...#... 4,1,1").unfolded_arrangement_count(),
        16
    );
    assert_eq!(
        Row::parse("????.######..#####. 1,6,5").unfolded_arrangement_count(),
        2500
    );
    assert_eq!(
        Row::parse("?###???????? 3,2,1").unfolded_arrangement_count(),
        506250
    );
}
