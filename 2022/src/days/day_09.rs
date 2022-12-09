use std::{collections::HashSet, fs};

#[derive(Debug, PartialEq, Eq)]
enum Motion {
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
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
    head: Point2,
    tail: Point2,
    tail_positions: HashSet<Point2>,
}

impl Rope {
    fn new() -> Self {
        let tail = Point2::new(0, 0);
        let mut tail_positions = HashSet::new();
        tail_positions.insert(tail);
        Self {
            head: Point2::new(0, 0),
            tail,
            tail_positions,
        }
    }

    fn move_tail(&mut self) {
        if !self.ends_are_adjacent() {
            let dx = self.head.x - self.tail.x;
            let dy = self.head.y - self.tail.y;
            if dx == 0 {
                // Same column, move tail vertically one step closer to the head
                if dy == -2 {
                    self.tail.y -= 1;
                } else if dy == 2 {
                    self.tail.y += 1;
                }
            } else if dy == 0 {
                // Same row, move tail horizontally one step closer to the head
                if dx == -2 {
                    self.tail.x -= 1;
                } else if dx == 2 {
                    self.tail.x += 1;
                }
            } else {
                // Move tail diagonally one step closer to the head
                if dx > 0 {
                    self.tail.x += 1;
                } else {
                    self.tail.x -= 1;
                }
                if dy > 0 {
                    self.tail.y += 1;
                } else {
                    self.tail.y -= 1;
                }
            }
            self.tail_positions.insert(self.tail);
        }
    }

    fn apply_motion(&mut self, motion: &Motion) {
        match motion {
            Motion::Up(num_steps) => {
                for _ in 0..*num_steps {
                    self.head.y += 1;
                    self.move_tail();
                }
            }
            Motion::Down(num_steps) => {
                for _ in 0..*num_steps {
                    self.head.y -= 1;
                    self.move_tail();
                }
            }
            Motion::Left(num_steps) => {
                for _ in 0..*num_steps {
                    self.head.x -= 1;
                    self.move_tail();
                }
            }
            Motion::Right(num_steps) => {
                for _ in 0..*num_steps {
                    self.head.x += 1;
                    self.move_tail();
                }
            }
        }
    }

    fn ends_are_adjacent(&self) -> bool {
        let dx = (self.head.x - self.tail.x).abs();
        let dy = (self.head.y - self.tail.y).abs();
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
            "U" => Motion::Up(num_steps),
            "D" => Motion::Down(num_steps),
            "L" => Motion::Left(num_steps),
            "R" => Motion::Right(num_steps),
            _ => panic!("Invalid direction"),
        }
    }
}

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let motions = parse_motions(&contents);
    let mut rope = Rope::new();
    for motion in &motions {
        rope.apply_motion(motion);
    }
    let tail_positions_count = rope.tail_positions_count();
    format!("{}", tail_positions_count)
}

pub fn part2() -> String {
    let _contents = get_input_file_contents();
    format!("")
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

    assert_eq!(motions[0], Motion::Right(4));
    assert_eq!(motions[1], Motion::Up(4));
    assert_eq!(motions[2], Motion::Left(3));
    assert_eq!(motions[3], Motion::Down(1));
    assert_eq!(motions[4], Motion::Right(4));
    assert_eq!(motions[5], Motion::Down(1));
    assert_eq!(motions[6], Motion::Left(5));
    assert_eq!(motions[7], Motion::Right(2));
}

#[test]
fn test_ends_are_adjacent() {
    let mut rope = Rope::new();
    rope.head = Point2::new(0, 0);
    rope.tail = Point2::new(0, 0);
    assert!(rope.ends_are_adjacent());

    rope.head = Point2::new(1, 0);
    assert!(rope.ends_are_adjacent());

    rope.head = Point2::new(-1, 1);
    assert!(rope.ends_are_adjacent());

    rope.head = Point2::new(2, 0);
    assert!(!rope.ends_are_adjacent());

    rope.head = Point2::new(0, -2);
    assert!(!rope.ends_are_adjacent());

    rope.head = Point2::new(1, 2);
    assert!(!rope.ends_are_adjacent());
}

#[test]
fn test_apply_motion() {
    let contents = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n";
    let motions = parse_motions(&contents);
    let mut rope = Rope::new();

    rope.apply_motion(&motions[0]);
    assert_eq!(rope.head, Point2::new(4, 0));
    assert_eq!(rope.tail, Point2::new(3, 0));

    rope.apply_motion(&motions[1]);
    assert_eq!(rope.head, Point2::new(4, 4));
    assert_eq!(rope.tail, Point2::new(4, 3));

    rope.apply_motion(&motions[2]);
    assert_eq!(rope.head, Point2::new(1, 4));
    assert_eq!(rope.tail, Point2::new(2, 4));

    rope.apply_motion(&motions[3]);
    assert_eq!(rope.head, Point2::new(1, 3));
    assert_eq!(rope.tail, Point2::new(2, 4));

    rope.apply_motion(&motions[4]);
    assert_eq!(rope.head, Point2::new(5, 3));
    assert_eq!(rope.tail, Point2::new(4, 3));

    rope.apply_motion(&motions[5]);
    assert_eq!(rope.head, Point2::new(5, 2));
    assert_eq!(rope.tail, Point2::new(4, 3));

    rope.apply_motion(&motions[6]);
    assert_eq!(rope.head, Point2::new(0, 2));
    assert_eq!(rope.tail, Point2::new(1, 2));

    rope.apply_motion(&motions[7]);
    assert_eq!(rope.head, Point2::new(2, 2));
    assert_eq!(rope.tail, Point2::new(1, 2));
}

#[test]
fn test_tail_positions_count() {
    let contents = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n";
    let motions = parse_motions(&contents);
    let mut rope = Rope::new();
    for motion in &motions {
        rope.apply_motion(motion);
    }
    let tail_positions_count = rope.tail_positions_count();
    assert_eq!(tail_positions_count, 13);
}
