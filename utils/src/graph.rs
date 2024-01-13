use std::collections::{BinaryHeap, HashMap, VecDeque, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Graph<T, U>
where
    T: Clone + Hash + Eq,
    U: Default + Clone,
{
    // A weighted graph over nodes: T with edge weights: U.
    // For an undirected graph, simply let weights be unit: ().
    //     This is equivalent to using HashSet.
    data: HashMap<T, HashMap<T, U>>,
}

// Graph manipulations
impl<T, U> Graph<T, U>
where
    T: Clone + Hash + Eq,
    U: Default + Clone,
{
    // Create new empty graph.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    // Iterate through nodes in random order.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.keys()
    }

    // Collects nodes into type A with: graph.nodes::<A>().
    pub fn nodes<A: FromIterator<T>>(&self) -> A {
        self.data.keys().cloned().collect()
    }

    pub fn neighbours<A: FromIterator<T>>(&self, node: &T) -> A {
        self.data
            .get(node)
            .unwrap_or(&HashMap::new())
            .keys()
            .cloned()
            .collect()
    }

    // Add node to graph. Do nothing if already present.
    pub fn add_node(&mut self, node: T) {
        self.data.entry(node).or_insert(HashMap::new());
    }

    // Remove node and all its edges.
    pub fn remove_node(&mut self, node: T) {
        match self.data.remove(&node) {
            None => {}
            Some(neighbours) => {
                for (neighbour, _) in neighbours {
                    self.remove_directed_edge(neighbour, node.clone());
                }
            }
        }
    }

    // Add edge from -> to. Create nodes if not present.
    pub fn add_directed_edge(&mut self, from: T, to: T, weight: U) {
        self.add_node(to.clone());
        self.data
            .entry(from)
            .or_insert(HashMap::new())
            .insert(to, weight);
    }
    pub fn remove_directed_edge(&mut self, from: T, to: T) {
        self.data.entry(from).and_modify(|x| {
            x.remove(&to);
        });
    }

    // Add edge u -> v and v -> u. Create nodes if not present.
    pub fn add_edge(&mut self, u: T, v: T, weight: U) {
        self.add_directed_edge(u.clone(), v.clone(), weight.clone());
        self.add_directed_edge(v, u, weight);
    }
    pub fn remove_edge(&mut self, u: T, v: T) {
        self.remove_directed_edge(u.clone(), v.clone());
        self.remove_directed_edge(v, u);
    }
}

impl<T, U> std::ops::Index<&T> for Graph<T, U>
where
    T: Clone + Hash + Eq,
    U: Default + Clone,
{
    type Output = HashMap<T, U>;

    #[inline(always)]
    fn index(&self, key: &T) -> &Self::Output {
        &self.data[key]
    }
}

pub fn grid_graph_2d(n: usize, m: usize) -> Graph<(usize, usize), u64> {
    let mut graph = Graph::new();
    for i in 0..n - 1 {
        for j in 0..m {
            graph.add_edge((i, j), (i + 1, j), 1);
        }
    }
    for j in 0..m - 1 {
        for i in 0..n {
            graph.add_edge((i, j), (i, j + 1), 1);
        }
    }
    return graph;
}

// Path finding
#[derive(Debug, PartialEq)]
struct State<T, U> {
    node: T,
    score: U,
}

impl<T, U> Eq for State<T, U>
where
    T: PartialEq,
    U: PartialEq,
{
    // This is a hack to get f64 to work. Eq is never used.
}

impl<T, U> PartialOrd for State<T, U>
where
    T: PartialEq,
    U: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // The comparison is swapped to make State min-sorted
        other.score.partial_cmp(&self.score)
    }
}

