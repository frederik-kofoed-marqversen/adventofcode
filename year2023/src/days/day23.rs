use rust_aoc_lib::graph::grid_graph_2d;
use std::collections::HashSet;

type Graph = rust_aoc_lib::graph::Graph<(usize, usize), u64>;

pub fn run(use_test_input: bool) {
    let input = super::read_input(23, use_test_input);
    let map: Vec<Vec<char>> = input
        .trim()
        .split('\n')
        .map(|x| x.chars().collect())
        .collect();
    // dbg!(map.len(), map[0].len());
    // dbg!(&map);

    let n = map.len();
    let start = (0, map[0].iter().position(|&x| x == '.').unwrap());
    let end = (n - 1, map[n - 1].iter().position(|&x| x == '.').unwrap());

    // // PART 1
    let graph = compute_graph(&map, false);
    let path_lengths = solve(&graph, start, end, HashSet::new());
    // dbg!(&path_lengths);
    println!("Result part 1: {}", path_lengths.iter().max().unwrap());

    // PART 2
    let graph = compute_graph(&map, true);
    let path_lengths = solve(&graph, start, end, HashSet::new());
    // dbg!(&path_lengths);
    println!("Result part 2: {}", path_lengths.iter().max().unwrap());
}

fn solve(
    graph: &Graph,
    start: (usize, usize),
    end: (usize, usize),
    mut visited: HashSet<(usize, usize)>,
) -> Vec<u64> {
    if start == end {
        return vec![0];
    }

    visited.insert(start);

    let mut result = Vec::new();
    for (neighbour, weight) in &graph[&start] {
        if visited.contains(&neighbour) {
            continue;
        } else {
            result.append(
                &mut solve(graph, *neighbour, end, visited.clone())
                    .iter()
                    .map(|x| x + weight)
                    .collect(),
            );
        }
    }

    return result;
}

fn compute_graph(map: &Vec<Vec<char>>, part2: bool) -> Graph {
    let n = map.len(); // Maps are square
    let mut graph = grid_graph_2d(n, n);

    for i in 0..n {
        for j in 0..n {
            let node = (i, j);
            let c = map[i][j];
            if c == '#' {
                graph.remove_node(&node);
            } else if !part2 {
                let bad_neighbours: Vec<(usize, usize)> = match c {
                    '>' => vec![(i + 1, j), (i - 1, j), (i, j - 1)],
                    '<' => vec![(i + 1, j), (i - 1, j), (i, j + 1)],
                    '^' => vec![(i + 1, j), (i, j + 1), (i, j - 1)],
                    'v' => vec![(i - 1, j), (i, j + 1), (i, j - 1)],
                    _ => Vec::new(),
                };

                for neighbour in bad_neighbours {
                    graph.remove_directed_edge(&node, &neighbour);
                }
            }
        }
    }

    for node in graph.nodes::<Vec<_>>() {
        let neighbours: Vec<(usize, usize)> = graph.neighbours(&node);
        if neighbours.len() == 2 {
            let combined_weight = graph[&node].values().sum();
            graph.remove_node(&node);
            graph.add_edge(neighbours[0], neighbours[1], combined_weight);
        }
    }

    return graph;
}
