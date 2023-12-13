use std::{collections::HashSet, fs};

pub fn part1() -> String {
    let input = get_input_file_contents();
    let tiles = parse_tiles(&input);
    flipped_tiles(&tiles).len().to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let tiles = parse_tiles(&input);
    let mut flipped_tiles = flipped_tiles(&tiles);
    for _ in 0..100 {
        daily_flip(&mut flipped_tiles);
    }
    flipped_tiles.len().to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input24").expect("Failed to open input file")
}

fn parse_tiles(input: &str) -> Vec<Tile> {
    input.lines().map(Tile::parse).collect()
}

fn flipped_tiles(tiles: &[Tile]) -> HashSet<Coord> {
    let mut flipped_tiles = HashSet::new();
    for tile in tiles {
        let coord = tile.coord();
        if flipped_tiles.contains(&coord) {
            flipped_tiles.remove(&coord);
        } else {
            flipped_tiles.insert(coord);
        }
    }
    flipped_tiles
}

fn daily_flip(flipped_tiles: &mut HashSet<Coord>) {
    // Flipped tiles to unflip
    let mut to_unflip = Vec::new();
    for coord in flipped_tiles.iter() {
        let mut flipped_neighbors = 0;
        for neighbor in coord.neighbors() {
            if flipped_tiles.contains(&neighbor) {
                flipped_neighbors += 1;
            }
        }
        if flipped_neighbors == 0 || flipped_neighbors > 2 {
            to_unflip.push(*coord);
        }
    }

    // Unflipped tiles to flip
    let mut to_flip = Vec::new();
    let min_x = flipped_tiles.iter().map(|c| c.x).min().unwrap();
    let max_x = flipped_tiles.iter().map(|c| c.x).max().unwrap();
    let min_y = flipped_tiles.iter().map(|c| c.y).min().unwrap();
    let max_y = flipped_tiles.iter().map(|c| c.y).max().unwrap();
    for x in min_x - 1..=max_x + 1 {
        for y in min_y - 1..=max_y + 1 {
            let coord = Coord { x, y };
            if flipped_tiles.contains(&coord) {
                continue;
            }
            let mut flipped_neighbors = 0;
            for neighbor in coord.neighbors() {
                if flipped_tiles.contains(&neighbor) {
                    flipped_neighbors += 1;
                }
            }
            if flipped_neighbors == 2 {
                to_flip.push(coord);
            }
        }
    }

    for coord in to_unflip {
        flipped_tiles.remove(&coord);
    }
    flipped_tiles.extend(to_flip);
}

#[derive(Debug, PartialEq)]
struct Tile(Vec<Direction>);

#[derive(Debug, PartialEq)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Tile {
    fn parse(input: &str) -> Self {
        let mut directions = Vec::new();
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            match chars[i] {
                'e' => {
                    directions.push(Direction::East);
                    i += 1;
                }
                's' => {
                    match chars[i + 1] {
                        'e' => directions.push(Direction::SouthEast),
                        'w' => directions.push(Direction::SouthWest),
                        _ => panic!("invalid direction"),
                    }
                    i += 2;
                }
                'w' => {
                    directions.push(Direction::West);
                    i += 1;
                }
                'n' => {
                    match chars[i + 1] {
                        'e' => directions.push(Direction::NorthEast),
                        'w' => directions.push(Direction::NorthWest),
                        _ => panic!("invalid direction"),
                    }
                    i += 2;
                }
                _ => panic!("invalid direction"),
            }
        }
        Tile(directions)
    }

    fn coord(&self) -> Coord {
        let mut coord = Coord { x: 0, y: 0 };
        for dir in self.0.iter() {
            coord = coord.move_dir(dir);
        }
        coord
    }
}

impl Coord {
    fn neighbors(&self) -> [Coord; 6] {
        [
            self.move_dir(&Direction::East),
            self.move_dir(&Direction::SouthEast),
            self.move_dir(&Direction::SouthWest),
            self.move_dir(&Direction::West),
            self.move_dir(&Direction::NorthWest),
            self.move_dir(&Direction::NorthEast),
        ]
    }

