use std::fs::File;
use std::io::{BufRead, BufReader};

type Node = (String, (String, String));
type DataStruct = Vec<Node>;

fn main() {    
    let (directions, map) = parse_file("./input.data").unwrap();
    // dbg!(&map);
    
    // PART 1
    let start = map.iter().find(|x| x.0 == "AAA").unwrap();
    println!("Result part 1: {}", length(start, &map, &directions, false));

    // PART 2
    let starts = map.iter().filter(|x| x.0.chars().last().unwrap() == 'A');
    let lengths = starts.map(|node: &Node| length(node, &map, &directions, true));
    println!("Result part 2: {}", lengths.fold(1, |lcm, length| least_common_multiple(lcm, length)));
}

fn length(start: &Node, map: &DataStruct, directions: &Vec<char>, part2: bool) -> u64 {
    let mut directions = directions.iter().cycle();
    
    let is_end = if !part2 {
        |x: &Node| -> bool {x.0 == "ZZZ"}
    } else {
        |x: &Node| -> bool {x.0.chars().last().unwrap() == 'Z'}
    };
    let mut steps = 0;
    let mut current_node = &start.clone();
    while !is_end(current_node) {
        match directions.next() {
            Some('L') => {
                current_node = map.iter().find(|x| x.0 == current_node.1.0).unwrap();
            },
            Some('R') => {
                current_node = map.iter().find(|x| x.0 == current_node.1.1).unwrap();
            },
            _ => break
        }
        steps += 1;
    }
    dbg!(&start.0, current_node);
    return steps
}

fn least_common_multiple(a: u64, b:u64) -> u64 {
    let (mut am, mut bm) = (a, b);
    while am != bm {
        if am < bm {
            am += a;
        } else {
            bm += b;
        }
    }
    return am
}

fn parse_file(filepath: &str) -> Result<(Vec<char>, DataStruct), std::io::Error> {
    let file = File::open(filepath)?;
    let mut lines = BufReader::new(file).lines();
    
    let directions = lines.next().unwrap()?.chars().collect();
    lines.next();

    let mut result = Vec::new();
    while let Some(line) = lines.next() {
        let string = line?;
        let args: Vec<&str> = string.split('=').collect();
        let location = args[0].trim().to_string();
        
        let string = args[1].replace('(', "");
        let string = string.replace(')', "");
        let steps: Vec<&str> = string.split(',').map(|x| x.trim()).collect();
        
        let node = (location, (steps[0].to_string(), steps[1].to_string()));
        result.push(node);
    }

    return Ok((directions, result));
}