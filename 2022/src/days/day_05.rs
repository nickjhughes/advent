use nom::{
    branch::alt,
    bytes::{complete::tag, complete::take_until, complete::take_while},
    character::complete::{anychar, digit1, multispace1, newline},
    combinator::{eof, map, opt},
    error::Error,
    multi::many_till,
    sequence::tuple,
    IResult,
};
use std::collections::VecDeque;
use std::fs;

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                tag::<_, &str, _>("move"),
                multispace1,
                digit1,
                multispace1,
                tag::<_, &str, _>("from"),
                multispace1,
                digit1,
                multispace1,
                tag::<_, &str, _>("to"),
                multispace1,
                digit1,
                opt(newline),
            )),
            |(_, _, count, _, _, _, from, _, _, _, to, _)| Self {
                from: from
                    .parse::<usize>()
                    .expect("Failed to parse move from stack")
                    - 1,
                to: to.parse::<usize>().expect("Failed to parse move to stack") - 1,
                count: count.parse::<usize>().expect("Failed to parse move count"),
            },
        )(input)
    }
}

#[derive(Debug, Clone)]
struct Crate(char);

impl Crate {
    fn parse(input: &str) -> IResult<&str, Option<Self>> {
        alt((
            map(
                tuple((
                    tag::<_, &str, _>("["),
                    anychar,
                    tag::<_, &str, _>("]"),
                    opt(tag::<_, &str, _>(" ")),
                )),
                |(_, c, _, _)| Some(Self(c)),
            ),
            map(
                alt((tag::<_, &str, _>("    "), tag::<_, &str, _>("   "))),
                |_| None,
            ),
        ))(input)
    }
}

#[derive(Debug, Clone)]
struct Stack(VecDeque<Crate>);

impl Stack {
    fn new() -> Self {
        Self(VecDeque::new())
    }
}

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let (rest, mut stacks) = parse_stacks(&contents).expect("Failed to parse stacks");
    let (_, instructions) = parse_instructions(rest).expect("Failed to parse move instructions");
    rearrange_stacks_individually(&mut stacks, &instructions);
    top_of_stacks(&stacks)
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let (rest, mut stacks) = parse_stacks(&contents).expect("Failed to parse stacks");
    let (_, instructions) = parse_instructions(rest).expect("Failed to parse move instructions");
    rearrange_stacks_in_groups(&mut stacks, &instructions);
    top_of_stacks(&stacks)
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input05").expect("Failed to open input file")
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    map(many_till(Instruction::parse, eof), |(instructions, _)| {
        instructions
    })(input)
}

fn parse_stacks(input: &str) -> IResult<&str, Vec<Stack>> {
    let (rest, crates) = map(
        many_till(
            map(many_till(Crate::parse, newline), |(crates, _)| crates),
            tag::<_, &str, _>(" 1"),
        ),
        |(crates, _)| crates,
    )(input)
    .expect("Failed to pass stacks");
    let (rest, _) = take_until::<_, _, Error<_>>("\n")(rest).expect("Failed to pass stack numbers");
    let (rest, _) =
        take_while::<_, _, Error<_>>(|c| c == '\n')(rest).expect("Failed to pass stack numbers");

    assert!(!crates.is_empty());
    let stack_count = crates[0].len();
    let mut stacks = vec![Stack::new(); stack_count];
    for level in &crates {
        for (i, c) in level.iter().enumerate() {
            if let Some(c) = c {
                stacks[i].0.push_front(c.clone());
            }
        }
    }

    Ok((rest, stacks))
}

fn rearrange_stacks_individually(stacks: &mut [Stack], instructions: &[Instruction]) {
    for instruction in instructions {
        for _ in 0..instruction.count {
            let c = stacks[instruction.from].0.pop_back().unwrap();
            stacks[instruction.to].0.push_back(c);
        }
    }
}

fn rearrange_stacks_in_groups(stacks: &mut [Stack], instructions: &[Instruction]) {
    for instruction in instructions {
        let mut move_queue = VecDeque::new();
        for _ in 0..instruction.count {
            let c = stacks[instruction.from].0.pop_back().unwrap();
            move_queue.push_back(c);
        }
        for _ in 0..instruction.count {
            let c = move_queue.pop_back().unwrap();
            stacks[instruction.to].0.push_back(c);
        }
    }
}