impl<T, U> Ord for State<T, U>
where
    T: Eq,
    U: PartialOrd,
{
    fn cmp(&self, other: &State<T, U>) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T, U> Graph<T, U>
where
    T: Clone + Hash + Eq,
    U: Default + Clone,
{
    fn reconstruct_path<V>(data: &HashMap<T, (V, Option<T>)>, end: T) -> Vec<T> {
        let mut path = Vec::from([end]);
        while let Some(predecessor) = &data[path.last().unwrap()].1 {
            path.push(predecessor.clone());
        }
        path.reverse();
        return path;
    }
}

impl<T> Graph<T, u64>
where
    T: Clone + Hash + PartialOrd + Eq,
{
    // Returns the shortest path between start and end. Is None if no path exists.
    pub fn a_star(
        &self,
        start: T,
        end: T,
        heuristic: Option<&dyn Fn(&T) -> u64>,
    ) -> Option<(u64, Vec<T>)> {
        // Without a heuristic this simply becomes Dijkstra's
        let heuristic = heuristic.unwrap_or(&|_| 0);

        // data holds tuples (shortest_distance, Some(predecessor)).
        let mut data = HashMap::<T, (u64, Option<T>)>::from([(start.clone(), (0, None))]);
        let mut queue = BinaryHeap::from([State {
            node: start,
            score: 0,
        }]);
        while let Some(state) = queue.pop() {
            let current = state.node;

            if current == end {
                let end_distance = data[&end].0;
                let end_path = Self::reconstruct_path(&data, end);
                return Some((end_distance, end_path));
            }

            for (next, weight) in &self.data[&current] {
                // Distance to next through current.
                let d = data[&current].0 + weight;
                // If entry does not exist, fill with default value.
                let entry = data.entry(next.clone()).or_insert((u64::MAX, None));

                if d < entry.0 {
                    // This path is shorter than previous best.
                    entry.0 = d;
                    entry.1 = Some(current.clone());
                    queue.push(State {
                        node: next.clone(),
                        score: d + heuristic(next),
                    });
                }
            }
        }
        return None;
    }
}

impl<T> Graph<T, f64>
where
    T: Clone + Hash + PartialOrd + Eq,
{
    // Returns the shortest path between start and end. Is None if no path exists.
    pub fn a_star(
        &self,
        start: T,
        end: T,
        heuristic: Option<&dyn Fn(&T) -> f64>,
    ) -> Option<(f64, Vec<T>)> {
        // Without a heuristic this simply becomes Dijkstra's
        let heuristic = heuristic.unwrap_or(&|_| 0.0);

        // data holds tuples (shortest_distance, Some(predecessor)).
        let mut data = HashMap::<T, (f64, Option<T>)>::from([(start.clone(), (0.0, None))]);
        let mut queue = BinaryHeap::from([State {
            node: start,
            score: 0.0,
        }]);
        while let Some(state) = queue.pop() {
            let current = state.node;

            if current == end {
                let end_distance = data[&end].0;
                let end_path = Self::reconstruct_path(&data, end);
                return Some((end_distance, end_path));
            }

            for (next, weight) in &self.data[&current] {
                // Distance to next through current.
                let d = data[&current].0 + weight;
                // If entry does not exist, fill with default value.
                let entry = data.entry(next.clone()).or_insert((f64::INFINITY, None));

                if d < entry.0 {
                    // This path is shorter than previous best.
                    entry.0 = d;
                    entry.1 = Some(current.clone());
                    queue.push(State {
                        node: next.clone(),
                        score: d + heuristic(next),
                    });
                }
            }
        }
        return None;
    }
}

// Max-flow min-cut
impl<T> Graph<T, i64>
where
    T: Debug + Clone + Hash + Eq,
{
    pub fn max_flow(&self, source: &T, sink: &T) -> Option<(i64, Self)> {
        // Edmond Karp: https://en.wikipedia.org/wiki/Ford%E2%80%93Fulkerson_algorithm
        let mut flow = Self::new();
        let mut flow_amount = 0;
        for node in self.iter() {
            for neighbour in self.neighbours::<Vec<_>>(node) {
                flow.add_edge(node.clone(), neighbour, 0);
            }
        }

        loop {
            // Do a breadth-first search to find shortest path between source and sink.
            let mut queue = VecDeque::from([source]);
            let mut predecessor = HashMap::<T, (T, i64)>::new();
            while let Some(vertex) = queue.pop_front() {
                let capacities = &self[vertex];
                for (neighbour, current_flow) in &flow[vertex] {
                    let capacity = capacities.get(neighbour).unwrap_or(&0);
                    let residual_capacity = capacity - current_flow;
                    // Only admit paths that has capacity left
                    if !predecessor.contains_key(neighbour)
                        && residual_capacity > 0
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
                let mut path_capacity = i64::MAX;
                let mut vertex = sink;
                while let Some((predecessor, capacity)) = predecessor.get(vertex) {
                    path_capacity = path_capacity.min(*capacity);
                    vertex = predecessor;
                }
                // Add this path to flow
                flow_amount += path_capacity;
                vertex = sink;
                while let Some((predecessor, _)) = predecessor.get(&vertex) {
                    // Add flow to predecessor -> vertex
                    flow.data.entry(predecessor.clone()).and_modify(|map| {
                        map.entry(vertex.clone()).and_modify(|val| {
                            *val += path_capacity;
                        });
                    });
                    // Remove flow from vertex -> predecessor
                    flow.data.entry(vertex.clone()).and_modify(|map| {
                        map.entry(predecessor.clone()).and_modify(|val| {
                            *val -= path_capacity;
                        });
                    });

                    vertex = predecessor;
                }
            }
        }
    }

    pub fn min_cut(&self, a: &T, b: &T) -> Option<(i64, (HashSet<T>, HashSet<T>))>{
        // Computes the capacity of the min-cut and a partition induced by the cut
        // Such a partition is non-unique
        // Max-flow min-cut theorem states: min-cut = max-flow
        
        // : https://en.wikipedia.org/wiki/Max-flow_min-cut_theorem
        let (max_flow, flow) = match self.max_flow(a, b) {
            Some(val) => val,
            _ => return None,
        };

        // The first subset contains the vetices that are still flow-connected to a.
        let mut g1 = HashSet::from([a.clone()]);
        let mut queue = VecDeque::from([a]);
        while let Some(vertex) = queue.pop_front() {
            let flows = &flow[&vertex];
            for (neighbour, capacity) in &self[&vertex] {
                if !g1.contains(&neighbour) && capacity > &flows[&neighbour] {
                    queue.push_back(&neighbour);
                    g1.insert(neighbour.clone());
                }
                
            }
        }
        // The other subset contains all other nodes
        let g2 = self.nodes::<HashSet<_>>().difference(&g1).cloned().collect();
        return Some((max_flow, (g1, g2)));
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn graph_manipulations() {
        let mut graph: Graph<u8, ()> = Graph::new();
        graph.add_edge(0, 1, ());
        graph.add_edge(1, 2, ());
        graph.remove_node(1);

        // Removing edges should not create nodes.
        graph.remove_edge(0, 10);

        assert_eq!(graph.nodes::<HashSet<u8>>(), HashSet::from([0, 2]));
        assert_eq!(graph.neighbours::<Vec<u8>>(&0).len(), 0);
        assert_eq!(graph[&2].len(), 0);
    }

    #[test]
    fn grid_graph() {
        let graph = grid_graph_2d(2, 3);

        // Check nodes
        assert_eq!(
            graph.nodes::<HashSet<_>>(),
            HashSet::from([(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)])
        );

        // Check connections
        assert_eq!(
            graph.neighbours::<HashSet<_>>(&(0, 0)),
            HashSet::from([(0, 1), (1, 0)])
        );
        assert_eq!(
            graph.neighbours::<HashSet<_>>(&(1, 2)),
            HashSet::from([(0, 2), (1, 1)])
        );
        assert_eq!(
            graph.neighbours::<HashSet<_>>(&(1, 1)),
            HashSet::from([(0, 1), (1, 0), (1, 2)])
        );
    }

    #[test]
    fn a_star() {
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
        let sol_dijkstra = graph.a_star(start, end, None);
        let sol_a_star = graph.a_star(start, end, Some(&heuristic));

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
    fn max_flow_min_cut() {
        // Example taken from https://en.wikipedia.org/wiki/Edmonds%E2%80%93Karp_algorithm
        let mut graph = Graph::new();
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

        let (max_flow, flow) = graph.max_flow(&'A', &'G').unwrap();
        assert_eq!(max_flow, 5);
        assert_eq!(flow[&'A'], HashMap::from([('B', 2), ('C', 0), ('D', 3)]));
        assert_eq!(flow[&'D'], HashMap::from([('A', -3), ('C', -1), ('E', 0), ('F', 4)]));
        assert_eq!(flow[&'E'], HashMap::from([('B', 0), ('C', -1), ('D', 0), ('G', 1)]));
        assert_eq!(flow[&'G'], HashMap::from([('E', -1), ('F', -4)]));

        let (_, (g1, g2)) = graph.min_cut(&'A', &'G').unwrap();
        assert_eq!(g1, HashSet::from(['A', 'B', 'C', 'E']));
        assert_eq!(g2, HashSet::from(['D', 'F', 'G']));
    }
}
