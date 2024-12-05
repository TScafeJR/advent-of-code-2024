use crate::days::Day;
use crate::util::arrays;
use regex::Regex;

enum Direction {
    FORWARD,
    BACKWARD,
}

fn get_xmas_regex(direction: Direction) -> Regex {
    match direction {
        Direction::FORWARD => Regex::new(r"XMAS").unwrap(),
        Direction::BACKWARD => Regex::new(r"SAMX").unwrap(),
    }
}

fn get_mas_regex(direction: Direction) -> Regex {
    match direction {
        Direction::FORWARD => Regex::new(r"MAS").unwrap(),
        Direction::BACKWARD => Regex::new(r"SAM").unwrap(),
    }
}

fn horizontal_scan(row: &String, re_fn: fn(d: Direction) -> Regex) -> u32 {
    let mut res = 0;

    let forward_re = re_fn(Direction::FORWARD);
    let backward_re = re_fn(Direction::BACKWARD);

    for _ in forward_re.captures_iter(&row) {
        res += 1;
    }

    for _ in backward_re.captures_iter(&row) {
        res += 1;
    }

    return res;
}

fn vertical_count(data: &Vec<String>) -> u32 {
    let mut res = 0;
    let two_d_array: Vec<Vec<char>> = data.iter().map(|x| x.chars().collect()).collect();

    let rotated_arr = arrays::rotate_90_clockwise(two_d_array);

    for row in rotated_arr {
        let row_str: String = row.into_iter().collect();
        res += horizontal_scan(&row_str, get_xmas_regex);
    }

    return res;
}

fn diagonal_count(data: &Vec<String>, re_fn: fn(d: Direction) -> Regex) -> u32 {
    let mut res = 0;
    let two_d_array: Vec<Vec<char>> = data.iter().map(|x| x.chars().collect()).collect();

    // count forwards
    for i in 0..two_d_array.len() - 3 {
        for j in 0..two_d_array.len() - 3 {
            let mut row = vec![];
            row.push(two_d_array[i][j]);
            row.push(two_d_array[i + 1][j + 1]);
            row.push(two_d_array[i + 2][j + 2]);
            row.push(two_d_array[i + 3][j + 3]);

            // Convert row into a string and analyze it
            let row_str: String = row.into_iter().collect();
            res += horizontal_scan(&row_str, re_fn);
        }
    }

    // count backwards
    for i in 0..two_d_array.len() - 3 {
        for j in 3..two_d_array.len() {
            let mut row = vec![];
            row.push(two_d_array[i][j]);
            row.push(two_d_array[i + 1][j - 1]);
            row.push(two_d_array[i + 2][j - 2]);
            row.push(two_d_array[i + 3][j - 3]);

            // Convert row into a string and analyze it
            let row_str: String = row.into_iter().collect();
            res += horizontal_scan(&row_str, re_fn);
        }
    }

    return res;
}

fn special_diagonal_count(data: &Vec<String>, re_fn: fn(d: Direction) -> Regex) -> u32 {
    let mut res = 0;
    let two_d_array: Vec<Vec<char>> = data.iter().map(|x| x.chars().collect()).collect();

    // count forwards
    for i in 1..two_d_array.len() - 1 {
        for j in 1..two_d_array.len() - 1 {
            let mut inner = 0;
            let mut f_row = vec![];
            f_row.push(two_d_array[i - 1][j - 1]);
            f_row.push(two_d_array[i][j]);
            f_row.push(two_d_array[i + 1][j + 1]);

            // Convert row into a string and analyze it
            let f_row_str: String = f_row.into_iter().collect();
            inner += horizontal_scan(&f_row_str, re_fn);

            if inner != 1 {
                continue;
            }

            let mut b_row = vec![];
            b_row.push(two_d_array[i + 1][j - 1]);
            b_row.push(two_d_array[i][j]);
            b_row.push(two_d_array[i - 1][j + 1]);

            // Convert row into a string and analyze it
            let b_row_str: String = b_row.into_iter().collect();
            res += horizontal_scan(&b_row_str, re_fn);
        }
    }

    return res;
}

fn part1(data: Vec<String>) -> () {
    let mut result = 0;

    for line in &data {
        result += horizontal_scan(&line, get_xmas_regex);
    }

    result += vertical_count(&data);

    result += diagonal_count(&data, get_xmas_regex);

    println!("day 4, part 1: {:?}", result);
}

fn part2(data: Vec<String>) -> () {
    let mut result = 0;
    result += special_diagonal_count(&data, get_mas_regex);
    println!("day 4, part 2: {:?}", result);
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
