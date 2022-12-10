use ndarray::{prelude::*, Array, Ix2};
use std::fs;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let tree_grid = parse_tree_grid(&contents);
    let visible_trees = count_visible_trees(&tree_grid);
    format!("{}", visible_trees)
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let tree_grid = parse_tree_grid(&contents);
    let scenic_scores = tree_scenic_scores(&tree_grid);
    let highest_scenic_score = scenic_scores.iter().max().unwrap();
    format!("{}", highest_scenic_score)
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input08").expect("Failed to open input file")
}

fn parse_tree_grid(contents: &str) -> Array<u8, Ix2> {
    let lines = contents.split('\n').collect::<Vec<&str>>();
    assert!(!lines.is_empty());
    assert!(!lines[0].is_empty());
    let rows = lines[0].len();
    let cols = lines.iter().filter(|l| !l.is_empty()).count();
    let mut tree_grid = Array::<u8, Ix2>::zeros((rows, cols).f());
    let mut row = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        for (col, ch) in line.chars().enumerate() {
            let height = ch
                .to_string()
                .parse::<u8>()
                .expect("Failed to parse tree height");
            tree_grid[[row, col]] = height;
        }
        row += 1;
    }
    tree_grid
}

fn is_tree_visible_from_direction(
    tree_grid: &Array<u8, Ix2>,
    row: usize,
    col: usize,
    direction: Direction,
) -> bool {
    let grid_shape = tree_grid.shape();
    let height = tree_grid[[row, col]];
    match direction {
        Direction::Left => {
            if col == 0 {
                true
            } else {
                let mut dc = 1;
                let mut is_visible = true;
                loop {
                    if tree_grid[[row, col - dc]] >= height {
                        is_visible = false;
                        break;
                    }
                    dc += 1;
                    if dc > col {
                        break;
                    }
                }
                is_visible
            }
        }
        Direction::Right => {
            if col == grid_shape[1] - 1 {
                true
            } else {
                let mut dc = 1;
                let mut is_visible = true;
                while col + dc < grid_shape[1] {
                    if tree_grid[[row, col + dc]] >= height {
                        is_visible = false;
                        break;
                    }
                    dc += 1;
                }
                is_visible
            }
        }
        Direction::Up => {
            if row == 0 {
                true
            } else {
                let mut dr = 1;
                let mut is_visible = true;
                while dr <= row {
                    if tree_grid[[row - dr, col]] >= height {
                        is_visible = false;
                        break;
                    }
                    dr += 1;
                }
                is_visible
            }
        }
        Direction::Down => {
            if row == grid_shape[0] - 1 {
                true
            } else {
                let mut dr = 1;
                let mut is_visible = true;
                while row + dr < grid_shape[0] {
                    if tree_grid[[row + dr, col]] >= height {
                        is_visible = false;
                        break;
                    }
                    dr += 1;
                }
                is_visible
            }
        }
    }
}

fn is_tree_visible(tree_grid: &Array<u8, Ix2>, row: usize, col: usize) -> bool {
    let grid_shape = tree_grid.shape();
    if col == 0 || row == 0 || col == grid_shape[1] - 1 || row == grid_shape[0] - 1 {
        return true;
    }
    if is_tree_visible_from_direction(tree_grid, row, col, Direction::Left) {
        return true;
    }
    if is_tree_visible_from_direction(tree_grid, row, col, Direction::Right) {
        return true;
    }
    if is_tree_visible_from_direction(tree_grid, row, col, Direction::Up) {
        return true;
    }
    if is_tree_visible_from_direction(tree_grid, row, col, Direction::Down) {
        return true;
    }
    false
}

fn count_visible_trees(tree_grid: &Array<u8, Ix2>) -> usize {
    let mut visible_tree_count = 0;
    let grid_shape = tree_grid.shape();
    for row in 0..grid_shape[0] {
        for col in 0..grid_shape[1] {
            if is_tree_visible(tree_grid, row, col) {
                visible_tree_count += 1;
            }
        }
    }
    visible_tree_count
}

