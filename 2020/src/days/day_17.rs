use std::{collections::HashSet, fs};

pub fn part1() -> String {
    let input = get_input_file_contents();
    let mut cubes = parse_3d(&input);
    for _ in 0..6 {
        cycle_3d(&mut cubes);
    }
    cubes.len().to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let mut hypercubes = parse_4d(&input);
    for _ in 0..6 {
        cycle_4d(&mut hypercubes);
    }
    hypercubes.len().to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input17").expect("Failed to open input file")
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
struct Cube3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Cube3 {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Cube3 { x, y, z }
    }
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
struct Cube4 {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Cube4 {
    fn new(x: isize, y: isize, z: isize, w: isize) -> Self {
        Cube4 { x, y, z, w }
    }
}

fn parse_3d(input: &str) -> HashSet<Cube3> {
    let mut cubes = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    cubes.insert(Cube3::new(x as isize, y as isize, 0));
                }
                '.' => {}
                _ => panic!("invalid input {:?}", ch),
            }
        }
    }
    cubes
}

fn parse_4d(input: &str) -> HashSet<Cube4> {
    let mut hypercubes = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    hypercubes.insert(Cube4::new(x as isize, y as isize, 0, 0));
                }
                '.' => {}
                _ => panic!("invalid input {:?}", ch),
            }
        }
    }
    hypercubes
}

fn cycle_3d(cubes: &mut HashSet<Cube3>) {
    let mut to_remove = Vec::new();
    for cube in cubes.iter() {
        // If a cube is active and exactly 2 or 3 of its neighbors are also active,
        // the cube remains active. Otherwise, the cube becomes inactive.
        let mut active_neighbors = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue;
                    }
                    if cubes.contains(&Cube3::new(cube.x + dx, cube.y + dy, cube.z + dz)) {
                        active_neighbors += 1;
                    }
                }
            }
        }
        if active_neighbors != 2 && active_neighbors != 3 {
            to_remove.push(*cube);
        }
    }

    let mut to_add: Vec<Cube3> = Vec::new();
    let min_x = cubes.iter().map(|c| c.x).min().unwrap();
    let max_x = cubes.iter().map(|c| c.x).max().unwrap();
    let min_y = cubes.iter().map(|c| c.y).min().unwrap();
    let max_y = cubes.iter().map(|c| c.y).max().unwrap();
    let min_z = cubes.iter().map(|c| c.z).min().unwrap();
    let max_z = cubes.iter().map(|c| c.z).max().unwrap();
    for x in min_x - 1..=max_x + 1 {
        for y in min_y - 1..=max_y + 1 {
            for z in min_z - 1..=max_z + 1 {
                // If a cube is inactive but exactly 3 of its neighbors are active,
                // the cube becomes active. Otherwise, the cube remains inactive.
                let mut active_neighbors = 0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        for dz in -1..=1 {
                            if dx == 0 && dy == 0 && dz == 0 {
                                continue;
                            }
                            if cubes.contains(&Cube3::new(x + dx, y + dy, z + dz)) {
                                active_neighbors += 1;
                            }
                        }
                    }
                }
                if active_neighbors == 3 {
                    to_add.push(Cube3::new(x, y, z));
                }
            }
        }
    }

    cubes.extend(to_add);
    for cube in to_remove {
        cubes.remove(&cube);
    }
}

