use hashbrown::HashSet;
use std::fs;

pub fn part1() -> String {
    let input = get_input_file_contents();
    let dig = DigPlan::parse(&input);
    dig.size_flood_fill().to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let dig = DigPlan::parse_hex(&input);
    dig.size_trapezoid_area().to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input18").expect("Failed to open input file")
}

#[derive(Debug)]
struct DigPlan {
    steps: Vec<DigStep>,
}

#[derive(Debug, PartialEq)]
struct DigStep {
    dir: Dir,
    steps: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    row: isize,
    col: isize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Turn {
    Left,
    Right,
}

impl DigPlan {
    fn parse(input: &str) -> Self {
        DigPlan {
            steps: input.lines().map(DigStep::parse).collect(),
        }
    }

    fn parse_hex(input: &str) -> Self {
        DigPlan {
            steps: input.lines().map(DigStep::parse_hex).collect(),
        }
    }

    fn size_trapezoid_area(&self) -> u64 {
        let mut points = Vec::new();
        let mut current_point = (0.0, 0.0);
        points.push(current_point);

        let mut last_turn = Turn::Right;

        for (step, next_step) in self.steps.iter().zip(
            self.steps
                .iter()
                .skip(1)
                .chain(std::iter::once(self.steps.first().unwrap())),
        ) {
            let next_turn = next_step.dir.turn(step.dir);
            let steps_mod = match (last_turn, next_turn) {
                (Turn::Left, Turn::Left) => -1.0,
                (Turn::Left, Turn::Right) => 0.0,
                (Turn::Right, Turn::Left) => 0.0,
                (Turn::Right, Turn::Right) => 1.0,
            };
            last_turn = next_turn;

            match step.dir {
                Dir::Up => {
                    current_point.1 -= step.steps as f64 + steps_mod;
                    points.push(current_point);
                }
                Dir::Down => {
                    current_point.1 += step.steps as f64 + steps_mod;
                    points.push(current_point);
                }
                Dir::Left => {
                    current_point.0 -= step.steps as f64 + steps_mod;
                    points.push(current_point);
                }
                Dir::Right => {
                    current_point.0 += step.steps as f64 + steps_mod;
                    points.push(current_point);
                }
            }
        }

        let mut area = 0.0;
        for (p0, p1) in points.iter().zip(points.iter().skip(1)) {
            let trapezoid_area = (p0.1 + p1.1) * (p0.0 - p1.0);
            area += trapezoid_area;
        }
        area /= 2.0;
        area as u64
    }

    fn size_flood_fill(&self) -> usize {
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
        let steps = parts.next().unwrap().parse::<u64>().unwrap();
        DigStep { dir, steps }
    }

    fn parse_hex(input: &str) -> Self {
        let (_, hex) = input.trim_end_matches(')').split_once('#').unwrap();
        let steps = u64::from_str_radix(&hex[0..5], 16).unwrap();
        let dir = Dir::parse_hex(hex.chars().last().unwrap());
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

    fn parse_hex(ch: char) -> Self {
        match ch {
            '0' => Dir::Right,
            '1' => Dir::Down,
            '2' => Dir::Left,
            '3' => Dir::Up,
            _ => panic!("invalid direction {ch}"),
        }
    }

    fn turn(&self, last_dir: Dir) -> Turn {
        match last_dir {
            Dir::Up => match self {
                Dir::Up => unreachable!(),
                Dir::Down => unreachable!(),
                Dir::Left => Turn::Left,
                Dir::Right => Turn::Right,
            },
            Dir::Down => match self {
                Dir::Up => unreachable!(),
                Dir::Down => unreachable!(),
                Dir::Left => Turn::Right,
                Dir::Right => Turn::Left,
            },
            Dir::Left => match self {
                Dir::Up => Turn::Right,
                Dir::Down => Turn::Left,
                Dir::Left => unreachable!(),
                Dir::Right => unreachable!(),
            },
            Dir::Right => match self {
                Dir::Up => Turn::Left,
                Dir::Down => Turn::Right,
                Dir::Left => unreachable!(),
                Dir::Right => unreachable!(),
            },
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
fn test_parse_hex() {
    let input = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)\n";
    let dig = DigPlan::parse_hex(input);
    #[rustfmt::skip]
    assert_eq!(dig.steps, vec![
        DigStep{dir: Dir::Right, steps: 461937},
        DigStep{dir: Dir::Down, steps: 56407},
        DigStep{dir: Dir::Right, steps: 356671},
        DigStep{dir: Dir::Down, steps: 863240},
        DigStep{dir: Dir::Right, steps: 367720},
        DigStep{dir: Dir::Down, steps: 266681},
        DigStep{dir: Dir::Left, steps: 577262},
        DigStep{dir: Dir::Up, steps: 829975},
        DigStep{dir: Dir::Left, steps: 112010},
        DigStep{dir: Dir::Down, steps: 829975},
        DigStep{dir: Dir::Left, steps: 491645},
        DigStep{dir: Dir::Up, steps: 686074},
        DigStep{dir: Dir::Left, steps: 5411},
        DigStep{dir: Dir::Up, steps: 500254},
    ]);
}

#[test]
fn test_size_flood_fill() {
    let input = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)\n";
    let dig = DigPlan::parse(input);
    assert_eq!(dig.size_flood_fill(), 62);
}

#[test]
fn test_size_trapezoid_area() {
    let input = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)\n";

    {
        let dig = DigPlan::parse(input);
        assert_eq!(dig.size_trapezoid_area(), 62);
    }

    {
        let dig = DigPlan::parse_hex(input);
        assert_eq!(dig.size_trapezoid_area(), 952408144115);
    }
}
