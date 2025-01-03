use rust_aoc_lib::graph::Graph;
use rust_aoc_lib::graph::algs::max_cliques;

pub fn run(use_test_input: bool) {
    let input = super::read_input(23, use_test_input);

    let mut network = Graph::new();
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        network.add_edge(a, b, ());
    }

    let mut nodes: Vec<&str> = network.nodes();
    nodes.sort_by_key(|n| network.neighbours::<Vec<&str>>(n).len());

    let mut triangles = Vec::new();
    let mut temp = network.clone();
    for node in nodes {
        let neighbours: Vec<&str> = temp.neighbours(&node);

        for (i, &n1) in neighbours.iter().enumerate() {
            for n2 in temp.neighbours::<Vec<_>>(&n1) {
                if neighbours[i + 1..].contains(&n2) {
                    triangles.push(vec![node, n1, n2]);
                }
            }
        }
        temp.remove_node(&node);
    }

    // PART 1
    let t_triangles: Vec<&Vec<&str>> = triangles
        .iter()
        .filter(|tri| tri.iter().any(|name| &name[0..1] == "t"))
        .collect();
    println!("Result part 1: {}", t_triangles.len());

    // PART 2
    let maximal_cliques = max_cliques(network.nodes(), &network);
    let mut maximum_clique: Vec<&str> = maximal_cliques
        .iter()
        .max_by_key(|clique| clique.len())
        .unwrap()
        .iter()
        .cloned()
        .collect();
    maximum_clique.sort();

    println!("Result part 2: {}", maximum_clique.join(","));
}
