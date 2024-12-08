use crate::days::Day;
use crate::util::directed_graph;

fn is_pair_row(row: &str) -> bool {
    row.contains("|")
}

fn process_pair_row(row: &str) -> (u64, u64) {
    let mut row = row.split("|");
    let first = row.next().unwrap().parse::<u64>().unwrap();
    let second = row.next().unwrap().parse::<u64>().unwrap();
    (first, second)
}

fn simple_can_handle_row(
    row: &str,
    graph: &directed_graph::DirectedGraph<u64>,
) -> (bool, Vec<String>) {
    let vals = row
        .split(",")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    for i in 0..vals.len() - 1 {
        let val1 = vals[i].parse::<u64>().unwrap();
        let val2 = vals[i + 1].parse::<u64>().unwrap();

        if !graph.has_node(val1) || !graph.has_node(val2) {
            continue;
        }

        let neighbors = graph.outgoing_neighbors(&val2).unwrap();
        if neighbors.contains(&val1) {
            return (false, vals);
        }
    }

    return (true, vals);
}

fn can_handle_row_and_fix_row(
    row: &str,
    graph: &directed_graph::DirectedGraph<u64>,
) -> (bool, Vec<String>) {
    let mut vals = row
        .split(",")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let mut process_row = false;

    for i in 0..vals.len() - 1 {
        let val1 = vals[i].parse::<u64>().unwrap();
        let val2 = vals[i + 1].parse::<u64>().unwrap();

        if !graph.has_node(val1) || !graph.has_node(val2) {
            continue;
        }

        let neighbors = graph.outgoing_neighbors(&val2).unwrap();
        if neighbors.contains(&val1) {
            let temp = vals[i].clone();
            vals[i] = vals[i + 1].clone();
            vals[i + 1] = temp;
            process_row = true;
        }
    }

    return (process_row, vals);
}

fn grab_middle_element(row: Vec<String>) -> u64 {
    let middle = row.len() / 2;
    row[middle].parse::<u64>().unwrap()
}

fn process_data(
    data: Vec<String>,
    should_sum_row: fn(r: &str, g: &directed_graph::DirectedGraph<u64>) -> (bool, Vec<String>),
) -> u64 {
    let mut processing_pairs = true;
    let mut curr = 0;
    let mut directed_graph = directed_graph::DirectedGraph::new();

    while processing_pairs {
        let row = &data[curr];
        if is_pair_row(row) {
            let (first, second) = process_pair_row(row);
            if !directed_graph.has_node(first) {
                directed_graph.add_node(first);
            }

            if !directed_graph.has_node(second) {
                directed_graph.add_node(second);
            }

            directed_graph.add_edge(first, second);
        } else {
            processing_pairs = false;
        }

        curr += 1;
    }

    let mut sum = 0;

    while curr < data.len() {
        let row = &data[curr];
        let (mut can_handle, mut vals) = should_sum_row(row, &directed_graph);
        if can_handle {
            while can_handle {
                let (new_can_handle, new_vals) = should_sum_row(&vals.join(","), &directed_graph);
                can_handle = new_can_handle;
                vals = new_vals;
            }
            sum += grab_middle_element(vals);
        }

        curr += 1;
    }

    return sum;
}

fn part1(data: Vec<String>) -> u64 {
    return process_data(data, simple_can_handle_row);
}

fn part2(data: Vec<String>) -> u64 {
    return process_data(data, can_handle_row_and_fix_row);
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
