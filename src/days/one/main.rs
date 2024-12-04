use crate::days::Day;

fn part1(data: Vec<String>) -> () {
    println!("Day 1, Part 1: {:?}", data);
}

fn part2(data: Vec<String>) -> () {
    println!("Day 1, Part 2: {:?}", data);
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2)
    }
}
