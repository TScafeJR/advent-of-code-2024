use crate::days::Day;
use regex::Regex;

type Point = (i64, i64);

#[derive(Debug)]
pub struct Prize {
    pub button_a: Point,
    pub button_b: Point,
    pub prize: Point,
}

pub fn calculate_presses(p: &Prize) -> i64 {
    let b = (p.prize.1 * p.button_a.0 - p.prize.0 * p.button_a.1)
        / (p.button_b.1 * p.button_a.0 - p.button_b.0 * p.button_a.1);
    let a = (p.prize.0 - b * p.button_b.0) / p.button_a.0;
    if (
        p.button_a.0 * a + p.button_b.0 * b,
        p.button_a.1 * a + p.button_b.1 * b,
    ) != (p.prize.0, p.prize.1)
    {
        return 0;
    }

    a * 3 + b
}

fn parse_info(str: String) -> Point {
    let re = Regex::new(r"(\d+)").unwrap();

    let mut results = vec![];
    for cap in re.captures_iter(&str) {
        results.push(cap[1].parse::<i64>().unwrap());
    }

    (results[0], results[1])
}

fn part1(data: Vec<String>) -> u64 {
    let mut curr_idx = 0;
    let mut prizes = vec![];

    while curr_idx < data.len() {
        let button_a = parse_info(data[curr_idx].clone());
        let button_b = parse_info(data[curr_idx + 1].clone());
        let prize_location = parse_info(data[curr_idx + 2].clone());

        let prize = Prize {
            button_a,
            button_b,
            prize: prize_location,
        };

        prizes.push(prize);

        curr_idx += 4;
    }

    prizes
        .into_iter()
        .fold(0, |acc, p| acc + calculate_presses(&p)) as u64
}

fn part2(data: Vec<String>) -> u64 {
    let mut curr_idx = 0;
    let mut prizes = vec![];
    const ADDITIONAL: i64 = 10000000000000;

    while curr_idx < data.len() {
        let button_a = parse_info(data[curr_idx].clone());
        let button_b = parse_info(data[curr_idx + 1].clone());
        let prize_location = parse_info(data[curr_idx + 2].clone());

        let prize = Prize {
            button_a,
            button_b,
            prize: (prize_location.0 + ADDITIONAL, prize_location.1 + ADDITIONAL),
        };

        prizes.push(prize);

        curr_idx += 4;
    }

    prizes
        .into_iter()
        .fold(0, |acc, p| acc + calculate_presses(&p)) as u64
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
