use ndarray::{prelude::*, Array, Ix2};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::map,
    multi::many_till,
    IResult,
};
use std::{fmt, fs};

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let mut board = parse_board(&contents);
    board.follow_instructions();
    let points = board.points();
    format!("{}", points)
}

pub fn part2() -> String {
    let _contents = get_input_file_contents();
    format!("")
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input22").expect("Failed to open input file")
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn points(&self) -> u32 {
        match self {
            Direction::Up => 3,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
        }
    }
}

#[derive(Debug)]
struct Board {
    map: Array<Tile, Ix2>,
    instructions: Vec<Instruction>,
    current_position: (usize, usize),
    current_direction: Direction,
}

impl Board {
    fn new(map: Array<Tile, Ix2>, instructions: Vec<Instruction>) -> Self {
        let mut initial_position = None;
        for (col, tile) in map.row(0).iter().enumerate() {
            if *tile == Tile::Empty {
                initial_position = Some((0, col));
                break;
            }
        }
        Self {
            map,
            instructions,
            current_position: initial_position.expect("could not find initial location"),
            current_direction: Direction::Right,
        }
    }

    fn turn_left(direction: &mut Direction) {
        *direction = match direction {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(direction: &mut Direction) {
        *direction = match direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn follow_instructions(&mut self) {
        let map_shape = self.map.shape();
        let map_rows = map_shape[0];
        let map_cols = map_shape[1];
        for instruction in &self.instructions {
            match instruction {
                Instruction::Forward(steps) => {
                    for _ in 0..*steps {
                        let (mut new_row, mut new_col) = match self.current_direction {
                            Direction::Up => {
                                if self.current_position.0 == 0 {
                                    (map_rows - 1, self.current_position.1)
                                } else {
                                    (self.current_position.0 - 1, self.current_position.1)
                                }
                            }
                            Direction::Down => {
                                if self.current_position.0 == map_rows - 1 {
                                    (0, self.current_position.1)
                                } else {
                                    (self.current_position.0 + 1, self.current_position.1)
                                }
                            }
                            Direction::Left => {
                                if self.current_position.1 == 0 {
                                    (self.current_position.0, map_cols - 1)
                                } else {
                                    (self.current_position.0, self.current_position.1 - 1)
                                }
                            }
                            Direction::Right => {
                                if self.current_position.1 == map_cols - 1 {
                                    (self.current_position.0, 0)
                                } else {
                                    (self.current_position.0, self.current_position.1 + 1)
                                }
                            }
                        };
                        match self.map[[new_row, new_col]] {
                            Tile::Blank => {
                                match self.current_direction {
                                    Direction::Up => {
                                        for row in (0..map_rows).rev() {
                                            if self.map[[row, new_col]] != Tile::Blank {
                                                new_row = row;
                                                break;
                                            }
                                        }
                                    }
                                    Direction::Down => {
                                        for row in 0..map_rows {
                                            if self.map[[row, new_col]] != Tile::Blank {
                                                new_row = row;
                                                break;
                                            }
                                        }
                                    }
                                    Direction::Left => {
                                        for col in (0..map_cols).rev() {
                                            if self.map[[new_row, col]] != Tile::Blank {
                                                new_col = col;
                                                break;
                                            }
                                        }
                                    }
                                    Direction::Right => {
                                        for col in 0..map_cols {
                                            if self.map[[new_row, col]] != Tile::Blank {
                                                new_col = col;
                                                break;
                                            }
                                        }
                                    }
                                }
                                if self.map[[new_row, new_col]] != Tile::Wall {
                                    self.current_position = (new_row, new_col);
                                }
                            }
                            Tile::Empty => self.current_position = (new_row, new_col),
                            Tile::Wall => {}
                        }
                    }
                }
                Instruction::TurnLeft => Self::turn_left(&mut self.current_direction),
                Instruction::TurnRight => Self::turn_right(&mut self.current_direction),
            }
        }
    }

    fn points(&self) -> u32 {
        let (current_row, current_col) = self.current_position;
        1000 * (current_row as u32 + 1)
            + 4 * (current_col as u32 + 1)
            + self.current_direction.points()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let map_shape = self.map.shape();
        for row in 0..map_shape[0] {
            for col in 0..map_shape[1] {
                if self.current_position == (row, col) {
                    match self.current_direction {
                        Direction::Up => write!(f, "^"),
                        Direction::Down => write!(f, "v"),
                        Direction::Left => write!(f, "<"),
                        Direction::Right => write!(f, ">"),
                    }?;
                } else {
                    match self.map[[row, col]] {
                        Tile::Blank => write!(f, " "),
                        Tile::Empty => write!(f, "."),
                        Tile::Wall => write!(f, "#"),
                    }?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Forward(usize),
    TurnLeft,
    TurnRight,
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(alt((digit1, tag("L"), tag("R"))), |value| match value {
            "L" => Self::TurnLeft,
            "R" => Self::TurnRight,
            num => Self::Forward(num.parse::<usize>().expect("failed to parse instruction")),
        })(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Blank,
    Empty,
    Wall,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Blank
    }
}

fn parse_map(contents: &str) -> IResult<&str, Array<Tile, Ix2>> {
    let mut tiles = vec![];
    let mut cols = 0;
    let mut char_count = 0;
    for line in contents.lines() {
        if line.is_empty() {
            break;
        }
        let mut row = Vec::new();
        for ch in line.chars() {
            let tile = match ch {
                ' ' => Tile::Blank,
                '.' => Tile::Empty,
                '#' => Tile::Wall,
                _ => panic!("invalid map character: {}", ch),
            };
            row.push(tile);
            char_count += 1;
        }
        if row.len() > cols {
            cols = row.len();
        }
        tiles.push(row);
        char_count += 1; // newline
    }
    char_count += 1; // blank line
    let rows = tiles.len();
    let rest = &contents[char_count..];

    let mut map = Array::<Tile, Ix2>::default((rows, cols).f());
    for r in 0..rows {
        for c in 0..cols {
            map[[r, c]] = if c >= tiles[r].len() {
                Tile::Blank
            } else {
                tiles[r][c]
            };
        }
    }

    Ok((rest, map))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    map(
        many_till(Instruction::parse, newline),
        |(instructions, _)| instructions,
    )(input)
}

fn parse_board(contents: &str) -> Board {
    let (rest, map) = parse_map(contents).expect("failed to parse map");
    let (_, instructions) = parse_instructions(rest).expect("failed to parse instructions");
    Board::new(map, instructions)
}

#[test]
fn test_parse_map() {
    let input = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.\n\n";
    let result = parse_map(input);
    assert!(result.is_ok());
    let (rest, map) = result.unwrap();
    assert!(rest.is_empty());
    assert_eq!(map.shape(), &[12, 16]);
}

#[test]
fn test_parse_instruction() {
    {
        let input = "10";
        let result = Instruction::parse(input);
        assert!(result.is_ok());
        let (rest, instruction) = result.unwrap();
        assert!(rest.is_empty());
        assert_eq!(instruction, Instruction::Forward(10));
    }

    {
        let input = "L";
        let result = Instruction::parse(input);
        assert!(result.is_ok());
        let (rest, instruction) = result.unwrap();
        assert!(rest.is_empty());
        assert_eq!(instruction, Instruction::TurnLeft);
    }

    {
        let input = "R";
        let result = Instruction::parse(input);
        assert!(result.is_ok());
        let (rest, instruction) = result.unwrap();
        assert!(rest.is_empty());
        assert_eq!(instruction, Instruction::TurnRight);
    }
}

#[test]
fn test_parse_instructions() {
    let input = "10R5L5R10L4R5L5\n";
    let result = parse_instructions(input);
    assert!(result.is_ok());
    let (rest, instructions) = result.unwrap();
    assert!(rest.is_empty());
    assert_eq!(instructions.len(), 13);
    assert_eq!(
        instructions,
        vec![
            Instruction::Forward(10),
            Instruction::TurnRight,
            Instruction::Forward(5),
            Instruction::TurnLeft,
            Instruction::Forward(5),
            Instruction::TurnRight,
            Instruction::Forward(10),
            Instruction::TurnLeft,
            Instruction::Forward(4),
            Instruction::TurnRight,
            Instruction::Forward(5),
            Instruction::TurnLeft,
            Instruction::Forward(5),
        ]
    );
}

#[test]
fn test_parse_board() {
    let contents = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.\n\n10R5L5R10L4R5L5\n";
    let board = parse_board(contents);
    assert_eq!(board.map.shape(), &[12, 16]);
    assert_eq!(board.instructions.len(), 13);
}

#[test]
fn test_follow_instructions() {
    let contents = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.\n\n10R5L5R10L4R5L5\n";
    let mut board = parse_board(contents);
    board.follow_instructions();
    assert_eq!(board.points(), 6032);
}
