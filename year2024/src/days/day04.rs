use rust_aoc_lib::complex::Complex;
use std::collections::HashMap;

const I: Complex<i32> = Complex::<i32>{real: 0, imag: 1};

pub fn run(use_test_input: bool) {
    let input = super::read_input(4, use_test_input);

    let mut map = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, letter) in line.chars().enumerate() {
            map.insert(i as i32 + I * j as i32, letter);
        }
    }

    // PART 1
    let dirs = [1 + 0 * I, 1 + I, 0 + I, -1 + I];
    let mut matches1 = 0;
    for pos in map.keys() {
        for dir in dirs {
            for sign in [1, -1] {
                let word = (0..4)
                    .map(|i| map.get(&(*pos + i * sign * dir)).unwrap_or(&'.'))
                    .collect::<String>();

                if word == "XMAS" {
                    matches1 += 1
                };
            }
        }
    }
    println!("Result part 1: {matches1}");

    // PART 2
    let diags = [[1 + I, 0 * I, -(1 + I)], [1 - I, 0 * I, -(1 - I)]];
    let mut matches2 = 0;
    for pos in map.keys() {
        let words = diags.map(|diag| {
            diag.iter()
                .map(|&step| map.get(&(*pos + step)).unwrap_or(&'.'))
                .collect::<String>()
        });

        if words.iter().all(|word| word == "MAS" || word == "SAM") {
            matches2 += 1;
        }
    }

    println!("Result part 2: {matches2}");
}
