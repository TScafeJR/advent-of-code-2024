use crate::days::Day;
use crate::util::arrays;
use std::collections::{HashMap, HashSet};

type Point = (usize, usize);

fn calculate_antinode(dim: Point, p1: Point, p2: Point) -> Vec<Point> {
    let horiz_offset = p2.0 as isize - p1.0 as isize;
    let vert_offset = p2.1 as isize - p1.1 as isize;

    let antinode_1 = (p1.0 as isize - horiz_offset, p1.1 as isize - vert_offset);
    let antinode_2 = (p2.0 as isize + horiz_offset, p2.1 as isize + vert_offset);

    let mut antinodes = vec![];

    if antinode_1.0 >= 0
        && antinode_1.1 >= 0
        && antinode_1.0 < dim.0 as isize
        && antinode_1.1 < dim.1 as isize
    {
        antinodes.push((antinode_1.0 as usize, antinode_1.1 as usize));
    }
    if antinode_2.0 >= 0
        && antinode_2.1 >= 0
        && antinode_2.0 < dim.0 as isize
        && antinode_2.1 < dim.1 as isize
    {
        antinodes.push((antinode_2.0 as usize, antinode_2.1 as usize));
    }

    antinodes
}

fn calculate_antinodes(dim: Point, m: HashMap<char, Vec<Point>>) -> HashSet<Point> {
    let mut set = HashSet::new();

    for (_key, locs) in &m {
        if locs.len() < 2 {
            continue;
        }

        for i in 0..locs.len() - 1 {
            for j in i + 1..locs.len() {
                let ps = calculate_antinode(dim, locs[i], locs[j]);
                for p in ps {
                    set.insert(p);
                }
            }
        }
    }

    set
}

fn part1(data: Vec<String>) -> u64 {
    let converted_data: Vec<Vec<char>> =
        data.iter().map(|l| arrays::convert_str_to_vec(l)).collect();

    let mut map: HashMap<char, Vec<Point>> = HashMap::new();

    for y in 0..converted_data.len() {
        for x in 0..converted_data[0].len() {
            let point = converted_data[y][x];
            if point != '.' {
                map.entry(point).or_insert_with(Vec::new).push((x, y));
            }
        }
    }

    let dim = (converted_data[0].len(), converted_data.len());

    let res = calculate_antinodes(dim, map);

    res.len() as u64
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
