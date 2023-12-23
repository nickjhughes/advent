use std::fs;

pub fn part1() -> String {
    let input = get_input_file_contents();
    let map = Map::parse(&input);
    (map.longest_path_length() - 1).to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let mut map = Map::parse(&input);
    map.replace_slopes();
    (map.longest_path_length() - 1).to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input23").expect("Failed to open input file")
}

struct Map {
    tiles: Vec<Tile>,
    width: usize,
}

#[derive(Debug, PartialEq)]
enum Tile {
    Path,
    Forest,
    SlopeUp,
    SlopeDown,
    SlopeLeft,
    SlopeRight,
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

    fn height(&self) -> usize {
        self.tiles.len() / self.width
    }

    fn replace_slopes(&mut self) {
        for tile in self.tiles.iter_mut() {
            if tile.is_slope() {
                *tile = Tile::Path;
            }
        }
    }

    fn goal_index(&self) -> usize {
        self.tiles.len() - 2
    }

    #[cfg(test)]
    fn longest_path(&self) -> Vec<usize> {
        self.longest_path_length_(1, Vec::new())
    }

    fn longest_path_length(&self) -> usize {
        self.longest_path_length_(1, Vec::new()).len()
    }

    fn longest_path_length_(&self, cur_idx: usize, mut path_so_far: Vec<usize>) -> Vec<usize> {
        path_so_far.push(cur_idx);

        let cur_row = cur_idx / self.width;
        let cur_col = cur_idx % self.width;

        let mut options = Vec::new();
        if self.tiles[cur_idx].is_slope() {
            // Must follow slope
            match self.tiles[cur_idx] {
                Tile::SlopeUp => options.push(cur_idx - self.width),
                Tile::SlopeDown => options.push(cur_idx + self.width),
                Tile::SlopeLeft => options.push(cur_idx - 1),
                Tile::SlopeRight => options.push(cur_idx + 1),
                _ => unreachable!(),
            }
        } else {
            // Up
            if cur_row > 0
                && self.tiles[cur_idx - self.width] != Tile::Forest
                && !path_so_far.contains(&(cur_idx - self.width))
                && self.tiles[cur_idx - self.width] != Tile::SlopeDown
            {
                options.push(cur_idx - self.width);
            }
            // Down
            if cur_row < self.height() - 1
                && self.tiles[cur_idx + self.width] != Tile::Forest
                && !path_so_far.contains(&(cur_idx + self.width))
                && self.tiles[cur_idx + self.width] != Tile::SlopeUp
            {
                options.push(cur_idx + self.width);
            }
            // Left
            if cur_col > 0
                && self.tiles[cur_idx - 1] != Tile::Forest
                && !path_so_far.contains(&(cur_idx - 1))
                && self.tiles[cur_idx - 1] != Tile::SlopeRight
            {
                options.push(cur_idx - 1);
            }
            // Right
            if cur_col < self.width - 1
                && self.tiles[cur_idx + 1] != Tile::Forest
                && !path_so_far.contains(&(cur_idx + 1))
                && self.tiles[cur_idx + 1] != Tile::SlopeLeft
            {
                options.push(cur_idx + 1);
            }
        }

        if options.is_empty() {
            // Dead end
            Vec::new()
        } else {
            let mut longest_path = Vec::new();
            for new_idx in options.iter() {
                let path = if *new_idx == self.goal_index() {
                    path_so_far.push(*new_idx);
                    path_so_far.clone()
                } else {
                    self.longest_path_length_(*new_idx, path_so_far.clone())
                };
                if path.len() > longest_path.len() {
                    longest_path = path;
                }
            }
            longest_path
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height() {
            for col in 0..self.width {
                let ch = match self.tiles[row * self.width + col] {
                    Tile::Path => '.',
                    Tile::Forest => '#',
                    Tile::SlopeUp => '^',
                    Tile::SlopeDown => 'v',
                    Tile::SlopeLeft => '<',
                    Tile::SlopeRight => '>',
                };
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Tile {
    fn parse(ch: char) -> Self {
        match ch {
            '.' => Tile::Path,
            '#' => Tile::Forest,
            '^' => Tile::SlopeUp,
            'v' => Tile::SlopeDown,
            '<' => Tile::SlopeLeft,
            '>' => Tile::SlopeRight,
            _ => panic!("invalid tile {ch}"),
        }
    }

    fn is_slope(&self) -> bool {
        matches!(
            self,
            Tile::SlopeUp | Tile::SlopeDown | Tile::SlopeLeft | Tile::SlopeRight
        )
    }
}

#[cfg(test)]
fn print_map_with_path(map: &Map, path: &[usize]) {
    for row in 0..map.height() {
        for col in 0..map.width {
            let ch = if row == 0 && col == 1 {
                'S'
            } else if path.contains(&(row * map.width + col)) {
                'O'
            } else {
                match map.tiles[row * map.width + col] {
                    Tile::Path => '.',
                    Tile::Forest => '#',
                    Tile::SlopeUp => '^',
                    Tile::SlopeDown => 'v',
                    Tile::SlopeLeft => '<',
                    Tile::SlopeRight => '>',
                }
            };
            print!("{}", ch);
        }
        println!();
    }
}

#[test]
fn test_parse_map() {
    let input = "#.#####################\n#.......#########...###\n#######.#########.#.###\n###.....#.>.>.###.#.###\n###v#####.#v#.###.#.###\n###.>...#.#.#.....#...#\n###v###.#.#.#########.#\n###...#.#.#.......#...#\n#####.#.#.#######.#.###\n#.....#.#.#.......#...#\n#.#####.#.#.#########v#\n#.#...#...#...###...>.#\n#.#.#v#######v###.###v#\n#...#.>.#...>.>.#.###.#\n#####v#.#.###v#.#.###.#\n#.....#...#...#.#.#...#\n#.#########.###.#.#.###\n#...###...#...#...#.###\n###.###.#.###v#####v###\n#...#...#.#.>.>.#.>.###\n#.###.###.#.###.#.#v###\n#.....###...###...#...#\n#####################.#\n";
    let map = Map::parse(input);
    assert_eq!(map.width, 23);
}

#[test]
fn test_longest_path_length() {
    let input = "#.#####################\n#.......#########...###\n#######.#########.#.###\n###.....#.>.>.###.#.###\n###v#####.#v#.###.#.###\n###.>...#.#.#.....#...#\n###v###.#.#.#########.#\n###...#.#.#.......#...#\n#####.#.#.#######.#.###\n#.....#.#.#.......#...#\n#.#####.#.#.#########v#\n#.#...#...#...###...>.#\n#.#.#v#######v###.###v#\n#...#.>.#...>.>.#.###.#\n#####v#.#.###v#.#.###.#\n#.....#...#...#.#.#...#\n#.#########.###.#.#.###\n#...###...#...#...#.###\n###.###.#.###v#####v###\n#...#...#.#.>.>.#.>.###\n#.###.###.#.###.#.#v###\n#.....###...###...#...#\n#####################.#\n";
    let map = Map::parse(input);
    print_map_with_path(&map, &map.longest_path());
    assert_eq!(map.longest_path_length() - 1, 94);
}

#[test]
fn test_longest_path_length_no_slopes() {
    let input = "#.#####################\n#.......#########...###\n#######.#########.#.###\n###.....#.>.>.###.#.###\n###v#####.#v#.###.#.###\n###.>...#.#.#.....#...#\n###v###.#.#.#########.#\n###...#.#.#.......#...#\n#####.#.#.#######.#.###\n#.....#.#.#.......#...#\n#.#####.#.#.#########v#\n#.#...#...#...###...>.#\n#.#.#v#######v###.###v#\n#...#.>.#...>.>.#.###.#\n#####v#.#.###v#.#.###.#\n#.....#...#...#.#.#...#\n#.#########.###.#.#.###\n#...###...#...#...#.###\n###.###.#.###v#####v###\n#...#...#.#.>.>.#.>.###\n#.###.###.#.###.#.#v###\n#.....###...###...#...#\n#####################.#\n";
    let mut map = Map::parse(input);
    map.replace_slopes();
    print_map_with_path(&map, &map.longest_path());
    assert_eq!(map.longest_path_length() - 1, 154);
}
