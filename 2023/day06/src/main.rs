use std::fs::read_to_string;

type DataStruct = Vec<(u64, u64)>;

fn main() {    
    let races = parse_file("./input.data").unwrap();
    // dbg!(&races);

    // PART 1
    let mut times = Vec::new();
    let mut distances = Vec::new();
    let mut num_possible_times = Vec::new();
    for (t, l) in races {
        times.push(t);
        distances.push(l);
        num_possible_times.push(possible_times(t, l));
    }
    // dbg!(&num_possible_times);
    println!("Result part 1: {}", num_possible_times.iter().fold(1, |prod, a| prod * a));

    // PART 2
    
    let t = concatenate(&times);
    let l = concatenate(&distances);
    println!("Result part 1: {}", possible_times(t, l));
}

fn parse_numbers(string: &str) -> Vec<u64> {
    string.split_whitespace().map(|x| x.trim().parse().unwrap()).collect()
}

fn possible_times(t: u64, l: u64) -> u64 {
    let d = f64::sqrt(1.0 - 4.0 * l as f64 / ((t*t) as f64));
    let mut min = f64::ceil(t as f64 / 2.0 * (1.0 - d)) as u64;
    let mut max = f64::floor(t as f64 / 2.0 * (1.0 + d)) as u64;
    if (t - min)*min == l {
        min += 1;
    }
    if (t - max)*max == l {
        max -= 1;
    }
    return max - min + 1;
}

fn concatenate(numbers: &Vec<u64>) -> u64 {
    let string = numbers.iter().fold("".to_string(), |acc, x| acc + &x.to_string());
    return string.parse().unwrap()
}

fn parse_file(filepath: &str) -> Result<DataStruct, std::io::Error> {
    let input = read_to_string(filepath)?;
    let lines: Vec<&str> = input.lines().collect();
    
    let time_string = lines[0].replace("Time:", "");
    let times = parse_numbers(time_string.trim());
    let distances_string = lines[1].replace("Distance:", "");
    let distances = parse_numbers(distances_string.trim());
    
    let mut result = Vec::new();
    for (t, d) in times.iter().zip(&distances) {
        result.push((*t, *d));
    }

    return Ok(result);
}