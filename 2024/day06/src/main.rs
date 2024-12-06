use aoc::multivec::Multivec2D;
use std::collections::HashSet;
use std::fs::read_to_string;

type Complex = Multivec2D<i64>;
const I: Complex = Complex::I;

fn main() {
    // Parsing
    let input = read_to_string("./input.data").unwrap();
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Record obstacle and start positions
    let bounds = [map.len() as i64, 0, 0, map[0].len() as i64];
    let mut obstacles = HashSet::new();
    let mut start = 0 * I;
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            let pos = i as i64 + I * j as i64;
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
    let simulate = |obstruction: Option<Complex>| -> (HashSet<Complex>, bool) {
        let mut dir = Complex::from(-1);
        let mut guard = start;
        let mut obsts = obstacles.clone();
        if obstruction.is_some() {
            obsts.insert(obstruction.unwrap());
        }

        let mut visited = HashSet::new();
        let mut is_loop = false;
        while [0, 3]
            .iter()
            .all(|&i| guard.data[i] >= 0 && guard.data[i] < bounds[i])
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

        let distinct_positions = HashSet::<Complex>::from_iter(visited.iter().map(|(pos, _)| *pos));
        return (distinct_positions, is_loop);
    };

    // PART 1
    let mut guard_path = simulate(None).0;
    println!("Result part 1: {}", guard_path.len());

    // PART 2
    guard_path.remove(&start);
    let loop_count = guard_path.into_iter().filter(|&pos| simulate(Some(pos)).1).count();
    println!("Result part 2: {loop_count}");
}
