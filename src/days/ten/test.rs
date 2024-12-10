#[cfg(test)]
use crate::days;
#[cfg(test)]
use crate::days::ten;
#[cfg(test)]
use crate::util;
#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data() -> Vec<String> {
        let data_path = PathBuf::from(format!("src/days/{}/data/test.txt", "ten"));
        util::files::read_file_line_by_line(data_path)
    }

    #[test]
    fn p1_mini() {
        let map = vec![
            "0123".to_string(),
            "1234".to_string(),
            "8765".to_string(),
            "9876".to_string(),
        ];
        let result = ten::main::part1(map);
        assert_eq!(result, 1);
    }

    #[test]
    fn p1_mini_2() {
        let map = vec![
            "8880888".to_string(),
            "8881888".to_string(),
            "8882888".to_string(),
            "6543456".to_string(),
            "7888887".to_string(),
            "8888888".to_string(),
            "9888889".to_string(),
        ];
        let result = ten::main::part1(map);
        assert_eq!(result, 2);
    }

    #[test]
    fn p1_mini_3() {
        let map = vec![
            "1190119".to_string(),
            "1111198".to_string(),
            "1112117".to_string(),
            "6543456".to_string(),
            "7651987".to_string(),
            "8761111".to_string(),
            "9871111".to_string(),
        ];
        let result = ten::main::part1(map);
        assert_eq!(result, 4);
    }

    #[test]
    fn can_navigate() {
        let start = (0, 0);
        let end = (0, 1);
        let res = ten::main::can_navigate(&start, &end);
        assert_eq!(res, true)
    }

    #[test]
    fn can_navigate_false() {
        let start = (0, 0);
        let end = (1, 2);
        let res = ten::main::can_navigate(&start, &end);
        assert_eq!(res, false)
    }

    #[test]
    fn p1() {
        if let Some(day) = days::get_day_from_str("ten") {
            if let Some(parsed_fns) = days::get_day(day) {
                if let Some(part1_fn) = parsed_fns.part1 {
                    let result = part1_fn(get_data());
                    assert_eq!(result, 36);
                    return;
                }

                panic!("Part 1 is not supported.");
            } else {
                panic!("Part 1 is not supported.");
            }
        } else {
            panic!("Day {} is not supported.", "ten");
        }
    }

    #[test]
    fn p2() {
        if let Some(day) = days::get_day_from_str("ten") {
            if let Some(parsed_fns) = days::get_day(day) {
                if let Some(part2_fn) = parsed_fns.part2 {
                    let result = part2_fn(get_data());
                    assert_eq!(result, 81);
                    return;
                }

                panic!("Part 2 is not supported.");
            } else {
                panic!("Part 2 is not supported.");
            }
        } else {
            panic!("Day {} is not supported.", "ten");
        }
    }
}
