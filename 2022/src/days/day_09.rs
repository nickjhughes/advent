use std::{collections::HashSet, fs};

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
struct Motion {
    direction: Direction,
    num_steps: u8,
}

impl Motion {
    fn new(direction: Direction, num_steps: u8) -> Self {
        Self {
            direction,
            num_steps,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point2 {
    x: i32,
    y: i32,
}

impl Point2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Point2>,
    tail_positions: HashSet<Point2>,
}

impl Rope {
    fn new(length: usize) -> Self {
        let knots = vec![Point2::new(0, 0); length];
        let mut tail_positions = HashSet::new();
        tail_positions.insert(*knots.last().unwrap());
        Self {
            knots,
            tail_positions,
        }
    }

    fn move_head_one_step(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => {
                self.knots[0].y += 1;
            }
            Direction::Down => {
                self.knots[0].y -= 1;
            }
            Direction::Left => {
                self.knots[0].x -= 1;
            }
            Direction::Right => {
                self.knots[0].x += 1;
            }
        }
    }

    fn move_knot(&mut self, i: usize) {
        assert!(i > 0);
        if !self.knots_are_adjacent(i, i - 1) {
            let dx = self.knots[i - 1].x - self.knots[i].x;
            let dy = self.knots[i - 1].y - self.knots[i].y;
            if dx == 0 {
                // Same column, move tail vertically one step closer to the head
                if dy == -2 {
                    self.knots[i].y -= 1;
                } else if dy == 2 {
                    self.knots[i].y += 1;
                }
            } else if dy == 0 {
                // Same row, move tail horizontally one step closer to the head
                if dx == -2 {
                    self.knots[i].x -= 1;
                } else if dx == 2 {
                    self.knots[i].x += 1;
                }
            } else {
                // Move tail diagonally one step closer to the head
                if dx > 0 {
                    self.knots[i].x += 1;
                } else {
                    self.knots[i].x -= 1;
                }
                if dy > 0 {
                    self.knots[i].y += 1;
                } else {
                    self.knots[i].y -= 1;
                }
            }
            self.tail_positions.insert(*self.knots.last().unwrap());
        }
    }

    fn apply_motion(&mut self, motion: &Motion) {
        for _ in 0..motion.num_steps {
            self.move_head_one_step(&motion.direction);
            for i in 1..self.knots.len() {
                self.move_knot(i);
            }
        }
    }

    fn knots_are_adjacent(&self, i: usize, j: usize) -> bool {
        let dx = (self.knots[i].x - self.knots[j].x).abs();
        let dy = (self.knots[i].y - self.knots[j].y).abs();
        if dx == 0 && dy == 0 {
            // Overlapping
            true
        } else if (dx == 0 && dy == 1) || (dy == 0 && dx == 1) {
            // Horizontally or vertically adjacent
            true
        } else if dx == 1 && dy == 1 {
            // Diagonally adjacent
            true
        } else {
            false
        }
    }

    fn tail_positions_count(&self) -> usize {
        self.tail_positions.len()
    }
}

impl Motion {
    fn parse(input: &str) -> Self {
        let parts = input.split(' ').collect::<Vec<&str>>();
        assert_eq!(parts.len(), 2);
        let num_steps = parts[1]
            .parse::<u8>()
            .expect("Failed to parse number of steps");
        match parts[0] {
            "U" => Motion::new(Direction::Up, num_steps),
            "D" => Motion::new(Direction::Down, num_steps),
            "L" => Motion::new(Direction::Left, num_steps),
            "R" => Motion::new(Direction::Right, num_steps),
            _ => panic!("Invalid direction"),
        }
    }
}

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let motions = parse_motions(&contents);
    let mut rope = Rope::new(2);
    for motion in &motions {
        rope.apply_motion(motion);
    }
    let tail_positions_count = rope.tail_positions_count();
    format!("{}", tail_positions_count)
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let motions = parse_motions(&contents);
    let mut rope = Rope::new(10);
    for motion in &motions {
        rope.apply_motion(motion);
    }
    let tail_positions_count = rope.tail_positions_count();
    format!("{}", tail_positions_count)
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input09").expect("Failed to open input file")
}

