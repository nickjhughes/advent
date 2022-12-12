use ndarray::{prelude::*, Array, Ix2};
use std::fs;

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let heightmap = parse_heightmap(&contents);
    let path_length = heightmap.find_shortest_path().expect("Failed to find path");
    format!("{}", path_length)
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let heightmap = parse_heightmap(&contents);
    let best_start_path_length = heightmap.best_start();
    format!("{}", best_start_path_length)
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input12").expect("Failed to open input file")
}

#[derive(Debug, PartialEq, Eq)]
struct Point2 {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Heightmap {
    heights: Array<u8, Ix2>,
    start: Point2,
    goal: Point2,
}

impl Heightmap {
    fn dijkstra(&self, goal: usize) -> Vec<u32> {
        let grid_shape = self.heights.shape();
        let rows = grid_shape[0];
        let cols = grid_shape[1];
        let num_nodes = rows * cols;

        let mut queue = Vec::with_capacity(num_nodes);
        let mut distance = Vec::with_capacity(num_nodes);
        for y in 0..rows {
            for x in 0..cols {
                let i = y * cols + x;
                distance.push(std::u32::MAX);
                queue.push(i);
            }
        }
        distance[goal] = 0;

        while !queue.is_empty() {
            let mut min_dist_index = 0;
            let mut min_dist = std::u32::MAX;
            for (index, node) in queue.iter().enumerate() {
                if distance[*node] < min_dist {
                    min_dist = distance[*node];
                    min_dist_index = index;
                }
            }
            if min_dist == std::u32::MAX {
                // No more nodes are reachable from the goal node
                break;
            }
            let node = queue.remove(min_dist_index);

            let neighbors = self.neighbors(node);
            for neighbor in neighbors {
                let new_distance = distance[node] + 1;
                if new_distance < distance[neighbor] {
                    distance[neighbor] = new_distance;
                }
            }
        }

        distance
    }

    fn find_shortest_path(&self) -> Option<u32> {
        let grid_shape = self.heights.shape();
        let cols = grid_shape[1];
        let distance = self.dijkstra(self.goal.y * cols + self.goal.x);
        if distance[self.start.y * cols + self.start.x] == std::u32::MAX {
            None
        } else {
            Some(distance[self.start.y * cols + self.start.x])
        }
    }

    fn best_start(&self) -> u32 {
        let grid_shape = self.heights.shape();
        let cols = grid_shape[1];
        let distance = self.dijkstra(self.goal.y * cols + self.goal.x);
        let mut min_dist = std::u32::MAX;
        for (node, dist) in distance.iter().enumerate() {
            if self.heights[[node / cols, node % cols]] == 0 {
                if *dist < min_dist {
                    min_dist = distance[node];
                }
            }
        }
        min_dist
    }

    fn neighbors(&self, i: usize) -> Vec<usize> {
        let grid_shape = self.heights.shape();
        let rows = grid_shape[0];
        let cols = grid_shape[1];
        let y = i / cols;
        let x = i % cols;
        let height = self.heights[[y, x]];

        let mut neighbors = Vec::new();
        if x > 0 {
            if self.heights[[y, x - 1]] >= height.saturating_sub(1) {
                neighbors.push(y * cols + (x - 1));
            }
        }
        if x < cols - 1 {
            if self.heights[[y, x + 1]] >= height.saturating_sub(1) {
                neighbors.push(y * cols + (x + 1));
            }
        }
        if y > 0 {
            if self.heights[[y - 1, x]] >= height.saturating_sub(1) {
                neighbors.push((y - 1) * cols + x);
            }
        }
        if y < rows - 1 {
            if self.heights[[y + 1, x]] >= height.saturating_sub(1) {
                neighbors.push((y + 1) * cols + x);
            }
        }
        neighbors
    }
}

fn parse_heightmap(contents: &str) -> Heightmap {
    let lines = contents.split('\n').collect::<Vec<&str>>();
    assert!(!lines.is_empty());
    assert!(!lines[0].is_empty());
    let cols = lines[0].len();
    let rows = lines.iter().filter(|l| !l.is_empty()).count();
    let mut heights = Array::<u8, Ix2>::zeros((rows, cols).f());
    let mut start = None;
    let mut goal = None;
    let mut row = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        for (col, ch) in line.chars().enumerate() {
            let height = if ('a'..='z').contains(&ch) {
                ch as u8 - 97
            } else if ch == 'S' {
                start = Some(Point2 { x: col, y: row });
                0
            } else if ch == 'E' {
                goal = Some(Point2 { x: col, y: row });
                b'z' - b'a'
            } else {
                panic!("Invalid height");
            };
            heights[[row, col]] = height;
        }
        row += 1;
    }
    if start.is_none() {
        panic!("Failed to find start position");
    }
    if goal.is_none() {
        panic!("Failed to find goal position");
    }
    Heightmap {
        heights,
        start: start.unwrap(),
        goal: goal.unwrap(),
    }
}

#[test]
fn test_parse_heightmap() {
    let contents = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n";
    let heightmap = parse_heightmap(&contents);

    assert_eq!(heightmap.start, Point2 { x: 0, y: 0 });
    assert_eq!(heightmap.goal, Point2 { x: 5, y: 2 });
    assert_eq!(
        heightmap.heights,
        array![
            [0, 0, 1, 16, 15, 14, 13, 12],
            [0, 1, 2, 17, 24, 23, 23, 11],
            [0, 2, 2, 18, 25, 25, 23, 10],
            [0, 2, 2, 19, 20, 21, 22, 9],
            [0, 1, 3, 4, 5, 6, 7, 8],
        ]
    );
}

#[test]
fn test_neighbors() {
    let contents = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n";
    let heightmap = parse_heightmap(&contents);

    {
        let neighbors = heightmap.neighbors(0);
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&1));
        assert!(neighbors.contains(&8));
    }

    {
        let neighbors = heightmap.neighbors(9);
        assert_eq!(neighbors.len(), 4);
        assert!(neighbors.contains(&8));
        assert!(neighbors.contains(&10));
        assert!(neighbors.contains(&1));
        assert!(neighbors.contains(&17));
    }

    {
        let neighbors = heightmap.neighbors(21);
        assert_eq!(neighbors.len(), 1);
        assert!(neighbors.contains(&20));
    }
}

#[test]
fn test_find_shortest_path() {
    let contents = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n";
    let heightmap = parse_heightmap(&contents);
    let path_length = heightmap.find_shortest_path().expect("Failed to find path");
    assert_eq!(path_length, 31);
}

#[test]
fn test_best_start() {
    let contents = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n";
    let heightmap = parse_heightmap(&contents);
    let best_start_path_length = heightmap.best_start();
    assert_eq!(best_start_path_length, 29);
}
