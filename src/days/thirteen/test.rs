#[cfg(test)]
use crate::days;
#[cfg(test)]
use crate::util;
#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data() -> Vec<String> {
        let data_path = PathBuf::from(format!("src/days/{}/data/test.txt", "thirteen"));
        util::files::read_file_line_by_line(data_path)
    }

    #[test]
    fn p1() {
        if let Some(day) = days::get_day_from_str("thirteen") {
            if let Some(parsed_fns) = days::get_day(day) {
                if let Some(part1_fn) = parsed_fns.part1 {
                    let result = part1_fn(get_data());
                    assert_eq!(result, 480);
                    return;
                }

                panic!("Part 1 is not supported.");
            } else {
                panic!("Part 1 is not supported.");
            }
        } else {
            panic!("Day {} is not supported.", "thirteen");
        }
    }

    #[ignore]
    #[test]
    fn p2() {
        if let Some(day) = days::get_day_from_str("thirteen") {
            if let Some(parsed_fns) = days::get_day(day) {
                if let Some(part2_fn) = parsed_fns.part2 {
                    let result = part2_fn(get_data());
                    assert_eq!(result, 1);
                    return;
                }

                panic!("Part 2 is not supported.");
            } else {
                panic!("Part 2 is not supported.");
            }
        } else {
            panic!("Day {} is not supported.", "thirteen");
        }
    }
}
