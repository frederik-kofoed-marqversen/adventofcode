use std::fs::File;
use std::io::{BufRead, BufReader};

type DataStruct = Vec<Vec<char>>;

fn main() {
    let map = parse_file("./input.data").unwrap();
    // dbg!(&map);

    let mut galaxies = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'#' {
                galaxies.push((i as i64, j as i64));
            }
        }
    }
    let empty_rows: Vec<i64> = (0..map.len()).filter(|i| !galaxies.iter().any(|x| x.0 == *i as i64)).map(|x| x as i64).collect();
    let empty_cols: Vec<i64> = (0..map[0].len()).filter(|j| !galaxies.iter().any(|x| x.1 == *j as i64)).map(|x| x as i64).collect();
    
    // Loop through all pairs of galaxies
    let mut distances_part1 = Vec::new();
    let mut distances_part2 = Vec::new();
    for (i, g0) in galaxies.iter().enumerate() {
        for g1 in galaxies[i+1..].iter() {
            let dist = (g0.0 - g1.0).abs() + (g0.1 - g1.1).abs();
            let row_exp = empty_rows.iter().filter(|i| *i > &g0.0.min(g1.0) && *i < &g0.0.max(g1.0)).count() as i64;
            let col_exp = empty_cols.iter().filter(|j| *j > &g0.1.min(g1.1) && *j < &g0.1.max(g1.1)).count() as i64;
            
            distances_part1.push(dist + row_exp + col_exp);
            distances_part2.push(dist + (row_exp + col_exp)*999999);
        }
    }

    println!("Result part 1: {}", distances_part1.iter().sum::<i64>());
    println!("Result part 2: {}", distances_part2.iter().sum::<i64>());
}

fn parse_file(filepath: &str) -> Result<DataStruct, std::io::Error> {
    let file = File::open(filepath)?;
    let mut lines = BufReader::new(file).lines();

    let mut result = Vec::new();
    while let Some(line) = lines.next() {
        let string = line?;
        let characters = string.chars().collect();
        result.push(characters);
    }

    return Ok(result);
}