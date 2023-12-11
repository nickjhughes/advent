use std::{collections::HashMap, fs};

pub fn part1() -> String {
    let input = get_input_file_contents();
    let space = Space::parse(&input);
    space.pairwise_distance_sum(2).to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let space = Space::parse(&input);
    space.pairwise_distance_sum(1000000).to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input11").expect("Failed to open input file")
}

struct Space {
    galaxies: Vec<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl Space {
    fn parse(input: &str) -> Self {
        let mut galaxies = Vec::new();
        let mut empty_rows = Vec::new();
        let mut galaxies_in_cols: HashMap<usize, usize> = HashMap::new();
        for (row, line) in input.lines().enumerate() {
            let mut galaxies_in_row = 0;
            for (col, ch) in line.chars().enumerate() {
                galaxies_in_cols.entry(col).or_insert(0);
                if ch == '#' {
                    galaxies.push((row, col));
                    galaxies_in_row += 1;
                    *galaxies_in_cols.entry(col).or_default() += 1;
                }
            }
            if galaxies_in_row == 0 {
                empty_rows.push(row);
            }
        }
        let empty_cols = galaxies_in_cols
            .iter()
            .filter(|(_, count)| **count == 0)
            .map(|(col, _)| *col)
            .collect();
        Space {
            galaxies,
            empty_rows,
            empty_cols,
        }
    }

    fn pairwise_distance_sum(&self, expansion: usize) -> usize {
        let mut distance_sum = 0;
        for (i, (row1, col1)) in self.galaxies.iter().enumerate() {
            for (row2, col2) in self.galaxies.iter().skip(i + 1) {
                let mut distance = 0;
                // Traverse columns
                let col_range = if col2 >= col1 {
                    *col1..*col2
                } else {
                    *col2..*col1
                };
                for col in col_range {
                    distance += if self.empty_cols.contains(&col) {
                        expansion
                    } else {
                        1
                    };
                }
                // Traverse rows
                let row_range = if row2 >= row1 {
                    *row1..*row2
                } else {
                    *row2..*row1
                };
                for row in row_range {
                    distance += if self.empty_rows.contains(&row) {
                        expansion
                    } else {
                        1
                    };
                }
                distance_sum += distance;
            }
        }
        distance_sum
    }
}

#[test]
fn test_parse() {
    let input = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....\n";
    let space = Space::parse(input);
    assert_eq!(
        space.galaxies,
        vec![
            (0, 3),
            (1, 7),
            (2, 0),
            (4, 6),
            (5, 1),
            (6, 9),
            (8, 7),
            (9, 0),
            (9, 4)
        ]
    );
    let empty_rows = {
        let mut a = space.empty_rows;
        a.sort();
        a
    };
    assert_eq!(empty_rows, vec![3, 7]);
    let empty_cols = {
        let mut a = space.empty_cols;
        a.sort();
        a
    };
    assert_eq!(empty_cols, vec![2, 5, 8]);
}

#[test]
fn test_pairwise_distances_sum() {
    let input = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....\n";
    let space = Space::parse(input);
    assert_eq!(space.pairwise_distance_sum(2), 374);
}
