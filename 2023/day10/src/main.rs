use std::fs::File;
use std::io::{BufRead, BufReader};

type DataStruct = Vec<Vec<char>>;

fn main() {
    let map = parse_file("./test.data").unwrap();
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

    // integration only works if path is possitively oriented.
    // Starting direction must be 'S' for input 'W' for test
    let mut direction = 'S';
    let mut steps = 0;
    let mut integral: i32 = 1;  // Initiate at 1 to count the starting position

    loop { // Walk through path
        // Take step
        steps +=1;
        match direction {
            'N' => {
                position.0 -=1;
                integral += 1;
            },
            'S' => {
                position.0 += 1;
            },
            'E' => {
                position.1 += 1;
                integral += position.0 as i32 + 1;
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
            ('|', _) => {},
            ('-', _) => {},
            ('F', 'N') => {direction = 'E';},
            ('F', 'W') => {direction = 'S';},
            ('7', 'E') => {direction = 'S';},
            ('7', 'N') => {direction = 'W';},
            ('J', 'E') => {direction = 'N';},
            ('J', 'S') => {direction = 'W';},
            ('L', 'S') => {direction = 'E';},
            ('L', 'W') => {direction = 'N';},
            _ => {
                dbg!("bad state");
                break
            },
        };
    }

    // PART 1
    println!("Result part 1: {}", steps / 2);

    // PART 2
    dbg!(&integral, &steps);
    println!("Result part 2: {}", integral - steps);
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