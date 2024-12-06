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
    let mut obstacles = HashSet::new();
    let mut start = I * 0;
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            let pos = i as i32 + I * j as i32;
            match c {
                '^' => start = pos,
                '#' => {
                    obstacles.insert(pos);
                }
                _ => {}
            }
        }
    }

    // Function that simulates the guards path
    let simulate = |obstruction: Option<Complex<i32>>| -> (HashSet<Complex<i32>>, bool) {
        let mut dir = Complex::<i32> { real: -1, imag: 0 };
        let mut guard = start;
        let mut obsts = obstacles.clone();
        if obstruction.is_some() {
            obsts.insert(obstruction.unwrap());
        }

        let mut visited = HashSet::new();
        let mut is_loop = false;
        while [guard.real, guard.imag]
            .iter()
            .zip(bounds)
            .all(|(&g, b)| g >= 0 && g < b)
        {
            let state = (guard, dir);
            // loop detection
            if visited.contains(&state) {
                is_loop = true;
                break;
            }
            // record state
            visited.insert(state);

            // Take step
            while obsts.contains(&(guard + dir)) {
                dir = -I * dir;
            }
            guard = guard + dir;
        }

        let distinct_positions =
            HashSet::<Complex<i32>>::from_iter(visited.iter().map(|(pos, _)| *pos));
        return (distinct_positions, is_loop);
    };

    // PART 1
    let mut guard_path = simulate(None).0;
    println!("Result part 1: {}", guard_path.len());

    // PART 2
    guard_path.remove(&start);
    let loop_count = guard_path
        .into_iter()
        .filter(|&pos| simulate(Some(pos)).1)
        .count();
    println!("Result part 2: {loop_count}");
}