fn top_of_stacks(stacks: &[Stack]) -> String {
    stacks
        .iter()
        .map(|s| s.0.back().unwrap().0)
        .collect::<String>()
}

#[test]
fn test_parse_crate() {
    let input = "[A] ";
    let (_, c) = Crate::parse(&input).unwrap();
    assert!(c.is_some());
    assert_eq!(c.unwrap().0, 'A');

    let input = "[Z]\n";
    let (_, c) = Crate::parse(&input).unwrap();
    assert!(c.is_some());
    assert_eq!(c.unwrap().0, 'Z');
}

#[test]
fn test_parse_stacks() {
    let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n";
    let (_, stacks) = parse_stacks(&input).unwrap();
    assert_eq!(stacks.len(), 3);

    assert_eq!(stacks[0].0.len(), 2);
    assert_eq!(stacks[0].0[0].0, 'Z');
    assert_eq!(stacks[0].0[1].0, 'N');

    assert_eq!(stacks[1].0.len(), 3);
    assert_eq!(stacks[1].0[0].0, 'M');
    assert_eq!(stacks[1].0[1].0, 'C');
    assert_eq!(stacks[1].0[2].0, 'D');

    assert_eq!(stacks[2].0.len(), 1);
    assert_eq!(stacks[2].0[0].0, 'P');
}

#[test]
fn test_parse_instructions() {
    let input = "move 1 from 2 to 1\nmove 3 from 1 to 3\n";
    let (_, instructions) = parse_instructions(&input).unwrap();
    assert_eq!(instructions.len(), 2);
    assert_eq!(
        instructions[0],
        Instruction {
            from: 1,
            to: 0,
            count: 1
        }
    );
    assert_eq!(
        instructions[1],
        Instruction {
            from: 0,
            to: 2,
            count: 3
        }
    );
}

#[test]
fn test_parse() {
    let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\n";
    let (rest, stacks) = parse_stacks(&input).unwrap();
    assert_eq!(stacks.len(), 3);
    assert_eq!(stacks[0].0.len(), 2);
    assert_eq!(stacks[0].0[0].0, 'Z');
    assert_eq!(stacks[0].0[1].0, 'N');
    assert_eq!(stacks[1].0.len(), 3);
    assert_eq!(stacks[1].0[0].0, 'M');
    assert_eq!(stacks[1].0[1].0, 'C');
    assert_eq!(stacks[1].0[2].0, 'D');
    assert_eq!(stacks[2].0.len(), 1);
    assert_eq!(stacks[2].0[0].0, 'P');

    let (_, instructions) = parse_instructions(rest).unwrap();
    assert_eq!(instructions.len(), 2);
    assert_eq!(
        instructions[0],
        Instruction {
            from: 1,
            to: 0,
            count: 1
        }
    );
    assert_eq!(
        instructions[1],
        Instruction {
            from: 0,
            to: 2,
            count: 3
        }
    );
}

#[test]
fn test_rearrange_stacks_individually() {
    let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n";
    let (rest, mut stacks) = parse_stacks(&input).unwrap();
    let (_, instructions) = parse_instructions(rest).unwrap();
    rearrange_stacks_individually(&mut stacks, &instructions);

    assert_eq!(stacks[0].0[0].0, 'C');

    assert_eq!(stacks[1].0[0].0, 'M');

    assert_eq!(stacks[2].0[0].0, 'P');
    assert_eq!(stacks[2].0[1].0, 'D');
    assert_eq!(stacks[2].0[2].0, 'N');
    assert_eq!(stacks[2].0[3].0, 'Z');
}

#[test]
fn test_rearrange_stacks_in_groups() {
    let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n";
    let (rest, mut stacks) = parse_stacks(&input).unwrap();
    let (_, instructions) = parse_instructions(rest).unwrap();
    rearrange_stacks_in_groups(&mut stacks, &instructions);

    assert_eq!(stacks[0].0[0].0, 'M');

    assert_eq!(stacks[1].0[0].0, 'C');

    assert_eq!(stacks[2].0[0].0, 'P');
    assert_eq!(stacks[2].0[1].0, 'Z');
    assert_eq!(stacks[2].0[2].0, 'N');
    assert_eq!(stacks[2].0[3].0, 'D');
}

#[test]
fn test_top_of_stacks() {
    let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n";
    let (_, stacks) = parse_stacks(&input).unwrap();
    let result = top_of_stacks(&stacks);
    assert_eq!(result, "NDP");
}
