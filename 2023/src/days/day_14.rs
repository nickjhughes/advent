use std::{collections::HashMap, fs};

pub fn part1() -> String {
    let input = get_input_file_contents();
    let mut platform = Platform::parse(&input);
    platform.tilt(Direction::North);
    platform.north_load().to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let mut platform = Platform::parse(&input);
    platform.cycle_n(1_000_000_000);
    platform.north_load().to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input14").expect("Failed to open input file")
}

#[derive(Debug, PartialEq)]
struct Platform {
    tiles: Vec<Tile>,
    width: usize,
    states: HashMap<String, usize>,
}

#[derive(Debug, PartialEq)]
enum Tile {
    RoundRock,
    CubeRock,
    Empty,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Platform {
    fn state_hash(&self) -> String {
        let mut hash = String::new();
        for tile in self.tiles.iter() {
            match tile {
                Tile::RoundRock => hash.push('O'),
                Tile::CubeRock => hash.push('#'),
                Tile::Empty => hash.push('.'),
            }
        }
        hash
    }

    fn parse(input: &str) -> Self {
        let mut width = None;
        let mut tiles = Vec::new();
        for line in input.lines() {
            if width.is_none() {
                width = Some(line.len());
            }
            for ch in line.chars() {
                tiles.push(Tile::parse(ch));
            }
        }

        Platform {
            tiles,
            width: width.unwrap(),
            states: HashMap::new(),
        }
    }

    fn height(&self) -> usize {
        self.tiles.len() / self.width
    }

    fn tilt(&mut self, direction: Direction) {
        loop {
            let mut any_moved = false;

            match direction {
                Direction::North => {
                    for row in 1..self.height() {
                        for col in 0..self.width {
                            if self.tiles[row * self.width + col] == Tile::RoundRock
                                && self.tiles[(row - 1) * self.width + col] == Tile::Empty
                            {
                                // Move round rock up one row
                                self.tiles[(row - 1) * self.width + col] = Tile::RoundRock;
                                self.tiles[row * self.width + col] = Tile::Empty;
                                any_moved = true;
                            }
                        }
                    }
                }
                Direction::East => {
                    for col in (0..self.width - 1).rev() {
                        for row in 0..self.height() {
                            if self.tiles[row * self.width + col] == Tile::RoundRock
                                && self.tiles[row * self.width + (col + 1)] == Tile::Empty
                            {
                                // Move round rock right one column
                                self.tiles[row * self.width + (col + 1)] = Tile::RoundRock;
                                self.tiles[row * self.width + col] = Tile::Empty;
                                any_moved = true;
                            }
                        }
                    }
                }
                Direction::South => {
                    for row in (0..self.height() - 1).rev() {
                        for col in 0..self.width {
                            if self.tiles[row * self.width + col] == Tile::RoundRock
                                && self.tiles[(row + 1) * self.width + col] == Tile::Empty
                            {
                                // Move round rock down one row
                                self.tiles[(row + 1) * self.width + col] = Tile::RoundRock;
                                self.tiles[row * self.width + col] = Tile::Empty;
                                any_moved = true;
                            }
                        }
                    }
                }
                Direction::West => {
                    for col in 1..self.width {
                        for row in 0..self.height() {
                            if self.tiles[row * self.width + col] == Tile::RoundRock
                                && self.tiles[row * self.width + (col - 1)] == Tile::Empty
                            {
                                // Move round rock left one column
                                self.tiles[row * self.width + (col - 1)] = Tile::RoundRock;
                                self.tiles[row * self.width + col] = Tile::Empty;
                                any_moved = true;
                            }
                        }
                    }
                }
            }

            if !any_moved {
                break;
            }
        }
    }

    fn north_load(&self) -> u64 {
        let mut total_load = 0;
        for row in 0..self.height() {
            let round_rocks_count = self.tiles[row * self.width..row * self.width + self.width]
                .iter()
                .filter(|t| **t == Tile::RoundRock)
                .count();
            total_load += round_rocks_count * (self.height() - row);
        }
        total_load as u64
    }

    fn cycle(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
    }

    fn cycle_n(&mut self, n: usize) {
        let mut cycle_count = 0;
        while cycle_count < n {
            self.cycle();
            cycle_count += 1;

            let key = self.state_hash();
            if let Some(prev_cycle_count) = self.states.get(&key) {
                let loop_size = cycle_count - prev_cycle_count;
                let remaining_cycles = n - cycle_count;
                for _ in 0..(remaining_cycles % loop_size) {
                    self.cycle();
                }
                break;
            } else {
                self.states.insert(key, cycle_count);
            }
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height() {
            for col in 0..self.width {
                match self.tiles[row * self.width + col] {
                    Tile::RoundRock => write!(f, "O")?,
                    Tile::CubeRock => write!(f, "#")?,
                    Tile::Empty => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl Tile {
    fn parse(ch: char) -> Self {
        match ch {
            'O' => Tile::RoundRock,
            '#' => Tile::CubeRock,
            '.' => Tile::Empty,
            _ => panic!("invalid tile '{ch}'"),
        }
    }
}

#[test]
fn test_parse() {
    let input = "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....\n";
    let platform = Platform::parse(input);
    assert_eq!(platform.width, 10);
}

#[test]
fn test_tilt_north() {
    let input = "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....\n";
    let tilted_input = "OOOO.#.O..\nOO..#....#\nOO..O##..O\nO..#.OO...\n........#.\n..#....#.#\n..O..#.O.O\n..O.......\n#....###..\n#....#....\n";

    let mut platform = Platform::parse(input);
    platform.tilt(Direction::North);
    assert_eq!(platform.tiles, Platform::parse(tilted_input).tiles);
}

#[test]
fn test_north_load() {
    let input = "OOOO.#.O..\nOO..#....#\nOO..O##..O\nO..#.OO...\n........#.\n..#....#.#\n..O..#.O.O\n..O.......\n#....###..\n#....#....\n";
    let platform = Platform::parse(input);
    assert_eq!(platform.north_load(), 136);
}

#[test]
fn test_cycle() {
    let input = "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....\n";
    let cycled_once_input = ".....#....\n....#...O#\n...OO##...\n.OO#......\n.....OOO#.\n.O#...O#.#\n....O#....\n......OOOO\n#...O###..\n#..OO#....\n";
    let cycled_twice_input = ".....#....\n....#...O#\n.....##...\n..O#......\n.....OOO#.\n.O#...O#.#\n....O#...O\n.......OOO\n#..OO###..\n#.OOO#...O\n";
    let cycled_thrice_input = ".....#....\n....#...O#\n.....##...\n..O#......\n.....OOO#.\n.O#...O#.#\n....O#...O\n.......OOO\n#...O###.O\n#.OOO#...O\n";

    let mut platform = Platform::parse(input);

    platform.cycle();
    assert_eq!(platform.tiles, Platform::parse(cycled_once_input).tiles);

    platform.cycle();
    assert_eq!(platform.tiles, Platform::parse(cycled_twice_input).tiles);

    platform.cycle();
    assert_eq!(platform.tiles, Platform::parse(cycled_thrice_input).tiles);
}

#[test]
fn test_billion_cycles() {
    let input = "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....\n";
    let mut platform = Platform::parse(input);
    platform.cycle_n(1_000_000_000);
    assert_eq!(platform.north_load(), 64);
}
