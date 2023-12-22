use hashbrown::HashSet;
use std::fs;

pub fn part1() -> String {
    let input = get_input_file_contents();
    let mut bricks = parse_bricks(&input);
    settle_bricks(&mut bricks);
    disintegratable_bricks(&bricks).to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let mut bricks = parse_bricks(&input);
    settle_bricks(&mut bricks);
    sum_bricks_moved(&bricks).to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input22").expect("Failed to open input file")
}

fn parse_bricks(input: &str) -> Vec<Brick> {
    input.lines().map(Brick::parse).collect()
}

fn settle_bricks(bricks: &mut [Brick]) -> usize {
    let mut occupied_cubes: HashSet<Cube> = HashSet::new();
    for brick in bricks.iter() {
        assert!(brick.end.x >= brick.start.x);
        assert!(brick.end.y >= brick.start.y);
        assert!(brick.end.z >= brick.start.z);
        for x in brick.start.x..=brick.end.x {
            for y in brick.start.y..=brick.end.y {
                for z in brick.start.z..=brick.end.z {
                    occupied_cubes.insert(Cube { x, y, z });
                }
            }
        }
    }

    let mut bricks_moved: HashSet<usize> = HashSet::new();
    loop {
        let mut brick_moved = false;
        for (i, brick) in bricks.iter_mut().enumerate() {
            if brick.start.z == 1 {
                continue;
            }
            if brick.is_vertical() {
                if !occupied_cubes.contains(&Cube {
                    x: brick.start.x,
                    y: brick.start.y,
                    z: brick.start.z - 1,
                }) {
                    brick.start.z -= 1;
                    brick.end.z -= 1;
                    occupied_cubes.remove(&Cube {
                        x: brick.start.x,
                        y: brick.start.y,
                        z: brick.end.z + 1,
                    });
                    occupied_cubes.insert(Cube {
                        x: brick.start.x,
                        y: brick.start.y,
                        z: brick.start.z,
                    });
                    brick_moved = true;
                    bricks_moved.insert(i);
                }
            } else if brick.is_horizontal() {
                let mut clear_below = true;
                if brick.end.x != brick.start.x {
                    for x in brick.start.x..=brick.end.x {
                        if occupied_cubes.contains(&Cube {
                            x,
                            y: brick.start.y,
                            z: brick.start.z - 1,
                        }) {
                            clear_below = false;
                            break;
                        }
                    }
                } else {
                    for y in brick.start.y..=brick.end.y {
                        if occupied_cubes.contains(&Cube {
                            x: brick.start.x,
                            y,
                            z: brick.start.z - 1,
                        }) {
                            clear_below = false;
                            break;
                        }
                    }
                }
                if clear_below {
                    brick.start.z -= 1;
                    brick.end.z -= 1;
                    for x in brick.start.x..=brick.end.x {
                        for y in brick.start.y..=brick.end.y {
                            occupied_cubes.remove(&Cube {
                                x,
                                y,
                                z: brick.start.z + 1,
                            });
                            occupied_cubes.insert(Cube {
                                x,
                                y,
                                z: brick.start.z,
                            });
                        }
                    }
                    brick_moved = true;
                    bricks_moved.insert(i);
                }
            }
        }
        if !brick_moved {
            break;
        }
    }
    bricks_moved.len()
}

fn disintegratable_bricks(bricks: &[Brick]) -> usize {
    let mut count = 0;
    for i in 0..bricks.len() {
        let mut new_bricks = bricks.to_vec();
        new_bricks.swap_remove(i);
        if settle_bricks(&mut new_bricks) == 0 {
            count += 1;
        }
    }
    count
}

fn sum_bricks_moved(bricks: &[Brick]) -> usize {
    let mut total_count = 0;
    for i in 0..bricks.len() {
        let mut new_bricks = bricks.to_vec();
        new_bricks.swap_remove(i);
        total_count += settle_bricks(&mut new_bricks);
    }
    total_count
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, PartialEq, Clone)]
struct Brick {
    start: Cube,
    end: Cube,
}

impl Cube {
    fn parse(input: &str) -> Self {
        let mut parts = input.split(',');
        let x = parts.next().unwrap().parse::<i32>().unwrap();
        let y = parts.next().unwrap().parse::<i32>().unwrap();
        let z = parts.next().unwrap().parse::<i32>().unwrap();
        Cube { x, y, z }
    }
}

impl Brick {
    fn parse(input: &str) -> Self {
        let (start, end) = input.split_once('~').unwrap();
        Brick {
            start: Cube::parse(start),
            end: Cube::parse(end),
        }
    }

    fn is_horizontal(&self) -> bool {
        self.start.z == self.end.z
    }

    fn is_vertical(&self) -> bool {
        self.start.z != self.end.z
    }
}

#[test]
fn test_parse_brick() {
    let input = "1,0,1~1,2,1";
    let brick = Brick::parse(input);
    assert_eq!(
        brick,
        Brick {
            start: Cube { x: 1, y: 0, z: 1 },
            end: Cube { x: 1, y: 2, z: 1 }
        }
    );
}

#[test]
fn test_parse_bricks() {
    let input = "1,0,1~1,2,1\n0,0,2~2,0,2\n0,2,3~2,2,3\n0,0,4~0,2,4\n2,0,5~2,2,5\n0,1,6~2,1,6\n1,1,8~1,1,9\n";
    let bricks = parse_bricks(input);
    assert_eq!(bricks.len(), 7);
}

#[test]
fn test_disintegratable_bricks() {
    let input = "1,0,1~1,2,1\n0,0,2~2,0,2\n0,2,3~2,2,3\n0,0,4~0,2,4\n2,0,5~2,2,5\n0,1,6~2,1,6\n1,1,8~1,1,9\n";
    let mut bricks = parse_bricks(input);
    settle_bricks(&mut bricks);
    assert_eq!(disintegratable_bricks(&bricks), 5);
}

#[test]
fn test_sum_bricks_moved() {
    let input = "1,0,1~1,2,1\n0,0,2~2,0,2\n0,2,3~2,2,3\n0,0,4~0,2,4\n2,0,5~2,2,5\n0,1,6~2,1,6\n1,1,8~1,1,9\n";
    let mut bricks = parse_bricks(input);
    settle_bricks(&mut bricks);
    assert_eq!(sum_bricks_moved(&bricks), 7);
}
