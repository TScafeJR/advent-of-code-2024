#[cfg(test)]
use crate::days;
#[cfg(test)]
use crate::days::thirteen;
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
    fn test_prize_quick() {
        let prize = thirteen::main::Prize {
            button_a: (17, 86),
            button_b: (84, 37),
            prize: (7870, 6450),
        };

        let presses = thirteen::main::calculate_presses(&prize);
        assert_eq!(presses, 200);
    }

    #[test]
    fn test_prize() {
        let prize = thirteen::main::Prize {
            button_a: (94, 34),
            button_b: (22, 67),
            prize: (8400, 5400),
        };

        let presses = thirteen::main::calculate_presses(&prize);
        assert_eq!(presses, 280);
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
}
