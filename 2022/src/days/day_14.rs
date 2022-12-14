use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};
use std::{collections::HashSet, fs};

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let paths = parse_paths(&contents);
    let sand_at_rest_count = simulate_sand(&paths, false);
    format!("{}", sand_at_rest_count)
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let paths = parse_paths(&contents);
    let sand_at_rest_count = simulate_sand(&paths, true);
    format!("{}", sand_at_rest_count)
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input14").expect("Failed to open input file")
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    #[allow(dead_code)]
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(digit1::<&str, _>, tag(","), digit1),
            |(x, y)| Self {
                x: x.parse::<u32>()
                    .expect("Failed to parse point x coordinate"),
                y: y.parse::<u32>()
                    .expect("Failed to parse point y coordinate"),
            },
        )(input)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Path {
    points: Vec<Point>,
}

impl Path {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list1(tag(" -> "), Point::parse), |points| Self {
            points,
        })(input)
    }
}

fn parse_paths(contents: &str) -> Vec<Path> {
    let (rest, paths) = terminated(separated_list1(newline, Path::parse), opt(newline))(contents)
        .expect("Failed to parse paths");
    assert!(rest.is_empty());
    paths
}

fn calc_rock_points(paths: &[Path]) -> HashSet<Point> {
    let mut rock_points = HashSet::new();
    for path in paths {
        for (p0, p1) in path.points.iter().tuple_windows() {
            let x_range = if p0.x <= p1.x {
                p0.x..p1.x + 1
            } else {
                p1.x..p0.x + 1
            };
            let y_range = if p0.y <= p1.y {
                p0.y..p1.y + 1
            } else {
                p1.y..p0.y + 1
            };
            for x in x_range {
                for y in y_range.clone() {
                    rock_points.insert(Point::new(x, y));
                }
            }
        }
    }
    rock_points
}

fn simulate_sand(paths: &[Path], has_floor: bool) -> usize {
    let sand_source: Point = Point::new(500, 0);

    let rock_points = calc_rock_points(paths);
    let max_rock_y = rock_points.iter().map(|p| p.y).max().unwrap();
    let floor_y = max_rock_y + 2;
    let is_floor = |point: &Point| {
        if !has_floor {
            false
        } else {
            point.y == floor_y
        }
    };

    let mut stationary_sand_points: HashSet<Point> = HashSet::new();

    let mut all_sand_is_rested = false;
    while !all_sand_is_rested {
        let mut sand = sand_source.clone();
        loop {
            let move_options = vec![
                Point::new(sand.x, sand.y + 1),     // One step down
                Point::new(sand.x - 1, sand.y + 1), // One step down and to the left
                Point::new(sand.x + 1, sand.y + 1), // One step down and to the right
            ];
            let mut at_rest = true;
            for option in &move_options {
                if !rock_points.contains(option)
                    && !stationary_sand_points.contains(option)
                    && !is_floor(option)
                {
                    at_rest = false;
                    sand.x = option.x;
                    sand.y = option.y;
                    break;
                }
            }
            if at_rest {
                if sand == sand_source {
                    all_sand_is_rested = true;
                }
                stationary_sand_points.insert(sand);
                break;
            }
            if !has_floor && sand.y > max_rock_y {
                // Into the abyss
                all_sand_is_rested = true;
                break;
            }
        }
    }
    stationary_sand_points.len()
}

#[test]
fn test_parse_paths() {
    let contents = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9\n";
    let paths = parse_paths(contents);
    assert_eq!(paths.len(), 2);

    assert_eq!(
        paths[0],
        Path {
            points: vec![Point::new(498, 4), Point::new(498, 6), Point::new(496, 6),]
        }
    );

    assert_eq!(
        paths[1],
        Path {
            points: vec![
                Point::new(503, 4),
                Point::new(502, 4),
                Point::new(502, 9),
                Point::new(494, 9),
            ]
        }
    );
}

#[test]
fn test_parse_path() {
    let input = "498,4 -> 498,6 -> 496,6";
    let result = Path::parse(input);
    assert!(result.is_ok());
    let (rest, path) = result.unwrap();
    assert!(rest.is_empty());
    assert_eq!(
        path,
        Path {
            points: vec![Point::new(498, 4), Point::new(498, 6), Point::new(496, 6),]
        }
    );
}

#[test]
fn test_calc_rock_points() {
    let contents = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9\n";
    let paths = parse_paths(contents);
    let rock_points = calc_rock_points(&paths);
    assert_eq!(rock_points.len(), 20);

    let mut expected_result = HashSet::new();
    // Path 1
    expected_result.insert(Point::new(498, 4));
    expected_result.insert(Point::new(498, 5));
    expected_result.insert(Point::new(498, 6));
    expected_result.insert(Point::new(497, 6));
    expected_result.insert(Point::new(496, 6));
    // Path 2
    expected_result.insert(Point::new(503, 4));
    expected_result.insert(Point::new(502, 4));
    expected_result.insert(Point::new(502, 5));
    expected_result.insert(Point::new(502, 6));
    expected_result.insert(Point::new(502, 7));
    expected_result.insert(Point::new(502, 8));
    expected_result.insert(Point::new(502, 9));
    expected_result.insert(Point::new(501, 9));
    expected_result.insert(Point::new(500, 9));
    expected_result.insert(Point::new(499, 9));
    expected_result.insert(Point::new(498, 9));
    expected_result.insert(Point::new(497, 9));
    expected_result.insert(Point::new(496, 9));
    expected_result.insert(Point::new(495, 9));
    expected_result.insert(Point::new(494, 9));

    assert_eq!(rock_points, expected_result);
}

#[test]
fn test_simulate_sand_no_floor() {
    let contents = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9\n";
    let paths = parse_paths(contents);
    let sand_at_rest_count = simulate_sand(&paths, false);
    assert_eq!(sand_at_rest_count, 24);
}

#[test]
fn test_simulate_sand_floor() {
    let contents = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9\n";
    let paths = parse_paths(contents);
    let sand_at_rest_count = simulate_sand(&paths, true);
    assert_eq!(sand_at_rest_count, 93);
}
