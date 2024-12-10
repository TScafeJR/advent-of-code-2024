use crate::days::Day;
use crate::util::graph;
use std::collections::HashMap;

type GraphKey = (usize, usize);

fn populate_graph(g: &mut graph::Graph<GraphKey>, p: Vec<Vec<usize>>) {
    let mut curr_index = 0;

    for i in 0..p.len() {
        for j in 0..p[i].len() {
            let key = (curr_index, p[i][j]);
            g.add_node(key);
            if p[i][j] == 0 {
                g.add_head(key);
            }

            if p[i][j] == 9 {
                g.add_tail(key);
            }

            if j < p[i].len() - 1 {
                g.add_edge(key, (curr_index + 1, p[i][j + 1]));
            }

            if j > 0 {
                g.add_edge(key, (curr_index - 1, p[i][j - 1]));
            }

            if i < p.len() - 1 {
                g.add_edge(key, (curr_index + p[i].len(), p[i + 1][j]));
            }

            if i > 0 {
                g.add_edge(key, (curr_index - p[i].len(), p[i - 1][j]));
            }

            curr_index += 1;
        }
    }
}

pub fn can_navigate(s: &GraphKey, e: &GraphKey) -> bool {
    e.1.checked_sub(s.1).unwrap_or(usize::MAX) == 1
}

pub fn part1(data: Vec<String>) -> u64 {
    let d: Vec<Vec<usize>> = data
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).expect("Invalid digit") as usize)
                .collect()
        })
        .collect();

    let mut g = graph::Graph::new();

    populate_graph(&mut g, d);

    let mut res = 0;

    let heads = g.get_heads();
    let tails = g.get_tails();

    for h in heads {
        for t in tails {
            let mut visited: HashMap<GraphKey, bool> = HashMap::new();
            if g.dfs_with_condition(&h, &t, can_navigate, &mut visited) {
                res += 1;
            }
        }
    }

    res as u64
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
