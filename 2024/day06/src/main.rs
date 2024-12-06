use aoc::complex::Complex;
use std::collections::HashSet;
use std::fs::read_to_string;

const I: Complex<i32> = Complex::<i32> { real: 0, imag: 1 };

fn main() {
    // Parsing
    let input = read_to_string("./input.data").unwrap();
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Record obstacle and start positions
    let bounds = [map.len() as i32, map[0].len() as i32];
    let mut obstructions = HashSet::new();
    let mut start = I * 0;
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            let pos = i as i32 + I * j as i32;
            match c {
                '^' => start = pos,
                '#' => {
                    obstructions.insert(pos);
                }
                _ => {}
            }
        }
    }

    // Function to simulate the guards path
    let simulate =
        |obstruction: Option<Complex<i32>>| -> (HashSet<(Complex<i32>, Complex<i32>)>, bool) {
            // Add obstruction
            let mut obsts = obstructions.clone();
            if obstruction.is_some() {
                obsts.insert(obstruction.unwrap());
            }

            // Guard starting state
            let mut dir = Complex::<i32> { real: -1, imag: 0 };
            let mut guard = start;

            // Simulate
            let mut guard_states = HashSet::new();
            let mut is_loop = false;
            while [guard.real, guard.imag]
                .iter()
                .zip(bounds)
                .all(|(&g, b)| g >= 0 && g < b)
            {
                let state = (guard, dir);
                // loop detection
                if guard_states.contains(&state) {
                    is_loop = true;
                    break;
                }
                // record state
                guard_states.insert(state);
                // Take step
                while obsts.contains(&(guard + dir)) {
                    dir = -I * dir;
                }
                guard = guard + dir;
            }

            return (guard_states, is_loop);
        };

    // PART 1
    let guard_states = simulate(None).0;
    // Filter out states occupying the same positions
    let mut distinct_positions: HashSet<Complex<i32>> =
        HashSet::from_iter(guard_states.into_iter().map(|(pos, _)| pos));
    println!("Result part 1: {}", distinct_positions.len());

    // PART 2
    distinct_positions.remove(&start);
    let loop_count = distinct_positions
        .into_iter()
        .filter(|&pos| simulate(Some(pos)).1)
        .count();
    println!("Result part 2: {loop_count}");
}
