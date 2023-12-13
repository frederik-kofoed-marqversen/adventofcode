use std::fs::File;
use std::io::{BufRead, BufReader};

type DataStruct = Vec<Vec<char>>;

fn main() {
    let mut map = parse_file("./input.data").unwrap();
    // dbg!(&map);

    // Find the starting position 'S'
    let mut position = (0, 0);
    for (i, row) in map.iter().enumerate() {
        if row.contains(&'S') {
            let j = row.iter().position(|x| x==&'S').unwrap();
            position = (i, j);
            break;
        }
    }

    // Turns out starting direction can be 'S' for all inputs
    let mut direction = 'S';
    let mut steps = 0;
    let mut integral: i32 = 0;

    loop { // Walk through path
        // (Only needed for part 2 alternative solution)
        // When shifting the path down by half a cell only 'J's, 'L's and '|'s are seen as vertical lines
        match map[position.0][position.1] {
            'J' => map[position.0][position.1] = '*',
            'L' => map[position.0][position.1] = '*',
            '|' => map[position.0][position.1] = '*',
            _ => map[position.0][position.1] = 'o',
        };
        
        // Take step
        steps +=1;
        match direction {
            'N' => position.0 -=1,
            'S' => position.0 += 1,
            'E' => {
                position.1 += 1;
                integral += position.0 as i32;
            },
            'W' => {
                position.1 -= 1;
                integral -= position.0 as i32;
            },
            _ => {
                dbg!("bad direction");
                break
            },
        }

        // Read map and get direction for next step
        match (map[position.0][position.1], direction) {
            ('S', _) => break,
            ('|' | '-', _) => {},
            ('F', 'N') => direction = 'E',
            ('F', 'W') => direction = 'S',
            ('7', 'E') => direction = 'S',
            ('7', 'N') => direction = 'W',
            ('J', 'E') => direction = 'N',
            ('J', 'S') => direction = 'W',
            ('L', 'S') => direction = 'E',
            ('L', 'W') => direction = 'N',
            _ => {
                dbg!("bad state");
                break
            },
        };
    }
    // dbg!(&integral, &steps);

    // PART 1
    println!("Result part 1: {}", steps / 2);

    // PART 2
    // Up to a sign (orientation of path) `integral` is the internal area of the line path
    // We must remove the contribution from the path having finite width
    // For a closed rectangular path, non-corner path segments contribute half a square
    // and the 4 corners contribute each 1/4th for a total of 1 square.
    // For a general closed path, all corners except four outside corners, 
    // pair up such that on average they contribute half a square.
    // The formula thus holds for any closed path.

    let path_internal_area = (steps - 4) / 2 + 1;
    println!("Result part 2: {}", integral.abs() - path_internal_area);

    // PART 2 (initial solution)
    // Rays eminating from internal points will intersect the path an even number of times
    // Check all points individually
    let mut num_internal_points = 0;
    for row in map.iter() {
        for (j, c) in row.iter().enumerate() {
            if ['*', 'o'].contains(c) {
                continue;
            }
            let intersections = row[j..].iter().filter(|&&x| x=='*').count();
            num_internal_points += intersections % 2;
        }
    }
    println!("Result part 2 (alternative): {num_internal_points}",);
}

fn parse_file(filepath: &str) -> Result<DataStruct, std::io::Error> {
    let file = File::open(filepath)?;
    let mut lines = BufReader::new(file).lines();

    let mut result = Vec::new();
    while let Some(line) = lines.next() {
        let string = line?;
        let numbers = string.chars().collect();
        result.push(numbers);
    }

    return Ok(result);
}