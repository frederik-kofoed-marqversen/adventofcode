use rust_aoc_lib::complex::Complex;
use std::collections::{HashMap, HashSet, VecDeque};

const I: Complex<i32> = Complex { real: 0, imag: 1 };

pub fn run(use_test_input: bool) {
    let input = super::read_input(10, use_test_input);
    let mut map: HashMap<Complex<i32>, u32> = HashMap::new();
    let mut trailheads = Vec::new();

    for (i, line) in input.lines().enumerate() {
        for (j, digit) in line.chars().enumerate() {
            let height = digit.to_digit(10).unwrap();
            let pos = i as i32 + I * j as i32;

            map.insert(pos, height);

            if height == 0 {
                trailheads.push(pos);
            }
        }
    }

    let mut trails = vec![Vec::new(); trailheads.len()];
    let mut queue: VecDeque<(usize, Complex<i32>)> =
        VecDeque::from_iter(trailheads.iter().cloned().enumerate());

    while let Some((index, pos)) = queue.pop_front() {
        let current_height = map[&pos];

        if current_height == 9 {
            trails[index].push(pos);
            continue;
        }

        for step in [1 + 0 * I, I, -1 + 0 * I, -I] {
            let next = pos + step;
            let next_height = map.get(&next).unwrap_or(&100);
            if *next_height == current_height + 1 {
                queue.push_front((index, next));
            }
        }
    }

    // PART 1
    let scores: Vec<usize> = trails
        .iter()
        .map(|end_points| HashSet::<&Complex<i32>>::from_iter(end_points).len())
        .collect();
    // dbg!(&scores);
    // 5, 6, 5, 3, 1, 3, 5, 3, 5
    println!("Result part 1: {}", scores.iter().sum::<usize>());


    // PART 2
    let ratings: Vec<usize> = trails.iter().map(|end_points| end_points.len()).collect();
    // dbg!(&ratings);
    // 20, 24, 10, 4, 1, 4, 5, 8, 5
    println!("Result part 2: {}", ratings.iter().sum::<usize>());

}