fn tree_viewing_distance(
    tree_grid: &Array<u8, Ix2>,
    row: usize,
    col: usize,
    direction: Direction,
) -> usize {
    let grid_shape = tree_grid.shape();
    let height = tree_grid[[row, col]];
    match direction {
        Direction::Left => {
            if col == 0 {
                0
            } else {
                let mut dc = 1;
                let mut viewing_distance = 0;
                loop {
                    viewing_distance += 1;
                    if tree_grid[[row, col - dc]] >= height {
                        break;
                    }
                    dc += 1;
                    if dc > col {
                        break;
                    }
                }
                viewing_distance
            }
        }
        Direction::Right => {
            if col == grid_shape[1] - 1 {
                0
            } else {
                let mut dc = 1;
                let mut viewing_distance = 0;
                while col + dc < grid_shape[1] {
                    viewing_distance += 1;
                    if tree_grid[[row, col + dc]] >= height {
                        break;
                    }
                    dc += 1;
                }
                viewing_distance
            }
        }
        Direction::Up => {
            if row == 0 {
                0
            } else {
                let mut dr = 1;
                let mut viewing_distance = 0;
                while dr <= row {
                    viewing_distance += 1;
                    if tree_grid[[row - dr, col]] >= height {
                        break;
                    }
                    dr += 1;
                }
                viewing_distance
            }
        }
        Direction::Down => {
            if row == grid_shape[0] - 1 {
                0
            } else {
                // Look down
                let mut dr = 1;
                let mut viewing_distance = 0;
                while row + dr < grid_shape[0] {
                    viewing_distance += 1;
                    if tree_grid[[row + dr, col]] >= height {
                        break;
                    }
                    dr += 1;
                }
                viewing_distance
            }
        }
    }
}

fn tree_scenic_score(tree_grid: &Array<u8, Ix2>, row: usize, col: usize) -> usize {
    let grid_shape = tree_grid.shape();
    if col == 0 || row == 0 || col == grid_shape[1] - 1 || row == grid_shape[0] - 1 {
        return 0;
    }
    let distance_left = tree_viewing_distance(tree_grid, row, col, Direction::Left);
    let distance_right = tree_viewing_distance(tree_grid, row, col, Direction::Right);
    let distance_up = tree_viewing_distance(tree_grid, row, col, Direction::Up);
    let distance_down = tree_viewing_distance(tree_grid, row, col, Direction::Down);
    distance_left * distance_right * distance_up * distance_down
}

fn tree_scenic_scores(tree_grid: &Array<u8, Ix2>) -> Vec<usize> {
    let mut scores = Vec::new();
    let grid_shape = tree_grid.shape();
    for row in 0..grid_shape[0] {
        for col in 0..grid_shape[1] {
            scores.push(tree_scenic_score(tree_grid, row, col));
        }
    }
    scores
}

#[test]
fn test_parse() {
    let contents = "30373\n25512\n65332\n33549\n35390\n";
    let tree_grid = parse_tree_grid(&contents);
    assert_eq!(
        tree_grid,
        array![
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0]
        ]
    );
}

#[test]
fn test_is_tree_visible() {
    let contents = "30373\n25512\n65332\n33549\n35390\n";
    let tree_grid = parse_tree_grid(&contents);

    // assert_eq!(is_tree_visible(&tree_grid, 0, 0), true);
    // assert_eq!(is_tree_visible(&tree_grid, 3, 0), true);
    // assert_eq!(is_tree_visible(&tree_grid, 0, 3), true);
    // assert_eq!(is_tree_visible(&tree_grid, 4, 4), true);

    assert_eq!(is_tree_visible(&tree_grid, 1, 1), true);
    // assert_eq!(is_tree_visible(&tree_grid, 3, 2), true);

    // assert_eq!(is_tree_visible(&tree_grid, 1, 3), false);
    // assert_eq!(is_tree_visible(&tree_grid, 2, 2), false);
}

#[test]
fn test_count_visible_trees() {
    let contents = "30373\n25512\n65332\n33549\n35390\n";
    let tree_grid = parse_tree_grid(&contents);
    let visible_trees = count_visible_trees(&tree_grid);
    assert_eq!(visible_trees, 21);
}

#[test]
fn test_tree_viewing_distance() {
    let contents = "30373\n25512\n65332\n33549\n35390\n";
    let tree_grid = parse_tree_grid(&contents);

    assert_eq!(tree_viewing_distance(&tree_grid, 1, 2, Direction::Up), 1);
    assert_eq!(tree_viewing_distance(&tree_grid, 1, 2, Direction::Left), 1);
    assert_eq!(tree_viewing_distance(&tree_grid, 1, 2, Direction::Right), 2);
    assert_eq!(tree_viewing_distance(&tree_grid, 1, 2, Direction::Down), 2);

    assert_eq!(tree_viewing_distance(&tree_grid, 3, 2, Direction::Up), 2);
    assert_eq!(tree_viewing_distance(&tree_grid, 3, 2, Direction::Left), 2);
    assert_eq!(tree_viewing_distance(&tree_grid, 3, 2, Direction::Right), 2);
    assert_eq!(tree_viewing_distance(&tree_grid, 3, 2, Direction::Down), 1);
}

#[test]
fn test_tree_scenic_score() {
    let contents = "30373\n25512\n65332\n33549\n35390\n";
    let tree_grid = parse_tree_grid(&contents);

    assert_eq!(tree_scenic_score(&tree_grid, 1, 2), 4);
    assert_eq!(tree_scenic_score(&tree_grid, 3, 2), 8);
}
