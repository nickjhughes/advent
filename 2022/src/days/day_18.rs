use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};
use std::{
    collections::{HashSet, VecDeque},
    fs,
};

lazy_static! {
    static ref NEIGHBORS: Vec<(i32, i32, i32)> = vec![
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
}

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let cubes = parse_cubes(&contents);
    let surface_area = calc_surface_area(&cubes);
    format!("{}", surface_area)
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let cubes = parse_cubes(&contents);
    let exterior_surface_area = calc_exterior_surface_area(&cubes);
    format!("{}", exterior_surface_area)
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input18").expect("Failed to open input file")
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Cube(Point);

impl Cube {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list1(tag(","), digit1), |values: Vec<&str>| {
            assert_eq!(values.len(), 3);
            let x = values[0]
                .parse::<i32>()
                .expect("Failed to parse x coordinate");
            let y = values[1]
                .parse::<i32>()
                .expect("Failed to parse y coordinate");
            let z = values[2]
                .parse::<i32>()
                .expect("Failed to parse z coordinate");
            Self(Point::new(x, y, z))
        })(input)
    }
}

fn parse_cubes(contents: &str) -> Vec<Cube> {
    let (rest, cubes) = terminated(separated_list1(newline, Cube::parse), opt(newline))(contents)
        .expect("Failed to parse cubes");
    assert!(rest.is_empty());
    cubes
}

fn calc_surface_area(cubes: &[Cube]) -> u32 {
    let total_surfaces = cubes.len() as u32 * 6;

    let mut adjacent_surfaces = 0;
    for i in 0..cubes.len() {
        for j in (i + 1)..cubes.len() {
            let cube1 = &cubes[i];
            let cube2 = &cubes[j];

            if cube1.0.x == cube2.0.x {
                if cube1.0.y == cube2.0.y {
                    if (cube1.0.z - cube2.0.z).abs() == 1 {
                        adjacent_surfaces += 2;
                    }
                } else if cube1.0.z == cube2.0.z {
                    if (cube1.0.y - cube2.0.y).abs() == 1 {
                        adjacent_surfaces += 2;
                    }
                }
            } else if cube1.0.y == cube2.0.y {
                if cube1.0.x == cube2.0.x {
                    if (cube1.0.z - cube2.0.z).abs() == 1 {
                        adjacent_surfaces += 2;
                    }
                } else if cube1.0.z == cube2.0.z {
                    if (cube1.0.x - cube2.0.x).abs() == 1 {
                        adjacent_surfaces += 2;
                    }
                }
            } else if cube1.0.z == cube2.0.z {
                if cube1.0.x == cube2.0.x {
                    if (cube1.0.y - cube2.0.y).abs() == 1 {
                        adjacent_surfaces += 2;
                    }
                } else if cube1.0.y == cube2.0.y {
                    if (cube1.0.x - cube2.0.x).abs() == 1 {
                        adjacent_surfaces += 2;
                    }
                }
            }
        }
    }

    total_surfaces - adjacent_surfaces
}

fn is_exterior(start: &Cube, cubes_set: &HashSet<Cube>) -> bool {
    let min_coords = Point::new(
        cubes_set.iter().map(|cube| cube.0.x).min().unwrap() - 1,
        cubes_set.iter().map(|cube| cube.0.y).min().unwrap() - 1,
        cubes_set.iter().map(|cube| cube.0.z).min().unwrap() - 1,
    );
    let max_coords = Point::new(
        cubes_set.iter().map(|cube| cube.0.x).max().unwrap() + 1,
        cubes_set.iter().map(|cube| cube.0.y).max().unwrap() + 1,
        cubes_set.iter().map(|cube| cube.0.z).max().unwrap() + 1,
    );

    let exterior_cube = Cube(min_coords.clone());

    let mut visited_cubes = HashSet::new();
    visited_cubes.insert(start.clone());

    let mut queue = VecDeque::new();
    queue.push_back(start.clone());

    while !queue.is_empty() {
        let current_cube = queue.pop_front().unwrap();
        if current_cube == exterior_cube {
            return true;
        }

        for (dx, dy, dz) in NEIGHBORS.iter() {
            let neighbor_cube = Cube(Point::new(
                current_cube.0.x + dx,
                current_cube.0.y + dy,
                current_cube.0.z + dz,
            ));
            if neighbor_cube.0.x > max_coords.x
                || neighbor_cube.0.y > max_coords.y
                || neighbor_cube.0.z > max_coords.z
                || neighbor_cube.0.x < min_coords.x
                || neighbor_cube.0.y < min_coords.y
                || neighbor_cube.0.z < min_coords.z
            {
                // Out of bounds
                continue;
            }
            if cubes_set.contains(&neighbor_cube) {
                continue;
            }
            if visited_cubes.contains(&neighbor_cube) {
                continue;
            }
            visited_cubes.insert(neighbor_cube.clone());
            queue.push_back(neighbor_cube);
        }
    }

    false
}

fn calc_exterior_surface_area(cubes: &[Cube]) -> u32 {
    let cubes_set: HashSet<Cube> = HashSet::from_iter(cubes.iter().cloned());
    let mut exterior_surface_area = 0;
    for cube in cubes {
        for (dx, dy, dz) in NEIGHBORS.iter() {
            let neighbor_cube = Cube(Point::new(cube.0.x + dx, cube.0.y + dy, cube.0.z + dz));
            if cubes_set.contains(&neighbor_cube) {
                continue;
            }
            if is_exterior(&neighbor_cube, &cubes_set) {
                exterior_surface_area += 1;
            }
        }
    }
    exterior_surface_area
}

#[test]
fn test_parse_cube() {
    let input = "1,2,3";
    let result = Cube::parse(input);
    assert!(result.is_ok());
    let (rest, cube) = result.unwrap();
    assert!(rest.is_empty());
    assert_eq!(cube, Cube(Point::new(1, 2, 3)));
}

#[test]
fn test_parse_cubes() {
    let contents = "1,1,1\n2,1,1\n";
    let cubes = parse_cubes(contents);
    assert_eq!(cubes.len(), 2);
    assert_eq!(cubes[0], Cube(Point::new(1, 1, 1)));
    assert_eq!(cubes[1], Cube(Point::new(2, 1, 1)));
}

#[test]
fn test_calc_surface_area_1() {
    let contents = "1,1,1\n2,1,1\n";
    let cubes = parse_cubes(contents);
    let surface_area = calc_surface_area(&cubes);
    assert_eq!(surface_area, 10);
}

#[test]
fn test_calc_surface_area_2() {
    let contents = "2,2,2\n1,2,2\n3,2,2\n2,1,2\n2,3,2\n2,2,1\n2,2,3\n2,2,4\n2,2,6\n1,2,5\n3,2,5\n2,1,5\n2,3,5\n";
    let cubes = parse_cubes(contents);
    let surface_area = calc_surface_area(&cubes);
    assert_eq!(surface_area, 64);
}

#[test]
fn test_calc_exterior_surface_area() {
    let contents = "2,2,2\n1,2,2\n3,2,2\n2,1,2\n2,3,2\n2,2,1\n2,2,3\n2,2,4\n2,2,6\n1,2,5\n3,2,5\n2,1,5\n2,3,5\n";
    let cubes = parse_cubes(contents);
    let exterior_surface_area = calc_exterior_surface_area(&cubes);
    assert_eq!(exterior_surface_area, 58);
}
