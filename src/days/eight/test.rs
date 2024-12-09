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
        let data_path = PathBuf::from(format!("src/days/{}/data/test.txt", "eight"));
        util::files::read_file_line_by_line(data_path)
    }

    #[ignore]
    #[test]
    fn p1() {
        if let Some(day) = days::get_day_from_str("eight") {
            if let Some(parsed_fns) = days::get_day(day) {
                if let Some(part1_fn) = parsed_fns.part1 {
                    let result = part1_fn(get_data());
                    assert_eq!(result, 14);
                    return;
                }

                panic!("Part 1 is not supported.");
            } else {
                panic!("Part 1 is not supported.");
            }
        } else {
            panic!("Day {} is not supported.", "eight");
        }
    }

    #[test]
    fn x1_gt_x2_p1() {
        let data = vec![
            "....".to_string(),
            ".A..".to_string(),
            "..A.".to_string(),
            "....".to_string(),
        ];

        if let Some(day) = days::get_day_from_str("eight") {
            if let Some(parsed_fns) = days::get_day(day) {
                if let Some(part1_fn) = parsed_fns.part1 {
                    let result = part1_fn(data);
                    assert_eq!(result, 2);
                    return;
                }

                panic!("Part 1 is not supported.");
            } else {
                panic!("Part 1 is not supported.");
            }
        } else {
            panic!("Day {} is not supported.", "eight");
        }
    }

    #[test]
    fn x1_lt_x2_p1() {
        let data = vec![
            "....".to_string(),
            ".A..".to_string(),
            "..A.".to_string(),
            "....".to_string(),
        ];

        if let Some(day) = days::get_day_from_str("eight") {
            if let Some(parsed_fns) = days::get_day(day) {
                if let Some(part1_fn) = parsed_fns.part1 {
                    let result = part1_fn(data);
                    assert_eq!(result, 2);
                    return;
                }

                panic!("Part 1 is not supported.");
            } else {
                panic!("Part 1 is not supported.");
            }
        } else {
            panic!("Day {} is not supported.", "eight");
        }
    }

    #[test]
    fn x1_ob() {
        let data = vec![
            "A...".to_string(),
            ".A..".to_string(),
            "....".to_string(),
            "....".to_string(),
        ];

        if let Some(day) = days::get_day_from_str("eight") {
            if let Some(parsed_fns) = days::get_day(day) {
                if let Some(part1_fn) = parsed_fns.part1 {
                    let result = part1_fn(data);
                    assert_eq!(result, 1);
                    return;
                }

                panic!("Part 1 is not supported.");
            } else {
                panic!("Part 1 is not supported.");
            }
        } else {
            panic!("Day {} is not supported.", "eight");
        }
    }

    #[test]
    fn x2_ob() {
        let data = vec![
            "....".to_string(),
            "....".to_string(),
            "..A.".to_string(),
            "...A".to_string(),
        ];

        if let Some(day) = days::get_day_from_str("eight") {
            if let Some(parsed_fns) = days::get_day(day) {
                if let Some(part1_fn) = parsed_fns.part1 {
                    let result = part1_fn(data);
                    assert_eq!(result, 1);
                    return;
                }

                panic!("Part 1 is not supported.");
            } else {
                panic!("Part 1 is not supported.");
            }
        } else {
            panic!("Day {} is not supported.", "eight");
        }
    }

    #[test]
    fn x1_eq_x2_p1() {
        let data = vec![
            "....".to_string(),
            ".A..".to_string(),
            ".A..".to_string(),
            "....".to_string(),
        ];

        if let Some(day) = days::get_day_from_str("eight") {
            if let Some(parsed_fns) = days::get_day(day) {
                if let Some(part1_fn) = parsed_fns.part1 {
                    let result = part1_fn(data);
                    assert_eq!(result, 2);
                    return;
                }

                panic!("Part 1 is not supported.");
            } else {
                panic!("Part 1 is not supported.");
            }
        } else {
            panic!("Day {} is not supported.", "eight");
        }
    }

    #[test]
    fn y1_eq_y2_p1() {
        let data = vec![
            "......".to_string(),
            "..AA..".to_string(),
            "......".to_string(),
            "......".to_string(),
        ];

        if let Some(day) = days::get_day_from_str("eight") {
            if let Some(parsed_fns) = days::get_day(day) {
                if let Some(part1_fn) = parsed_fns.part1 {
                    let result = part1_fn(data);
                    assert_eq!(result, 2);
                    return;
                }

                panic!("Part 1 is not supported.");
            } else {
                panic!("Part 1 is not supported.");
            }
        } else {
            panic!("Day {} is not supported.", "eight");
        }
    }

    #[ignore]
    #[test]
    fn p2() {
        if let Some(day) = days::get_day_from_str("eight") {
            if let Some(parsed_fns) = days::get_day(day) {
                if let Some(part2_fn) = parsed_fns.part2 {
                    let result = part2_fn(get_data());
                    assert_eq!(result, 31);
                    return;
                }

                panic!("Part 2 is not supported.");
            } else {
                panic!("Part 2 is not supported.");
            }
        } else {
            panic!("Day {} is not supported.", "eight");
        }
    }
}
