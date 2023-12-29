use std::fs::read_to_string;
use std::collections::{HashSet, HashMap};

type DataStruct = Vec<Vec<char>>;
type Position = (usize, usize);

fn main() {
    let map = parse_file("./input.data").unwrap();
    // dbg!(map.len(), map[0].len());
    // dbg!(&map);

    let n = map.len();
    let start = (0, map[0].iter().position(|&x| x=='.').unwrap());
    let end = (n-1, map[n-1].iter().position(|&x| x=='.').unwrap());

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

fn solve(graph: &Graph, start: Position, end: Position, mut visited: HashSet<Position>) -> Vec<u32> {
    if start == end {
        return vec![0];
    }

    visited.insert(start);

    let mut result = Vec::new();
    for (neighbour, weight) in &graph[&start] {
        if visited.contains(&neighbour) {
            continue;
        } else {
            result.append(&mut solve(graph, *neighbour, end, visited.clone()).iter().map(|x| x+weight).collect());
        }
    }

    return result;
}

type Graph = HashMap<Position, HashMap<Position, u32>>;

fn compute_graph(map: &DataStruct, part2: bool) -> Graph {
    let mut graph = HashMap::new();
    let n = map.len(); // Maps are square
    for i in 0..n {
        for j in 0..n {
            graph.insert((i, j), HashMap::new());
        }
    }
    for i in 0..n-1 {
        for j in 0..n-1 {
            let node = graph.get_mut(&(i, j)).unwrap();
            node.insert((i+1, j), 1);
            node.insert((i, j+1), 1);

            graph.get_mut(&(i+1, j)).unwrap().insert((i, j), 1);
            graph.get_mut(&(i, j+1)).unwrap().insert((i, j), 1);
        }
    }

    for i in 0..n {
        for j in 0..n {
            let c = map[i][j];
            if c == '#' {
                let neighbours = graph.remove(&(i, j)).unwrap();
                for neighbour in neighbours.keys() {
                    graph.get_mut(neighbour).unwrap().remove(&(i, j));
                }
            } else if !part2 {
                let bad_neighbours: Vec<Position> = match c {
                    '>' => vec![(i+1, j), (i-1, j), (i, j-1)],
                    '<' => vec![(i+1, j), (i-1, j), (i, j+1)],
                    '^' => vec![(i+1, j), (i, j+1), (i, j-1)],
                    'v' => vec![(i-1, j), (i, j+1), (i, j-1)],
                    _ => Vec::new(),
                };
                
                let node = graph.get_mut(&(i, j)).unwrap();
                for neighbour in bad_neighbours {
                    node.remove(&neighbour);
                }
            }
        }
    }

    let nodes: Vec<Position> = graph.keys().map(|x| *x).collect();
    for node in &nodes {
        if graph[node].len() == 2 {
            let data = graph.remove(node).unwrap();
            let neighbours: Vec<&Position> = data.keys().collect();
            let edge_weight: u32 = data.values().sum();
            
            let n = graph.get_mut(neighbours[0]).unwrap();
            if n.remove(node) != None {
                n.insert(*neighbours[1], edge_weight);
            }

            let n = graph.get_mut(neighbours[1]).unwrap();
            if n.remove(node) != None {
                n.insert(*neighbours[0], edge_weight);
            }
        }
    }

    return graph
}

fn parse_file(filepath: &str) -> Result<DataStruct, std::io::Error> {
    Ok(read_to_string(filepath)?.trim().split('\n').map(
        |x| x.chars().collect()
    ).collect())
}