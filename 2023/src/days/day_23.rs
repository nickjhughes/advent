use hashbrown::HashMap;
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
    let graph_edges = map.graph();
    let (_, longest_path_length) = find_longest_path(
        &graph_edges,
        map.start_index(),
        map.goal_index(),
        Vec::new(),
        0,
    );
    longest_path_length.to_string()
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

    fn start_index(&self) -> usize {
        1
    }

    fn goal_index(&self) -> usize {
        self.tiles.len() - 2
    }

    fn longest_path_length(&self) -> usize {
        self.longest_path(self.start_index(), Vec::new()).len()
    }

    fn longest_path(&self, mut cur_idx: usize, mut path_so_far: Vec<usize>) -> Vec<usize> {
        let mut options = Vec::new();
        loop {
            path_so_far.push(cur_idx);
            let cur_row = cur_idx / self.width;
            let cur_col = cur_idx % self.width;

            options.clear();
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
            if options.len() == 1 && options[0] != self.goal_index() {
                // Don't need to recurse if there's only one option
                cur_idx = options[0];
            } else {
                break;
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
                    self.longest_path(*new_idx, path_so_far.clone())
                };
                if path.len() > longest_path.len() {
                    longest_path = path;
                }
            }
            longest_path
        }
    }

    fn nodes(&self) -> Vec<usize> {
        let mut nodes = Vec::new();
        nodes.push(self.start_index());
        for row in 0..self.height() {
            for col in 0..self.width {
                if self.tiles[row * self.width + col] != Tile::Path {
                    continue;
                }

                let mut paths = 0;
                if row > 0 && self.tiles[(row - 1) * self.width + col] == Tile::Path {
                    paths += 1;
                }
                if row < self.height() - 1 && self.tiles[(row + 1) * self.width + col] == Tile::Path
                {
                    paths += 1;
                }
                if col > 0 && self.tiles[row * self.width + col - 1] == Tile::Path {
                    paths += 1;
                }
                if col < self.width - 1 && self.tiles[row * self.width + col + 1] == Tile::Path {
                    paths += 1;
                }
                if paths > 2 {
                    nodes.push(row * self.width + col);
                }
            }
        }
        nodes.push(self.goal_index());
        nodes
    }

    fn next_node(&self, nodes: &[usize], start_node: usize, next_path: usize) -> (usize, usize) {
        let mut dist = 1;
        let mut cur_idx = next_path;
        let mut prev_idx = start_node;
        while !nodes.contains(&cur_idx) {
            let row = cur_idx / self.width;
            let col = cur_idx % self.width;
            if row > 0
                && self.tiles[(row - 1) * self.width + col] == Tile::Path
                && (row - 1) * self.width + col != prev_idx
            {
                prev_idx = cur_idx;
                cur_idx = (row - 1) * self.width + col;
            } else if row < self.height() - 1
                && self.tiles[(row + 1) * self.width + col] == Tile::Path
                && (row + 1) * self.width + col != prev_idx
            {
                prev_idx = cur_idx;
                cur_idx = (row + 1) * self.width + col;
            } else if col > 0
                && self.tiles[row * self.width + col - 1] == Tile::Path
                && row * self.width + col - 1 != prev_idx
            {
                prev_idx = cur_idx;
                cur_idx = row * self.width + col - 1;
            } else if col < self.width - 1
                && self.tiles[row * self.width + col + 1] == Tile::Path
                && row * self.width + col + 1 != prev_idx
            {
                prev_idx = cur_idx;
                cur_idx = row * self.width + col + 1;
            } else {
                panic!("dead end");
            }
            dist += 1;
        }
        (cur_idx, dist)
    }

    fn graph(&self) -> HashMap<(usize, usize), usize> {
        let nodes = self.nodes();
        let mut edges = HashMap::new();
        for node in nodes.iter() {
            let row = node / self.width;
            let col = node % self.width;
            if row > 0 && self.tiles[(row - 1) * self.width + col] == Tile::Path {
                // Go up and find the next node
                let (next_node, dist) = self.next_node(&nodes, *node, (row - 1) * self.width + col);
                edges.insert((*node, next_node), dist);
            }
            if row < self.height() - 1 && self.tiles[(row + 1) * self.width + col] == Tile::Path {
                // Go down and find the next node
                let (next_node, dist) = self.next_node(&nodes, *node, (row + 1) * self.width + col);
                edges.insert((*node, next_node), dist);
            }
            if col > 0 && self.tiles[row * self.width + col - 1] == Tile::Path {
                // Go left and find the next node
                let (next_node, dist) = self.next_node(&nodes, *node, row * self.width + col - 1);
                edges.insert((*node, next_node), dist);
            }
            if col < self.width - 1 && self.tiles[row * self.width + col + 1] == Tile::Path {
                // Go right and find the next node
                let (next_node, dist) = self.next_node(&nodes, *node, row * self.width + col + 1);
                edges.insert((*node, next_node), dist);
            }
        }
        edges
    }
}

