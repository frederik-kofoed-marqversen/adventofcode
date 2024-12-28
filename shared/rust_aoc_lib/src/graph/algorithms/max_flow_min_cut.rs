use super::{Graph, Num};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::ops::{Add, Sub};

pub fn max_flow<T, U>(graph: &Graph<T, U>, source: &T, sink: &T) -> Option<(U, Graph<T, U>)>
where
    T: Eq + Hash + Clone,
    U: Num<U> + Copy + Clone + PartialOrd + Add<Output = U> + Sub<Output = U>,
{
    // Computes the maximal flow and flow function as a graph using
    // Edmond Karp: https://en.wikipedia.org/wiki/Ford%E2%80%93Fulkerson_algorithm.
    // Flow graph will only contain nodes connected to source and sink.

    let mut flow_amount = U::ZERO;
    let mut flow = Graph::new();

    // Built flow graph.
    flow.add_node(source.clone());
    flow.add_node(sink.clone());
    for node in graph.nodes::<Vec<_>>() {
        for neighbour in graph.neighbours::<Vec<_>>(&node) {
            flow.add_edge(node.clone(), neighbour, U::ZERO);
        }
    }

    loop {
        // Do a breadth-first search to find shortest path between source and sink.
        let mut queue = VecDeque::from([source]);
        let mut predecessor = HashMap::<T, (T, U)>::new();
        while let Some(vertex) = queue.pop_front() {
            let capacities = &graph[vertex];
            for (neighbour, &current_flow) in &flow[vertex] {
                let capacity = *capacities.get(neighbour).unwrap_or(&U::ZERO);
                let residual_capacity = capacity - current_flow;
                // Only admit paths that has capacity left
                if !predecessor.contains_key(neighbour)
                    && residual_capacity > U::ZERO
                    && neighbour != source
                {
                    predecessor.insert(neighbour.clone(), (vertex.clone(), residual_capacity));
                    if neighbour == sink {
                        break;
                    }
                    queue.push_back(neighbour);
                };
            }
        }

        if !predecessor.contains_key(sink) {
            // No path with capacity exists => no more flow can be added
            return Some((flow_amount, flow));
        } else {
            // Have found a path with capacity
            let mut path_capacity = U::INF;
            let mut vertex = sink;
            while let Some((predecessor, capacity)) = predecessor.get(vertex) {
                path_capacity = if path_capacity < *capacity {
                    path_capacity
                } else {
                    *capacity
                };
                vertex = predecessor;
            }
            // Add this path to flow
            flow_amount = flow_amount + path_capacity;
            vertex = sink;
            while let Some((predecessor, _)) = predecessor.get(&vertex) {
                // Add flow to predecessor -> vertex
                flow.data.entry(predecessor.clone()).and_modify(|map| {
                    map.entry(vertex.clone()).and_modify(|val| {
                        *val = *val + path_capacity;
                    });
                });
                // Remove flow from vertex -> predecessor
                flow.data.entry(vertex.clone()).and_modify(|map| {
                    map.entry(predecessor.clone()).and_modify(|val| {
                        *val = *val - path_capacity;
                    });
                });

                vertex = predecessor;
            }
        }
    }
}

pub fn min_cut<T, U>(graph: &Graph<T, U>, a: &T, b: &T) -> Option<(U, (HashSet<T>, HashSet<T>))>
where
    T: Eq + Hash + Clone,
    U: Num<U> + Copy + Clone + PartialOrd + Add<Output = U> + Sub<Output = U>,
{
    // Computes the capacity of the min-cut and a partition induced by the cut.
    // The partition is in general non-unique and will only include nodes connected to a and b.

    // Max-flow min-cut theorem states: min-cut = max-flow
    // : https://en.wikipedia.org/wiki/Max-flow_min-cut_theorem
    let (max_flow, flow) = match max_flow(graph, a, b) {
        Some(val) => val,
        _ => return None,
    };

    // The first subset contains the vetices that are still flow-connected to a.
    let mut g1 = HashSet::from([a.clone()]);
    let mut queue = VecDeque::from([a]);
    while let Some(vertex) = queue.pop_front() {
        let flows = &flow[vertex];
        for (neighbour, capacity) in &graph[vertex] {
            if !g1.contains(&neighbour) && capacity > &flows[&neighbour] {
                queue.push_back(&neighbour);
                g1.insert(neighbour.clone());
            }
        }
    }
    // The other subset contains all other nodes
    let g2 = flow
        .nodes::<HashSet<_>>()
        .difference(&g1)
        .cloned()
        .collect();
    return Some((max_flow, (g1, g2)));
}
