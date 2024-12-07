use std::collections::HashMap;

#[derive(Debug)]
pub struct DirectedGraph<T> {
    pub nodes: HashMap<T, Vec<T>>,
}

impl<T: Eq + std::hash::Hash + Clone> DirectedGraph<T> {
    pub fn new() -> Self {
        DirectedGraph {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, value: T) {
        self.nodes.entry(value).or_insert(Vec::new());
    }

    pub fn add_edge(&mut self, from: T, to: T) {
        self.nodes
            .entry(from.clone())
            .or_insert(Vec::new())
            .push(to.clone());

        self.nodes.entry(to).or_insert(Vec::new());
    }

    pub fn outgoing_neighbors(&self, node: &T) -> Option<&Vec<T>> {
        self.nodes.get(node)
    }

    pub fn has_node(&self, node: T) -> bool {
        self.nodes.contains_key(&node)
    }

    #[allow(dead_code)]
    pub fn dfs(&self, start: &T, end: &T, visited: &mut HashMap<T, bool>) -> bool {
        if visited.contains_key(start) {
            return false;
        }

        visited.insert(start.clone(), true);

        if start == end {
            return true;
        }

        if let Some(neighbors) = self.outgoing_neighbors(start) {
            for neighbor in neighbors {
                if self.dfs(neighbor, end, visited) {
                    return true;
                }
            }
        }

        false
    }

    #[allow(dead_code)]
    pub fn path_exists(&self, start: T, end: T) -> bool {
        let mut visited = HashMap::new();
        self.dfs(&start, &end, &mut visited)
    }

    pub fn is_predecessor(&self, start: T, end: T) -> bool {
        let mut visited = HashMap::new();
        self.dfs_reverse_dynamic(&start, &end, &mut visited)
    }

    #[warn(dead_code)]
    pub fn dfs_reverse_dynamic(&self, start: &T, end: &T, visited: &mut HashMap<T, bool>) -> bool {
        if visited.contains_key(start) {
            return false;
        }

        visited.insert(start.clone(), true);

        if start == end {
            return true;
        }

        for (node, neighbors) in &self.nodes {
            if neighbors.contains(start) {
                if self.dfs_reverse_dynamic(node, end, visited) {
                    return true;
                }
            }
        }

        false
    }
}
