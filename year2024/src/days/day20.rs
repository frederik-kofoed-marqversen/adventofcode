use rust_aoc_lib::complex::Complex;
use std::collections::{HashMap, HashSet};

const I: Complex<i32> = Complex { real: 0, imag: 1 };

pub fn run(use_test_input: bool) {
    let input = super::read_input(20, use_test_input);

    // Read map
    let mut path = HashMap::new();
    let mut start = 0 * I;
    let mut end = 0 * I;
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let pos = i as i32 + I * j as i32;
            match c {
                '#' => continue,
                'S' => start = pos,
                'E' => end = pos,
                '.' | _ => {}
            }
            path.insert(pos, i32::MAX);
        }
    }

    // Compute distance from start without cheat for all points in path
    let mut state = (start, 0);
    while state.0 != end {
        let (pos, distance) = state;
        path.insert(pos, distance);

        for step in [1 + I * 0, I, -1 + I * 0, -I] {
            let next = pos + step;
            if path.get(&next).is_some_and(|&cost| cost > distance) {
                state = (next, distance + 1);
            }
        }
    }
    path.insert(end, state.1);

    // Function for solving both parts
    let solve = |cheat_time: i32| -> usize {
        // Compute all reachable points within cheat_time ps.
        let mut steps = HashSet::new();
        for i in 0..=cheat_time {
            for j in 0..=cheat_time - i {
                steps.insert(i + I * j);
                steps.insert(-i + I * j);
                steps.insert(i - I * j);
                steps.insert(-i - I * j);
            }
        }
        // Count number of cheats saving at least 100ps
        let good_cheat = |pos: &Complex<i32>, step: &Complex<i32>| -> bool {
            path.get(&(*pos + *step))
                .is_some_and(|d1| d1 - path[pos] - step.real.abs() - step.imag.abs() >= 100)
        };
        return path
            .keys()
            .map(|pos| steps.iter().filter(|&step| good_cheat(pos, step)).count())
            .sum();
    };

    // PART 1
    println!("Result part 1: {}", solve(2));

    // PART 2
    println!("Result part 2: {}", solve(20));
}
