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
        10 => Some(day_functions!(day_10)),
        _ => None,
    }
}
