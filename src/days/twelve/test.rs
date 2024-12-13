#[cfg(test)]
use crate::days;
#[cfg(test)]
use crate::days::twelve::main;
#[cfg(test)]
use crate::util;
#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data() -> Vec<String> {
        let data_path = PathBuf::from(format!("src/days/{}/data/test.txt", "twelve"));
        util::files::read_file_line_by_line(data_path)
    }

    #[test]
    fn calculate_perimeter_test() {
        let points = vec![(0, 1), (0, 2)];

        let calculated_perimeter = main::calculate_perimeter(&points);

        assert_eq!(calculated_perimeter, 6);
    }

    #[test]
    fn calculate_perimeter_larger_case() {
        let points = vec![(0, 0), (0, 1), (1, 0), (1, 1)];
        let calculated_perimeter = main::calculate_perimeter(&points);
        assert_eq!(calculated_perimeter, 8);
    }

    #[test]
    fn calculate_perimeter_single_square() {
        let points = vec![(2, 3)];
        let calculated_perimeter = main::calculate_perimeter(&points);
        assert_eq!(calculated_perimeter, 4);
    }

    #[test]
    fn calculate_sides_single_square() {
        let points = vec![(2, 3)];
        let calculated_perimeter = main::calculate_sides(&points);
        assert_eq!(calculated_perimeter, 4);
    }

    #[test]
    fn calculate_sides_large_square() {
        let points = vec![(0, 0), (0, 1), (1, 0), (1, 1)];
        let calculated_perimeter = main::calculate_sides(&points);
        assert_eq!(calculated_perimeter, 4);
    }

    #[test]
    fn calculate_sides_special() {
        let points = vec![(0, 0), (1, 0), (1, 1), (2, 1)];
        let calculated_perimeter = main::calculate_sides(&points);
        assert_eq!(calculated_perimeter, 8);
    }

    #[test]
    fn p1() {
        if let Some(day) = days::get_day_from_str("twelve") {
            if let Some(parsed_fns) = days::get_day(day) {
                if let Some(part1_fn) = parsed_fns.part1 {
                    let result = part1_fn(get_data());
                    assert_eq!(result, 1930);
                    return;
                }

                panic!("Part 1 is not supported.");
            } else {
                panic!("Part 1 is not supported.");
            }
        } else {
            panic!("Day {} is not supported.", "twelve");
        }
    }

    #[test]
    fn p2() {
        if let Some(day) = days::get_day_from_str("twelve") {
            if let Some(parsed_fns) = days::get_day(day) {
                if let Some(part2_fn) = parsed_fns.part2 {
                    let result = part2_fn(get_data());
                    assert_eq!(result, 1206);
                    return;
                }

                panic!("Part 2 is not supported.");
            } else {
                panic!("Part 2 is not supported.");
            }
        } else {
            panic!("Day {} is not supported.", "twelve");
        }
    }

    #[test]
    fn p2_extra() {
        if let Some(day) = days::get_day_from_str("twelve") {
            if let Some(parsed_fns) = days::get_day(day) {
                if let Some(part2_fn) = parsed_fns.part2 {
                    let data = vec![
                        "AAAAAA".to_string(),
                        "AAABBA".to_string(),
                        "AAABBA".to_string(),
                        "ABBAAA".to_string(),
                        "ABBAAA".to_string(),
                        "AAAAAA".to_string(),
                    ];
                    let result = part2_fn(data);
                    assert_eq!(result, 368);
                    return;
                }

                panic!("Part 2 is not supported.");
            } else {
                panic!("Part 2 is not supported.");
            }
        } else {
            panic!("Day {} is not supported.", "twelve");
        }
    }
}
