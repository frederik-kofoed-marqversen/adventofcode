use rust_aoc_lib::complex::Complex;
use std::collections::HashSet;

const I: Complex<i32> = Complex::<i32> { real: 0, imag: 1 };

pub fn run(use_test_input: bool) {
    let input = super::read_input(6, use_test_input);
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Record start and obstruction positions
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

    // Function to simulate the guards path.
    // Returns the set of guard states and whether or not the path falls into a loop.
    let simulate =
        |obstruction: Option<Complex<i32>>| -> (HashSet<(Complex<i32>, Complex<i32>)>, bool) {
            // Add obstruction
            let mut obsts = obstructions.clone();
            if obstruction.is_some() {
                obsts.insert(obstruction.unwrap());
            }

            // Guard starting state
            let mut guard = start;
            let mut dir = Complex::<i32> { real: -1, imag: 0 };

            // Simulate
            let mut guard_states = HashSet::new();
            while [guard.real, guard.imag]
                .iter()
                .zip(bounds)
                .all(|(&g, b)| g >= 0 && g < b)
            {
                let state = (guard, dir);
                // loop detection
                if guard_states.contains(&state) {
                    return (guard_states, true);
                }
                // record state
                guard_states.insert(state);
                // Take step
                while obsts.contains(&(guard + dir)) {
                    dir = -I * dir;
                }
                guard = guard + dir;
            }

            return (guard_states, false);
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
