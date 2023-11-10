use chrono::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() -> String {
    format!("{}", sleepiest_guard(None))
}

pub fn part2() -> String {
    "".to_string()
}

#[derive(Debug)]
enum Action {
    StartShift,
    WakeUp,
    FallAsleep,
}

#[derive(Debug)]
struct LogEntry {
    guard_id: u16,
    action: Action,
    timestamp: chrono::DateTime<Utc>,
}

fn get_log_lines() -> Vec<String> {
    let file = File::open("inputs/input04").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.expect("Failed to read line"));
    }
    lines
}

fn get_log_entries() -> Vec<LogEntry> {
    let mut lines = get_log_lines();
    lines.sort();
    let mut log_entries = Vec::new();
    let mut guard_id: Option<u16> = None;
    for line in &lines {
        let parts: Vec<&str> = line.split("] ").collect();
        if parts.len() != 2 {
            panic!("Failed to parse line");
        }
        let timestamp_string = (parts[0][1..].to_string() + ":00Z").replace(' ', "T");
        let timestamp = timestamp_string
            .parse::<DateTime<Utc>>()
            .expect("Failed to parse timestamp");
        let action = if parts[1] == "wakes up" {
            Action::WakeUp
        } else if parts[1] == "falls asleep" {
            Action::FallAsleep
        } else {
            let hash_pos = parts[1].find('#').expect("Failed to parse guard ID");
            let id_end = parts[1][(hash_pos + 1)..]
                .find(' ')
                .expect("Failed to parse guard ID");
            let id_string = parts[1][(hash_pos + 1)..(hash_pos + 1 + id_end)].to_string();
            guard_id = Some(
                id_string
                    .parse::<u16>()
                    .expect("Failed to parse guard ID to integer"),
            );
            Action::StartShift
        };
        if guard_id.is_none() {
            panic!("First entry isn't a shift start");
        }
        log_entries.push(LogEntry {
            guard_id: guard_id.unwrap(),
            action,
            timestamp,
        });
    }
    log_entries
}

fn sleepiest_guard(entries: Option<Vec<LogEntry>>) -> u32 {
    let entries = entries.unwrap_or_else(get_log_entries);

    let mut guard_sleep_duration: HashMap<u16, i64> = HashMap::new();
    let mut asleep_timestamp: Option<DateTime<Utc>> = None;
    for entry in &entries {
        match entry.action {
            Action::FallAsleep => {
                asleep_timestamp = Some(entry.timestamp);
            }
            Action::WakeUp => {
                if asleep_timestamp.is_none() {
                    panic!("Failed to parse log entries");
                }
                let sleep_duration = (entry.timestamp - asleep_timestamp.unwrap()).num_minutes();
                *guard_sleep_duration.entry(entry.guard_id).or_insert(0) += sleep_duration;
            }
            _ => {}
        }
    }

    let mut sleepiest_guard_id: Option<u16> = None;
    let mut longest_sleep_duration: i64 = 0;
    for (&guard_id, &sleep_duration) in &guard_sleep_duration {
        if sleep_duration > longest_sleep_duration {
            longest_sleep_duration = sleep_duration;
            sleepiest_guard_id = Some(guard_id);
        }
    }
    if sleepiest_guard_id.is_none() {
        panic!("Failed to find sleepiest guard");
    }

    let mut minute_sleepiness: HashMap<u8, u16> = HashMap::new();
    let mut asleep_timestamp: Option<DateTime<Utc>> = None;
    for entry in &entries {
        if entry.guard_id != sleepiest_guard_id.unwrap() {
            continue;
        }
        match entry.action {
            Action::FallAsleep => {
                asleep_timestamp = Some(entry.timestamp);
            }
            Action::WakeUp => {
                if asleep_timestamp.is_none() {
                    panic!("Failed to parse log entries");
                }
                for minute in asleep_timestamp.unwrap().minute()..entry.timestamp.minute() {
                    *minute_sleepiness.entry(minute as u8).or_insert(0) += 1;
                }
            }
            _ => {}
        }
    }

    let mut sleepiest_minute: Option<u8> = None;
    let mut most_sleepiness: u16 = 0;
    for (&minute, &sleepiness) in &minute_sleepiness {
        if sleepiness > most_sleepiness {
            most_sleepiness = sleepiness;
            sleepiest_minute = Some(minute);
        }
    }
    if sleepiest_minute.is_none() {
        panic!("Failed to find sleepiest guard's sleepiest minute")
    }
    (sleepiest_guard_id.unwrap() as u32) * (sleepiest_minute.unwrap() as u32)
}

