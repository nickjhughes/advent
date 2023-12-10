use std::{collections::HashSet, fs};

pub fn part1() -> String {
    let input = get_input_file_contents();
    let map = Map::parse(&input);
    map.furthest_point_in_loop().to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let map = Map::parse(&input);
    map.tiles_enclosed_by_loop().to_string()
}

#[derive(Debug, PartialEq)]
struct Map {
    tiles: Vec<Tile>,
    width: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut tiles = Vec::new();
        let mut width = None;
        for line in input.lines() {
            if width.is_none() {
                width = Some(line.len());
            }
            for ch in line.chars() {
                tiles.push(Tile::parse(ch));
            }
        }
        Map {
            tiles,
            width: width.unwrap(),
        }
    }

    fn start_index(&self) -> usize {
        self.tiles.iter().position(|t| *t == Tile::Start).unwrap()
    }

    fn start_tile_type(&self) -> Tile {
        let start_index = self.start_index();

        let mut directions = Vec::new();
        if self.index_to_row(start_index) > 0
            && self.tiles[start_index - self.width].has_direction(Direction::South)
        {
            directions.push(Direction::North);
        }
        if self.index_to_row(start_index) < self.tiles.len() - 1
            && self.tiles[start_index + self.width].has_direction(Direction::North)
        {
            directions.push(Direction::South);
        }
        if self.index_to_col(start_index) > 0
            && self.tiles[start_index - 1].has_direction(Direction::East)
        {
            directions.push(Direction::West);
        }
        if self.index_to_col(start_index) < self.width - 1
            && self.tiles[start_index + 1].has_direction(Direction::West)
        {
            directions.push(Direction::East);
        }

        match (directions[0], directions[1]) {
            (Direction::North, Direction::South) => Tile::NorthSouth,
            (Direction::North, Direction::East) => Tile::NorthEast,
            (Direction::North, Direction::West) => Tile::NorthWest,
            (Direction::South, Direction::North) => Tile::NorthSouth,
            (Direction::South, Direction::East) => Tile::SouthEast,
            (Direction::South, Direction::West) => Tile::SouthWest,
            (Direction::East, Direction::North) => Tile::NorthEast,
            (Direction::East, Direction::South) => Tile::SouthEast,
            (Direction::East, Direction::West) => Tile::EastWest,
            (Direction::West, Direction::North) => Tile::NorthWest,
            (Direction::West, Direction::South) => Tile::SouthWest,
            (Direction::West, Direction::East) => Tile::EastWest,
            _ => unreachable!(),
        }
    }

    fn find_loop(&self) -> Vec<usize> {
        let mut loop_ = Vec::new();

        let start_index = self.start_index();
        loop_.push(start_index);

        let start_tile = self.start_tile_type();

        let prev_index = start_index;
        let (mut cur_index, mut src_direction) = if start_tile.has_direction(Direction::North) {
            // Can go north from start
            (prev_index - self.width, Direction::South)
        } else if start_tile.has_direction(Direction::South) {
            // Can go south from start
            (prev_index + self.width, Direction::North)
        } else if start_tile.has_direction(Direction::West) {
            // Can go west from start
            (prev_index - 1, Direction::East)
        } else if start_tile.has_direction(Direction::East) {
            // Can go east from start
            (prev_index + 1, Direction::West)
        } else {
            unreachable!()
        };
        loop_.push(cur_index);

        while cur_index != start_index {
            let next_direction = self.tiles[cur_index].direction(src_direction);
            cur_index = match next_direction {
                Direction::North => cur_index - self.width,
                Direction::South => cur_index + self.width,
                Direction::East => cur_index + 1,
                Direction::West => cur_index - 1,
            };
            loop_.push(cur_index);
            src_direction = next_direction.opposite();
        }

        loop_
    }

    fn index_to_row(&self, idx: usize) -> usize {
        idx / self.width
    }

    fn index_to_col(&self, idx: usize) -> usize {
        idx % self.width
    }

    fn height(&self) -> usize {
        self.tiles.len() / self.width
    }

    fn furthest_point_in_loop(&self) -> usize {
        let loop_ = self.find_loop();
        loop_.len() / 2
    }

    fn tile_has_direction(&self, index: usize, dir: Direction) -> bool {
        if index == self.start_index() {
            self.start_tile_type().has_direction(dir)
        } else {
            self.tiles[index].has_direction(dir)
        }
    }

    fn expanded_tiles(&self) -> Vec<Tile> {
        let loop_ = self.find_loop();

        let mut expanded_tiles = Vec::with_capacity((2 * self.height() - 1) * (2 * self.width - 1));
        for row in 0..self.height() {
            for col in 0..self.width {
                let i = row * self.width + col;
                if loop_.contains(&i) {
                    expanded_tiles.push(self.tiles[i]);
                } else {
                    expanded_tiles.push(Tile::Ground);
                }
                if col != self.width - 1 {
                    if self.tile_has_direction(i, Direction::East)
                        && self.tile_has_direction(i + 1, Direction::West)
                        && loop_.contains(&i)
                        && loop_.contains(&(i + 1))
                    {
                        // This expanded tile is part of the loop
                        expanded_tiles.push(Tile::EastWest);
                    } else {
                        // This expanded tile is not part of the loop
                        expanded_tiles.push(Tile::Ground);
                    }
                }
            }
            if row != self.height() - 1 {
                for col in 0..self.width {
                    let i = row * self.width + col;
                    if self.tile_has_direction(i, Direction::South)
                        && self.tile_has_direction(i + self.width, Direction::North)
                        && loop_.contains(&i)
                        && loop_.contains(&(i + self.width))
                    {
                        expanded_tiles.push(Tile::NorthSouth);
                    } else {
                        expanded_tiles.push(Tile::Ground);
                    }
                    if col != self.width - 1 {
                        expanded_tiles.push(Tile::Ground);
                    }
                }
            }
        }

        expanded_tiles
    }

    fn tiles_enclosed_by_loop(&self) -> usize {
        let expanded_tiles = self.expanded_tiles();
        let loop_ = self.find_loop();

        let mut inside_count = 0;
        for row in 0..self.height() {
            for col in 0..self.width {
                let i = row * self.width + col;
                if loop_.contains(&i) {
                    continue;
                }

                let j = (row * 2) * (2 * self.width - 1) + (col * 2);
                if !path_to_edge(
                    j,
                    &expanded_tiles,
                    2 * self.width - 1,
                    2 * self.height() - 1,
                ) {
                    inside_count += 1;
                }
            }
        }

        inside_count
    }
}

