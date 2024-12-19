#[cfg(test)]
use crate::days;
#[cfg(test)]
use crate::days::fourteen::main::{Board, Point, Robot, Velocity};
#[cfg(test)]
use crate::util;
#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data() -> Vec<String> {
        let data_path = PathBuf::from(format!("src/days/{}/data/test.txt", "fourteen"));
        util::files::read_file_line_by_line(data_path)
    }

    #[test]
    fn predict_final_location() {
        let board = Board(11, 7);
        let mut robot = Robot::new(Point(2, 4), Velocity(2, -3));
        robot.predict_end_position(&board, 5);

        assert_eq!(robot.end_position, Point(1, 3));
    }

    #[test]
    fn p1() {
        if let Some(day) = days::get_day_from_str("fourteen") {
            if let Some(parsed_fns) = days::get_day(day) {
                if let Some(part1_fn) = parsed_fns.part1 {
                    let result = part1_fn(get_data());
                    assert_eq!(result, 12);
                    return;
                }

                panic!("Part 1 is not supported.");
            } else {
                panic!("Part 1 is not supported.");
            }
        } else {
            panic!("Day {} is not supported.", "fourteen");
        }
    }
}
