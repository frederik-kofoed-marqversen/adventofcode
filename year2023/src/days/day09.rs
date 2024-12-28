use rust_aoc_lib::utils::extrapolate;

pub fn run(use_test_input: bool) {
    let input = super::read_input(9, use_test_input);
    let sequences = parse_input(&input);
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

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    let mut lines = input.lines();

    let mut result = Vec::new();
    while let Some(line) = lines.next() {
        let numbers = line
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();
        result.push(numbers);
    }

    return result;
}
