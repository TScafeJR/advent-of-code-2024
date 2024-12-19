use crate::days::Day;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Board(pub usize, pub usize);
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point(pub i64, pub i64);
#[derive(Debug)]
pub struct Velocity(pub i64, pub i64);

#[derive(Debug)]
pub struct Robot {
    starting_position: Point,
    pub end_position: Point,
    velocity: Velocity,
}

impl Robot {
    pub fn new(s: Point, v: Velocity) -> Robot {
        Robot {
            starting_position: s,
            end_position: Point(0 as i64, 0 as i64),
            velocity: v,
        }
    }

    pub fn predict_end_position(&mut self, board: &Board, moves: i64) {
        let x = modulus(
            self.starting_position.0 + (moves * self.velocity.0),
            board.0 as i64,
        );
        let y = modulus(
            self.starting_position.1 + (moves * self.velocity.1),
            board.1 as i64,
        );
        self.end_position = Point(x, y);
    }
}

fn modulus(a: i64, b: i64) -> i64 {
    ((a % b) + b) % b
}

fn parse_row_to_robot(str: String) -> Robot {
    let re = Regex::new(r"(-?\d+)").unwrap();

    let mut results = vec![];
    for cap in re.captures_iter(&str) {
        results.push(cap[1].parse::<i64>().unwrap());
    }

    Robot::new(
        Point(results[0], results[1]),
        Velocity(results[2], results[3]),
    )
}

pub fn count_quadrants(b: Board, robots: &Vec<Robot>) -> u64 {
    let mut qs = (0, 0, 0, 0);
    let (ignore_x, ignore_y) = (b.0 as i64 / 2, b.1 as i64 / 2);

    for robot in robots {
        let e = &robot.end_position;

        if e.0 == ignore_x || e.1 == ignore_y {
            continue;
        }

        match (e.0 < ignore_x, e.1 < ignore_y) {
            (true, true) => qs.0 += 1,
            (true, false) => qs.1 += 1,
            (false, true) => qs.2 += 1,
            (false, false) => qs.3 += 1,
        }
    }

    qs.0 * qs.1 * qs.2 * qs.3
}

fn part1(data: Vec<String>) -> u64 {
    // Note: change to 101, 103 for actual input
    let board = Board(11, 7);
    let mut robots: Vec<Robot> = data.into_iter().map(|x| parse_row_to_robot(x)).collect();

    for r in &mut robots {
        r.predict_end_position(&board, 100);
    }

    count_quadrants(board, &robots)
}

fn count_hori_max(r: &Vec<Robot>) -> u64 {
    let mut counts = HashMap::new();

    r.iter().for_each(|robot| {
        let count = counts.entry(robot.end_position.1).or_insert(0);
        *count += 1;
    });

    *counts.values().max().unwrap()
}

fn print_robot_grid(robots: &Vec<Robot>, b: &Board) {
    let mut grid: HashMap<Point, usize> = HashMap::new();

    for robot in robots {
        *grid.entry(robot.end_position).or_insert(0) += 1;
    }

    for y in 0..b.1 {
        for x in 0..b.0 {
            if let Some(_count) = grid.get(&Point(x as i64, y as i64)) {
                print!("{}", 1);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
fn part2(data: Vec<String>) -> u64 {
    let board = Board(101, 103);
    let mut robots: Vec<Robot> = data.into_iter().map(|x| parse_row_to_robot(x)).collect();
    let mut ans = 1;

    loop {
        for r in &mut robots {
            r.predict_end_position(&board, ans);
        }

        let max_x = count_hori_max(&robots);

        if max_x > 30 {
            println!("ans: {}", ans);
            print_robot_grid(&robots, &board);
        }
        if max_x > 50 {
            break;
        }

        if ans > 1000000 {
            break;
        }

        ans += 1;
    }

    ans as u64
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
