use ndarray::{prelude::*, s, Array, Ix2};
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
    let contents = get_input_file_contents();
    let mut cube = parse_cube(&contents);
    cube.follow_instructions();
    let points = cube.points();
    format!("{}", points)
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input22").expect("Failed to open input file")
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
struct Cube {
    sides: [Array<Tile, Ix2>; 6],
    instructions: Vec<Instruction>,
    current_position: (usize, usize, usize),
    current_direction: Direction,
}

impl Cube {
    fn new(sides: [Array<Tile, Ix2>; 6], instructions: Vec<Instruction>) -> Self {
        Self {
            sides,
            instructions,
            current_position: (0, 0, 0),
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
        let side_shape = self.sides[0].shape();
        let side_rows = side_shape[0];
        let side_cols = side_shape[1];
        for instruction in &self.instructions {
            match instruction {
                Instruction::TurnLeft => Self::turn_left(&mut self.current_direction),
                Instruction::TurnRight => Self::turn_right(&mut self.current_direction),
                Instruction::Forward(steps) => {
                    for _ in 0..*steps {
                        let (current_side, current_row, current_col) = self.current_position;
                        let (mut new_side, mut new_row, mut new_col) = self.current_position;
                        let mut new_direction = self.current_direction;

                        if side_rows == 4 {
                            // Example
                            //       000
                            //       000
                            //       000
                            // 111222333
                            // 111222333
                            // 111222333
                            //       444555
                            //       444555
                            //       444555
                            match self.current_direction {
                                Direction::Up => {
                                    if current_row == 0 {
                                        match current_side {
                                            0 => {
                                                new_side = 1;
                                                new_col = side_cols - 1 - current_col;
                                                new_direction = Direction::Down;
                                            }
                                            1 => {
                                                new_side = 0;
                                                new_col = side_cols - 1 - current_col;
                                                new_direction = Direction::Down;
                                            }
                                            2 => {
                                                new_side = 0;
                                                new_col = 0;
                                                new_row = current_col;
                                                new_direction = Direction::Right;
                                            }
                                            3 => {
                                                new_side = 0;
                                                new_row = side_rows - 1;
                                            }
                                            4 => {
                                                new_side = 3;
                                                new_row = side_rows - 1;
                                            }
                                            5 => {
                                                new_side = 3;
                                                new_col = side_cols - 1;
                                                new_row = side_rows - 1 - current_col;
                                                new_direction = Direction::Left;
                                            }
                                            _ => unreachable!(),
                                        }
                                    } else {
                                        new_row -= 1;
                                    }
                                }
                                Direction::Down => {
                                    if current_row == side_rows - 1 {
                                        match current_side {
                                            0 => {
                                                new_side = 3;
                                                new_row = 0;
                                            }
                                            1 => {
                                                new_side = 4;
                                                new_col = side_cols - 1 - current_col;
                                                new_direction = Direction::Up;
                                            }
                                            2 => {
                                                new_side = 4;
                                                new_row = current_col;
                                                new_col = 0;
                                                new_direction = Direction::Right;
                                            }
                                            3 => {
                                                new_side = 4;
                                                new_row = 0;
                                            }
                                            4 => {
                                                new_side = 1;
                                                new_col = side_cols - 1 - current_col;
                                                new_direction = Direction::Up;
                                            }
                                            5 => {
                                                new_side = 1;
                                                new_row = current_col;
                                                new_col = 0;
                                                new_direction = Direction::Right;
                                            }
                                            _ => unreachable!(),
                                        }
                                    } else {
                                        new_row += 1;
                                    }
                                }
                                Direction::Left => {
                                    if current_col == 0 {
                                        match current_side {
                                            0 => {
                                                new_side = 2;
                                                new_row = 0;
                                                new_col = current_row;
                                                new_direction = Direction::Down;
                                            }
                                            1 => {
                                                new_side = 5;
                                                new_row = side_rows - 1;
                                                new_col = side_cols - 1 - current_row;
                                                new_direction = Direction::Up;
                                            }
                                            2 => {
                                                new_side = 1;
                                                new_col = side_cols - 1;
                                            }
                                            3 => {
                                                new_side = 2;
                                                new_col = side_cols - 1;
                                            }
                                            4 => {
                                                new_side = 2;
                                                new_row = side_rows - 1;
                                                new_col = side_cols - 1 - current_row;
                                                new_direction = Direction::Up;
                                            }
                                            5 => {
                                                new_side = 4;
                                                new_col = side_cols - 1;
                                            }
                                            _ => unreachable!(),
                                        }
                                    } else {
                                        new_col -= 1;
                                    }
                                }
                                Direction::Right => {
                                    if current_col == side_cols - 1 {
                                        match current_side {
                                            0 => {
                                                new_side = 5;
                                                new_row = side_rows - 1 - current_row;
                                                new_col = side_cols - 1;
                                                new_direction = Direction::Left;
                                            }
                                            1 => {
                                                new_side = 2;
                                                new_col = 0;
                                            }
                                            2 => {
                                                new_side = 3;
                                                new_col = 0;
                                            }
                                            3 => {
                                                new_side = 5;
                                                new_row = 0;
                                                new_col = side_cols - 1 - current_row;
                                                new_direction = Direction::Down;
                                            }
                                            4 => {
                                                new_side = 5;
                                                new_col = 0;
                                            }
                                            5 => {
                                                new_side = 0;
                                                new_row = side_rows - 1 - current_row;
                                                new_col = side_cols - 1;
                                                new_direction = Direction::Left;
                                            }
                                            _ => unreachable!(),
                                        }
                                    } else {
                                        new_col += 1;
                                    }
                                }
                            }
                        } else {
                            // Input
                            //    000111
                            //    000111
                            //    000111
                            //    222
                            //    222
                            //    222
                            // 333444
                            // 333444
                            // 333444
                            // 555
                            // 555
                            // 555
                            match self.current_direction {
                                Direction::Up => {
                                    if current_row == 0 {
                                        match current_side {
                                            0 => {
                                                new_side = 5;
                                                new_row = current_col;
                                                new_col = 0;
                                                new_direction = Direction::Right;
                                            }
                                            1 => {
                                                new_side = 5;
                                                new_row = side_rows - 1;
                                            }
                                            2 => {
                                                new_side = 0;
                                                new_row = side_rows - 1;
                                            }
                                            3 => {
                                                new_side = 2;
                                                new_col = 0;
                                                new_row = current_col;
                                                new_direction = Direction::Right;
                                            }
                                            4 => {
                                                new_side = 2;
                                                new_row = side_rows - 1;
                                            }
                                            5 => {
                                                new_side = 3;
                                                new_row = side_rows - 1;
                                            }
                                            _ => unreachable!(),
                                        }
                                    } else {
                                        new_row -= 1;
                                    }
                                }
                                Direction::Down => {
                                    if current_row == side_rows - 1 {
                                        match current_side {
                                            0 => {
                                                new_side = 2;
                                                new_row = 0;
                                            }
                                            1 => {
                                                new_side = 2;
                                                new_col = side_cols - 1;
                                                new_row = current_col;
                                                new_direction = Direction::Left;
                                            }
                                            2 => {
                                                new_side = 4;
                                                new_row = 0;
                                            }
                                            3 => {
                                                new_side = 5;
                                                new_row = 0;
                                            }
                                            4 => {
                                                new_side = 5;
                                                new_col = side_cols - 1;
                                                new_row = current_col;
                                                new_direction = Direction::Left;
                                            }
                                            5 => {
                                                new_side = 1;
                                                new_row = 0;
                                            }
                                            _ => unreachable!(),
                                        }
                                    } else {
                                        new_row += 1;
                                    }
                                }
                                Direction::Left => {
                                    if current_col == 0 {
                                        match current_side {
                                            0 => {
                                                new_side = 3;
                                                new_col = 0;
                                                new_row = side_rows - 1 - current_row;
                                                new_direction = Direction::Right;
                                            }
                                            1 => {
                                                new_side = 0;
                                                new_col = side_cols - 1;
                                            }
                                            2 => {
                                                new_side = 3;
                                                new_row = 0;
                                                new_col = current_row;
                                                new_direction = Direction::Down;
                                            }
                                            3 => {
                                                new_side = 0;
                                                new_row = side_rows - 1 - current_row;
                                                new_col = 0;
                                                new_direction = Direction::Right;
                                            }
                                            4 => {
                                                new_side = 3;
                                                new_col = side_cols - 1;
                                            }
                                            5 => {
                                                new_side = 0;
                                                new_row = 0;
                                                new_col = current_row;
                                                new_direction = Direction::Down;
                                            }
                                            _ => unreachable!(),
                                        }
                                    } else {
                                        new_col -= 1;
                                    }
                                }
                                Direction::Right => {
                                    if current_col == side_cols - 1 {
                                        match current_side {
                                            0 => {
                                                new_side = 1;
                                                new_col = 0;
                                            }
                                            1 => {
                                                new_side = 4;
                                                new_row = side_rows - 1 - current_row;
                                                new_col = side_cols - 1;
                                                new_direction = Direction::Left;
                                            }
                                            2 => {
                                                new_side = 1;
                                                new_row = side_rows - 1;
                                                new_col = current_row;
                                                new_direction = Direction::Up;
                                            }
                                            3 => {
                                                new_side = 4;
                                                new_col = 0;
                                            }
                                            4 => {
                                                new_side = 1;
                                                new_col = side_cols - 1;
                                                new_row = side_rows - 1 - current_row;
                                                new_direction = Direction::Left;
                                            }
                                            5 => {
                                                new_side = 4;
                                                new_row = side_rows - 1;
                                                new_col = current_row;
                                                new_direction = Direction::Up;
                                            }
                                            _ => unreachable!(),
                                        }
                                    } else {
                                        new_col += 1;
                                    }
                                }
                            }
                        }

                        if self.sides[new_side][[new_row, new_col]] != Tile::Wall {
                            self.current_position = (new_side, new_row, new_col);
                            self.current_direction = new_direction;
                        }
                    }
                }
            }
        }
    }

    fn points(&self) -> u32 {
        let side_shape = self.sides[0].shape();
        let (current_side, side_row, side_col) = self.current_position;
        let (current_row, current_col) = if side_shape[0] == 4 {
            // Example
            //   0
            // 123
            //   45
            match current_side {
                0 => (side_row, 2 * side_shape[1] + side_col),
                1 => (side_shape[0] + side_row, side_col),
                2 => (side_shape[0] + side_row, side_shape[1] + side_col),
                3 => (side_shape[0] + side_row, 2 * side_shape[1] + side_col),
                4 => (2 * side_shape[0] + side_row, 2 * side_shape[1] + side_col),
                5 => (2 * side_shape[0] + side_row, 3 * side_shape[1] + side_col),
                _ => unreachable!(),
            }
        } else {
            // Input
            //  01
            //  2
            // 34
            // 5
            match current_side {
                0 => (side_row, side_shape[1] + side_col),
                1 => (side_row, 2 * side_shape[1] + side_col),
                2 => (side_shape[0] + side_row, side_shape[1] + side_col),
                3 => (2 * side_shape[0] + side_row, side_col),
                4 => (2 * side_shape[0] + side_row, side_shape[1] + side_col),
                5 => (3 * side_shape[0] + side_row, side_col),
                _ => unreachable!(),
            }
        };

        1000 * (current_row as u32 + 1)
            + 4 * (current_col as u32 + 1)
            + self.current_direction.points()
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

// fn rotate_matrix<T>(array: &ArrayView<T, Ix2>, count: usize) -> Array<T, Ix2>
// where
//     T: Default + Copy,
// {
//     let shape = array.shape();
//     assert_eq!(shape[0], shape[1]);

//     let mut result = array.to_owned();
//     for _ in 0..count {
//         result = {
//             let mut rotated = Array::<T, Ix2>::default((shape[0], shape[1]).f());
//             for row in 0..shape[0] {
//                 for col in 0..shape[1] {
//                     rotated[[row, col]] = result[[shape[1] - 1 - col, row]];
//                 }
//             }
//             rotated
//         };
//     }
//     result
// }

fn parse_cube(contents: &str) -> Cube {
    let (rest, map) = parse_map(contents).expect("failed to parse map");
    let (_, instructions) = parse_instructions(rest).expect("failed to parse instructions");

    let map_shape = map.shape();
    let (side_rows, side_cols) = if map_shape[0] == 12 {
        // Example
        (map_shape[0] / 3, map_shape[1] / 4)
    } else {
        // Input
        (map_shape[0] / 4, map_shape[1] / 3)
    };

    let sides = if side_rows == 4 {
        // Example
        //   1
        // 234
        //   56
        [
            map.slice(s![0..side_rows, 2 * side_cols..3 * side_cols])
                .to_owned(),
            map.slice(s![side_rows..2 * side_rows, 0..side_cols])
                .to_owned(),
            map.slice(s![side_rows..2 * side_rows, side_cols..2 * side_cols])
                .to_owned(),
            map.slice(s![side_rows..2 * side_rows, 2 * side_cols..3 * side_cols])
                .to_owned(),
            map.slice(s![
                2 * side_rows..3 * side_rows,
                2 * side_cols..3 * side_cols
            ])
            .to_owned(),
            map.slice(s![
                2 * side_rows..3 * side_rows,
                3 * side_cols..4 * side_cols
            ])
            .to_owned(),
        ]
    } else {
        // Input
        //  01
        //  2
        // 34
        // 5
        [
            map.slice(s![0..side_rows, side_cols..2 * side_cols])
                .to_owned(),
            map.slice(s![0..side_rows, 2 * side_cols..3 * side_cols])
                .to_owned(),
            map.slice(s![side_rows..2 * side_rows, side_cols..2 * side_cols])
                .to_owned(),
            map.slice(s![2 * side_rows..3 * side_rows, 0..side_cols])
                .to_owned(),
            map.slice(s![2 * side_rows..3 * side_rows, side_cols..2 * side_cols])
                .to_owned(),
            map.slice(s![3 * side_rows..4 * side_rows, 0..side_cols])
                .to_owned(),
        ]
    };

    Cube::new(sides, instructions)
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
fn test_follow_instructions_board() {
    let contents = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.\n\n10R5L5R10L4R5L5\n";
    let mut board = parse_board(contents);
    board.follow_instructions();
    assert_eq!(board.points(), 6032);
}

#[test]
fn test_parse_cube() {
    let contents = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.\n\n10R5L5R10L4R5L5\n";
    let cube = parse_cube(contents);
    for i in 0..6 {
        assert_eq!(cube.sides[i].shape(), &[4, 4]);
    }
    assert_eq!(cube.instructions.len(), 13);
}

#[test]
fn test_follow_instructions_cube() {
    let contents = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.\n\n10R5L5R10L4R5L5\n";
    let mut cube = parse_cube(contents);
    cube.follow_instructions();
    assert_eq!(cube.points(), 5031);
}

// #[test]
// fn test_rotate_matrix() {
//     {
//         let array = array![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
//         let rotated = rotate_matrix(&array.view(), 1);
//         assert_eq!(rotated, array![[7, 4, 1], [8, 5, 2], [9, 6, 3],]);
//     }
//     {
//         let array = array![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
//         let rotated = rotate_matrix(&array.view(), 2);
//         assert_eq!(rotated, array![[9, 8, 7], [6, 5, 4], [3, 2, 1],]);
//     }
//     {
//         let array = array![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
//         let rotated = rotate_matrix(&array.view(), 4);
//         assert_eq!(rotated, array);
//     }
// }
