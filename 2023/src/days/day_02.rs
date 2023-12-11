use std::fs;

pub fn part1() -> String {
    let input = get_input_file_contents();
    let games = parse_games(&input);
    possible_games_id_sum(&games, (12, 13, 14)).to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let games = parse_games(&input);
    powers_sum(&games).to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input02").expect("Failed to open input file")
}

fn parse_games(input: &str) -> Vec<Game> {
    input.lines().map(Game::parse).collect()
}

fn possible_games_id_sum(games: &[Game], counts: (u8, u8, u8)) -> usize {
    games
        .iter()
        .enumerate()
        .map(|(idx, game)| (idx + 1, game))
        .filter(|(_, game)| game.is_possible(counts))
        .map(|(id, _)| id)
        .sum::<usize>()
}

fn powers_sum(games: &[Game]) -> u32 {
    games.iter().map(|game| game.minimal_set_power()).sum()
}

#[derive(Debug, PartialEq)]
struct Game(Vec<(u8, u8, u8)>);

impl Game {
    fn parse(line: &str) -> Self {
        let (_, set_strs) = line.split_once(':').expect("no semi-colon in line");
        let mut sets = Vec::new();
        for set_str in set_strs.split(';') {
            let mut set = (0, 0, 0);
            for cubes_str in set_str.split(',') {
                let (count_str, color) = cubes_str.trim().split_once(' ').unwrap();
                let count = count_str.parse::<u8>().unwrap();
                match color {
                    "red" => set.0 = count,
                    "green" => set.1 = count,
                    "blue" => set.2 = count,
                    _ => panic!("invalid color {}", color),
                }
            }
            sets.push(set);
        }
        Game(sets)
    }

    fn is_possible(&self, counts: (u8, u8, u8)) -> bool {
        for set in self.0.iter() {
            if set.0 > counts.0 || set.1 > counts.1 || set.2 > counts.2 {
                return false;
            }
        }
        true
    }

    fn minimal_set(&self) -> (u8, u8, u8) {
        let mut minimal_set = (0, 0, 0);
        for set in self.0.iter() {
            if set.0 > minimal_set.0 {
                minimal_set.0 = set.0;
            }
            if set.1 > minimal_set.1 {
                minimal_set.1 = set.1;
            }
            if set.2 > minimal_set.2 {
                minimal_set.2 = set.2;
            }
        }
        minimal_set
    }

    fn minimal_set_power(&self) -> u32 {
        let minimal_set = self.minimal_set();
        minimal_set.0 as u32 * minimal_set.1 as u32 * minimal_set.2 as u32
    }
}

#[test]
fn test_parse_games() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n";
    let games = parse_games(input);
    assert_eq!(games.len(), 5);
    assert_eq!(
        games,
        vec![
            Game(vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]),
            Game(vec![(0, 2, 1), (1, 3, 4), (0, 1, 1)]),
            Game(vec![(20, 8, 6), (4, 13, 5), (1, 5, 0)]),
            Game(vec![(3, 1, 6), (6, 3, 0), (14, 3, 15)]),
            Game(vec![(6, 3, 1), (1, 2, 2)]),
        ]
    );
}

#[test]
fn test_is_possible() {
    assert!(
        Game(vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]).is_possible((12, 13, 14))
    );
    assert!(
        Game(vec![(0, 2, 1), (1, 3, 4), (0, 1, 1)]).is_possible((12, 13, 14))
    );
    assert!(
        !Game(vec![(20, 8, 6), (4, 13, 5), (1, 5, 0)]).is_possible((12, 13, 14))
    );
    assert!(
        !Game(vec![(3, 1, 6), (6, 3, 0), (14, 3, 15)]).is_possible((12, 13, 14))
    );
    assert!(
        Game(vec![(6, 3, 1), (1, 2, 2)]).is_possible((12, 13, 14))
    );
}

#[test]
fn test_possible_games_id_sum() {
    let games = vec![
        Game(vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]),
        Game(vec![(0, 2, 1), (1, 3, 4), (0, 1, 1)]),
        Game(vec![(20, 8, 6), (4, 13, 5), (1, 5, 0)]),
        Game(vec![(3, 1, 6), (6, 3, 0), (14, 3, 15)]),
        Game(vec![(6, 3, 1), (1, 2, 2)]),
    ];
    assert_eq!(possible_games_id_sum(&games, (12, 13, 14)), 8);
}

#[test]
fn test_minimal_set() {
    assert_eq!(
        Game(vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]).minimal_set(),
        (4, 2, 6)
    );
    assert_eq!(
        Game(vec![(0, 2, 1), (1, 3, 4), (0, 1, 1)]).minimal_set(),
        (1, 3, 4)
    );
    assert_eq!(
        Game(vec![(20, 8, 6), (4, 13, 5), (1, 5, 0)]).minimal_set(),
        (20, 13, 6)
    );
    assert_eq!(
        Game(vec![(3, 1, 6), (6, 3, 0), (14, 3, 15)]).minimal_set(),
        (14, 3, 15)
    );
    assert_eq!(Game(vec![(6, 3, 1), (1, 2, 2)]).minimal_set(), (6, 3, 2));
}

#[test]
fn test_powers_sum() {
    let games = vec![
        Game(vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]),
        Game(vec![(0, 2, 1), (1, 3, 4), (0, 1, 1)]),
        Game(vec![(20, 8, 6), (4, 13, 5), (1, 5, 0)]),
        Game(vec![(3, 1, 6), (6, 3, 0), (14, 3, 15)]),
        Game(vec![(6, 3, 1), (1, 2, 2)]),
    ];
    assert_eq!(powers_sum(&games), 2286);
}
