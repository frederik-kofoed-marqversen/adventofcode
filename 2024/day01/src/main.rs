use std::fs::read_to_string;

fn main() {
    // Parsing
    let input = read_to_string("./input.data").unwrap();

    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.trim().split('\n') {
        let mut numbers = line.split("   ").map(|s| s.parse::<u32>().unwrap());
        left.push(numbers.next().unwrap());
        right.push(numbers.next().unwrap());
    }
    // dbg!(&left, &right);

    
    // PART 1
    left.sort();
    right.sort();
    let difference = left.iter().zip(&right).fold(0, |acc, (a, b)| acc + a.abs_diff(*b));
    println!("Result part 1: {difference}");

    // PART 2
    let mut similarity = 0;
    for a in left.iter() {
        let count = right.iter().filter(|&b| b==a).count();
        similarity += a * count as u32;
    }
    println!("Result part 2: {similarity}");
}