fn path_to_edge(start: usize, tiles: &[Tile], width: usize, height: usize) -> bool {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    stack.push(start);
    while let Some(tile) = stack.pop() {
        let row = tile / width;
        let col = tile % width;
        if row == 0 || row == height - 1 || col == 0 || col == width - 1 {
            return true;
        }

        if !visited.contains(&tile) {
            visited.insert(tile);

            if row > 0 && tiles[tile - width] == Tile::Ground {
                stack.push(tile - width);
            }
            if row < height - 1 && tiles[tile + width] == Tile::Ground {
                stack.push(tile + width);
            }
            if col > 0 && tiles[tile - 1] == Tile::Ground {
                stack.push(tile - 1);
            }
            if col < width - 1 && tiles[tile + 1] == Tile::Ground {
                stack.push(tile + 1);
            }
        }
    }
    false
}

impl Tile {
    fn parse(ch: char) -> Self {
        match ch {
            '|' => Tile::NorthSouth,
            '-' => Tile::EastWest,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!("invalid tile {ch}"),
        }
    }

    fn has_direction(&self, dir: Direction) -> bool {
        match dir {
            Direction::North => {
                matches!(self, Tile::NorthEast | Tile::NorthSouth | Tile::NorthWest)
            }
            Direction::South => {
                matches!(self, Tile::SouthEast | Tile::SouthWest | Tile::NorthSouth)
            }
            Direction::East => matches!(self, Tile::EastWest | Tile::NorthEast | Tile::SouthEast),
            Direction::West => matches!(self, Tile::EastWest | Tile::NorthWest | Tile::SouthWest),
        }
    }

