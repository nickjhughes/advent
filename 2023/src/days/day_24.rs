use std::fs;

pub fn part1() -> String {
    let input = get_input_file_contents();
    let hailstones = parse_hailstones(&input);
    path_intersections_2d(&hailstones, 200000000000000.0, 400000000000000.0).to_string()
}

pub fn part2() -> String {
    let _input = get_input_file_contents();
    "".into()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input24").expect("Failed to open input file")
}

fn parse_hailstones(input: &str) -> Vec<Hailstone> {
    input.lines().map(Hailstone::parse).collect()
}

fn path_intersections_2d(hailstones: &[Hailstone], min: f64, max: f64) -> usize {
    let mut count = 0;
    for (i, h1) in hailstones.iter().enumerate() {
        for (_j, h2) in hailstones.iter().enumerate().skip(i + 1) {
            let u = (h1.vel.x * h2.pos.y - h1.vel.x * h1.pos.y - h1.vel.y * h2.pos.x
                + h1.vel.y * h1.pos.x)
                / (h2.vel.x * h1.vel.y - h1.vel.x * h2.vel.y);
            let t = (h2.pos.x + u * h2.vel.x - h1.pos.x) / h1.vel.x;

            if t < 0.0 || u < 0.0 {
                // Intersection occurs in the past
                continue;
            }
            if t.is_infinite() || u.is_infinite() {
                // Lines are parallel
                continue;
            }

            let int_x = h1.pos.x + h1.vel.x * t;
            let int_y = h1.pos.y + h1.vel.y * t;
            if int_x >= min && int_x <= max && int_y >= min && int_y <= max {
                // Collision is within test range
                count += 1;
            }
        }
    }
    count
}

#[derive(Debug, PartialEq)]
struct Hailstone {
    pos: Vec3,
    vel: Vec3,
}

#[derive(Debug, PartialEq, Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Hailstone {
    fn parse(input: &str) -> Self {
        let (pos_str, vel_str) = input.split_once(" @ ").unwrap();
        Hailstone {
            pos: Vec3::parse(pos_str),
            vel: Vec3::parse(vel_str),
        }
    }
}

impl Vec3 {
    fn parse(input: &str) -> Self {
        let mut parts = input.split(',');
        let x = parts.next().unwrap().trim().parse::<f64>().unwrap();
        let y = parts.next().unwrap().trim().parse::<f64>().unwrap();
        let z = parts.next().unwrap().trim().parse::<f64>().unwrap();
        Vec3 { x, y, z }
    }
}

#[test]
fn test_parse_vec3() {
    let input = "-2,  1, -2";
    let vec = Vec3::parse(input);
    assert_eq!(
        vec,
        Vec3 {
            x: -2.0,
            y: 1.0,
            z: -2.0
        }
    );
}

#[test]
fn test_parse_hailstone() {
    let input = "19, 13, 30 @ -2,  1, -2";
    let hailstone = Hailstone::parse(input);
    assert_eq!(
        hailstone,
        Hailstone {
            pos: Vec3 {
                x: 19.0,
                y: 13.0,
                z: 30.0
            },
            vel: Vec3 {
                x: -2.0,
                y: 1.0,
                z: -2.0
            }
        }
    );
}

#[test]
fn test_path_intersections() {
    let input = "19, 13, 30 @ -2,  1, -2\n18, 19, 22 @ -1, -1, -2\n20, 25, 34 @ -2, -2, -4\n12, 31, 28 @ -1, -2, -1\n20, 19, 15 @  1, -5, -3\n";
    let hailstones = parse_hailstones(input);
    assert_eq!(path_intersections_2d(&hailstones, 7.0, 27.0), 2);
}