fn parse_motions(contents: &str) -> Vec<Motion> {
    let mut motions = Vec::new();
    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }
        motions.push(Motion::parse(line));
    }
    motions
}

#[test]
fn test_parse_motions() {
    let contents = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n";
    let motions = parse_motions(&contents);
    assert_eq!(motions.len(), 8);

    assert_eq!(motions[0], Motion::new(Direction::Right, 4));
    assert_eq!(motions[1], Motion::new(Direction::Up, 4));
    assert_eq!(motions[2], Motion::new(Direction::Left, 3));
    assert_eq!(motions[3], Motion::new(Direction::Down, 1));
    assert_eq!(motions[4], Motion::new(Direction::Right, 4));
    assert_eq!(motions[5], Motion::new(Direction::Down, 1));
    assert_eq!(motions[6], Motion::new(Direction::Left, 5));
    assert_eq!(motions[7], Motion::new(Direction::Right, 2));
}

#[test]
fn test_knots_are_adjacent() {
    let mut rope = Rope::new(2);
    rope.knots[0] = Point2::new(0, 0);
    rope.knots[1] = Point2::new(0, 0);
    assert!(rope.knots_are_adjacent(0, 1));

    rope.knots[0] = Point2::new(1, 0);
    assert!(rope.knots_are_adjacent(0, 1));

    rope.knots[0] = Point2::new(-1, 1);
    assert!(rope.knots_are_adjacent(0, 1));

    rope.knots[0] = Point2::new(2, 0);
    assert!(!rope.knots_are_adjacent(0, 1));

    rope.knots[0] = Point2::new(0, -2);
    assert!(!rope.knots_are_adjacent(0, 1));

    rope.knots[0] = Point2::new(1, 2);
    assert!(!rope.knots_are_adjacent(0, 1));
}

#[test]
fn test_apply_motion_two_knots() {
    let contents = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n";
    let motions = parse_motions(&contents);
    let mut rope = Rope::new(2);

    rope.apply_motion(&motions[0]);
    assert_eq!(rope.knots[0], Point2::new(4, 0));
    assert_eq!(rope.knots[1], Point2::new(3, 0));

    rope.apply_motion(&motions[1]);
    assert_eq!(rope.knots[0], Point2::new(4, 4));
    assert_eq!(rope.knots[1], Point2::new(4, 3));

    rope.apply_motion(&motions[2]);
    assert_eq!(rope.knots[0], Point2::new(1, 4));
    assert_eq!(rope.knots[1], Point2::new(2, 4));

    rope.apply_motion(&motions[3]);
    assert_eq!(rope.knots[0], Point2::new(1, 3));
    assert_eq!(rope.knots[1], Point2::new(2, 4));

    rope.apply_motion(&motions[4]);
    assert_eq!(rope.knots[0], Point2::new(5, 3));
    assert_eq!(rope.knots[1], Point2::new(4, 3));

    rope.apply_motion(&motions[5]);
    assert_eq!(rope.knots[0], Point2::new(5, 2));
    assert_eq!(rope.knots[1], Point2::new(4, 3));

    rope.apply_motion(&motions[6]);
    assert_eq!(rope.knots[0], Point2::new(0, 2));
    assert_eq!(rope.knots[1], Point2::new(1, 2));

    rope.apply_motion(&motions[7]);
    assert_eq!(rope.knots[0], Point2::new(2, 2));
    assert_eq!(rope.knots[1], Point2::new(1, 2));
}

#[test]
fn test_tail_positions_count_two_knots() {
    let contents = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n";
    let motions = parse_motions(&contents);
    let mut rope = Rope::new(2);
    for motion in &motions {
        rope.apply_motion(motion);
    }
    let tail_positions_count = rope.tail_positions_count();
    assert_eq!(tail_positions_count, 13);
}

#[test]
fn test_tail_positions_count_ten_knots() {
    let contents = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20\n";
    let motions = parse_motions(&contents);
    let mut rope = Rope::new(10);
    for motion in &motions {
        rope.apply_motion(motion);
    }
    let tail_positions_count = rope.tail_positions_count();
    assert_eq!(tail_positions_count, 36);
}
