use std::fs::File;
use std::io::{BufRead, BufReader};
use aoc::utils::extrapolate;

fn main() {    
    let sequences = parse_file("./input.data").unwrap();
    // dbg!(&sequences);

    // PART 1
    let mut extrapolated_values = Vec::new();
    for seq in &sequences {
        extrapolated_values.push(extrapolate(&seq, seq.len()));
    }
    // dbg!(&extrapolated_values);

    println!("Result part 1: {}", extrapolated_values.iter().sum::<i64>());

    // PART 2
    let mut extrapolated_values = Vec::new();
    for seq in &sequences {
        let reversed = seq.iter().rev().map(|x| *x).collect();
        extrapolated_values.push(extrapolate(&reversed, reversed.len()));
    }
    // dbg!(&extrapolated_values);

    println!("Result part 2: {}", extrapolated_values.iter().sum::<i64>());
}

fn parse_file(filepath: &str) -> Result<Vec<Vec<i64>>, std::io::Error> {
    let file = File::open(filepath)?;
    let mut lines = BufReader::new(file).lines();

    let mut result = Vec::new();
    while let Some(line) = lines.next() {
        let string = line?;
        let numbers = string.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
        result.push(numbers);
    }

    return Ok(result);
}