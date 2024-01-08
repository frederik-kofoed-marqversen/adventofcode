use std::collections::{HashMap, BinaryHeap};
use std::hash::Hash;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Graph<T, U> where T: Clone + Hash + Eq, U: Default + Clone {
    // A weighted graph over nodes: T with edge weights: U.
    // For an undirected graph, simply let weights be unit: ().
    //     This is equivalent to using HashSet.
    data: HashMap<T, HashMap<T, U>>,
}

// Graph manipulations
impl<T, U> Graph<T, U> where T: Clone + Hash + Eq, U: Default + Clone {
    // Create new empty graph.
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    // Iterate through nodes in random order.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.keys()
    }

    // Collects nodes into type A with: graph.nodes::<A>().
    pub fn nodes<A: FromIterator<T>> (&self) -> A {
        self.data.keys().cloned().collect()
    }

    pub fn neighbours<A: FromIterator<T>> (&self, node: &T) -> A {
        self.data.get(node).unwrap_or(&HashMap::new()).keys().cloned().collect()
    }
    
    // Add node to graph. Do nothing if already present.
    pub fn add_node(&mut self, node: T) {
        self.data.entry(node).or_insert(HashMap::new());
    }

    // Remove node and all its edges.
    pub fn remove_node(&mut self, node: T) {
        match self.data.remove(&node) {
            None => {},
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
        self.data.entry(from)
            .or_insert(HashMap::new())
            .insert(to, weight.unwrap_or_default());
    }
    pub fn remove_directed_edge(&mut self, from: T, to: T) {
        self.data.entry(from).and_modify(|x| {x.remove(&to);});
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

// Path finding
#[derive(Debug, PartialEq, Eq)]
struct State<T> {
    node: T,
    score: u64,
}

impl<T> PartialOrd for State<T> where T: PartialEq {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // The comparison is swapped to make State min-sorted
        other.score.partial_cmp(&self.score)
    }
}

impl<T> Ord for State<T> where T: Eq {
    fn cmp(&self, other: &State<T>) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T> Graph<T, u64> where T: Clone + Hash + PartialOrd + Eq {
    // Returns the shortest path between start and end. Is None if no path exists.
    pub fn a_star(&self, start: T, end: T, heuristic: Option<&dyn Fn(&T) -> u64>) -> Option<(u64, Vec<T>)> {
        // Without a heuristic this simply becomes Dijkstra's
        let heuristic = heuristic.unwrap_or(&|_| 0);
        
        // data holds tuples (shortest_distance, Some(predecessor)).
        let mut data = HashMap::<T, (u64, Option<T>)>::from([(start.clone(), (0, None))]);
        let mut queue = BinaryHeap::from([State{ node: start, score: 0 }]);
        while let Some(state) = queue.pop() {            
            let current = state.node;

            if current == end {
                let end_distance = data[&end].0;
                let end_path = Self::reconstruct_path(&data, end);
                return Some((end_distance, end_path))
            }

            for (next, weight) in &self.data[&current] {
                // Distance to next through current.
                let d = data[&current].0 + weight;
                // If entry does not exist, fill with default value.
                let entry = data.entry(next.clone()).or_insert((u64::MAX, None));
                
                if d < entry.0 { // This path is shorter than previous best.
                    entry.0 = d;
                    entry.1 = Some(current.clone());
                    queue.push(State { node: next.clone(), score: d + heuristic(next) });
                }
            }
        }
        return None
    }

    fn reconstruct_path(data: &HashMap::<T, (u64, Option<T>)>, end: T) -> Vec<T> {
        let mut path = Vec::from([end]);
        while let Some(predecessor) = &data[path.last().unwrap()].1 {
            path.push(predecessor.clone());
        }
        path.reverse();
        return path
    }
}

impl<T, U> Graph<T, U> where T: Debug + Clone + Hash + Eq, U: Default + Clone {
    pub fn max_flow() {
        // Edmond Karp: https://en.wikipedia.org/wiki/Ford%E2%80%93Fulkerson_algorithm
    }

    pub fn min_cut() {
        // Max-flow min-cut: https://en.wikipedia.org/wiki/Max-flow_min-cut_theorem
    }
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
        
        assert_eq!(graph.nodes::<HashSet<u8>>(), HashSet::from([0, 2]));
        assert!(
            graph.neighbours::<Vec<u8>>(&0).len() == 0 
                && 
            graph.neighbours::<Vec<u8>>(&2).len() == 0
        )
    }

    #[test]
    fn dijkstra() {
        let mut graph: Graph<u8, u64> = Graph::new();
        graph.add_edge(0, 1, Some(12));
        graph.add_edge(1, 2, Some(12));
        graph.add_edge(2, 3, Some(12));
        graph.add_edge(3, 6, Some(20));
        graph.add_edge(0, 4, Some(12));
        graph.add_edge(0, 5, Some(30));
        graph.add_edge(5, 6, Some(40));
        
        assert_eq!(
            graph.a_star(0, 6, None), 
            Some((56, vec![0, 1, 2, 3, 6]))
        );
    }
}