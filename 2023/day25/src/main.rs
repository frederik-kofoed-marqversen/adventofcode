use std::fs::read_to_string;
use std::collections::{HashMap, HashSet};
use fastrand::Rng;

type Graph = HashMap<String, Vec<String>>;

fn main() {
    let input = read_to_string("./input.data").unwrap();

    let mut graph: Graph = HashMap::new();
    for line in input.trim().split('\n') {
        let args: Vec<&str> = line.split(' ').collect();

        let node = args[0].replace(':', "");
        let neighbours: Vec<String> = args[1..].iter().map(|x| x.to_string()).collect();

        match graph.get_mut(&node) {
            Some(ns) => {ns.append(&mut neighbours.clone())},
            None => {graph.insert(node.clone(), neighbours.clone());},
        }
        
        for neighbour in &neighbours {
            match graph.get_mut(neighbour) {
                Some(ns) => {ns.push(node.to_string());},
                None => {graph.insert(neighbour.to_string(), vec![node.to_string()]);},
            }
        }
    }

    // PART 1
    let (mut cut, mut sizes) = karger(graph.clone());
    while cut != 3 { // We know the cut-size is 3
        (cut, sizes) = karger(graph.clone());
    }
    // dbg!(&cut, &sizes);
    println!("Result part 1: {}", sizes[0]*sizes[1]);
}

fn karger(mut graph: Graph) -> (usize, Vec<i32>) {
    // Karger min-cut algorithm
    let mut rng = Rng::new();
    let mut nodes: Vec<String> = graph.keys().map(|x| x.clone()).collect();
    let mut weights = vec![1; nodes.len()];
    while nodes.len() > 2 {
        // Pick random edge
        let i1 = rng.usize(0..nodes.len());
        let n1 = &nodes.remove(i1);
        let n2 = &graph[n1][rng.usize(0..graph[n1].len())].clone();
        let i2 = nodes.iter().position(|x| x==n2).unwrap();
        weights[i2] += weights.remove(i1);
        // Contract that edge
        contract(&mut graph, n1, n2, n2);
    }

    let cut_size = graph[&nodes[1]].len();
    return (cut_size, weights)
}

fn contract(graph: &mut Graph, n1: &String, n2: &String, new: &String) {
    // Will contract n2 into n1
    let mut n2s = graph.remove(n2).unwrap();
    let mut ns = graph.remove(n1).unwrap();
    ns.append(&mut n2s);
    
    // Remove self loops
    ns = ns.into_iter().filter(|x| !(x==n1 || x==n2)).collect();

    // Rename n2 and n1 in graph
    for n in HashSet::<&String>::from_iter(&ns) {
        graph.insert(
            n.to_string(),
            graph[n].iter().map(|x| if x==n2 || x==n1 {new} else {x}.to_string()).collect()
        );
    }

    // Add contracted node
    graph.insert(new.to_string(), ns);
}