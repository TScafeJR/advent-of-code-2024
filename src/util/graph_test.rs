#[cfg(test)]
use crate::util::graph;

#[cfg(test)]
mod tests {
    use super::*;

    fn get_base_graph() -> graph::Graph<i32> {
        let mut g = graph::Graph::new();
        g.add_node(1);
        g.add_node(2);
        g.add_node(3);

        g.add_edge(1, 2);
        g.add_edge(2, 3);

        g
    }

    #[test]
    fn test_dfs_with_condition() {
        let g = get_base_graph();

        let mut visited = std::collections::HashMap::new();

        fn condition(s: &i32, e: &i32) -> bool {
            e - s == 1
        }

        let res = g.dfs_with_condition(&1, &3, condition, &mut visited);

        assert_eq!(res, true);
    }

    #[test]
    fn test_dfs_with_condition_false() {
        let g = get_base_graph();

        let mut visited = std::collections::HashMap::new();

        fn condition(s: &i32, e: &i32) -> bool {
            e - s == 2
        }

        let res = g.dfs_with_condition(&1, &3, condition, &mut visited);

        assert_eq!(res, false);
    }
}
