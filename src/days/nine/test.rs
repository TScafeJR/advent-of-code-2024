#[cfg(test)]
use crate::days;
#[cfg(test)]
use crate::days::nine;
#[cfg(test)]
use crate::util;
#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data() -> Vec<String> {
        let data_path = PathBuf::from(format!("src/days/{}/data/test.txt", "nine"));
        util::files::read_file_line_by_line(data_path)
    }

    #[test]
    fn p1() {
        if let Some(day) = days::get_day_from_str("nine") {
            if let Some(parsed_fns) = days::get_day(day) {
                if let Some(part1_fn) = parsed_fns.part1 {
                    let result = part1_fn(get_data());
                    assert_eq!(result, 1928);
                    return;
                }

                panic!("Part 1 is not supported.");
            } else {
                panic!("Part 1 is not supported.");
            }
        } else {
            panic!("Day {} is not supported.", "nine");
        }
    }

    #[test]
    fn p2() {
        if let Some(day) = days::get_day_from_str("nine") {
            if let Some(parsed_fns) = days::get_day(day) {
                if let Some(part2_fn) = parsed_fns.part2 {
                    let result = part2_fn(get_data());
                    assert_eq!(result, 2858);
                    return;
                }

                panic!("Part 2 is not supported.");
            } else {
                panic!("Part 2 is not supported.");
            }
        } else {
            panic!("Day {} is not supported.", "nine");
        }
    }

    #[test]
    fn last_file_id() {
        let files = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = nine::main::find_last_file_id(files);
        assert_eq!(result, 9);
    }

    #[test]
    fn last_file_id_w_zeros() {
        let files = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 8, 0];
        let result = nine::main::find_last_file_id(files);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_scan_open_spaces() {
        let input = vec![0, 0, 0, 1, 1, 0, 0, 0, 0, 2, 2, 2, 2];
        let result = nine::main::scan_for_open_spaces(input, 1, 2);

        assert_eq!(result, vec![(1, 2), (5, 4)]);
    }

    #[test]
    fn test_scan_open_spaces_cplx() {
        let input = vec![
            0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 2, 0, 0, 0, 3, 3, 3, 0, 4, 4, 0, 5, 5, 5, 5, 0, 6, 6,
            6, 6, 0, 7, 7, 7, 0, 8, 8, 8, 8, 9, 9,
        ];
        let result = nine::main::scan_for_open_spaces(input, 2, 9);

        assert_eq!(
            result,
            vec![
                (2, 3),
                (8, 3),
                (12, 3),
                (18, 1),
                (21, 1),
                (26, 1),
                (31, 1),
                (35, 1)
            ]
        );
    }
}
