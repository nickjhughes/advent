use std::{collections::HashSet, fs};

pub fn part1() -> String {
    let input = get_input_file_contents();
    let contraption = Contraption::parse(&input);
    contraption.energized_tiles(0, 0, Dir::Right).to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let contraption = Contraption::parse(&input);
    contraption.max_energized_tiles().to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input16").expect("Failed to open input file")
}

#[derive(Debug)]
struct Contraption {
    tiles: Vec<Tile>,
    width: usize,
}

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
    ForwardMirror,
    BackwardMirror,
    VertSplitter,
    HorzSplitter,
}

#[derive(Debug, PartialEq)]
struct Beam {
    row: usize,
    col: usize,
    dir: Dir,
}

impl Beam {
    fn index(&self, width: usize) -> usize {
        self.row * width + self.col
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Contraption {
    fn parse(input: &str) -> Self {
        let mut tiles = Vec::new();
        let mut width = None;
        for line in input.lines() {
            if width.is_none() {
                width = Some(line.len());
            }
            tiles.extend(line.chars().map(Tile::parse));
        }
        Contraption {
            tiles,
            width: width.unwrap(),
        }
    }

    fn height(&self) -> usize {
        self.tiles.len() / self.width
    }

    fn energized_tiles(&self, start_row: usize, start_col: usize, start_dir: Dir) -> usize {
        let mut energized_tiles = vec![false; self.tiles.len()];
        let mut beams = vec![Beam {
            row: start_row,
            col: start_col,
            dir: start_dir,
        }];
        let mut past_beams: HashSet<(usize, Dir)> = HashSet::new();

        while let Some(mut beam) = beams.pop() {
            loop {
                energized_tiles[beam.index(self.width)] = true;

                // Deal with mirrors and splitters
                match self.tiles[beam.index(self.width)] {
                    Tile::Empty => {}
                    Tile::ForwardMirror => match beam.dir {
                        Dir::Left => beam.dir = Dir::Down,
                        Dir::Right => beam.dir = Dir::Up,
                        Dir::Up => beam.dir = Dir::Right,
                        Dir::Down => beam.dir = Dir::Left,
                    },
                    Tile::BackwardMirror => match beam.dir {
                        Dir::Left => beam.dir = Dir::Up,
                        Dir::Right => beam.dir = Dir::Down,
                        Dir::Up => beam.dir = Dir::Left,
                        Dir::Down => beam.dir = Dir::Right,
                    },
                    Tile::VertSplitter => match beam.dir {
                        Dir::Left | Dir::Right => {
                            beam.dir = Dir::Up;
                            if past_beams.insert((beam.index(self.width), Dir::Down)) {
                                beams.push(Beam {
                                    row: beam.row,
                                    col: beam.col,
                                    dir: Dir::Down,
                                });
                            }
                        }
                        Dir::Up | Dir::Down => {}
                    },
                    Tile::HorzSplitter => match beam.dir {
                        Dir::Up | Dir::Down => {
                            beam.dir = Dir::Left;
                            if past_beams.insert((beam.index(self.width), Dir::Right)) {
                                beams.push(Beam {
                                    row: beam.row,
                                    col: beam.col,
                                    dir: Dir::Right,
                                });
                            }
                        }
                        Dir::Left | Dir::Right => {}
                    },
                }

                // Move beam
                match &beam.dir {
                    Dir::Left => {
                        if beam.col == 0 {
                            // Left the contraption
                            break;
                        } else {
                            beam.col -= 1;
                        }
                    }
                    Dir::Right => {
                        if beam.col == self.width - 1 {
                            // Left the contraption
                            break;
                        } else {
                            beam.col += 1;
                        }
                    }
                    Dir::Up => {
                        if beam.row == 0 {
                            // Left the contraption
                            break;
                        } else {
                            beam.row -= 1;
                        }
                    }
                    Dir::Down => {
                        if beam.row == self.height() - 1 {
                            // Left the contraption
                            break;
                        } else {
                            beam.row += 1;
                        }
                    }
                }

                if !past_beams.insert((beam.index(self.width), beam.dir)) {
                    break;
                }
            }
        }

        energized_tiles.iter().filter(|t| **t).count()
    }

    fn max_energized_tiles(&self) -> usize {
        let mut max_energized_tiles = 0;

        // Top row
        for col in 0..self.width {
            let energized_tiles = self.energized_tiles(0, col, Dir::Down);
            max_energized_tiles = max_energized_tiles.max(energized_tiles);
        }
        // Bottom row
        for col in 0..self.width {
            let energized_tiles = self.energized_tiles(self.height() - 1, col, Dir::Up);
            max_energized_tiles = max_energized_tiles.max(energized_tiles);
        }
        // Left column
        for row in 0..self.height() {
            let energized_tiles = self.energized_tiles(row, 0, Dir::Right);
            max_energized_tiles = max_energized_tiles.max(energized_tiles);
        }
        // Right column
        for row in 0..self.height() {
            let energized_tiles = self.energized_tiles(row, self.width - 1, Dir::Left);
            max_energized_tiles = max_energized_tiles.max(energized_tiles);
        }

        max_energized_tiles
    }
}

impl Tile {
    fn parse(ch: char) -> Self {
        match ch {
            '.' => Tile::Empty,
            '\\' => Tile::BackwardMirror,
            '/' => Tile::ForwardMirror,
            '-' => Tile::HorzSplitter,
            '|' => Tile::VertSplitter,
            _ => panic!("invalid tile {ch}"),
        }
    }
}

#[test]
fn test_parse() {
    let input = ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....\n";
    let contraption = Contraption::parse(input);
    assert_eq!(contraption.width, 10);
}

#[test]
fn test_energized_tiles() {
    let input = ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....\n";
    let contraption = Contraption::parse(input);
    assert_eq!(contraption.energized_tiles(0, 0, Dir::Right), 46);
}

#[test]
fn test_max_energized_tiles() {
    let input = ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....\n";
    let contraption = Contraption::parse(input);
    assert_eq!(contraption.max_energized_tiles(), 51);
}