fn find_longest_path(
    edges: &HashMap<(usize, usize), usize>,
    cur_idx: usize,
    goal_idx: usize,
    mut path_so_far: Vec<usize>,
    path_length_so_far: usize,
) -> (Vec<usize>, usize) {
    path_so_far.push(cur_idx);

    let mut longest_path = Vec::new();
    let mut longest_path_length = 0;
    for ((cur_node, next_node), dist) in edges.iter() {
        if *cur_node != cur_idx {
            continue;
        }
        if path_so_far.contains(next_node) {
            continue;
        }
        if *cur_node == 118 {
            println!("{cur_node} -> {next_node}: {dist}");
        }
        let (path, path_length) = if *next_node == goal_idx {
            let mut path = path_so_far.clone();
            path.push(*next_node);
            (path, path_length_so_far + dist)
        } else {
            find_longest_path(
                edges,
                *next_node,
                goal_idx,
                path_so_far.clone(),
                path_length_so_far + dist,
            )
        };
        if path_length > longest_path_length {
            longest_path_length = path_length;
            longest_path = path;
        }
    }
    (longest_path, longest_path_length)
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
    assert_eq!(map.longest_path_length() - 1, 94);
}

#[test]
fn test_longest_path_length_no_slopes() {
    let input = "#.#####################\n#.......#########...###\n#######.#########.#.###\n###.....#.>.>.###.#.###\n###v#####.#v#.###.#.###\n###.>...#.#.#.....#...#\n###v###.#.#.#########.#\n###...#.#.#.......#...#\n#####.#.#.#######.#.###\n#.....#.#.#.......#...#\n#.#####.#.#.#########v#\n#.#...#...#...###...>.#\n#.#.#v#######v###.###v#\n#...#.>.#...>.>.#.###.#\n#####v#.#.###v#.#.###.#\n#.....#...#...#.#.#...#\n#.#########.###.#.#.###\n#...###...#...#...#.###\n###.###.#.###v#####v###\n#...#...#.#.>.>.#.>.###\n#.###.###.#.###.#.#v###\n#.....###...###...#...#\n#####################.#\n";
    let mut map = Map::parse(input);
    map.replace_slopes();
    assert_eq!(map.longest_path_length() - 1, 154);
}

#[test]
fn test_nodes() {
    let input = "#.#####################\n#.......#########...###\n#######.#########.#.###\n###.....#.>.>.###.#.###\n###v#####.#v#.###.#.###\n###.>...#.#.#.....#...#\n###v###.#.#.#########.#\n###...#.#.#.......#...#\n#####.#.#.#######.#.###\n#.....#.#.#.......#...#\n#.#####.#.#.#########v#\n#.#...#...#...###...>.#\n#.#.#v#######v###.###v#\n#...#.>.#...>.>.#.###.#\n#####v#.#.###v#.#.###.#\n#.....#...#...#.#.#...#\n#.#########.###.#.#.###\n#...###...#...#...#.###\n###.###.#.###v#####v###\n#...#...#.#.>.>.#.>.###\n#.###.###.#.###.#.#v###\n#.....###...###...#...#\n#####################.#\n";
    let mut map = Map::parse(input);
    map.replace_slopes();
    let nodes = map.nodes();
    assert_eq!(nodes, vec![1, 80, 118, 274, 304, 312, 450, 456, 527]);
}

#[test]
fn test_graph_longest_path() {
    let input = "#.#####################\n#.......#########...###\n#######.#########.#.###\n###.....#.>.>.###.#.###\n###v#####.#v#.###.#.###\n###.>...#.#.#.....#...#\n###v###.#.#.#########.#\n###...#.#.#.......#...#\n#####.#.#.#######.#.###\n#.....#.#.#.......#...#\n#.#####.#.#.#########v#\n#.#...#...#...###...>.#\n#.#.#v#######v###.###v#\n#...#.>.#...>.>.#.###.#\n#####v#.#.###v#.#.###.#\n#.....#...#...#.#.#...#\n#.#########.###.#.#.###\n#...###...#...#...#.###\n###.###.#.###v#####v###\n#...#...#.#.>.>.#.>.###\n#.###.###.#.###.#.#v###\n#.....###...###...#...#\n#####################.#\n";
    let mut map = Map::parse(input);
    map.replace_slopes();
    let graph_edges = map.graph();
    let (_, longest_path_length) = find_longest_path(
        &graph_edges,
        map.start_index(),
        map.goal_index(),
        Vec::new(),
        0,
    );
    assert_eq!(longest_path_length, 154);
}
