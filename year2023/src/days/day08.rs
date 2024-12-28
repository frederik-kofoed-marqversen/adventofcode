use rust_aoc_lib::utils::lcm;

type Node = (String, (String, String));
type DataStruct = Vec<Node>;

pub fn run(use_test_input: bool) {
    let input = super::read_input(8, use_test_input);
    let (directions, map) = parse_input(&input);
    // dbg!(&map);

    // PART 1
    let start = map.iter().find(|x| x.0 == "AAA").unwrap();
    println!("Result part 1: {}", length(start, &map, &directions, false));

    // PART 2
    let starts = map.iter().filter(|x| x.0.chars().last().unwrap() == 'A');
    let lengths = starts.map(|node: &Node| length(node, &map, &directions, true));
    println!(
        "Result part 2: {}",
        lengths.fold(1, |acc, length| lcm(acc, length))
    );
}

fn length(start: &Node, map: &DataStruct, directions: &Vec<char>, part2: bool) -> u64 {
    let mut directions = directions.iter().cycle();

    let is_end = if !part2 {
        |x: &Node| -> bool { x.0 == "ZZZ" }
    } else {
        |x: &Node| -> bool { x.0.chars().last().unwrap() == 'Z' }
    };
    let mut steps = 0;
    let mut current_node = &start.clone();
    while !is_end(current_node) {
        match directions.next() {
            Some('L') => {
                current_node = map.iter().find(|x| x.0 == current_node.1 .0).unwrap();
            }
            Some('R') => {
                current_node = map.iter().find(|x| x.0 == current_node.1 .1).unwrap();
            }
            _ => break,
        }
        steps += 1;
    }
    // dbg!(&start.0, current_node);
    return steps;
}

fn parse_input(input: &str) -> (Vec<char>, DataStruct) {
    let mut lines = input.lines();

    let directions = lines.next().unwrap().chars().collect();
    lines.next();

    let mut result = Vec::new();
    while let Some(line) = lines.next() {
        let args: Vec<&str> = line.split('=').collect();
        let location = args[0].trim().to_string();

        let string = args[1].replace('(', "");
        let string = string.replace(')', "");
        let steps: Vec<&str> = string.split(',').map(|x| x.trim()).collect();

        let node = (location, (steps[0].to_string(), steps[1].to_string()));
        result.push(node);
    }

    return (directions, result);
}
