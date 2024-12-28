use rust_aoc_lib::complex::Complex;
use std::collections::{HashMap, HashSet};

const I: Complex<i32> = Complex { real: 0, imag: 1 };

pub fn run(use_test_input: bool) {
    let input = super::read_input(12, use_test_input);

    let map: HashMap<Complex<i32>, char> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (x as i32 + I * y as i32, c))
        })
        .flatten()
        .collect();

    let mut regions = Vec::new();
    let mut unexplored_coords = HashSet::<Complex<i32>>::from_iter(map.keys().cloned());
    // HashSet does not implement .pop(), this is a workarround.
    while let Some(&coord) = unexplored_coords.iter().next().to_owned() {
        let garden_type = map[&coord];
        let mut region = HashSet::from([coord]);
        let mut boundary = Vec::new();
        let mut queue = vec![coord];
        while let Some(coord) = queue.pop() {
            unexplored_coords.remove(&coord);

            for step in [1 + I * 0, I, -1 + I * 0, -I] {
                let neighbour = coord + step;
                // Check if neighbour is part of the same region
                if map
                    .get(&neighbour)
                    .is_some_and(|other| other == &garden_type)
                {
                    // If neighbour is NOT already in region, also check its neighbours
                    if region.insert(neighbour) {
                        queue.push(neighbour);
                    }
                } else {
                    // Record the boundary together with the boundary normal.
                    boundary.push((neighbour, step));
                }
            }
        }

        regions.push((garden_type, region, boundary));
    }

    // PART 1
    let part1 = regions
        .iter()
        .map(|(_, r, b)| r.len() * b.len())
        .sum::<usize>();

    println!("Result part 1: {part1}");

    // PART 2
    // The function: number_of_edges utilises an idea from my collegue much nicer than
    // my own initial solution: number_of_edges_obsolete
    let part2 = regions
        .iter()
        .map(|(_, r, b)| r.len() * number_of_edges(b))
        .sum::<usize>();

    println!("Result part 2: {part2}");
}

fn number_of_edges(boundary: &Vec<(Complex<i32>, Complex<i32>)>) -> usize {
    // Count the number of boundary points which has no neighbour to the right
    boundary
        .iter()
        .filter(|&&(pos, normal)| !boundary.contains(&(pos + normal * I, normal)))
        .count()
}

#[allow(dead_code)]
fn number_of_edges_obsolete(
    region: &HashSet<Complex<i32>>,
    boundary: &Vec<(Complex<i32>, Complex<i32>)>,
) -> usize {
    // Count number of edges by walking around the boundary and counting the number of turns.
    let mut result = 0;
    let mut nonvisited_boundary: HashSet<(Complex<i32>, Complex<i32>)> =
        HashSet::from_iter(boundary.iter().cloned());

    // Loop until all of the boundary has been considered
    while let Some(&(pos, normal)) = nonvisited_boundary.iter().next().to_owned() {
        // Compute coordinate inside region neighbouring this boundary
        let start = pos - normal;
        let start_dir = normal * (-I); // Direction such that boundary is to the right

        // Walk while keeping boundary on the right
        let mut pos = start;
        let mut dir = start_dir;
        let mut first = true;
        while !(pos == start && dir == start_dir) || first {
            first = false;
            if region.contains(&(pos + dir * I)) {
                // Turn and move right
                dir = dir * I;
                result += 1;
                pos = pos + dir;
            } else if region.contains(&(pos + dir)) {
                // Move forward
                pos = pos + dir;
            } else {
                // Turn left
                dir = dir * (-I);
                result += 1;
            }
            // Remove the currently adjacent boundary (always to the right)
            nonvisited_boundary.remove(&(pos + dir * I, dir * I));
        }
    }
    return result;
}