fn cycle_4d(hypercubes: &mut HashSet<Cube4>) {
    let mut to_remove = Vec::new();
    for hypercube in hypercubes.iter() {
        // If a cube is active and exactly 2 or 3 of its neighbors are also active,
        // the cube remains active. Otherwise, the cube becomes inactive.
        let mut active_neighbors = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for dw in -1..=1 {
                        if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                            continue;
                        }
                        if hypercubes.contains(&Cube4::new(
                            hypercube.x + dx,
                            hypercube.y + dy,
                            hypercube.z + dz,
                            hypercube.w + dw,
                        )) {
                            active_neighbors += 1;
                        }
                    }
                }
            }
        }
        if active_neighbors != 2 && active_neighbors != 3 {
            to_remove.push(*hypercube);
        }
    }

    let mut to_add: Vec<Cube4> = Vec::new();
    let min_x = hypercubes.iter().map(|c| c.x).min().unwrap();
    let max_x = hypercubes.iter().map(|c| c.x).max().unwrap();
    let min_y = hypercubes.iter().map(|c| c.y).min().unwrap();
    let max_y = hypercubes.iter().map(|c| c.y).max().unwrap();
    let min_z = hypercubes.iter().map(|c| c.z).min().unwrap();
    let max_z = hypercubes.iter().map(|c| c.z).max().unwrap();
    let min_w = hypercubes.iter().map(|c| c.w).min().unwrap();
    let max_w = hypercubes.iter().map(|c| c.w).max().unwrap();
    for x in min_x - 1..=max_x + 1 {
        for y in min_y - 1..=max_y + 1 {
            for z in min_z - 1..=max_z + 1 {
                for w in min_w - 1..=max_w + 1 {
                    // If a cube is inactive but exactly 3 of its neighbors are active,
                    // the cube becomes active. Otherwise, the cube remains inactive.
                    let mut active_neighbors = 0;
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            for dz in -1..=1 {
                                for dw in -1..=1 {
                                    if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                                        continue;
                                    }
                                    if hypercubes.contains(&Cube4::new(
                                        x + dx,
                                        y + dy,
                                        z + dz,
                                        w + dw,
                                    )) {
                                        active_neighbors += 1;
                                    }
                                }
                            }
                        }
                    }
                    if active_neighbors == 3 {
                        to_add.push(Cube4::new(x, y, z, w));
                    }
                }
            }
        }
    }

    hypercubes.extend(to_add);
    for hypercube in to_remove {
        hypercubes.remove(&hypercube);
    }
}

#[test]
fn test_parse_3d() {
    let input = ".#.\n..#\n###\n";
    let cubes = parse_3d(input);
    assert_eq!(cubes.len(), 5);
    assert!(cubes.contains(&Cube3::new(1, 0, 0)));
    assert!(cubes.contains(&Cube3::new(2, 1, 0)));
    assert!(cubes.contains(&Cube3::new(0, 2, 0)));
    assert!(cubes.contains(&Cube3::new(1, 2, 0)));
    assert!(cubes.contains(&Cube3::new(2, 2, 0)));
}

#[test]
fn test_parse_4d() {
    let input = ".#.\n..#\n###\n";
    let cubes = parse_4d(input);
    assert_eq!(cubes.len(), 5);
    assert!(cubes.contains(&Cube4::new(1, 0, 0, 0)));
    assert!(cubes.contains(&Cube4::new(2, 1, 0, 0)));
    assert!(cubes.contains(&Cube4::new(0, 2, 0, 0)));
    assert!(cubes.contains(&Cube4::new(1, 2, 0, 0)));
    assert!(cubes.contains(&Cube4::new(2, 2, 0, 0)));
}

#[test]
fn test_cycle_3d() {
    let input = ".#.\n..#\n###\n";
    let mut cubes = parse_3d(input);

    cycle_3d(&mut cubes);
    assert_eq!(cubes.len(), 11);

    cycle_3d(&mut cubes);
    assert_eq!(cubes.len(), 21);

    cycle_3d(&mut cubes);
    assert_eq!(cubes.len(), 38);

    for _ in 0..3 {
        cycle_3d(&mut cubes);
    }
    assert_eq!(cubes.len(), 112);
}

#[test]
#[ignore]
fn test_cycle_4d() {
    let input = ".#.\n..#\n###\n";
    let mut hypercubes = parse_4d(input);

    for _ in 0..6 {
        cycle_4d(&mut hypercubes);
    }
    assert_eq!(hypercubes.len(), 848);
}
