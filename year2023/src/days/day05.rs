#[derive(Debug)]
struct Map {
    vec: Vec<Vec<u64>>,
}
impl Map {
    fn new() -> Self {
        Self { vec: Vec::new() }
    }

    fn destination(&self, source: u64) -> u64 {
        for vec in self.vec.iter() {
            let (dest_start, source_start, range) = (vec[0], vec[1], vec[2]);
            if source >= source_start && source <= source_start + range {
                return dest_start + source - source_start;
            }
        }
        return source;
    }

    fn source(&self, dest: u64) -> u64 {
        for vec in self.vec.iter() {
            let (dest_start, source_start, range) = (vec[0], vec[1], vec[2]);
            if dest >= dest_start && dest <= dest_start + range {
                return source_start + dest - dest_start;
            }
        }
        return dest;
    }
}

pub fn run(use_test_input: bool) {
    let input = super::read_input(5, use_test_input);
    let (seeds, almanac) = parse_input(&input);

    // PART 1
    let mut locations = Vec::new();
    for seed in &seeds {
        let mut result = *seed;
        for map in &almanac {
            result = map.destination(result);
        }
        locations.push(result);
    }
    println!("Result part 1: {}", locations.iter().min().unwrap());

    // PART 2
    let mut smallest_location: Option<u64> = None;
    // let mut temp = Vec::new();
    for location in 0..2_usize.pow(50) {
        // for location in locations {
        let mut result = location as u64;
        for map in almanac.iter().rev() {
            result = map.source(result);
        }
        // temp.push(result);
        if is_seed(result, &seeds) {
            smallest_location = Some(location as u64);
            break;
        }
    }
    // dbg!(temp);
    match smallest_location {
        Some(loc) => println!("Result part 2: {}", loc),
        None => println!("Result part 2: UNDETERMINED"),
    }
}

fn is_seed(num: u64, ranges: &Vec<u64>) -> bool {
    for i in 0..ranges.len() / 2 {
        let start = ranges[2 * i];
        let len = ranges[2 * i + 1];
        if num >= start && num < start + len {
            return true;
        }
    }
    return false;
}

fn parse_numbers(string: &str) -> Vec<u64> {
    string
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<Map>) {
    let mut lines = input.lines();

    let string = lines.next().unwrap().replace("seeds: ", "");
    let seeds = parse_numbers(&string);
    lines.next();

    let mut result = Vec::new();
    while let Some(line) = lines.next() {
        let string = line;

        if string.contains("map") {
            result.push(Map::new());
            continue;
        }
        if string.is_empty() {
            continue;
        }

        result.last_mut().unwrap().vec.push(parse_numbers(&string));
    }

    return (seeds, result);
}
