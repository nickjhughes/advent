use std::{
    collections::{HashMap, HashSet},
    fmt, fs,
};

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let pattern = parse_pattern(&contents);
    let height = run_simulation(pattern, 2022);
    format!("{}", height)
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let pattern = parse_pattern(&contents);
    let height = run_simulation(pattern, 1000000000000);
    format!("{}", height)
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input17").expect("Failed to open input file")
}

#[derive(Debug, PartialEq, Eq)]
enum Push {
    Left,
    Right,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum RockType {
    Horizontal,
    Plus,
    Corner,
    Vertical,
    Square,
}

lazy_static! {
    static ref ROCK_SIZES: HashMap<RockType, (u32, u32)> = {
        let mut m = HashMap::new();
        m.insert(RockType::Horizontal, (4, 1));
        m.insert(RockType::Plus, (3, 3));
        m.insert(RockType::Corner, (3, 3));
        m.insert(RockType::Vertical, (1, 4));
        m.insert(RockType::Square, (2, 2));
        m
    };
    static ref ROCK_SHAPES: HashMap<RockType, Vec<(u32, u32)>> = {
        let mut m = HashMap::new();
        m.insert(RockType::Horizontal, vec![(0, 0), (1, 0), (2, 0), (3, 0)]);
        m.insert(RockType::Plus, vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]);
        m.insert(
            RockType::Corner,
            vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        );
        m.insert(RockType::Vertical, vec![(0, 0), (0, 1), (0, 2), (0, 3)]);
        m.insert(RockType::Square, vec![(0, 0), (1, 0), (0, 1), (1, 1)]);
        m
    };
}

#[derive(Debug, Clone)]
struct Rock {
    ty: RockType,
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Room {
    falling_rock: Option<Rock>,
    stationary_rock_points: HashSet<(u32, u32)>,
    rocks_dropped_count: usize,
    next_rock_type: RockType,
    pattern: Vec<Push>,
    pattern_index: usize,
}

impl Room {
    fn new(pattern: Vec<Push>) -> Self {
        Self {
            falling_rock: None,
            stationary_rock_points: HashSet::new(),
            rocks_dropped_count: 0,
            next_rock_type: RockType::Horizontal,
            pattern,
            pattern_index: 0,
        }
    }

    fn release_rock(&mut self) {
        if self.falling_rock.is_some() {
            panic!("Only one rock can be falling at a time");
        }

        let ty = self.next_rock_type;
        self.next_rock_type = match self.next_rock_type {
            RockType::Horizontal => RockType::Plus,
            RockType::Plus => RockType::Corner,
            RockType::Corner => RockType::Vertical,
            RockType::Vertical => RockType::Square,
            RockType::Square => RockType::Horizontal,
        };
        let x = 2;
        let y = self.height() + 3;
        self.falling_rock = Some(Rock { ty, x, y });
    }

    fn drop_rock(&mut self) {
        if self.falling_rock.is_none() {
            panic!("Can't drop a rock that isn't falling")
        }
        let falling_rock = self.falling_rock.as_mut().unwrap();
        let (rock_width, _) = ROCK_SIZES[&falling_rock.ty];

        loop {
            // Being pushed by a jet of hot gas
            let push = &self.pattern[self.pattern_index];
            match push {
                Push::Left => {
                    if falling_rock.x > 0 {
                        let mut new_rock = falling_rock.clone();
                        new_rock.x -= 1;
                        if !check_for_collision(&new_rock, &self.stationary_rock_points) {
                            falling_rock.x -= 1;
                        }
                    }
                }
                Push::Right => {
                    if falling_rock.x + rock_width - 1 < 6 {
                        let mut new_rock = falling_rock.clone();
                        new_rock.x += 1;
                        if !check_for_collision(&new_rock, &self.stationary_rock_points) {
                            falling_rock.x += 1;
                        }
                    }
                }
            }
            self.pattern_index += 1;
            if self.pattern_index == self.pattern.len() {
                self.pattern_index = 0;
            }

            // Falling one unit down
            if falling_rock.y == 0 {
                // On the floor
                break;
            } else {
                let mut new_rock = falling_rock.clone();
                new_rock.y -= 1;
                if check_for_collision(&new_rock, &self.stationary_rock_points) {
                    // Collided with stationary rock
                    break;
                } else {
                    falling_rock.y -= 1;
                }
            }
        }
        self.solidify_falling_rock();
    }

