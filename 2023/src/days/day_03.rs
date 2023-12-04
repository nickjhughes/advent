use std::fs;

pub fn part1() -> String {
    let input = get_input_file_contents();
    let grid = input_to_grid(&input);
    let part_numbers = get_part_numbers(&grid);
    part_numbers.iter().sum::<u64>().to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let grid = input_to_grid(&input);
    let gear_ratios = get_gear_ratios(&grid);
    gear_ratios.iter().sum::<u64>().to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input03").expect("Failed to open input file")
}

fn input_to_grid(input: &str) -> Vec<&[u8]> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| line.as_bytes())
        .collect::<Vec<&[u8]>>()
}

fn get_number_positions(grid: &[&[u8]]) -> Vec<(usize, usize, usize)> {
    let mut number_positions = Vec::new();
    #[allow(clippy::needless_range_loop)]
    for row in 0..grid.len() {
        let mut start_col = None;
        for col in 0..grid[row].len() {
            if grid[row][col].is_ascii_digit() {
                if start_col.is_none() {
                    start_col = Some(col);
                }
            } else if let Some(start_col) = start_col.take() {
                number_positions.push((row, start_col, col - 1));
            }
        }
        if let Some(start_col) = start_col.take() {
            number_positions.push((row, start_col, grid[row].len() - 1));
        }
    }
    number_positions
}

fn position_to_number(grid: &[&[u8]], row: usize, start_col: usize, end_col: usize) -> u64 {
    std::str::from_utf8(&grid[row][start_col..=end_col])
        .unwrap()
        .parse::<u64>()
        .unwrap()
}

fn get_part_numbers(grid: &[&[u8]]) -> Vec<u64> {
    let number_positions = get_number_positions(grid);

    let mut part_numbers = Vec::new();
    for (row, start_col, end_col) in number_positions {
        if is_part_number(grid, row, start_col, end_col) {
            part_numbers.push(position_to_number(grid, row, start_col, end_col));
        }
    }
    part_numbers
}

fn is_part_number(grid: &[&[u8]], row: usize, start_col: usize, end_col: usize) -> bool {
    let mut adjacent_row_start_col = start_col;
    let mut adjacent_row_end_col = end_col;

    // Left
    if start_col > 0 {
        adjacent_row_start_col -= 1;
        if grid[row][start_col - 1] != b'.' {
            return true;
        }
    }
    // Right
    if end_col < grid[row].len() - 1 {
        adjacent_row_end_col += 1;
        if grid[row][end_col + 1] != b'.' {
            return true;
        }
    }
    // Above
    if row > 0 {
        for col in adjacent_row_start_col..=adjacent_row_end_col {
            if !grid[row - 1][col].is_ascii_digit() && grid[row - 1][col] != b'.' {
                return true;
            }
        }
    }
    // Below
    if row < grid.len() - 1 {
        for col in adjacent_row_start_col..=adjacent_row_end_col {
            if !grid[row + 1][col].is_ascii_digit() && grid[row + 1][col] != b'.' {
                return true;
            }
        }
    }

    false
}

fn get_gear_ratios(grid: &[&[u8]]) -> Vec<u64> {
    get_gears(grid).iter().map(|g| g.0 * g.1).collect()
}

fn get_gears(grid: &[&[u8]]) -> Vec<(u64, u64)> {
    let number_positions = get_number_positions(grid);
    let mut gears = Vec::new();
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == b'*' {
                if let Some((number1, number2)) = is_gear(grid, &number_positions, row, col) {
                    gears.push((number1, number2));
                }
            }
        }
    }
    gears
}

fn is_gear(
    grid: &[&[u8]],
    number_positions: &[(usize, usize, usize)],
    row: usize,
    col: usize,
) -> Option<(u64, u64)> {
    let mut numbers = Vec::new();
    let mut adjacent_row_start_col = col;
    let mut adjacent_row_end_col = col;
    // Left
    if col > 0 {
        adjacent_row_start_col -= 1;
        if let Some((number_row, number_start_col, number_end_col)) =
            number_positions
                .iter()
                .find(|(number_row, _, number_end_col)| {
                    *number_row == row && *number_end_col == col - 1
                })
        {
            numbers.push(position_to_number(
                grid,
                *number_row,
                *number_start_col,
                *number_end_col,
            ));
        };
    }
    // Right
    if col < grid[row].len() - 1 {
        adjacent_row_end_col += 1;
        if let Some((number_row, number_start_col, number_end_col)) =
            number_positions
                .iter()
                .find(|(number_row, number_start_col, _)| {
                    *number_row == row && *number_start_col == col + 1
                })
        {
            numbers.push(position_to_number(
                grid,
                *number_row,
                *number_start_col,
                *number_end_col,
            ));
        }
    }
    // Above
    if row > 0 {
        let mut col = adjacent_row_start_col;
        while col <= adjacent_row_end_col {
            if let Some((number_row, number_start_col, number_end_col)) = number_positions
                .iter()
                .find(|(number_row, number_start_col, number_end_col)| {
                    *number_row == row - 1 && (*number_start_col..=*number_end_col).contains(&col)
                })
            {
                numbers.push(position_to_number(
                    grid,
                    *number_row,
                    *number_start_col,
                    *number_end_col,
                ));
                col = number_end_col + 1;
            } else {
                col += 1;
            }
        }
    }
    // Below
    if row < grid.len() - 1 {
        let mut col = adjacent_row_start_col;
        while col <= adjacent_row_end_col {
            if let Some((number_row, number_start_col, number_end_col)) = number_positions
                .iter()
                .find(|(number_row, number_start_col, number_end_col)| {
                    *number_row == row + 1 && (*number_start_col..=*number_end_col).contains(&col)
                })
            {
                numbers.push(position_to_number(
                    grid,
                    *number_row,
                    *number_start_col,
                    *number_end_col,
                ));
                col = number_end_col + 1;
            } else {
                col += 1;
            }
        }
    }

    if numbers.len() == 2 {
        Some((numbers[0], numbers[1]))
    } else {
        None
    }
}

#[test]
fn test_part_numbers_sum() {
    let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..\n";
    let grid = input_to_grid(input);
    let part_numbers = get_part_numbers(&grid);
    assert_eq!(part_numbers.iter().sum::<u64>(), 4361);
}

#[test]
fn test_get_gears() {
    let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..\n";
    let grid = input_to_grid(input);
    let gears = get_gears(&grid);
    assert_eq!(gears.len(), 2);
    assert_eq!(gears, vec![(467, 35), (755, 598)]);
}

#[test]
fn test_gear_ratios_sum() {
    let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..\n";
    let grid = input_to_grid(input);
    let gear_ratios = get_gear_ratios(&grid);
    assert_eq!(gear_ratios.iter().sum::<u64>(), 467835);
}
