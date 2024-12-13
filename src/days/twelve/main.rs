use crate::days::Day;
use crate::util::arrays;
use crate::util::graph;
use std::collections::HashSet;

type Point = (usize, usize);

pub fn calculate_perimeter(points: &[(usize, usize)]) -> usize {
    let points_set: HashSet<(usize, usize)> = points.iter().cloned().collect();
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

pub fn calculate_sides(points: &[(usize, usize)]) -> usize {
    let points_set: HashSet<(usize, usize)> = points.iter().cloned().collect();
    let mut corners = HashSet::new();

    for &(x, y) in points {
        let top_left = (x, y);
        let top_right = (x + 1, y);
        let bottom_left = (x, y + 1);
        let bottom_right = (x + 1, y + 1);

        for corner in [top_left, top_right, bottom_left, bottom_right] {
            if !corners.insert(corner) {
                corners.remove(&corner);
            }
        }
    }

    corners.len()
}

fn find_siblings(
    data: &Vec<Vec<char>>,
    c: char,
    i: usize,
    j: usize,
    g: &mut graph::Graph<(Point, char)>,
    visited: &mut HashSet<Point>,
) {
    if visited.contains(&(i, j)) {
        return;
    }

    visited.insert((i, j));

    if data[i as usize][j as usize] != c {
        return;
    }

    if data[i][j] == c {
        g.add_node(((i as usize, j as usize), c));

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

fn calculate_price(data: Vec<String>, calc_method: fn(&[(usize, usize)]) -> usize) -> u64 {
    let converted_data: Vec<Vec<char>> =
        data.iter().map(|l| arrays::convert_str_to_vec(l)).collect();

    let mut graphs: Vec<graph::Graph<(Point, char)>> = vec![];

    for i in 0..converted_data.len() {
        for j in 0..converted_data[i].len() {
            let mut graph = graph::Graph::new();
            let node = ((i, j), converted_data[i][j]);
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
            let perimeter = calc_method(&g.nodes.keys().map(|n| n.0).collect::<Vec<Point>>());
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
