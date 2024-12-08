use aoc::complex::Complex;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

const I: Complex<i32> = Complex { real: 0, imag: 1 };

fn main() {
    // Parsing
    let input = read_to_string("./input.data").unwrap().replace(':', "");

    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let bounds = [map[0].len() as i32, map.len() as i32];
    let in_bounds = |loc: &Complex<i32>| -> bool {
        [loc.real, loc.imag]
            .iter()
            .zip(bounds)
            .all(|(&l, b)| l >= 0 && l < b)
    };

    // Record locations for each type of antenna
    let mut antennas: HashMap<&char, Vec<Complex<i32>>> = HashMap::new();
    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if c == &'.' {
                continue;
            } else {
                let loc = i as i32 + I * j as i32;
                antennas.entry(c).or_insert(Vec::new()).push(loc);
            }
        }
    }

    // Compute antinodes
    let mut primary_antinodes = HashSet::new();
    let mut antinodes = HashSet::new();
    for locations in antennas.values() {
        for (i, &a) in locations.iter().enumerate() {
            for &b in &locations[i + 1..] {
                let diff = a - b;
                primary_antinodes.insert(a + diff);
                primary_antinodes.insert(b - diff);

                let mut antinode = a;
                while in_bounds(&antinode) {
                    antinodes.insert(antinode);
                    antinode = antinode + diff;
                }
                antinode = b;
                while in_bounds(&antinode) {
                    antinodes.insert(antinode);
                    antinode = antinode - diff;
                }
            }
        }
    }
    primary_antinodes.retain(|loc| in_bounds(loc));

    println!("Result part 1: {}", primary_antinodes.len());
    println!("Result part 2: {}", antinodes.len());
}
