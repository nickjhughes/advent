use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

#[derive(Debug, PartialEq, Eq)]
struct Round {
    opponent_choice: Choice,
    my_choice: Choice,
    outcome: Outcome,
}

impl Round {
    pub fn new_with_my_choice(opponent_choice: Choice, my_choice: Choice) -> Self {
        let outcome = match opponent_choice {
            Choice::Rock => match my_choice {
                Choice::Rock => Outcome::Draw,
                Choice::Paper => Outcome::Win,
                Choice::Scissors => Outcome::Loss,
            },
            Choice::Paper => match my_choice {
                Choice::Rock => Outcome::Loss,
                Choice::Paper => Outcome::Draw,
                Choice::Scissors => Outcome::Win,
            },
            Choice::Scissors => match my_choice {
                Choice::Rock => Outcome::Win,
                Choice::Paper => Outcome::Loss,
                Choice::Scissors => Outcome::Draw,
            },
        };
        Self {
            opponent_choice,
            my_choice,
            outcome,
        }
    }

    pub fn new_with_desired_outcome(opponent_choice: Choice, desired_outcome: Outcome) -> Self {
        let my_choice = match opponent_choice {
            Choice::Rock => match desired_outcome {
                Outcome::Loss => Choice::Scissors,
                Outcome::Draw => Choice::Rock,
                Outcome::Win => Choice::Paper,
            },
            Choice::Paper => match desired_outcome {
                Outcome::Loss => Choice::Rock,
                Outcome::Draw => Choice::Paper,
                Outcome::Win => Choice::Scissors,
            },
            Choice::Scissors => match desired_outcome {
                Outcome::Loss => Choice::Paper,
                Outcome::Draw => Choice::Scissors,
                Outcome::Win => Choice::Rock,
            },
        };
        Self {
            opponent_choice,
            my_choice,
            outcome: desired_outcome,
        }
    }

    fn choice_score(&self) -> u32 {
        match self.my_choice {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn result_score(&self) -> u32 {
        match self.outcome {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }

    pub fn score(&self) -> u32 {
        self.choice_score() + self.result_score()
    }
}

pub fn part1() -> String {
    let contents = get_input_file_contents();
    let strategy = parse_strategy_choice(&contents);
    let score = strategy_score(&strategy);
    format!("{}", score).to_string()
}

pub fn part2() -> String {
    let contents = get_input_file_contents();
    let strategy = parse_strategy_outcome(&contents);
    let score = strategy_score(&strategy);
    format!("{}", score).to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input02").expect("Failed to open input file")
}

fn parse_strategy_choice(contents: &str) -> Vec<Round> {
    let mut strategy = Vec::new();
    for line in contents.split("\n") {
        if line.is_empty() {
            continue;
        }
        let choices = line.split(" ").collect::<Vec<&str>>();
        assert_eq!(choices.len(), 2);
        let opponent_choice = match choices[0] {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            _ => unreachable!(),
        };
        let my_choice = match choices[1] {
            "X" => Choice::Rock,
            "Y" => Choice::Paper,
            "Z" => Choice::Scissors,
            _ => unreachable!(),
        };
        strategy.push(Round::new_with_my_choice(opponent_choice, my_choice));
    }
    strategy
}

fn parse_strategy_outcome(contents: &str) -> Vec<Round> {
    let mut strategy = Vec::new();
    for line in contents.split("\n") {
        if line.is_empty() {
            continue;
        }
        let choices = line.split(" ").collect::<Vec<&str>>();
        assert_eq!(choices.len(), 2);
        let opponent_choice = match choices[0] {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            _ => unreachable!(),
        };
        let desired_outcome = match choices[1] {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => unreachable!(),
        };
        strategy.push(Round::new_with_desired_outcome(
            opponent_choice,
            desired_outcome,
        ));
    }
    strategy
}

fn strategy_score(strategy: &[Round]) -> u32 {
    let mut score = 0;
    for round in strategy.iter() {
        score += round.score();
    }
    score
}

#[test]
fn test_parse_choice() {
    let strategy = parse_strategy_choice("A Y\nB X\nC Z\n");
    assert_eq!(strategy.len(), 3);
    assert_eq!(strategy[0].opponent_choice, Choice::Rock);
    assert_eq!(strategy[1].opponent_choice, Choice::Paper);
    assert_eq!(strategy[2].opponent_choice, Choice::Scissors);
    assert_eq!(strategy[0].my_choice, Choice::Paper);
    assert_eq!(strategy[1].my_choice, Choice::Rock);
    assert_eq!(strategy[2].my_choice, Choice::Scissors);

    let strategy = parse_strategy_choice("");
    assert!(strategy.is_empty());

    let strategy = parse_strategy_choice("\n");
    assert!(strategy.is_empty());
}

#[test]
fn test_parse_outcome() {
    let strategy = parse_strategy_outcome("A Y\nB X\nC Z\n");
    assert_eq!(strategy.len(), 3);
    assert_eq!(strategy[0].opponent_choice, Choice::Rock);
    assert_eq!(strategy[1].opponent_choice, Choice::Paper);
    assert_eq!(strategy[2].opponent_choice, Choice::Scissors);
    assert_eq!(strategy[0].outcome, Outcome::Draw);
    assert_eq!(strategy[1].outcome, Outcome::Loss);
    assert_eq!(strategy[2].outcome, Outcome::Win);

    let strategy = parse_strategy_outcome("");
    assert!(strategy.is_empty());

    let strategy = parse_strategy_outcome("\n");
    assert!(strategy.is_empty());
}

#[test]
fn test_strategy_score_choice() {
    let strategy = parse_strategy_choice("A Y\nB X\nC Z\n");
    let score = strategy_score(&strategy);
    assert_eq!(score, 15);
}

#[test]
fn test_strategy_score_outcome() {
    let strategy = parse_strategy_outcome("A Y\nB X\nC Z\n");
    let score = strategy_score(&strategy);
    assert_eq!(score, 12);
}
