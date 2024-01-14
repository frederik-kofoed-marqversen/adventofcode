use std::fs::read_to_string;

use aoc::graph::Graph;
use aoc::graph::algs::min_cut;

fn main() {
    let input = read_to_string("./input.data").unwrap();

    let mut graph = Graph::<String, i64>::new();
    for line in input.trim().split('\n') {
        let args: Vec<&str> = line.split(' ').collect();

        let node = args[0].replace(':', "");
        for neighbour in args[1..].iter() {
            graph.add_edge(node.clone(), neighbour.to_string(), 1);
        }
    }

    // PART 1
    let nodes: Vec<String> = graph.nodes();
    let (a, b) = (&nodes[0], &nodes[1]);
    let (cut, (g1, g2)) = min_cut(&graph, a, b).unwrap();
    let (size1, size2) = (g1.len(), g2.len());
    // dbg!(&cut, &size1, &size2);
    println!("Result part 1: {}", size1 * size2);
}