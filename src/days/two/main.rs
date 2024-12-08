use crate::days::Day;
use crate::util::arrays;
use regex::Regex;

#[derive(PartialEq, Debug)]
enum Direction {
    INCREASING,
    DECREASING,
    INVALID,
    INITIAL,
}

fn parse_row(row: &str) -> Vec<i64> {
    let mut row_vec: Vec<i64> = vec![];
    let re = Regex::new(r"(\d+)").unwrap();
    for cap in re.captures_iter(&row) {
        row_vec.push(cap[1].parse::<i64>().unwrap());
    }

    return row_vec;
}

fn get_direction(n1: i64, n2: i64) -> Direction {
    match n1.cmp(&n2) {
        std::cmp::Ordering::Less => Direction::INCREASING,
        std::cmp::Ordering::Greater => Direction::DECREASING,
        std::cmp::Ordering::Equal => Direction::INVALID,
    }
}

fn row_safe(row: &Vec<i64>) -> bool {
    let max_diff = 3;
    let mut curr = 0;
    let mut direction = Direction::INITIAL;

    while curr < row.len() - 1 {
        let n1 = row[curr];
        let n2 = row[curr + 1];

        let new_direction = get_direction(n1, n2);
        if direction == Direction::INITIAL {
            if new_direction == Direction::INVALID {
                return false;
            }
        } else {
            if new_direction == Direction::INVALID
                || direction == Direction::INVALID
                || direction != new_direction
            {
                return false;
            }
        }

        if (n1 - n2).abs() > max_diff {
            return false;
        }

        direction = new_direction;
        curr += 1;
    }

    return true;
}

fn part1(data: Vec<String>) -> u64 {
    let mut result = 0;

    for i in 0..data.len() {
        let row = parse_row(&data[i]);
        if row_safe(&row) {
            result += 1;
        }
    }

    return result;
}

fn part2(data: Vec<String>) -> u64 {
    let mut result = 0;

    for i in 0..data.len() {
        let row = parse_row(&data[i]);
        if row_safe(&row) {
            result += 1;
            continue;
        }

        for j in 0..row.len() {
            let row_copy = row.clone();
            let (_, deleted_row) = arrays::remove_element(row_copy, j);
            if row_safe(&deleted_row) {
                result += 1;
                break;
            }
        }
    }

    return result;
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