    fn solidify_falling_rock(&mut self) {
        let falling_rock = self
            .falling_rock
            .as_ref()
            .expect("Can't solidify a rock that isn't falling");
        for (x, y) in &ROCK_SHAPES[&falling_rock.ty] {
            self.stationary_rock_points
                .insert((x + falling_rock.x, y + falling_rock.y));
        }
        self.falling_rock = None;
        self.rocks_dropped_count += 1;

        // We don't need to remember the full tower
        let height = self.height();
        self.stationary_rock_points
            .retain(|(_, y)| *y >= height.saturating_sub(100));
    }

    fn height(&self) -> u32 {
        if self.stationary_rock_points.is_empty() {
            0
        } else {
            self.stationary_rock_points
                .iter()
                .map(|(_, y)| *y)
                .max()
                .unwrap()
                + 1
        }
    }

    fn state_hash(&self) -> (RockType, usize, Vec<(u32, u32)>) {
        let min_y = self
            .stationary_rock_points
            .iter()
            .map(|(_, y)| *y)
            .min()
            .unwrap_or(0);
        let mut rock_points = self
            .stationary_rock_points
            .iter()
            .map(|(x, y)| (*x, *y - min_y))
            .collect::<Vec<(u32, u32)>>();
        rock_points.sort();
        (
            self.falling_rock.as_ref().unwrap().ty,
            self.pattern_index,
            rock_points,
        )
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_y = self.height();
        for y in (0..=max_y + 5).rev() {
            write!(f, "|")?;
            for x in 0..7 {
                if self.stationary_rock_points.contains(&(x, y)) {
                    write!(f, "#")?;
                } else if let Some(falling_rock) = self.falling_rock.as_ref() {
                    let mut in_falling_rock = false;
                    for point in &ROCK_SHAPES[&falling_rock.ty] {
                        if x == point.0 + falling_rock.x && y == point.1 + falling_rock.y {
                            write!(f, "@")?;
                            in_falling_rock = true;
                            break;
                        }
                    }
                    if !in_falling_rock {
                        write!(f, ".")?;
                    }
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+-------+")?;
        Ok(())
    }
}

fn check_for_collision(rock: &Rock, stationary_rock_points: &HashSet<(u32, u32)>) -> bool {
    for (x, y) in &ROCK_SHAPES[&rock.ty] {
        let point = (x + rock.x, y + rock.y);
        if stationary_rock_points.contains(&point) {
            return true;
        }
    }
    false
}

fn parse_pattern(contents: &str) -> Vec<Push> {
    contents
        .chars()
        .filter_map(|ch| match ch {
            '<' => Some(Push::Left),
            '>' => Some(Push::Right),
            _ => None,
        })
        .collect::<Vec<Push>>()
}

fn run_simulation(pattern: Vec<Push>, num_rocks: u64) -> u64 {
    let mut unique_states = HashMap::new();
    let mut found_repeat_state = false;
    let mut total_rocks_dropped_count: u64 = 0;
    let mut height_from_repeats: u64 = 0;
    let mut room = Room::new(pattern);
    loop {
        room.release_rock();

        if !found_repeat_state {
            let state = room.state_hash();
            if unique_states.contains_key(&state) {
                found_repeat_state = true;
                let (previous_rocks_dropped_count, previous_height) = unique_states[&state];
                let repeat_height = (room.height() - previous_height) as u64;
                let repeat_rocks = (room.rocks_dropped_count - previous_rocks_dropped_count) as u64;
                while total_rocks_dropped_count + repeat_rocks <= num_rocks {
                    total_rocks_dropped_count += repeat_rocks;
                    height_from_repeats += repeat_height;
                }
            } else {
                unique_states.insert(state, (room.rocks_dropped_count, room.height()));
            }
        }

        room.drop_rock();

        total_rocks_dropped_count += 1;
        if total_rocks_dropped_count == num_rocks {
            break;
        }
    }
    room.height() as u64 + height_from_repeats
}

#[test]
fn test_parse_pattern() {
    let contents = ">>><<>\n";
    let pattern = parse_pattern(contents);
    assert_eq!(
        pattern,
        vec![
            Push::Right,
            Push::Right,
            Push::Right,
            Push::Left,
            Push::Left,
            Push::Right
        ]
    );
}

#[test]
fn test_run_simulation() {
    let contents = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    let pattern = parse_pattern(contents);
    let height = run_simulation(pattern, 2022);
    assert_eq!(height, 3068);
}

#[test]
fn test_run_simulation_big() {
    let contents = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    let pattern = parse_pattern(contents);
    let height = run_simulation(pattern, 1000000000000);
    assert_eq!(height, 1514285714288);
}
