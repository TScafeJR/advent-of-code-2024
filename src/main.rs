mod days;
mod util;

use clap::Parser;
use days::{get_day, Day};
use std::path::PathBuf;

#[derive(Parser)]
struct Submission {
    day: u8,
    part: u8,
}

fn get_day_str(day: u8) -> Option<String> {
    match day {
        1 => Some("one".to_string()),
        2 => Some("two".to_string()),
        3 => Some("three".to_string()),
        _ => None,
    }
}

fn get_data(args: &Submission) -> Vec<String> {
    let parsed_day = get_day_str(args.day).unwrap_or_else(|| {
        panic!("Day {} is not supported.", args.day);
    });
    let data_path = PathBuf::from(format!(
        "src/days/{}/data/part{}.txt",
        parsed_day, args.part
    ));
    util::files::read_file_line_by_line(data_path)
}

fn get_functions(args: &Submission) -> Option<Day> {
    get_day(args.day)
}

fn main() {
    let args = Submission::parse();
    let data = get_data(&args);

    if let Some(parsed_fns) = get_functions(&args) {
        if args.part == 1 {
            if let Some(part1_fn) = parsed_fns.part1 {
                part1_fn(data.clone());
            } else {
                println!("part1 is not defined for day {}.", args.day);
            }
        }

        if args.part == 2 {
            if let Some(part2_fn) = parsed_fns.part2 {
                part2_fn(data.clone());
            } else {
                println!("part2 is not defined for day {}.", args.day);
            }
        }
    } else {
        println!("Day {} is not supported.", args.day);
    }
}
