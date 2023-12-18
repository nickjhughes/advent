use hashbrown::HashMap;
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, fs};

pub fn part1() -> String {
    let input = get_input_file_contents();
    let map = Map::parse(&input);
    map.minimal_heat_loss(1, 3).to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let map = Map::parse(&input);
    map.minimal_heat_loss(4, 10).to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input17").expect("Failed to open input file")
}

#[derive(Debug)]
struct Map {
    blocks: Vec<u8>,
    width: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct SearchNode {
    index: usize,
    dir: Dir,
    steps: u8,
}

impl SearchNode {
    fn new(index: usize, dir: Dir, steps: u8) -> Self {
        SearchNode { index, dir, steps }
    }
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut blocks = Vec::new();
        let mut width = None;
        for line in input.lines() {
            if width.is_none() {
                width = Some(line.len());
            }
            blocks.extend(line.chars().map(|ch| ch as u8 - b'0'));
        }
        Map {
            blocks,
            width: width.unwrap(),
        }
    }

    fn height(&self) -> usize {
        self.blocks.len() / self.width
    }

    fn minimal_heat_loss(&self, min_steps: u8, max_steps: u8) -> u32 {
        let mut open_set = PriorityQueue::new();
        open_set.push(SearchNode::new(0, Dir::Right, 0), Reverse(0));

        let mut heat_loss: HashMap<SearchNode, u32> = HashMap::new();
        heat_loss.insert(SearchNode::new(0, Dir::Right, 0), 0);

        while !open_set.is_empty() {
            let (current, _) = open_set.pop().unwrap();
            let current_row = current.index / self.width;
            let current_col = current.index % self.width;

            if current_col == self.width - 1 && current_row == self.height() - 1 {
                // Reached goal
                return *heat_loss.get(&current).unwrap();
            }

            // Go through neighbors
            // Up
            let up = if current_row > 0
                && current.dir != Dir::Down
                && !(current.dir == Dir::Up && current.steps == max_steps)
                && !(current.dir != Dir::Up && current.steps < min_steps)
            {
                Some(SearchNode::new(
                    (current_row - 1) * self.width + current_col,
                    Dir::Up,
                    if current.dir == Dir::Up {
                        current.steps + 1
                    } else {
                        1
                    },
                ))
            } else {
                None
            };
            // Down
            let down = if current_row < self.height() - 1
                && current.dir != Dir::Up
                && !(current.dir == Dir::Down && current.steps == max_steps)
                && (!(current.dir != Dir::Down && current.steps < min_steps) || current.index == 0)
                && !(min_steps > 1
                    && current_row == self.height() - 2
                    && current_col == self.width - 1
                    && (current.dir != Dir::Down || current.steps < min_steps - 1))
            {
                Some(SearchNode::new(
                    (current_row + 1) * self.width + current_col,
                    Dir::Down,
                    if current.dir == Dir::Down {
                        current.steps + 1
                    } else {
                        1
                    },
                ))
            } else {
                None
            };
            // Left
            let left = if current_col > 0
                && current.dir != Dir::Right
                && !(current.dir == Dir::Left && current.steps == max_steps)
                && !(current.dir != Dir::Left && current.steps < min_steps)
            {
                Some(SearchNode::new(
                    current_row * self.width + current_col - 1,
                    Dir::Left,
                    if current.dir == Dir::Left {
                        current.steps + 1
                    } else {
                        1
                    },
                ))
            } else {
                None
            };
            // Right
            let right = if current_col < self.width - 1
                && current.dir != Dir::Left
                && !(current.dir == Dir::Right && current.steps == max_steps)
                && !(current.dir != Dir::Right && current.steps < min_steps)
                && !(min_steps > 1
                    && current_row == self.height() - 1
                    && current_col == self.width - 2
                    && (current.dir != Dir::Right || current.steps < min_steps - 1))
            {
                Some(SearchNode::new(
                    current_row * self.width + current_col + 1,
                    Dir::Right,
                    if current.dir == Dir::Right && current.index != 0 {
                        current.steps + 1
                    } else {
                        1
                    },
                ))
            } else {
                None
            };
            for neighbor in [up, down, left, right].into_iter().flatten() {
                let tentative_heat_loss =
                    heat_loss.get(&current).unwrap() + self.blocks[neighbor.index] as u32;
                if tentative_heat_loss < *heat_loss.get(&neighbor).unwrap_or(&u32::MAX) {
                    heat_loss.insert(neighbor, tentative_heat_loss);
                    open_set.push(neighbor, Reverse(tentative_heat_loss));
                }
            }
        }

        panic!("no path found")
    }
}

#[test]
fn test_parse() {
    let input = "2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533\n";
    let map = Map::parse(input);
    assert_eq!(map.width, 13);
}

#[test]
fn test_minimal_heat_loss() {
    let input = "2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533\n";
    let map = Map::parse(input);
    assert_eq!(map.minimal_heat_loss(1, 3), 102);
}

#[test]
fn test_minimal_heat_loss_ultra() {
    {
        let input = "2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533\n";
        let map = Map::parse(input);
        assert_eq!(map.minimal_heat_loss(4, 10), 94);
    }

    {
        let input = "111111111111\n999999999991\n999999999991\n999999999991\n999999999991\n";
        let map = Map::parse(input);
        assert_eq!(map.minimal_heat_loss(4, 10), 71);
    }

    {
        let input = "111111111111\n999999999991\n999999999991\n999999999991\n999999999991\n";
        let map = Map::parse(input);
        assert_eq!(map.minimal_heat_loss(4, 10), 71);
    }

    {
        let input =
            "19999\n19999\n19999\n19999\n19999\n19999\n19999\n19999\n19999\n19999\n19999\n11111\n";
        let map = Map::parse(input);
        assert_eq!(map.minimal_heat_loss(4, 10), 71);
    }
}
