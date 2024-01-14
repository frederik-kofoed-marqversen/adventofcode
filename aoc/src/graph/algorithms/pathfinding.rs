use super::{Num, Traversible};
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;

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

fn reconstruct_path<T, U>(data: &HashMap<&T, (U, Option<&T>)>, end: &T) -> Vec<T>
where
    T: Eq + Hash + Clone,
{
    let mut path = Vec::from([end]);
    while let Some(predecessor) = data[path.last().unwrap()].1 {
        path.push(predecessor);
    }
    return path.into_iter().cloned().rev().collect();
}

pub fn a_star<T, U>(
    graph: &dyn Traversible<T, U>,
    start: &T,
    end: &T,
    heuristic: Option<&dyn Fn(&T) -> U>,
) -> Option<(U, Vec<T>)>
where
    T: Eq + Hash + Clone,
    U: Num<U> + Copy + Clone + std::ops::Add<Output = U> + PartialOrd,
{
    // Without a heuristic this simply becomes Dijkstra's.
    // Heuristic should be admissible => underestimates the actual cost.
    let heuristic = heuristic.unwrap_or(&|_| U::ZERO);

    // data holds tuples (shortest_distance, Some(predecessor)).
    let mut data = HashMap::<&T, (U, Option<&T>)>::from([(start, (U::ZERO, None))]);
    let mut queue = BinaryHeap::from([State {
        node: start,
        score: U::ZERO,
    }]);
    while let Some(state) = queue.pop() {
        let current = state.node;

        if current == end {
            let end_distance = data[&end].0;
            let end_path: Vec<T> = reconstruct_path(&data, end);
            return Some((end_distance, end_path));
        }

        for (next, &weight) in graph.connections(current) {
            // Distance to next through current.
            let d = data[&current].0 + weight;
            // If entry does not exist, fill with default value.
            let entry = data.entry(next).or_insert((U::INF, None));

            if d < entry.0 {
                // This path is shorter than previous best.
                entry.0 = d;
                entry.1 = Some(current);
                queue.push(State {
                    node: next,
                    score: d + heuristic(next),
                });
            }
        }
    }
    return None;
}
