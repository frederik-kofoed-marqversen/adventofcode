use super::Graph;
use std::collections::HashSet;
use std::hash::Hash;

pub fn max_cliques<T: Hash + Eq + Copy + Clone>(
    nodes: HashSet<T>,
    graph: &Graph<T, ()>,
) -> Vec<HashSet<T>> {
    // Bron-Kerbosch Algorithm for Maximal Cliques Detection
    // Computes the maximal clique for all nodes in `nodes`. If all nodes of `graph` are 
    // given, will compute all maximal cliques, among which the maximum clique can 
    // trivially be found.
    bron_kerbosch(HashSet::new(), nodes, HashSet::new(), graph)
}

fn bron_kerbosch<T: Hash + Eq + Copy + Clone>(
    clique: HashSet<T>,
    mut unchecked: HashSet<T>,
    mut checked: HashSet<T>,
    graph: &Graph<T, ()>,
) -> Vec<HashSet<T>> {
    if unchecked.is_empty() && checked.is_empty() {
        return vec![clique];
    }

    let mut cliques = Vec::new();
    while let Some(&node) = unchecked.iter().next() {
        let mut new_clique = clique.clone();
        new_clique.insert(node);

        let new_unchecked =
            HashSet::from_iter(unchecked.intersection(&graph.neighbours(&node)).cloned());
        let new_checked =
            HashSet::from_iter(checked.intersection(&graph.neighbours(&node)).cloned());

        cliques.append(&mut bron_kerbosch(
            new_clique,
            new_unchecked,
            new_checked,
            graph,
        ));

        unchecked.remove(&node);
        checked.insert(node);
    }
    return cliques;
}
