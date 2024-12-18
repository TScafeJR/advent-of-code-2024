use crate::days::Day;
use regex::Regex;

type Point = (usize, usize);

#[derive(Debug)]
pub struct Prize {
    pub button_a: Point,
    pub button_b: Point,
    pub prize: Point
}

use std::collections::{VecDeque, HashSet};

fn calculate_presses(p: &Prize) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((0, 0, 0));
    visited.insert((0, 0));

    while let Some((l, r, cost)) = queue.pop_front() {
        let current_x = l * p.button_a.0 + r * p.button_b.0;
        let current_y = l * p.button_a.1 + r * p.button_b.1;

        if l > 100 || r > 100 {
            continue;
        }

        if current_x == p.prize.0 && current_y == p.prize.1 {
            return cost;
        }

        if current_x > p.prize.0 || current_y > p.prize.1 {
            continue;
        }

        if !visited.contains(&(l + 1, r)) {
            queue.push_back((l + 1, r, cost + 3));
            visited.insert((l + 1, r));
        }

        if !visited.contains(&(l, r + 1)) {
            queue.push_back((l, r + 1, cost + 1));
            visited.insert((l, r + 1));
        }
    }

    0
}

fn parse_info(str: String) -> Point {
    let re = Regex::new(r"(\d+)").unwrap();

    let mut results = vec![];
    for cap in re.captures_iter(&str) {
        results.push(cap[1].parse::<usize>().unwrap());
    }

    (results[0], results[1])
}

fn part1(data: Vec<String>) -> u64 {
    let mut curr_idx = 0;
    let mut prizes = vec![];

    while curr_idx < data.len(){
        let button_a = parse_info(data[curr_idx].clone());
        let button_b = parse_info(data[curr_idx + 1].clone());
        let prize_location = parse_info(data[curr_idx + 2].clone());

        let prize = Prize {
            button_a,
            button_b,
            prize: prize_location
        };

        prizes.push(prize);

        curr_idx += 4;
    }

    prizes.into_iter().fold(0, |acc, p| acc + calculate_presses(&p)) as u64
}

fn part2(_data: Vec<String>) -> u64 {
    0
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
