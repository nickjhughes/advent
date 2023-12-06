use std::fs;

pub fn part1() -> String {
    let input = get_input_file_contents();
    let races = parse_races(&input);
    product_of_ways_to_win(&races).to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    let race = parse_race_ignore_whitespace(&input);
    race.ways_to_win().to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input06").expect("Failed to open input file")
}

fn product_of_ways_to_win(races: &[Race]) -> u64 {
    races.iter().map(|r| r.ways_to_win()).product()
}

#[derive(Debug, PartialEq)]
struct Race {
    /// Time allowed in milliseconds.
    time_allowed: u64,
    /// Record distance in millimeters.
    record_distance: u64,
}

impl Race {
    #[allow(dead_code)]
    fn simulate(&self, button_hold_time: u64) -> u64 {
        assert!(button_hold_time <= self.time_allowed);
        (self.time_allowed - button_hold_time) * button_hold_time
    }

    fn ways_to_win(&self) -> u64 {
        let a = -1.0;
        let b = self.time_allowed as f64;
        let c = -(self.record_distance as f64 + 0.0001);
        let mut roots = (
            (-b + (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a),
            (-b - (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a),
        );
        roots.0 = roots.0.clamp(0.0, self.time_allowed as f64);
        roots.1 = roots.1.clamp(0.0, self.time_allowed as f64);
        (if roots.1 > roots.0 {
            roots.1.floor() - roots.0.ceil() + 1.0
        } else {
            roots.0.floor() - roots.1.ceil() + 1.0
        }) as u64
    }
}

fn parse_races(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let time_line = lines.next().unwrap();
    let distance_line = lines.next().unwrap();
    time_line
        .split_whitespace()
        .skip(1)
        .zip(distance_line.split_whitespace().skip(1))
        .map(|(time_str, dist_str)| Race {
            time_allowed: time_str.parse::<u64>().unwrap(),
            record_distance: dist_str.parse::<u64>().unwrap(),
        })
        .collect()
}

fn parse_race_ignore_whitespace(input: &str) -> Race {
    let mut lines = input.lines();
    let time_line = lines.next().unwrap();
    let distance_line = lines.next().unwrap();

    let (_, time_str) = time_line.split_once(':').unwrap();
    let time = time_str.replace(' ', "").parse::<u64>().unwrap();

    let (_, dist_str) = distance_line.split_once(':').unwrap();
    let dist = dist_str.replace(' ', "").parse::<u64>().unwrap();

    Race {
        time_allowed: time,
        record_distance: dist,
    }
}

#[test]
fn test_parse_races() {
    let input = "Time:      7  15   30\nDistance:  9  40  200\n";
    let races = parse_races(input);
    assert_eq!(
        races,
        vec![
            Race {
                time_allowed: 7,
                record_distance: 9
            },
            Race {
                time_allowed: 15,
                record_distance: 40
            },
            Race {
                time_allowed: 30,
                record_distance: 200
            },
        ]
    );
}

#[test]
fn test_parse_race_ignore_whitespace() {
    let input = "Time:      7  15   30\nDistance:  9  40  200\n";
    let races = parse_race_ignore_whitespace(input);
    assert_eq!(
        races,
        Race {
            time_allowed: 71530,
            record_distance: 940200
        }
    );
}

#[test]
fn test_simulate() {
    let race = Race {
        time_allowed: 7,
        record_distance: 9,
    };
    assert_eq!(race.simulate(0), 0);
    assert_eq!(race.simulate(1), 6);
    assert_eq!(race.simulate(2), 10);
    assert_eq!(race.simulate(3), 12);
    assert_eq!(race.simulate(4), 12);
    assert_eq!(race.simulate(5), 10);
    assert_eq!(race.simulate(6), 6);
    assert_eq!(race.simulate(7), 0);
}

#[test]
fn test_ways_to_win() {
    let race = Race {
        time_allowed: 7,
        record_distance: 9,
    };
    assert_eq!(race.ways_to_win(), 4);
}

#[test]
fn test_product_of_ways_to_win() {
    let input = "Time:      7  15   30\nDistance:  9  40  200\n";
    let races = parse_races(input);
    assert_eq!(product_of_ways_to_win(&races), 288);
}
