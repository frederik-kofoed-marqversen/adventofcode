use std::fs::read_to_string;
// use ndarray::{s, array, Array1, Array2};
use ndarray::prelude::*;
use ndarray_linalg::Solve;
use regex::Regex;

fn main() {
    // PARSING
    let input = read_to_string("./input.data").unwrap();
    let re = Regex::new(r"\d+").unwrap();
    let machines: Vec<Vec<f64>> = input
        .split("\n\n")
        .map(|block| {
            re.find_iter(block)
                .map(|num| num.as_str().parse::<f64>().unwrap())
                .collect()
        })
        .collect();

    // PART 1
    println!(
        "Result part 1: {}",
        machines
            .iter()
            .map(|machine| num_tokens(machine, 0.0))
            .sum::<u64>()
    );

    // PART 2
    println!(
        "Result part 2: {}",
        machines
            .iter()
            .map(|machine| num_tokens(machine, 10_000_000_000_000.0))
            .sum::<u64>()
    );
}

fn num_tokens(machine: &Vec<f64>, offset: f64) -> u64 {
    let a: Array2<f64> = array![
        [machine[0], machine[2]],
        [machine[1], machine[3]]
    ];
    let b: Array1<f64> = array![machine[4] + offset, machine[5] + offset];
    let mut button_presses = a.solve(&b).unwrap();
    
    button_presses.map_inplace(|val| {
        *val = val.round();
    });

    if a.dot(&button_presses) == b {
        return 3 * button_presses[0].round() as u64 + button_presses[1].round() as u64;
    } else {
        return 0;
    }
}
