pub mod one;
pub mod three;
pub mod two;

pub struct Day {
    pub part1: Option<fn(data: Vec<String>) -> ()>,
    pub part2: Option<fn(data: Vec<String>) -> ()>,
}

pub fn get_day(day: u8) -> Option<Day> {
    match day {
        1 => Some(one::functions()),
        2 => Some(two::functions()),
        3 => Some(three::functions()),
        _ => None,
    }
}
