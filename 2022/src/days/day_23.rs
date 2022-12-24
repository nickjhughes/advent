use std::{collections::HashMap, fmt, fs};

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let mut map = Map::parse(&contents);
    for _ in 0..10 {
        map.move_elves();
    }
    let ground_tiles_count = map.get_ground_tiles_count();
    format!("{}", ground_tiles_count)
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let mut map = Map::parse(&contents);
    let num_rounds = map.num_rounds_to_finalise();
    format!("{}", num_rounds)
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input23").expect("Failed to open input file")
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug)]
struct Map {
    elves: Vec<Point>,
    directions: [Direction; 4],
}

impl Map {
    fn parse(contents: &str) -> Self {
        let mut elves = Vec::new();
        for (row, line) in contents.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    elves.push(Point::new(row as i32, col as i32));
                }
            }
        }
        Self {
            elves,
            directions: [
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ],
        }
    }

    fn next_direction(&mut self) {
        self.directions.rotate_left(1);
    }

    fn neighbors(elf: &Point, elves: &[Point]) -> [bool; 8] {
        [
            elves.contains(&Point::new(elf.row - 1, elf.col - 1)), // NW
            elves.contains(&Point::new(elf.row - 1, elf.col)),     // N
            elves.contains(&Point::new(elf.row - 1, elf.col + 1)), // NE
            elves.contains(&Point::new(elf.row, elf.col + 1)),     // E
            elves.contains(&Point::new(elf.row + 1, elf.col + 1)), // SE
            elves.contains(&Point::new(elf.row + 1, elf.col)),     // S
            elves.contains(&Point::new(elf.row + 1, elf.col - 1)), // SW
            elves.contains(&Point::new(elf.row, elf.col - 1)),     // W
        ]
    }

    fn move_elves(&mut self) -> usize {
        // First half
        let mut new_positions = Vec::with_capacity(self.elves.len());
        for elf in &self.elves {
            let neighbors = Self::neighbors(elf, &self.elves);
            if neighbors.iter().map(|b| usize::from(*b)).sum::<usize>() == 0 {
                new_positions.push(*elf);
                continue;
            }
            let mut found_new_position = false;
            for direction in &self.directions {
                match direction {
                    Direction::North => {
                        if !neighbors[0] && !neighbors[1] && !neighbors[2] {
                            new_positions.push(Point::new(elf.row - 1, elf.col));
                            found_new_position = true;
                            break;
                        }
                    }
                    Direction::South => {
                        if !neighbors[4] && !neighbors[5] && !neighbors[6] {
                            new_positions.push(Point::new(elf.row + 1, elf.col));
                            found_new_position = true;
                            break;
                        }
                    }
                    Direction::West => {
                        if !neighbors[0] && !neighbors[6] && !neighbors[7] {
                            new_positions.push(Point::new(elf.row, elf.col - 1));
                            found_new_position = true;
                            break;
                        }
                    }
                    Direction::East => {
                        if !neighbors[2] && !neighbors[3] && !neighbors[4] {
                            new_positions.push(Point::new(elf.row, elf.col + 1));
                            found_new_position = true;
                            break;
                        }
                    }
                }
            }
            if !found_new_position {
                new_positions.push(*elf);
            }
        }

        // Second half
        let mut elves_moved = 0;
        let mut new_position_count = HashMap::new();
        for position in &new_positions {
            *new_position_count.entry(*position).or_insert(0) += 1;
        }
        for (i, elf) in self.elves.iter_mut().enumerate() {
            if new_position_count[&new_positions[i]] == 1 {
                if *elf != new_positions[i] {
                    elf.row = new_positions[i].row;
                    elf.col = new_positions[i].col;
                    elves_moved += 1;
                }
            }
        }

        self.next_direction();
        elves_moved
    }

    fn smallest_rectangle(&self) -> (i32, i32, i32, i32) {
        let min_row = self.elves.iter().map(|p| p.row).min().unwrap();
        let max_row = self.elves.iter().map(|p| p.row).max().unwrap();
        let min_col = self.elves.iter().map(|p| p.col).min().unwrap();
        let max_col = self.elves.iter().map(|p| p.col).max().unwrap();
        (min_row, max_row, min_col, max_col)
    }

    fn get_ground_tiles_count(&self) -> usize {
        let mut ground_tiles_count = 0;
        let (min_row, max_row, min_col, max_col) = self.smallest_rectangle();
        for row in min_row..=max_row {
            for col in min_col..=max_col {
                if !self.elves.contains(&Point::new(row, col)) {
                    ground_tiles_count += 1;
                }
            }
        }
        ground_tiles_count
    }

    fn num_rounds_to_finalise(&mut self) -> usize {
        let mut num_rounds = 0;
        loop {
            let elves_moved = self.move_elves();
            num_rounds += 1;
            if elves_moved == 0 {
                break;
            }
        }
        num_rounds
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (min_row, max_row, min_col, max_col) = self.smallest_rectangle();
        write!(f, "   ")?;
        for col in min_col..=max_col {
            write!(f, "{:02} ", col)?;
        }
        writeln!(f)?;
        for row in min_row..=max_row {
            write!(f, "{:02} ", row)?;
            for col in min_col..=max_col {
                if self.elves.contains(&Point::new(row, col)) {
                    let pos = self
                        .elves
                        .iter()
                        .position(|e| *e == Point::new(row, col))
                        .unwrap();
                    write!(f, "{:02} ", pos)?;
                } else {
                    write!(f, ".. ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[test]
fn test_parse_map() {
    let contents = ".....\n..##.\n..#..\n.....\n..##.\n.....\n";
    let map = Map::parse(contents);
    assert_eq!(map.elves.len(), 5);
    assert!(map.elves.contains(&Point::new(1, 2)));
    assert!(map.elves.contains(&Point::new(1, 3)));
    assert!(map.elves.contains(&Point::new(2, 2)));
    assert!(map.elves.contains(&Point::new(4, 2)));
    assert!(map.elves.contains(&Point::new(4, 3)));
}

#[test]
fn test_move_elves() {
    let contents = ".....\n..##.\n..#..\n.....\n..##.\n.....\n";
    let mut map = Map::parse(contents);
    map.move_elves();
    assert!(map.elves.contains(&Point::new(0, 2)));
    assert!(map.elves.contains(&Point::new(0, 3)));
    assert!(map.elves.contains(&Point::new(2, 2)));
    assert!(map.elves.contains(&Point::new(3, 3)));
    assert!(map.elves.contains(&Point::new(4, 2)));
    assert_eq!(
        map.directions,
        [
            Direction::South,
            Direction::West,
            Direction::East,
            Direction::North
        ]
    );
}

#[test]
fn test_move_elves_3_rounds() {
    let contents = ".....\n..##.\n..#..\n.....\n..##.\n.....\n";
    let mut map = Map::parse(contents);
    for _ in 0..3 {
        map.move_elves();
    }
    assert!(map.elves.contains(&Point::new(0, 2)));
    assert!(map.elves.contains(&Point::new(1, 4)));
    assert!(map.elves.contains(&Point::new(2, 0)));
    assert!(map.elves.contains(&Point::new(3, 4)));
    assert!(map.elves.contains(&Point::new(5, 2)));
}

#[test]
fn test_get_ground_tiles_count() {
    let contents = "....#..\n..###.#\n#...#.#\n.#...##\n#.###..\n##.#.##\n.#..#..\n";
    let mut map = Map::parse(contents);
    for _ in 0..10 {
        map.move_elves();
    }
    let ground_tiles_count = map.get_ground_tiles_count();
    assert_eq!(ground_tiles_count, 110);
}

#[test]
fn test_num_rounds_to_finalise() {
    let contents = "....#..\n..###.#\n#...#.#\n.#...##\n#.###..\n##.#.##\n.#..#..\n";
    let mut map = Map::parse(contents);
    let num_rounds = map.num_rounds_to_finalise();
    assert_eq!(num_rounds, 20);
}
