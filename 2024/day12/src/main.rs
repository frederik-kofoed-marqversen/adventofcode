use aoc::complex::Complex;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

const I: Complex<i32> = Complex { real: 0, imag: 1 };

fn main() {
    // PARSING
    let input = read_to_string("./input.data").unwrap();

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

    let mut regions: Vec<(char, HashSet<Complex<i32>>, Vec<Complex<i32>>)> = Vec::new();
    let mut unexplored_coords = HashSet::<Complex<i32>>::from_iter(map.keys().cloned());
    // HashSet does not implement .pop(), this is a workarround.
    while let Some(&coord) = unexplored_coords.iter().next().to_owned() {
        let garden_type = map[&coord];
        let mut region = HashSet::from([coord]);
        let mut boundary = Vec::new();
        let mut queue = vec![coord];
        while let Some(coord) = queue.pop() {
            unexplored_coords.remove(&coord);

            for neighbour in [coord + 1, coord + I, coord - 1, coord - I] {
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
                    boundary.push(neighbour);
                }
            }
        }

        regions.push((garden_type, region, boundary));
    }
    /* for region in &regions {
        dbg!((
            region.0,                              // garden type
            region.1.len(),                        // area
            region.2.len(),                        // perimeter
            number_of_edges(&region.1, &region.2)  // num of edges
        ));
    } */

    // PART 1
    let part1 = regions
        .iter()
        .map(|(_, r, b)| r.len() * b.len())
        .sum::<usize>();

    println!("Result part 1: {part1}");

    // PART 2
    let part2 = regions
        .iter()
        .map(|(_, r, b)| r.len() * number_of_edges(r, b))
        .sum::<usize>();

    println!("Result part 2: {part2}");
}

fn number_of_edges(region: &HashSet<Complex<i32>>, boundary: &Vec<Complex<i32>>) -> usize {
    let mut result = 0;
    let mut nonvisited_boundary: HashSet<Complex<i32>> =
        HashSet::from_iter(boundary.iter().cloned());

    // Loop until all of the boundary has been considered
    while let Some(&out) = nonvisited_boundary.iter().next().to_owned() {
        // Find coordinate inside region neighbouring this boundary
        let start = [out + 1, out + I, out - 1, out - I]
            .into_iter()
            .find(|coord| region.contains(coord))
            .unwrap(); // Always exists by construction.
        let start_dir = (out - start) * (-I); // Direction such that boundary is to the right

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
            nonvisited_boundary.remove(&(pos + dir * I));
        }
    }
    return result;
}
