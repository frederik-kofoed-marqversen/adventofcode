mod max_flow_min_cut;
mod pathfinding;

pub use max_flow_min_cut::*;
pub use pathfinding::*;

use super::Graph;

// Internal trait to define the default values of numbers
pub trait Num<T> {
    const ZERO: T;
    const INF: T;
}

impl Num<f64> for f64 {
    const ZERO: f64 = 0.0;
    const INF: f64 = f64::INFINITY;
}

impl Num<i64> for i64 {
    const ZERO: i64 = 0;
    const INF: i64 = i64::MAX;
}

impl Num<u64> for u64 {
    const ZERO: u64 = 0;
    const INF: u64 = u64::MAX;
}

#[cfg(test)]
mod tests {
    use super::super::grid_graph_2d;
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn max_flow_min_cut() {
        // Example taken from https://en.wikipedia.org/wiki/Edmonds%E2%80%93Karp_algorithm
        let mut graph: Graph<char, i64> = Graph::new();
        graph.add_directed_edge('A', 'B', 3);
        graph.add_directed_edge('A', 'D', 3);
        graph.add_directed_edge('B', 'C', 4);
        graph.add_directed_edge('C', 'A', 3);
        graph.add_directed_edge('C', 'D', 1);
        graph.add_directed_edge('C', 'E', 2);
        graph.add_directed_edge('D', 'E', 2);
        graph.add_directed_edge('D', 'F', 6);
        graph.add_directed_edge('E', 'B', 1);
        graph.add_directed_edge('E', 'G', 1);
        graph.add_directed_edge('F', 'G', 9);

        let (max_flow, flow) = max_flow(&graph, &'A', &'G').unwrap();
        assert_eq!(max_flow, 5);
        assert_eq!(flow[&'A'], HashMap::from([('B', 2), ('C', 0), ('D', 3)]));
        assert_eq!(
            flow[&'D'],
            HashMap::from([('A', -3), ('C', -1), ('E', 0), ('F', 4)])
        );
        assert_eq!(
            flow[&'E'],
            HashMap::from([('B', 0), ('C', -1), ('D', 0), ('G', 1)])
        );
        assert_eq!(flow[&'G'], HashMap::from([('E', -1), ('F', -4)]));

        let (_, (g1, g2)) = min_cut(&graph, &'A', &'G').unwrap();
        assert_eq!(g1, HashSet::from(['A', 'B', 'C', 'E']));
        assert_eq!(g2, HashSet::from(['D', 'F', 'G']));
    }

    #[test]
    fn a_star_pathfinding() {
        let maze = [
            [0, 0, 0, 1, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 0, 1, 0],
        ];
        let n = maze.len();
        let (start, end) = ((0, 0), (n - 1, n - 1));

        let mut graph = grid_graph_2d(n, n);
        for (i, row) in maze.iter().enumerate() {
            for (j, num) in row.iter().enumerate() {
                if num == &1 {
                    graph.remove_node((i, j));
                }
            }
        }

        // Manhattan distance to end
        let heuristic =
            |pos: &(usize, usize)| (pos.0.abs_diff(end.0) + pos.1.abs_diff(end.1)) as u64;
        let end_condition = |pos: &(usize, usize)| pos == &end;
        let sol_dijkstra = a_star(&graph, &start, &end_condition, None);
        let sol_a_star = a_star(&graph, &start, &end_condition, Some(&heuristic));

        assert_eq!(sol_dijkstra, sol_a_star);
        assert_eq!(
            sol_a_star,
            Some((
                8,
                vec![
                    (0, 0),
                    (0, 1),
                    (0, 2),
                    (1, 2),
                    (1, 3),
                    (1, 4),
                    (2, 4),
                    (3, 4),
                    (4, 4)
                ]
            ))
        );
    }

    #[test]
    fn floats() {
        // Example taken from https://en.wikipedia.org/wiki/Edmonds%E2%80%93Karp_algorithm
        let mut graph: Graph<char, f64> = Graph::new();
        graph.add_node('A');
        graph.add_node('B');
        let end_condition = |node: &char| node == &'B';

        let res = min_cut(&graph, &'A', &'B');
        assert_eq!(
            res,
            Some((0.0, (HashSet::from(['A']), HashSet::from(['B']))))
        );
        let res = a_star(&graph, &'A', &end_condition, None);
        assert_eq!(res, None);
    }
}
