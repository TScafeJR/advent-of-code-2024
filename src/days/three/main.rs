use crate::days::Day;
use regex::Regex;

fn process_mul_row(row: &str) -> u64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let cap = re.captures(row).unwrap();
    let n1 = cap[1].parse::<u64>().unwrap();
    let n2 = cap[2].parse::<u64>().unwrap();
    n1 * n2
}

fn part1(data: Vec<String>) -> () {
    let mul_re = Regex::new(r"mul\(\d+\,\d+\)").unwrap();
    let mut mul_rows: Vec<String> = vec![];

    for line in data {
        for cap in mul_re.captures_iter(&line) {
            mul_rows.push(cap[0].parse::<String>().unwrap());
        }
    }

    let mut result: u64 = 0;

    for row in mul_rows {
        result += process_mul_row(&row);
    }

    println!("day 3, part 1: {:?}", result);
}

// don't NEED a tokenizer: let's try to replace brackets of
// don't()*do() with an empty string
fn part2(data: Vec<String>) -> () {
    let mut input = "".to_owned();

    for line in data {
        input.push_str(&line);
    }

    let re = Regex::new(r"don't\(\).+?do\(\)").unwrap();
    let result = re.replace_all(&input, "");

    let formatted_input = vec![result.to_string()];

    part1(formatted_input);
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
