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

pub fn get_day_functions(day: u8) -> DayFunctions {
    match day {
        22 => day_functions!(day_22),
        24 => day_functions!(day_24),
        _ => panic!("Code for day not found"),
    }
}
