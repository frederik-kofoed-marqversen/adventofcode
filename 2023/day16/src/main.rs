use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

type DataStruct = Vec<Vec<char>>;

fn main() {
    let interferometer = parse_file("./input.data").unwrap();
    
    // PART 1
    println!("Result part 1: {}", energy(&interferometer, ((0, 0), "east")));

    // PART 2
    let mut energies: Vec<usize> = Vec::new();
    let n = interferometer.len();
    energies.extend((0..n).map(|i| energy(&interferometer, ((i, 0), "east"))));
    energies.extend((0..n).map(|i| energy(&interferometer, ((0, i), "south"))));
    energies.extend((0..n).map(|i| energy(&interferometer, ((i, n-1), "west"))));
    energies.extend((0..n).map(|i| energy(&interferometer, ((n-1, i), "north"))));
    println!("Result part 2: {}", energies.iter().max().unwrap());
}

fn energy(interferometer: &DataStruct, start: ((usize, usize), &str)) -> usize {
    // Input is square
    let n = interferometer.len();

    let mut beams = vec![start];
    let mut states = HashSet::new();
    while let Some(beam) = beams.pop() {
        if !states.insert(beam) {
            continue;
        }
        let mut position = beam.0;
        let mut directions = vec![beam.1];
        let element = interferometer[position.0][position.1];
        
        // Read position and update direction
        match (element, directions[0]) {
            ('.', _) => {},
            ('|', "south" | "north") => {},
            ('|', "east" | "west") => {
                directions[0] = "north";
                directions.push("south");
            },
            ('-', "south" | "north") => {
                directions[0] = "east";
                directions.push("west");
            },
            ('-', "east" | "west") => {},
            ('/', "east") | ('\\', "west") => directions[0] = "north",
            ('/', "south") | ('\\', "north") => directions[0] = "west",
            ('/', "north") | ('\\', "south") => directions[0] = "east",
            ('/', "west") | ('\\', "east") => directions[0] = "south",
            _ => {
                dbg!("\n!!!BAD STATE!!!");
                break
            }
        }

        // Take step if possible for all outgoing beams
        for direction in directions {
            match direction {
                "north" => if position.0 > 0 {
                    position.0 -= 1;
                    beams.push((position, direction));
                },
                "south" => if position.0 < n-1 {
                    position.0 += 1;
                    beams.push((position, direction));
                },
                "west" => if position.1 > 0 {
                    position.1 -= 1;
                    beams.push((position, direction));
                },
                "east" => if position.1 < n-1 {
                    position.1 += 1;
                    beams.push((position, direction));
                },
                _ => {
                    dbg!("\n!!!BAD DIRECTION!!!");
                    break
                }
            }
        }
    }

    return HashSet::<_>::from_iter(states.iter().map(|(pos, _)| pos)).len();
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