    fn direction(&self, src: Direction) -> Direction {
        match self {
            Tile::NorthSouth => match src {
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                _ => unreachable!(),
            },
            Tile::EastWest => match src {
                Direction::East => Direction::West,
                Direction::West => Direction::East,
                _ => unreachable!(),
            },
            Tile::NorthEast => match src {
                Direction::North => Direction::East,
                Direction::East => Direction::North,
                _ => unreachable!(),
            },
            Tile::NorthWest => match src {
                Direction::North => Direction::West,
                Direction::West => Direction::North,
                _ => unreachable!(),
            },
            Tile::SouthWest => match src {
                Direction::South => Direction::West,
                Direction::West => Direction::South,
                _ => unreachable!(),
            },
            Tile::SouthEast => match src {
                Direction::East => Direction::South,
                Direction::South => Direction::East,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Tile::NorthSouth => '|',
            Tile::EastWest => '-',
            Tile::NorthEast => 'L',
            Tile::NorthWest => 'J',
            Tile::SouthWest => '7',
            Tile::SouthEast => 'F',
            Tile::Ground => '.',
            Tile::Start => 'S',
        };
        write!(f, "{}", ch)
    }
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input10").expect("Failed to open input file")
}

#[test]
fn test_parse() {
    let input = ".....\n.S-7.\n.|.|.\n.L-J.\n.....\n";
    let map = Map::parse(input);
    assert_eq!(
        map,
        Map {
            tiles: vec![
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Start,
                Tile::EastWest,
                Tile::SouthWest,
                Tile::Ground,
                Tile::Ground,
                Tile::NorthSouth,
                Tile::Ground,
                Tile::NorthSouth,
                Tile::Ground,
                Tile::Ground,
                Tile::NorthEast,
                Tile::EastWest,
                Tile::NorthWest,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
                Tile::Ground,
            ],
            width: 5
        }
    );
}

#[test]
fn test_furthest_point_in_loop() {
    {
        let input = "-L|F7\n7S-7|\nL|7||\n-L-J|\nL|-JF\n";
        let map = Map::parse(&input);
        assert_eq!(map.furthest_point_in_loop(), 4);
    }

    {
        let input = "7-F7-\n.FJ|7\nSJLL7\n|F--J\nLJ.LJ\n";
        let map = Map::parse(&input);
        assert_eq!(map.furthest_point_in_loop(), 8);
    }
}

#[test]
fn test_tiles_enclosed_by_loop() {
    {
        let input = "...........\n.S-------7.\n.|F-----7|.\n.||.....||.\n.||.....||.\n.|L-7.F-J|.\n.|..|.|..|.\n.L--J.L--J.\n...........\n";
        let map = Map::parse(&input);
        assert_eq!(map.tiles_enclosed_by_loop(), 4);
    }

    {
        let input = "..........\n.S------7.\n.|F----7|.\n.||....||.\n.||....||.\n.|L-7F-J|.\n.|..||..|.\n.L--JL--J.\n..........\n";
        let map = Map::parse(&input);
        assert_eq!(map.tiles_enclosed_by_loop(), 4);
    }

    {
        let input = ".F----7F7F7F7F-7....\n.|F--7||||||||FJ....\n.||.FJ||||||||L7....\nFJL7L7LJLJ||LJ.L-7..\nL--J.L7...LJS7F-7L7.\n....F-J..F7FJ|L7L7L7\n....L7.F7||L7|.L7L7|\n.....|FJLJ|FJ|F7|.LJ\n....FJL-7.||.||||...\n....L---J.LJ.LJLJ...\n";
        let map = Map::parse(&input);
        assert_eq!(map.tiles_enclosed_by_loop(), 8);
    }

    {
        let input = "FF7FSF7F7F7F7F7F---7\nL|LJ||||||||||||F--J\nFL-7LJLJ||||||LJL-77\nF--JF--7||LJLJ7F7FJ-\nL---JF-JLJ.||-FJLJJ7\n|F|F-JF---7F7-L7L|7|\n|FFJF7L7F-JF7|JL---7\n7-L-JL7||F7|L7F-7F7|\nL.L7LFJ|||||FJL7||LJ\nL7JLJL-JLJLJL--JLJ.L\n";
        let map = Map::parse(&input);
        assert_eq!(map.tiles_enclosed_by_loop(), 10);
    }
}
