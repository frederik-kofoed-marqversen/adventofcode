use std::fs::File;
use std::io::{BufRead, BufReader};

type DataStruct = Vec<Vec<i64>>;

fn main() {    
    let sequences = parse_file("./input.data").unwrap();
    // dbg!(&sequences);

    // PART 1
    let mut extrapolated_values = Vec::new();
    for seq in &sequences {
        extrapolated_values.push(extrapolate(&seq));
    }
    // dbg!(&extrapolated_values);

    println!("Result part 1: {}", extrapolated_values.iter().sum::<i64>());

    // PART 2
    let mut extrapolated_values = Vec::new();
    for seq in &sequences {
        let reversed = seq.iter().rev().map(|x| *x).collect();
        extrapolated_values.push(extrapolate(&reversed));
    }
    // dbg!(&extrapolated_values);

    println!("Result part 2: {}", extrapolated_values.iter().sum::<i64>());
}

fn extrapolate(sequence: &Vec<i64>) -> i64 {
    // Extrapolate the sequence using Newtons forward iterpolation formula
    
    let mut coefficients = vec![sequence[0]];
    let mut line = sequence.clone();
    while !line.iter().all(|x| x == &line[0]) {
        let diff: Vec<i64> = line[1..].iter().enumerate().map(|(i, x)| x - line[i]).collect();
        coefficients.push(diff[0]);
        line = diff;
    }

    let degree = coefficients.len() as i64;
    let n = sequence.len() as i64;
    let new_value: i64 = coefficients.iter().zip(binoms(n, degree)).map(|(a, b)| a*b).sum();
    
    return new_value
}

fn binoms(n: i64, k: i64) -> Vec<i64> {
    // all binomial coefficients (n \\ i) with 0 <= i <= k

    let mut result = vec![1];
    for i in 0..k {
        result.push(result[i as usize] * (n - i) / (i + 1));
    }
    return result
}

fn parse_file(filepath: &str) -> Result<DataStruct, std::io::Error> {
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