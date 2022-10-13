use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace1},
    combinator::{map, opt},
    multi::many1,
    sequence::tuple,
    IResult,
};
use std::fs;

pub fn part1() -> String {
    let cubes = load_cubes()
        .into_iter()
        .filter(|c| {
            c.xmin >= -50
                && c.xmax <= 50
                && c.ymin >= -50
                && c.ymax <= 50
                && c.zmin >= -50
                && c.zmax <= 50
        })
        .collect::<Vec<Cube>>();
    let num_cubes_on = cube_calculator(&cubes);
    format!("{}", num_cubes_on).to_string()
}

pub fn part2() -> String {
    let cubes = load_cubes();
    let num_cubes_on = cube_calculator(&cubes);
    format!("{}", num_cubes_on).to_string()
}

fn cube_intersection(a: &Cube, b: &Cube) -> Option<Cube> {
    let xmin = a.xmin.max(b.xmin);
    let xmax = a.xmax.min(b.xmax);
    let ymin = a.ymin.max(b.ymin);
    let ymax = a.ymax.min(b.ymax);
    let zmin = a.zmin.max(b.zmin);
    let zmax = a.zmax.min(b.zmax);
    if xmin > xmax || ymin > ymax || zmin > zmax {
        None
    } else {
        Some(Cube {
            on: !b.on,
            xmin,
            xmax,
            ymin,
            ymax,
            zmin,
            zmax,
        })
    }
}

fn cube_calculator(cubes: &[Cube]) -> i64 {
    let mut core_cubes = Vec::new();
    for cube in cubes {
        let mut to_add = Vec::new();
        if cube.on {
            to_add.push(cube.clone());
        }
        for existing_cube in &core_cubes {
            if let Some(intersection) = cube_intersection(cube, existing_cube) {
                to_add.push(intersection);
            }
        }
        core_cubes.extend(to_add);
    }

    let mut on_count = 0;
    for cube in &core_cubes {
        let cube_size = cube.size();
        if cube.on {
            on_count += cube_size;
        } else {
            on_count -= cube_size;
        }
    }

    on_count
}

#[derive(Debug, Clone)]
struct Cube {
    on: bool,
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
    zmin: i64,
    zmax: i64,
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
    map(
        tuple((opt(tag::<_, &str, _>("-")), digit1)),
        |(neg, num)| {
            let mut parsed = num.parse::<i64>().unwrap();
            if neg.is_some() {
                parsed *= -1
            }
            parsed
        },
    )(input)
}

impl Cube {
    fn size(&self) -> i64 {
        (self.xmax - self.xmin + 1) * (self.ymax - self.ymin + 1) * (self.zmax - self.zmin + 1)
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                alt((tag("on"), tag("off"))),
                multispace1,
                tag("x="),
                parse_i64,
                tag(".."),
                parse_i64,
                tag(","),
                tag("y="),
                parse_i64,
                tag(".."),
                parse_i64,
                tag(","),
                tag("z="),
                parse_i64,
                tag(".."),
                parse_i64,
            )),
            |(on, _, _, xmin, _, xmax, _, _, ymin, _, ymax, _, _, zmin, _, zmax)| Cube {
                on: match on {
                    "on" => true,
                    "off" => false,
                    _ => panic!("Failed to parse cube on/off."),
                },
                xmin,
                xmax,
                ymin,
                ymax,
                zmin,
                zmax,
            },
        )(input)
    }
}

fn parse_cubes(input: &str) -> IResult<&str, Vec<Cube>> {
    many1(map(
        tuple((Cube::parse, opt(line_ending))),
        |(instruction, _)| instruction,
    ))(input)
}

fn load_cubes() -> Vec<Cube> {
    let contents = fs::read_to_string("inputs/input22").expect("Failed to open input file");
    let (_, instructions) = parse_cubes(&contents).expect("Failed to parse input file");
    instructions
}

#[test]
fn parse_test() {
    let input = "on x=2..47,y=-22..22,z=-23..27";
    let (_, cube) = Cube::parse(input).unwrap();
    assert_eq!(cube.on, true);
    assert_eq!(cube.xmin, 2);
    assert_eq!(cube.xmax, 47);
    assert_eq!(cube.ymin, -22);
    assert_eq!(cube.ymax, 22);
    assert_eq!(cube.zmin, -23);
    assert_eq!(cube.zmax, 27);
}

#[test]
fn test_1() {
    let cubes = vec![
        Cube {
            on: true,
            xmin: 10,
            xmax: 12,
            ymin: 10,
            ymax: 12,
            zmin: 10,
            zmax: 12,
        },
        Cube {
            on: true,
            xmin: 11,
            xmax: 13,
            ymin: 11,
            ymax: 13,
            zmin: 11,
            zmax: 13,
        },
        Cube {
            on: false,
            xmin: 9,
            xmax: 11,
            ymin: 9,
            ymax: 11,
            zmin: 9,
            zmax: 11,
        },
        Cube {
            on: true,
            xmin: 10,
            xmax: 10,
            ymin: 10,
            ymax: 10,
            zmin: 10,
            zmax: 10,
        },
    ];
    let num_cubes_on = cube_calculator(&cubes);
    assert_eq!(num_cubes_on, 39);
}