#[allow(dead_code)]
fn sleepiest_minute_guard(_entries: Option<Vec<LogEntry>>) -> u32 {
    0
}

#[test]
fn part1_tests() {
    let entries = vec![
        LogEntry {
            guard_id: 10,
            action: Action::StartShift,
            timestamp: "1518-11-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 10,
            action: Action::FallAsleep,
            timestamp: "1518-11-01T00:05:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 10,
            action: Action::WakeUp,
            timestamp: "1518-11-01T00:25:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 10,
            action: Action::FallAsleep,
            timestamp: "1518-11-01T00:30:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 10,
            action: Action::WakeUp,
            timestamp: "1518-11-01T00:55:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 99,
            action: Action::StartShift,
            timestamp: "1518-11-01T23:58:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 99,
            action: Action::FallAsleep,
            timestamp: "1518-11-02T00:40:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 99,
            action: Action::WakeUp,
            timestamp: "1518-11-02T00:50:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 10,
            action: Action::StartShift,
            timestamp: "1518-11-03T00:05:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 10,
            action: Action::FallAsleep,
            timestamp: "1518-11-03T00:24:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 10,
            action: Action::WakeUp,
            timestamp: "1518-11-03T00:29:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 99,
            action: Action::StartShift,
            timestamp: "1518-11-04T00:02:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 99,
            action: Action::FallAsleep,
            timestamp: "1518-11-04T00:36:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 99,
            action: Action::WakeUp,
            timestamp: "1518-11-04T00:46:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 99,
            action: Action::StartShift,
            timestamp: "1518-11-05T00:03:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 99,
            action: Action::FallAsleep,
            timestamp: "1518-11-05T00:45:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 99,
            action: Action::WakeUp,
            timestamp: "1518-11-05T00:55:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
    ];
    assert_eq!(sleepiest_guard(Some(entries)), 240);
}

#[test]
fn part2_tests() {
    let entries = vec![
        LogEntry {
            guard_id: 10,
            action: Action::StartShift,
            timestamp: "1518-11-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 10,
            action: Action::FallAsleep,
            timestamp: "1518-11-01T00:05:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 10,
            action: Action::WakeUp,
            timestamp: "1518-11-01T00:25:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 10,
            action: Action::FallAsleep,
            timestamp: "1518-11-01T00:30:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 10,
            action: Action::WakeUp,
            timestamp: "1518-11-01T00:55:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 99,
            action: Action::StartShift,
            timestamp: "1518-11-01T23:58:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 99,
            action: Action::FallAsleep,
            timestamp: "1518-11-02T00:40:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 99,
            action: Action::WakeUp,
            timestamp: "1518-11-02T00:50:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 10,
            action: Action::StartShift,
            timestamp: "1518-11-03T00:05:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 10,
            action: Action::FallAsleep,
            timestamp: "1518-11-03T00:24:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 10,
            action: Action::WakeUp,
            timestamp: "1518-11-03T00:29:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 99,
            action: Action::StartShift,
            timestamp: "1518-11-04T00:02:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 99,
            action: Action::FallAsleep,
            timestamp: "1518-11-04T00:36:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 99,
            action: Action::WakeUp,
            timestamp: "1518-11-04T00:46:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 99,
            action: Action::StartShift,
            timestamp: "1518-11-05T00:03:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 99,
            action: Action::FallAsleep,
            timestamp: "1518-11-05T00:45:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
        LogEntry {
            guard_id: 99,
            action: Action::WakeUp,
            timestamp: "1518-11-05T00:55:00Z".parse::<DateTime<Utc>>().unwrap(),
        },
    ];
    assert_eq!(sleepiest_minute_guard(Some(entries)), 4455);
}
