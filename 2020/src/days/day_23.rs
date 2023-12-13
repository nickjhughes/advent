use std::fs;

pub fn part1() -> String {
    let input = get_input_file_contents();
    let mut game = Game::parse(&input, false);
    for _ in 0..100 {
        game.do_move();
    }
    game.order().to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let mut game = Game::parse(&input, true);
    for _ in 0..10_000_000 {
        game.do_move();
    }
    let (cup1, cup2) = game.two_cups_after_cup_one();
    (cup1 as u64 * cup2 as u64).to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input23").expect("Failed to open input file")
}

#[derive(Debug, PartialEq)]
struct Game {
    cups: Vec<u32>,
    current_cup: u32,
    min_cup: u32,
    max_cup: u32,
}

impl Game {
    const BIG_GAME_MAX_CUP: u32 = 1_000_000;

    fn new(mut cups: Vec<u32>, big_game: bool) -> Self {
        let current_cup = cups[0];
        let min_cup = *cups.iter().min().unwrap();
        let mut max_cup = *cups.iter().max().unwrap();

        if big_game {
            cups.reserve(Self::BIG_GAME_MAX_CUP as usize - cups.len());
            let mut cup = max_cup + 1;
            while cup <= Self::BIG_GAME_MAX_CUP {
                cups.push(cup);
                cup += 1;
            }
            assert_eq!(cups.len(), Self::BIG_GAME_MAX_CUP as usize);
            max_cup = Self::BIG_GAME_MAX_CUP;
        } else {
        }

        Game {
            cups,
            current_cup,
            min_cup,
            max_cup,
        }
    }

    fn parse(input: &str, big_game: bool) -> Self {
        Game::new(
            input
                .trim_end()
                .chars()
                .map(|ch| (ch as u8 - b'0') as u32)
                .collect(),
            big_game,
        )
    }

    fn do_move(&mut self) {
        // - The crab picks up the three cups that are immediately clockwise of the
        //   current cup. They are removed from the circle; cup spacing is adjusted
        //   as necessary to maintain the circle.
        let mut picked_up_cups = [0; 3];
        let mut picked_up_idxs = [0; 3];
        let mut picked_up_idx = {
            let current_cup_idx = self
                .cups
                .iter()
                .position(|cup| *cup == self.current_cup)
                .unwrap();
            if current_cup_idx == self.cups.len() - 1 {
                0
            } else {
                current_cup_idx + 1
            }
        };
        for i in 0..3 {
            picked_up_cups[i] = self.cups[picked_up_idx];
            picked_up_idxs[i] = picked_up_idx;
            picked_up_idx += 1;
            if picked_up_idx == self.cups.len() {
                picked_up_idx = 0;
            }
        }
        picked_up_idxs.sort();
        for idx in picked_up_idxs.iter().rev() {
            self.cups.remove(*idx);
        }

        // - The crab selects a destination cup: the cup with a label equal to the
        //   current cup's label minus one. If this would select one of the cups that
        //   was just picked up, the crab will keep subtracting one until it finds a cup
        //   that wasn't just picked up. If at any point in this process the value goes
        //   below the lowest value on any cup's label, it wraps around to the highest
        //   value on any cup's label instead.
        let mut destination_cup = {
            if self.current_cup == self.min_cup {
                self.max_cup
            } else {
                self.current_cup - 1
            }
        };
        while picked_up_cups.contains(&destination_cup) {
            if destination_cup == self.min_cup {
                destination_cup = self.max_cup
            } else {
                destination_cup -= 1;
            }
        }
        let destination_cup_idx = {
            let i = self
                .cups
                .iter()
                .position(|cup| *cup == destination_cup)
                .unwrap();
            if i == self.cups.len() - 1 {
                0
            } else {
                i + 1
            }
        };

        // - The crab places the cups it just picked up so that they are immediately
        //   clockwise of the destination cup. They keep the same order as when they
        //   were picked up.
        let mut dst_idx = destination_cup_idx;
        for i in 0..3 {
            self.cups.insert(dst_idx, picked_up_cups[i]);
            dst_idx += 1;
            if dst_idx == self.cups.len() {
                dst_idx = 0;
            }
        }

        // - The crab selects a new current cup: the cup which is immediately clockwise
        //   of the current cup.
        let current_cup_idx = self
            .cups
            .iter()
            .position(|cup| *cup == self.current_cup)
            .unwrap();
        self.current_cup = if current_cup_idx == self.cups.len() - 1 {
            self.cups[0]
        } else {
            self.cups[current_cup_idx + 1]
        };
    }

    fn order(&self) -> u32 {
        let cup_one_idx = self.cups.iter().position(|cup| *cup == 1).unwrap();
        let mut digits = String::new();
        let mut i = cup_one_idx + 1;
        while digits.len() < self.cups.len() - 1 {
            digits.push_str(&self.cups[i].to_string());
            i += 1;
            if i == self.cups.len() {
                i = 0;
            }
        }
        digits.parse::<u32>().unwrap()
    }

    fn two_cups_after_cup_one(&self) -> (u32, u32) {
        let cup_one_idx = self.cups.iter().position(|cup| *cup == 1).unwrap();
        let next_idx = if cup_one_idx == self.cups.len() {
            0
        } else {
            cup_one_idx + 1
        };
        let next_next_idx = if next_idx == self.cups.len() {
            0
        } else {
            next_idx + 1
        };
        (self.cups[next_idx], self.cups[next_next_idx])
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cup in self.cups.iter() {
            if *cup == self.current_cup {
                write!(f, "({}) ", cup)?;
            } else {
                write!(f, "{}  ", cup)?;
            }
        }
        Ok(())
    }
}

#[test]
fn test_parse() {
    let input = "32415\n";
    let game = Game::parse(input, false);
    assert_eq!(
        game,
        Game {
            cups: vec![3, 2, 4, 1, 5],
            current_cup: 3,
            min_cup: 1,
            max_cup: 5,
        }
    );
}

#[test]
fn test_parse_big_game() {
    let input = "54321\n";
    let game = Game::parse(input, true);
    assert_eq!(game.cups.len(), 1_000_000);
    assert_eq!(game.cups[0..5], vec![5, 4, 3, 2, 1]);
    assert_eq!(game.current_cup, 5);
    assert_eq!(game.min_cup, 1);
    assert_eq!(game.max_cup, 1_000_000);
}

#[test]
fn test_order() {
    let game = Game {
        cups: vec![5, 8, 3, 7, 4, 1, 9, 2, 6],
        current_cup: 8,
        min_cup: 1,
        max_cup: 9,
    };
    assert_eq!(game.order(), 92658374);
}

#[test]
fn test_do_move() {
    let mut game = Game::parse("389125467", false);

    game.do_move();
    assert_eq!(game.order(), 54673289);

    for _ in 0..9 {
        game.do_move();
    }
    assert_eq!(game.order(), 92658374);

    for _ in 0..90 {
        game.do_move();
    }
    assert_eq!(game.order(), 67384529);
}

#[test]
#[ignore]
fn test_do_move_big_game() {
    let mut game = Game::parse("389125467", true);
    for _ in 0..10_000_000 {
        game.do_move();
    }
    assert_eq!(game.two_cups_after_cup_one(), (934001, 159792));
}
