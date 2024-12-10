use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Graph<T> {
    pub nodes: HashMap<T, Vec<T>>,
    pub heads_of_graph: Vec<T>,
    pub tails_of_graph: Vec<T>,
}

impl<T: Eq + std::hash::Hash + Clone + Debug> Graph<T> {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            heads_of_graph: Vec::new(),
            tails_of_graph: Vec::new(),
        }
    }

    pub fn add_node(&mut self, value: T) {
        self.nodes.entry(value).or_insert(Vec::new());
    }

    pub fn add_head(&mut self, value: T) {
        self.heads_of_graph.push(value);
    }

    pub fn get_heads(&self) -> &Vec<T> {
        &self.heads_of_graph
    }

    pub fn get_tails(&self) -> &Vec<T> {
        &self.tails_of_graph
    }

    pub fn add_tail(&mut self, value: T) {
        self.tails_of_graph.push(value);
    }

    pub fn add_edge(&mut self, from: T, to: T) {
        self.nodes
            .entry(from.clone())
            .or_insert(Vec::new())
            .push(to.clone());

        self.nodes
            .entry(to.clone())
            .or_insert(Vec::new())
            .push(from.clone());

        self.nodes.entry(to).or_insert(Vec::new());
        self.nodes.entry(from).or_insert(Vec::new());
    }

    pub fn outgoing_neighbors(&self, node: &T) -> Option<&Vec<T>> {
        self.nodes.get(node)
    }

    pub fn dfs_with_condition(
        &self,
        start: &T,
        end: &T,
        condition: fn(s: &T, e: &T) -> bool,
        visited: &mut HashMap<T, bool>,
    ) -> bool {
        if visited.contains_key(start) {
            return false;
        }

        visited.insert(start.clone(), true);

        if start == end {
            return true;
        }

        if let Some(neighbors) = self.outgoing_neighbors(start) {
            for neighbor in neighbors {
                if !condition(start, neighbor) {
                    continue;
                }

                if self.dfs_with_condition(neighbor, end, condition, visited) {
                    return true;
                }
            }
        }

        false
    }
}
