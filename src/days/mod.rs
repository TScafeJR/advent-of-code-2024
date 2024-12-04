pub mod one;

pub struct Day {
    pub part1: Option<fn(data: Vec<String>) -> ()>,
    pub part2: Option<fn(data: Vec<String>) -> ()>
}

pub fn get_day(day: u8) -> Option<Day> {
    match day {
        1 => Some(one::functions()),
        _ => None,
    }
}
