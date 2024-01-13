use std::collections::{BinaryHeap, HashMap};
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
    pub fn add_directed_edge(&mut self, from: T, to: T, weight: Option<U>) {
        self.add_node(to.clone());
        self.data
            .entry(from)
            .or_insert(HashMap::new())
            .insert(to, weight.unwrap_or_default());
    }
    pub fn remove_directed_edge(&mut self, from: T, to: T) {
        self.data.entry(from).and_modify(|x| {
            x.remove(&to);
        });
    }

    // Add edge u -> v and v -> u. Create nodes if not present.
    pub fn add_edge(&mut self, u: T, v: T, weight: Option<U>) {
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

impl<T, U> Graph<T, U>
where
    T: Debug + Clone + Hash + Eq,
    U: Default + Clone,
{
    pub fn max_flow() {
        // Edmond Karp: https://en.wikipedia.org/wiki/Ford%E2%80%93Fulkerson_algorithm
    }

    pub fn min_cut() {
        // Max-flow min-cut: https://en.wikipedia.org/wiki/Max-flow_min-cut_theorem
    }
}

pub fn grid_graph_2d(n: usize, m: usize) -> Graph<(usize, usize), u64> {
    let mut graph = Graph::new();
    for i in 0..n-1 {
        for j in 0..m {
            graph.add_edge((i, j), (i+1, j), Some(1));
        }
    }
    for j in 0..m-1 {
        for i in 0..n {
            graph.add_edge((i, j), (i, j+1), Some(1));
        }
    }
    return graph
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn graph_manipulations() {
        let mut graph: Graph<u8, u64> = Graph::new();
        graph.add_edge(0, 1, None);
        graph.add_edge(1, 2, None);
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
        let (start, end) = ((0, 0), (n-1, n-1));
        
        let mut graph = grid_graph_2d(n, n);
        for (i, row) in maze.iter().enumerate() {
            for (j, num) in row.iter().enumerate() {
                if num == &1 {
                    graph.remove_node((i, j));
                }
            }
        }

        // Manhattan distance to end
        let heuristic = |pos: &(usize, usize)| (pos.0.abs_diff(end.0) + pos.1.abs_diff(end.1)) as u64;
        let sol_dijkstra = graph.a_star(start, end, None);
        let sol_a_star = graph.a_star(start, end, Some(&heuristic));

        assert_eq!(sol_dijkstra, sol_a_star);
        assert_eq!(
            sol_a_star,
            Some((8, vec![(0, 0), (0, 1), (0, 2), (1, 2), (1, 3), (1, 4), (2, 4), (3, 4), (4, 4)]))
        );
    }
}
