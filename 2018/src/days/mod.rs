pub mod day_01;
pub mod day_02;

pub struct DayFunctions {
    pub part1: fn() -> String,
    pub part2: fn() -> String,
}

pub fn get_day_functions(day: u8) -> DayFunctions {
    match day {
        1 => DayFunctions {
            part1: day_01::part1,
            part2: day_01::part2,
        },
        2 => DayFunctions {
            part1: day_02::part1,
            part2: day_02::part2,
        },
        _ => panic!("Code for day not found"),
    }
}
