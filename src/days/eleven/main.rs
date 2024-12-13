use crate::days::Day;
use regex::Regex;
use std::collections::HashMap;

fn split_num(num: u64) -> Vec<u64> {
    let mut num_vec = Vec::new();
    let num_str = num.to_string();

    let (first, rest) = num_str.split_at(num_str.len() / 2);
    num_vec.push(first.parse().unwrap());
    num_vec.push(rest.parse().unwrap());

    num_vec
}

fn is_splittable(num: u64) -> bool {
    num.to_string().len() % 2 == 0
}

fn blink(
    stone_number: u64,
    blinks: u64,
    max_blinks: u64,
    cache: &mut HashMap<(u64, u64), u64>,
) -> u64 {
    if blinks >= max_blinks {
        return 1;
    }

    if let Some(&cached_result) = cache.get(&(stone_number, blinks)) {
        return cached_result;
    }

    let result = match stone_number {
        0 => blink(1, blinks + 1, max_blinks, cache),
        num if is_splittable(stone_number) => {
            let split = split_num(num);
            blink(split[0], blinks + 1, max_blinks, cache)
                + blink(split[1], blinks + 1, max_blinks, cache)
        }
        _ => blink(stone_number * 2024, blinks + 1, max_blinks, cache),
    };

    cache.insert((stone_number, blinks), result);
    result
}

fn count_blinks(data: Vec<String>, blinks: u64) -> u64 {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut vals: Vec<u64> = Vec::new();

    for cap in re.captures_iter(&data[0]) {
        vals.push(cap[1].parse::<u64>().unwrap());
    }

    vals.into_iter()
        .map(|stone| {
            let mut local_cache = HashMap::new();
            blink(stone, 0, blinks, &mut local_cache)
        })
        .sum()
}

fn part1(data: Vec<String>) -> u64 {
    count_blinks(data, 25)
}

fn part2(data: Vec<String>) -> u64 {
    count_blinks(data, 75)
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
