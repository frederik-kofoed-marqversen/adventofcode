use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    // PARSING
    let input = read_to_string("./input.data").unwrap();
    let secrets: Vec<u64> = input
        .lines()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();
    // let secrets: Vec<u64> = vec![1, 2, 3, 2024];

    // PART 1
    let secret_2001: Vec<Vec<u64>> = secrets
        .iter()
        .map(|&s| {
            std::iter::successors(Some(s), |&prev| Some(next(prev)))
                .take(2001)
                .collect()
        })
        .collect();

    println!(
        "Result part 1: {}",
        secret_2001
            .iter()
            .map(|secrets| secrets.last().unwrap())
            .sum::<u64>()
    );

    // PART 2
    let mut total_sequence_price = HashMap::new();
    for numbers in secret_2001 {
        let prices: Vec<i32> = numbers.iter().map(|num| (num % 10) as i32).collect();
        let changes: Vec<i32> = std::iter::zip(&prices, &prices[1..])
            .map(|(a, b)| b - a)
            .collect();
        
        let mut sequences = HashSet::new();
        for (seq, price) in changes.windows(4).zip(&prices[4..]) {
            if sequences.insert(seq) {
                *total_sequence_price.entry(seq.to_vec()).or_insert(0) += price;
            }
        }
    }

    println!("Result part 2: {}", total_sequence_price.iter().max_by_key(|(_, &b)| b).unwrap().1);
}

fn next(mut number: u64) -> u64 {
    number = (number ^ (number * 64)) % 16777216;
    number = (number ^ (number / 32)) % 16777216;
    number = (number ^ (number * 2048)) % 16777216;
    return number;
}
