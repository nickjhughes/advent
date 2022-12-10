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
        2 => Some(day_functions!(day_02)),
        3 => Some(day_functions!(day_03)),
        4 => Some(day_functions!(day_04)),
        5 => Some(day_functions!(day_05)),
        6 => Some(day_functions!(day_06)),
        7 => Some(day_functions!(day_07)),
        8 => Some(day_functions!(day_08)),
        9 => Some(day_functions!(day_09)),
        10 => Some(day_functions!(day_10)),
        _ => None,
    }
}
