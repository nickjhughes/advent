use num::Integer;
use std::{
    collections::{HashMap, HashSet},
    fmt, fs,
};

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let state = State::parse(&contents);
    let shortest_path_length = find_shortest_path(&state);
    format!("{}", shortest_path_length)
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let state = State::parse(&contents);
    let leg_lengths = find_shortest_back_and_forth_path(&state);
    format!("{}", leg_lengths.0 + leg_lengths.1 + leg_lengths.2)
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input24").expect("Failed to open input file")
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
struct Blizzard {
    position: Point,
    direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    blizzards: Vec<Blizzard>,
    rows: usize,
    cols: usize,
    start: Point,
    goal: Point,
}

impl State {
    fn parse(contents: &str) -> Self {
        let mut blizzards = Vec::new();
        let lines = contents.lines().collect::<Vec<&str>>();
        let rows = lines.len() as usize;
        let cols = lines[0].len() as usize;
        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '^' => blizzards.push(Blizzard {
                        position: Point::new(row, col),
                        direction: Direction::Up,
                    }),
                    'v' => blizzards.push(Blizzard {
                        position: Point::new(row, col),
                        direction: Direction::Down,
                    }),
                    '<' => blizzards.push(Blizzard {
                        position: Point::new(row, col),
                        direction: Direction::Left,
                    }),
                    '>' => blizzards.push(Blizzard {
                        position: Point::new(row, col),
                        direction: Direction::Right,
                    }),
                    '#' | '.' => {}
                    _ => panic!("invalid map character: {}", ch),
                }
            }
        }
        Self {
            blizzards,
            rows,
            cols,
            start: Point::new(0, 1),
            goal: Point::new(rows - 1, cols - 2),
        }
    }

    fn update(&mut self) {
        for blizzard in &mut self.blizzards {
            match blizzard.direction {
                Direction::Up => {
                    if blizzard.position.row == 1 {
                        blizzard.position.row = self.rows - 2;
                    } else {
                        blizzard.position.row -= 1;
                    }
                }
                Direction::Down => {
                    if blizzard.position.row == self.rows - 2 {
                        blizzard.position.row = 1;
                    } else {
                        blizzard.position.row += 1;
                    }
                }
                Direction::Left => {
                    if blizzard.position.col == 1 {
                        blizzard.position.col = self.cols - 2;
                    } else {
                        blizzard.position.col -= 1;
                    }
                }
                Direction::Right => {
                    if blizzard.position.col == self.cols - 2 {
                        blizzard.position.col = 1;
                    } else {
                        blizzard.position.col += 1;
                    }
                }
            }
        }
    }

    fn heuristic(&self, node: &Node3) -> usize {
        ((node.row as i32 - self.goal.row as i32).abs()
            + (node.col as i32 - self.goal.col as i32).abs()) as usize
    }

    fn fmt_with_position(&self, f: &mut fmt::Formatter, position: Option<&Point>) -> fmt::Result {
        write!(f, "#.")?;
        for _ in 0..self.cols - 2 {
            write!(f, "#")?;
        }
        writeln!(f)?;
        for row in 1..self.rows - 1 {
            write!(f, "#")?;
            for col in 1..self.cols - 1 {
                let blizzards = self
                    .blizzards
                    .iter()
                    .filter(|b| b.position == Point::new(row, col))
                    .collect::<Vec<&Blizzard>>();
                if position.is_some()
                    && position.unwrap().row == row
                    && position.unwrap().col == col
                {
                    write!(f, "E")?;
                } else if blizzards.is_empty() {
                    write!(f, ".")?;
                } else if blizzards.len() == 1 {
                    match blizzards[0].direction {
                        Direction::Up => write!(f, "^")?,
                        Direction::Down => write!(f, "v")?,
                        Direction::Left => write!(f, "<")?,
                        Direction::Right => write!(f, ">")?,
                    }
                } else {
                    write!(f, "{}", blizzards.len())?;
                }
            }
            writeln!(f, "#")?;
        }
        for _ in 0..self.cols - 2 {
            write!(f, "#")?;
        }
        write!(f, ".#")?;
        Ok(())
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_with_position(f, None)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node3 {
    state_index: usize,
    row: usize,
    col: usize,
}

impl Node3 {
    fn new(state_index: usize, row: usize, col: usize) -> Self {
        Self {
            state_index,
            row,
            col,
        }
    }
}

fn get_neighbors(node: &Node3, all_states: &[State]) -> Vec<Node3> {
    let next_state_index = if node.state_index == all_states.len() - 1 {
        0
    } else {
        node.state_index + 1
    };
    let next_state = &all_states[next_state_index];
    let blizzard_points = next_state
        .blizzards
        .iter()
        .map(|b| Node3::new(next_state_index, b.position.row, b.position.col))
        .collect::<Vec<Node3>>();

    let mut neighbors = Vec::new();

    // Wait
    let wait = Node3::new(next_state_index, node.row, node.col);
    if !blizzard_points.contains(&wait) {
        neighbors.push(wait);
    }

    // Move up
    if node.row > 1 || (node.row == 1 && node.col == 1) {
        let up = Node3::new(next_state_index, node.row - 1, node.col);
        if !blizzard_points.contains(&up) {
            neighbors.push(up);
        }
    }

    // Move down
    if node.row < next_state.rows - 2
        || (node.row == next_state.rows - 2 && node.col == next_state.cols - 2)
    {
        let down = Node3::new(next_state_index, node.row + 1, node.col);
        if !blizzard_points.contains(&down) {
            neighbors.push(down);
        }
    }

    // Move left
    if node.col > 1 && node.row > 0 && node.row < next_state.rows - 1 {
        let left = Node3::new(next_state_index, node.row, node.col - 1);
        if !blizzard_points.contains(&left) {
            neighbors.push(left);
        }
    }

    // Move right
    if node.col < next_state.cols - 2 && node.row > 0 && node.row < next_state.rows - 1 {
        let right = Node3::new(next_state_index, node.row, node.col + 1);
        if !blizzard_points.contains(&right) {
            neighbors.push(right);
        }
    }

    neighbors
}

fn get_all_states(state: &State) -> Vec<State> {
    let cycle_length = (state.cols - 2).lcm(&(state.rows - 2));
    let mut all_states = Vec::with_capacity(cycle_length as usize);
    let mut state = state.clone();
    for _ in 0..cycle_length {
        all_states.push(state.clone());
        state.update();
    }
    all_states
}

fn find_shortest_path(state: &State) -> usize {
    let all_states = get_all_states(state);

    let start_node = Node3::new(0, state.start.row, state.start.col);

    let mut open_set = HashSet::new();
    open_set.insert(start_node.clone());

    let mut g_score = HashMap::new();
    g_score.insert(start_node.clone(), 0);

    let mut f_score = HashMap::new();
    f_score.insert(start_node.clone(), state.heuristic(&start_node));

    while !open_set.is_empty() {
        let mut min_score = usize::MAX;
        let mut min_score_node = None;
        for node in open_set.iter() {
            if *f_score.get(node).unwrap_or(&usize::MAX) < min_score {
                min_score = f_score[node];
                min_score_node = Some(node);
            }
        }
        if min_score_node.is_none() {
            panic!("all nodes in open_set have usize::MAX f_score");
        }
        let current_node = open_set.take(&min_score_node.unwrap().clone()).unwrap();

        if current_node.row == state.goal.row && current_node.col == state.goal.col {
            return g_score[&current_node];
        }

        let node_neighbors = get_neighbors(&current_node, &all_states);
        for neighbor_node in node_neighbors {
            let tentative_g_score = g_score.get(&current_node).unwrap() + 1;
            if tentative_g_score < *g_score.get(&neighbor_node).unwrap_or(&usize::MAX) {
                g_score.insert(neighbor_node.clone(), tentative_g_score);
                f_score.insert(
                    neighbor_node.clone(),
                    tentative_g_score + state.heuristic(&neighbor_node),
                );
                if !open_set.contains(&neighbor_node) {
                    open_set.insert(neighbor_node.clone());
                }
            }
        }
    }

    panic!("failed to find path");
}

fn find_shortest_back_and_forth_path(state: &State) -> (usize, usize, usize) {
    let first_state = state.clone();
    let first_leg_length = find_shortest_path(&first_state);

    let mut second_state = state.clone();
    for _ in 0..first_leg_length {
        second_state.update();
    }
    second_state.start = state.goal;
    second_state.goal = state.start;
    let second_leg_length = find_shortest_path(&second_state);

    let mut third_state = state.clone();
    for _ in 0..(first_leg_length + second_leg_length) {
        third_state.update();
    }
    let third_leg_length = find_shortest_path(&third_state);

    (first_leg_length, second_leg_length, third_leg_length)
}

#[test]
fn test_parse_state() {
    let contents = "#.#####\n#.....#\n#>....#\n#.....#\n#...v.#\n#.....#\n#####.#\n";
    let state = State::parse(contents);

    assert_eq!(state.start, Point::new(0, 1));
    assert_eq!(state.goal, Point::new(6, 5));

    assert_eq!(state.blizzards.len(), 2);
    assert!(state.blizzards.contains(&Blizzard {
        position: Point::new(2, 1),
        direction: Direction::Right
    }));
    assert!(state.blizzards.contains(&Blizzard {
        position: Point::new(4, 4),
        direction: Direction::Down
    }));
}

#[test]
fn test_state_update() {
    let contents = "#.#####\n#.....#\n#.>...#\n#.....#\n#.....#\n#...v.#\n#####.#\n";
    let mut state = State::parse(contents);
    state.update();

    assert_eq!(state.blizzards.len(), 2);
    assert!(state.blizzards.contains(&Blizzard {
        position: Point::new(2, 3),
        direction: Direction::Right
    }));
    assert!(state.blizzards.contains(&Blizzard {
        position: Point::new(1, 4),
        direction: Direction::Down
    }));
}

#[test]
fn test_get_neighbors() {
    {
        let contents = "#.#####\n#.....#\n#.....#\n#.....#\n#.....#\n#.....#\n#####.#\n";
        let state = State::parse(contents);
        let all_states = get_all_states(&state);
        let node = Node3::new(0, 0, 1);
        let node_neighbors = get_neighbors(&node, &all_states);
        assert_eq!(node_neighbors.len(), 2);
        assert!(node_neighbors.contains(&Node3::new(1, 0, 1)));
        assert!(node_neighbors.contains(&Node3::new(1, 1, 1)));
    }

    {
        let contents = "#.######\n#>>.<^<#\n#.<..<<#\n#>v.><>#\n#<^v^^>#\n######.#\n";
        let state = {
            let mut state = State::parse(contents);
            state.update();
            state
        };
        let all_states = get_all_states(&state);
        let node = Node3::new(0, 1, 1);
        let node_neighbors = get_neighbors(&node, &all_states);
        assert_eq!(node_neighbors.len(), 3);
        assert!(node_neighbors.contains(&Node3::new(1, 0, 1)));
        assert!(node_neighbors.contains(&Node3::new(1, 1, 1)));
        assert!(node_neighbors.contains(&Node3::new(1, 2, 1)));
    }
}

#[test]
fn test_find_shortest_path() {
    {
        let contents = "#.#####\n#.....#\n#.....#\n#.....#\n#.....#\n#.....#\n#####.#\n";
        let state = State::parse(contents);
        let shortest_path_length = find_shortest_path(&state);
        assert_eq!(shortest_path_length, 10);
    }
    {
        let contents = "#.######\n#>>.<^<#\n#.<..<<#\n#>v.><>#\n#<^v^^>#\n######.#\n";
        let state = State::parse(contents);
        let shortest_path_length = find_shortest_path(&state);
        assert_eq!(shortest_path_length, 18);
    }
}

#[test]
fn test_find_shortest_back_and_forth_path() {
    {
        let contents = "#.#####\n#.....#\n#.....#\n#.....#\n#.....#\n#.....#\n#####.#\n";
        let state = State::parse(contents);
        let leg_lengths = find_shortest_back_and_forth_path(&state);
        assert_eq!(leg_lengths.0 + leg_lengths.1 + leg_lengths.2, 30);
    }
    {
        let contents = "#.######\n#>>.<^<#\n#.<..<<#\n#>v.><>#\n#<^v^^>#\n######.#\n";
        let state = State::parse(contents);
        let leg_lengths = find_shortest_back_and_forth_path(&state);
        assert_eq!(leg_lengths.0 + leg_lengths.1 + leg_lengths.2, 54);
        assert_eq!(leg_lengths.0, 18);
        assert_eq!(leg_lengths.1, 23);
        assert_eq!(leg_lengths.2, 13);
    }
}
