use hashbrown::HashSet;
use std::fs;

pub fn part1() -> String {
    let input = get_input_file_contents();
    let dig = DigPlan::parse(&input);
    dig.size().to_string()
}

pub fn part2() -> String {
    let _input = get_input_file_contents();
    "".into()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input18").expect("Failed to open input file")
}

struct DigPlan {
    steps: Vec<DigStep>,
}

struct DigStep {
    dir: Dir,
    steps: u8,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    row: isize,
    col: isize,
}

#[derive(Debug, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl DigPlan {
    fn parse(input: &str) -> Self {
        DigPlan {
            steps: input.lines().map(DigStep::parse).collect(),
        }
    }

    fn size(&self) -> usize {
        let mut filled = HashSet::new();
        let mut pos = Position { row: 0, col: 0 };
        filled.insert(pos);
        for step in self.steps.iter() {
            match step.dir {
                Dir::Up => {
                    for _ in 0..step.steps {
                        pos.row -= 1;
                        filled.insert(pos);
                    }
                }
                Dir::Down => {
                    for _ in 0..step.steps {
                        pos.row += 1;
                        filled.insert(pos);
                    }
                }
                Dir::Left => {
                    for _ in 0..step.steps {
                        pos.col -= 1;
                        filled.insert(pos);
                    }
                }
                Dir::Right => {
                    for _ in 0..step.steps {
                        pos.col += 1;
                        filled.insert(pos);
                    }
                }
            }
        }

        let min_row = filled.iter().map(|p| p.row).min().unwrap();
        let max_row = filled.iter().map(|p| p.row).max().unwrap();
        let min_col = filled.iter().map(|p| p.col).min().unwrap();
        let max_col = filled.iter().map(|p| p.col).max().unwrap();
        let grid_height = (max_row - min_row + 1) as usize;
        let grid_width = (max_col - min_col + 1) as usize;

        let mut grid: Vec<bool> = vec![false; grid_width * grid_height];
        for row in 0..grid_height {
            for col in 0..grid_width {
                if filled.contains(&Position {
                    row: row as isize + min_row,
                    col: col as isize + min_col,
                }) {
                    grid[row * grid_width + col] = true;
                }
            }
        }

        // Flood fill the interior of the loop
        let start_row = (-min_row) as usize;
        let start_col = (-min_col) as usize;
        let mut filled_grid = grid.clone();
        let mut queue = Vec::new();
        queue.push((start_row + 1) * grid_width + start_col + 1);
        while let Some(index) = queue.pop() {
            filled_grid[index] = true;

            let row = index / grid_width;
            let col = index % grid_width;

            // Up
            if row > 0 && !grid[index - grid_width] && !filled_grid[index - grid_width] {
                queue.push(index - grid_width);
            }
            // Down
            if row < grid_height - 1
                && !grid[index + grid_width]
                && !filled_grid[index + grid_width]
            {
                queue.push(index + grid_width);
            }
            // Left
            if col > 0 && !grid[index - 1] && !filled_grid[index - 1] {
                queue.push(index - 1);
            }
            // Right
            if col < grid_width - 1 && !grid[index + 1] && !filled_grid[index + 1] {
                queue.push(index + 1);
            }
        }

        filled_grid.iter().filter(|t| **t).count()
    }
}

impl DigStep {
    fn parse(input: &str) -> Self {
        let mut parts = input.split_whitespace();
        let dir = Dir::parse(parts.next().unwrap());
        let steps = parts.next().unwrap().parse::<u8>().unwrap();
        DigStep { dir, steps }
    }
}

impl Dir {
    fn parse(input: &str) -> Self {
        match input {
            "D" => Dir::Down,
            "U" => Dir::Up,
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => panic!("invalid direction {input}"),
        }
    }
}

#[test]
fn test_parse() {
    let input = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)\n";
    let dig = DigPlan::parse(input);
    assert_eq!(dig.steps.len(), 14);
}

#[test]
fn test_dig_size() {
    let input = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)\n";
    let dig = DigPlan::parse(input);
    assert_eq!(dig.size(), 62);
}
