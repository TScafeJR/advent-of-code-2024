use crate::days::Day;
use crate::util::arrays;
use crate::util::graph;
use std::collections::HashSet;

type Point = (isize, isize);

pub fn calculate_perimeter(points: &[Point]) -> isize {
    let points_set: HashSet<Point> = points.iter().cloned().collect();
    let mut perimeter = 0;

    for &(x, y) in points {
        perimeter += 4;

        if points_set.contains(&(x + 1, y)) {
            perimeter -= 2;
        }
        if points_set.contains(&(x, y + 1)) {
            perimeter -= 2;
        }
    }

    perimeter
}

pub fn calculate_sides(points: &[Point]) -> isize {
    if points.len() < 3 {
        return 4;
    }

    let mut edges = HashSet::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for &(x, y) in points {
        for (idx, (dx, dy)) in directions.iter().enumerate() {
            let next = (x + dx, y + dy);
            if !points.contains(&next) {
                edges.insert((x, y, idx));
            }
        }
    }

    let mut visited_edges = HashSet::new();
    let mut sides = 0;

    for &(x, y, dir) in &edges {
        if visited_edges.contains(&(x, y, dir)) {
            continue;
        }

        sides += 1;
        let mut stack = vec![(x, y, dir)];

        while let Some((cx, cy, cdir)) = stack.pop() {
            if visited_edges.contains(&(cx, cy, cdir)) {
                continue;
            }
            visited_edges.insert((cx, cy, cdir));

            let perp_dirs = match cdir {
                0 | 2 => [(1, 0), (-1, 0)],
                1 | 3 => [(0, 1), (0, -1)],
                _ => unreachable!(),
            };

            for &(dx, dy) in &perp_dirs {
                let next = (cx + dx, cy + dy, cdir);
                if edges.contains(&next) && !visited_edges.contains(&next) {
                    stack.push(next);
                }
            }
        }
    }

    sides
}

fn find_siblings(
    data: &Vec<Vec<char>>,
    c: char,
    i: usize,
    j: usize,
    g: &mut graph::Graph<(Point, char)>,
    visited: &mut HashSet<Point>,
) {
    if visited.contains(&(i as isize, j as isize)) {
        return;
    }

    visited.insert((i as isize, j as isize));

    if data[i as usize][j as usize] != c {
        return;
    }

    if data[i][j] == c {
        g.add_node(((i as isize, j as isize), c));

        if i > 0 {
            find_siblings(data, c, i - 1, j, g, visited);
        }

        if j > 0 {
            find_siblings(data, c, i, j - 1, g, visited);
        }

        if i < data.len() - 1 {
            find_siblings(data, c, i + 1, j, g, visited);
        }

        if j < data[i].len() - 1 {
            find_siblings(data, c, i, j + 1, g, visited);
        }
    }
}

fn calculate_price(data: Vec<String>, calc_method: fn(&[Point]) -> isize) -> u64 {
    let converted_data: Vec<Vec<char>> =
        data.iter().map(|l| arrays::convert_str_to_vec(l)).collect();

    let mut graphs: Vec<graph::Graph<(Point, char)>> = vec![];

    for i in 0..converted_data.len() {
        for j in 0..converted_data[i].len() {
            let mut graph = graph::Graph::new();
            let node = ((i as isize, j as isize), converted_data[i][j]);
            graph.add_node(node);
            let v = &mut HashSet::new();

            find_siblings(&converted_data, converted_data[i][j], i, j, &mut graph, v);

            let mut found = false;

            for g in &mut graphs {
                if let Some(_found_graph) = g.find_node(node) {
                    g.concat(&graph);
                    found = true;
                    break;
                }
            }

            if !found {
                graphs.push(graph);
            }
        }
    }

    graphs
        .iter()
        .map(|g| {
            let perimeter =
                calc_method(&g.nodes.keys().map(|n| n.0).collect::<Vec<Point>>()) as usize;
            let area = g.size();
            (perimeter * area) as u64
        })
        .sum()
}

fn part1(data: Vec<String>) -> u64 {
    calculate_price(data, calculate_perimeter)
}

fn part2(data: Vec<String>) -> u64 {
    calculate_price(data, calculate_sides)
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
