automod::dir!("src/days");

macro_rules! day_functions {
    ($day:ident) => {
        DayFunctions {
            part1: $day::part1,
            part2: $day::part2,
        }
    };
}

pub struct DayFunctions {
    pub part1: fn() -> String,
    pub part2: fn() -> String,
}

pub fn get_day_functions(day: u8) -> Option<DayFunctions> {
    match day {
        1 => Some(day_functions!(day_01)),
        // 2 => Some(day_functions!(day_02)),
        // 3 => Some(day_functions!(day_03)),
        // 4 => Some(day_functions!(day_04)),
        // 5 => Some(day_functions!(day_05)),
        // 6 => Some(day_functions!(day_06)),
        // 7 => Some(day_functions!(day_07)),
        // 8 => Some(day_functions!(day_08)),
        // 9 => Some(day_functions!(day_09)),
        // 10 => Some(day_functions!(day_10)),
        // 11 => Some(day_functions!(day_11)),
        // 12 => Some(day_functions!(day_12)),
        // 13 => Some(day_functions!(day_13)),
        // 14 => Some(day_functions!(day_14)),
        // 15 => Some(day_functions!(day_15)),
        // 16 => Some(day_functions!(day_16)),
        // 17 => Some(day_functions!(day_17)),
        // 18 => Some(day_functions!(day_18)),
        // 19 => Some(day_functions!(day_19)),
        // 20 => Some(day_functions!(day_20)),
        // 21 => Some(day_functions!(day_21)),
        // 22 => Some(day_functions!(day_22)),
        // 23 => Some(day_functions!(day_23)),
        // 24 => Some(day_functions!(day_24)),
        // 25 => Some(day_functions!(day_25)),
        _ => None,
    }
}
