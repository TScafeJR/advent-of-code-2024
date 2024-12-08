use crate::days::Day;
use regex::Regex;
use std::collections::HashMap;

fn part1(data: Vec<String>) -> u64 {
    let mut row_1: Vec<i64> = vec![];
    let mut row_2: Vec<i64> = vec![];
    let re = Regex::new(r"(\d+)").unwrap();

    for i in 0..data.len() {
        let mut results = vec![];
        for cap in re.captures_iter(&data[i]) {
            results.push(cap[1].parse::<i64>().unwrap());
        }

        row_1.push(results[0]);
        row_2.push(results[1]);
    }

    row_1.sort();
    row_2.sort();

    let mut result = 0;

    for i in 0..row_1.len() {
        let abs_diff = (row_2[i] - row_1[i]).abs();
        result += abs_diff;
    }

    return result as u64;
}

fn part2(data: Vec<String>) -> u64 {
    let mut row_1: Vec<i64> = vec![];
    let mut row_2: Vec<i64> = vec![];
    let mut occur_map = HashMap::new();
    let re = Regex::new(r"(\d+)").unwrap();

    for i in 0..data.len() {
        let mut results = vec![];
        for cap in re.captures_iter(&data[i]) {
            results.push(cap[1].parse::<i64>().unwrap());
        }

        let r_1 = results[0];

        row_1.push(r_1);
        if !occur_map.contains_key(&r_1) {
            occur_map.insert(r_1, 0);
        }

        row_2.push(results[1]);
    }

    for i in 0..row_2.len() {
        if occur_map.contains_key(&row_2[i]) {
            occur_map.entry(row_2[i]).and_modify(|occur| *occur += 1);
        }
    }

    let mut result = 0;

    for (key, value) in occur_map.iter() {
        result += key * value;
    }

    return result as u64;
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
