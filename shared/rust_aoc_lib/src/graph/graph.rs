use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Graph<T, U>
where
    T: Clone + Hash + Eq,
    U: Clone,
{
    // A weighted graph over nodes: T with edge weights: U.
    // For an undirected graph, simply let weights be unit: ().
    //     This is equivalent to using HashSet.
    pub data: HashMap<T, HashMap<T, U>>,
}

// Graph manipulations
impl<T, U> Graph<T, U>
where
    T: Clone + Hash + Eq,
    U: Clone,
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

    pub fn weight(&self, from: &T, to: &T) -> U {
        self.data[from][to].clone()
    }

    // Add node to graph. Returns wether the node was inserted or not.
    pub fn add_node(&mut self, node: T) -> bool {
        if self.data.contains_key(&node) {
            return false;
        } else {
            self.data.insert(node, HashMap::new());
            return true;
        }
    }

    // Remove node and all its edges.
    pub fn remove_node(&mut self, node: &T) -> Option<HashMap<T, U>> {
        match self.data.remove(node) {
            None => return None,
            Some(neighbours) => {
                for (neighbour, _) in &neighbours {
                    self.remove_directed_edge(neighbour, node);
                }
                return Some(neighbours);
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
    pub fn remove_directed_edge(&mut self, from: &T, to: &T) -> Option<U> {
        match self.data.get_mut(from) {
            Some(neighbours) => neighbours.remove(to),
            None => None,
        }
    }

    // Add edge u -> v and v -> u. Create nodes if not present.
    pub fn add_edge(&mut self, u: T, v: T, weight: U) {
        self.add_directed_edge(u.clone(), v.clone(), weight.clone());
        self.add_directed_edge(v, u, weight);
    }
    pub fn remove_edge(&mut self, u: &T, v: &T) -> (Option<U>, Option<U>) {
        (
            self.remove_directed_edge(u, v),
            self.remove_directed_edge(v, u),
        )
    }
}

impl<T, U> std::ops::Index<&T> for Graph<T, U>
where
    T: Clone + Hash + Eq,
    U: Clone,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

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
}
