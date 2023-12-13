use std::fs;

pub fn part1() -> String {
    let input = get_input_file_contents();
    let patterns = parse_patterns(&input);
    answer(&patterns).to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let mut patterns = parse_patterns(&input);
    answer_smudge(&mut patterns).to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input13").expect("Failed to open input file")
}

fn parse_patterns(input: &str) -> Vec<Pattern> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut pattern_start = 0;
    let mut patterns = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            patterns.push(Pattern::parse(&lines[pattern_start..i].join("\n")));
            pattern_start = i + 1;
        }
    }
    patterns.push(Pattern::parse(&lines[pattern_start..].join("\n")));
    patterns
}

fn answer(patterns: &[Pattern]) -> usize {
    patterns
        .iter()
        .map(
            |p| match p.find_symmetries().first().expect("no symmetry found") {
                Symmetry::Horizontal(rows) => 100 * rows,
                Symmetry::Vertical(cols) => *cols,
            },
        )
        .sum::<usize>()
}

fn answer_smudge(patterns: &mut [Pattern]) -> usize {
    patterns
        .iter_mut()
        .map(|p| match p.find_symmetry_smudge() {
            Symmetry::Horizontal(rows) => 100 * rows,
            Symmetry::Vertical(cols) => cols,
        })
        .sum::<usize>()
}

#[derive(Debug, PartialEq)]
struct Pattern {
    tiles: Vec<Tile>,
    width: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Ash,
    Rock,
}

#[derive(Debug, PartialEq)]
enum Symmetry {
    Horizontal(usize),
    Vertical(usize),
}

impl Pattern {
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
        Pattern {
            tiles,
            width: width.unwrap(),
        }
    }

    fn height(&self) -> usize {
        self.tiles.len() / self.width
    }

    fn find_symmetries(&self) -> Vec<Symmetry> {
        let mut symmetries = Vec::new();

        // Horizontal
        for rows_above in 1..self.height() {
            let mut mismatch = false;
            for (row_above, row_below) in (rows_above..(rows_above + rows_above).min(self.height()))
                .zip((0..rows_above).rev())
            {
                for col in 0..self.width {
                    if self.tiles[row_above * self.width + col]
                        != self.tiles[row_below * self.width + col]
                    {
                        mismatch = true;
                        break;
                    }
                }
                if mismatch {
                    break;
                }
            }
            if !mismatch {
                symmetries.push(Symmetry::Horizontal(rows_above));
            }
        }

        // Vertical
        for cols_left in 1..self.width {
            let mut mismatch = false;
            for (col_left, col_right) in
                (cols_left..(cols_left + cols_left).min(self.width)).zip((0..cols_left).rev())
            {
                for row in 0..self.height() {
                    if self.tiles[row * self.width + col_left]
                        != self.tiles[row * self.width + col_right]
                    {
                        mismatch = true;
                        break;
                    }
                }
                if mismatch {
                    break;
                }
            }
            if !mismatch {
                symmetries.push(Symmetry::Vertical(cols_left));
            }
        }

        symmetries
    }

    fn find_symmetry_smudge(&mut self) -> Symmetry {
        let original_symmetry = self.find_symmetries().remove(0);
        for smudge_idx in 0..self.tiles.len() {
            *self.tiles.get_mut(smudge_idx).unwrap() = self.tiles[smudge_idx].swap();
            let symmetries = self.find_symmetries();
            *self.tiles.get_mut(smudge_idx).unwrap() = self.tiles[smudge_idx].swap();
            for symmetry in symmetries {
                if symmetry != original_symmetry {
                    return symmetry;
                }
            }
        }

        panic!("no smudge symmetry found in pattern:\n{}", self)
    }
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height() {
            for col in 0..self.width {
                write!(f, "{}", self.tiles[row * self.width + col])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Tile {
    fn parse(ch: char) -> Self {
        match ch {
            '.' => Tile::Ash,
            '#' => Tile::Rock,
            _ => panic!("invalid tile {ch}"),
        }
    }

    fn swap(&self) -> Tile {
        match self {
            Tile::Ash => Tile::Rock,
            Tile::Rock => Tile::Ash,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Ash => write!(f, "."),
            Tile::Rock => write!(f, "#"),
        }
    }
}

#[test]
fn test_parse() {
    let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\n\n#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#\n";
    let patterns = parse_patterns(input);

    assert_eq!(patterns.len(), 2);
}

#[test]
fn test_find_symmetry() {
    let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\n\n#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#\n";
    let patterns = parse_patterns(input);

    assert_eq!(patterns[0].find_symmetries(), vec![Symmetry::Vertical(5)]);
    assert_eq!(patterns[1].find_symmetries(), vec![Symmetry::Horizontal(4)]);
}

#[test]
fn test_answer() {
    let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\n\n#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#\n";
    let patterns = parse_patterns(input);
    assert_eq!(answer(&patterns), 405);
}

#[test]
fn test_patterns() {
    let input = "..##..##....#\n.##....##..##\n#.##..##.##.#\n....##.......\n##.#....####.\n..##..##....#\n.#.#..#.#..#.\n";
    let patterns = parse_patterns(input);
    assert_eq!(patterns[0].find_symmetries(), vec![Symmetry::Vertical(10)]);
}

#[test]
fn test_find_symmetry_smudges() {
    let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\n\n#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#\n";
    let mut patterns = parse_patterns(input);

    assert_eq!(patterns[0].find_symmetry_smudge(), Symmetry::Horizontal(3));
    assert_eq!(patterns[1].find_symmetry_smudge(), Symmetry::Horizontal(1));
}

#[test]
fn test_answer_smudge() {
    let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\n\n#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#\n";
    let mut patterns = parse_patterns(input);
    assert_eq!(answer_smudge(&mut patterns), 400);
}
