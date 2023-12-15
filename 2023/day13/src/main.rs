use std::fs::read_to_string;
use std::iter::zip;

type Pattern = Vec<Vec<char>>;

fn main() {
    let patterns = parse_file("./input.data").unwrap();
    // dbg!(&patterns[0], transpose(&patterns[0]));

    // PART 1
    let mirror_positions: Vec<usize> = patterns.iter().map(
        |pattern| match find_mirror_rows(pattern, false) {
            Some(i) => 100 * i,
            None => find_mirror_rows(&transpose(pattern), false).unwrap(),
        }).collect();
    // dbg!(mirror_positions);
    println!("Result part 1: {}", mirror_positions.iter().sum::<usize>());

    // PART 2
    let mirror_positions: Vec<usize> = patterns.iter().map(
        |pattern| match find_mirror_rows(pattern, true) {
            Some(i) => 100 * i,
            None => find_mirror_rows(&transpose(pattern), true).unwrap(),
        }).collect();
    // dbg!(mirror_positions);
    println!("Result part 2: {}", mirror_positions.iter().sum::<usize>());
}

fn find_mirror_rows(pattern: &Pattern, smutch: bool) -> Option<usize> {
    // Returns the number of rows above mirror position
    for i in 1..pattern.len() {
        let distance_to_edge = usize::min(i, pattern.len()-i);
        let mut mismatches = 0;
        for d in 1..=distance_to_edge {
            mismatches += zip(&pattern[i-d], &pattern[i+d-1]).filter(|(a, b)| a!=b).count();
        }
        if mismatches == if smutch {1} else {0} {
            return Some(i)
        }
    }
    return None
}

fn transpose(pattern: &Pattern) -> Pattern {
    let mut result = vec![Vec::<char>::with_capacity(pattern.len()); pattern[0].len()];
    for row in pattern {
        for (i, c) in row.iter().enumerate() {
            result[i].push(*c);
        }
    }
    return result
}

fn parse_file(filepath: &str) -> Result<Vec<Pattern>, std::io::Error> {
    Ok(read_to_string(filepath)?.split("\n\n").map(
        |pattern| pattern.trim().split('\n').map(
            |line| line.chars().collect()
        ).collect()
    ).collect())
}