    fn move_dir(&self, dir: &Direction) -> Coord {
        match dir {
            Direction::East => Coord {
                x: self.x + 1,
                y: self.y,
            },
            Direction::SouthEast => {
                if self.y % 2 != 0 {
                    Coord {
                        x: self.x + 1,
                        y: self.y - 1,
                    }
                } else {
                    Coord {
                        x: self.x,
                        y: self.y - 1,
                    }
                }
            }
            Direction::SouthWest => {
                if self.y % 2 == 0 {
                    Coord {
                        x: self.x - 1,
                        y: self.y - 1,
                    }
                } else {
                    Coord {
                        x: self.x,
                        y: self.y - 1,
                    }
                }
            }
            Direction::West => Coord {
                x: self.x - 1,
                y: self.y,
            },
            Direction::NorthWest => {
                if self.y % 2 == 0 {
                    Coord {
                        x: self.x - 1,
                        y: self.y + 1,
                    }
                } else {
                    Coord {
                        x: self.x,
                        y: self.y + 1,
                    }
                }
            }
            Direction::NorthEast => {
                if self.y % 2 != 0 {
                    Coord {
                        x: self.x + 1,
                        y: self.y + 1,
                    }
                } else {
                    Coord {
                        x: self.x,
                        y: self.y + 1,
                    }
                }
            }
        }
    }
}

#[test]
fn test_parse_tile() {
    assert_eq!(
        Tile::parse("esenee"),
        Tile(vec![
            Direction::East,
            Direction::SouthEast,
            Direction::NorthEast,
            Direction::East
        ])
    );
}

#[test]
fn test_tile_coord() {
    assert_eq!(Tile::parse("esew").coord(), Coord { x: 0, y: -1 });
    assert_eq!(Tile::parse("nwwswee").coord(), Coord { x: 0, y: 0 });
}

#[test]
fn test_flipped_tiles_count() {
    let input = "sesenwnenenewseeswwswswwnenewsewsw\nneeenesenwnwwswnenewnwwsewnenwseswesw\nseswneswswsenwwnwse\nnwnwneseeswswnenewneswwnewseswneseene\nswweswneswnenwsewnwneneseenw\neesenwseswswnenwswnwnwsewwnwsene\nsewnenenenesenwsewnenwwwse\nwenwwweseeeweswwwnwwe\nwsweesenenewnwwnwsenewsenwwsesesenwne\nneeswseenwwswnwswswnw\nnenwswwsewswnenenewsenwsenwnesesenew\nenewnwewneswsewnwswenweswnenwsenwsw\nsweneswneswneneenwnewenewwneswswnese\nswwesenesewenwneswnwwneseswwne\nenesenwswwswneneswsenwnewswseenwsese\nwnwnesenesenenwwnenwsewesewsesesew\nnenewswnwewswnenesenwnesewesw\neneswnwswnwsenenwnwnwwseeswneewsenese\nneswnwewnwnwseenwseesewsenwsweewe\nwseweeenwnesenwwwswnew\n";
    let tiles = parse_tiles(input);
    assert_eq!(flipped_tiles(&tiles).len(), 10);
}

#[test]
fn test_daily_flip() {
    let input = "sesenwnenenewseeswwswswwnenewsewsw\nneeenesenwnwwswnenewnwwsewnenwseswesw\nseswneswswsenwwnwse\nnwnwneseeswswnenewneswwnewseswneseene\nswweswneswnenwsewnwneneseenw\neesenwseswswnenwswnwnwsewwnwsene\nsewnenenenesenwsewnenwwwse\nwenwwweseeeweswwwnwwe\nwsweesenenewnwwnwsenewsenwwsesesenwne\nneeswseenwwswnwswswnw\nnenwswwsewswnenenewsenwsenwnesesenew\nenewnwewneswsewnwswenweswnenwsenwsw\nsweneswneswneneenwnewenewwneswswnese\nswwesenesewenwneswnwwneseswwne\nenesenwswwswneneswsenwnewswseenwsese\nwnwnesenesenenwwnenwsewesewsesesew\nnenewswnwewswnenesenwnesewesw\neneswnwswnwsenenwnwnwwseeswneewsenese\nneswnwewnwnwseenwseesewsenwsweewe\nwseweeenwnesenwwwswnew\n";
    let tiles = parse_tiles(input);
    let mut flipped_tiles = flipped_tiles(&tiles);

    daily_flip(&mut flipped_tiles);
    assert_eq!(flipped_tiles.len(), 15);

    daily_flip(&mut flipped_tiles);
    assert_eq!(flipped_tiles.len(), 12);

    daily_flip(&mut flipped_tiles);
    assert_eq!(flipped_tiles.len(), 25);

    for _ in 0..97 {
        daily_flip(&mut flipped_tiles);
    }
    assert_eq!(flipped_tiles.len(), 2208);
}
