pub mod five;
pub mod four;
pub mod one;
pub mod seven;
pub mod six;
pub mod three;
pub mod two;

pub struct Day {
    pub part1: Option<fn(data: Vec<String>) -> u64>,
    pub part2: Option<fn(data: Vec<String>) -> u64>,
}

pub fn get_day_from_str(day: &str) -> Option<u8> {
    match day {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        _ => None,
    }
}

pub fn get_day_str(day: u8) -> Option<String> {
    match day {
        1 => Some("one".to_string()),
        2 => Some("two".to_string()),
        3 => Some("three".to_string()),
        4 => Some("four".to_string()),
        5 => Some("five".to_string()),
        6 => Some("six".to_string()),
        7 => Some("seven".to_string()),
        _ => None,
    }
}

pub fn get_day(day: u8) -> Option<Day> {
    match day {
        1 => Some(one::functions()),
        2 => Some(two::functions()),
        3 => Some(three::functions()),
        4 => Some(four::functions()),
        5 => Some(five::functions()),
        6 => Some(six::functions()),
        7 => Some(seven::functions()),
        _ => None,
    }
}
