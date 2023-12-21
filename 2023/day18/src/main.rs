use std::fs::read_to_string;

type Step = (String, i64, String);
type DataStruct = Vec<Step>;

fn main() {
    let dig_plan = parse_file("./input.data").unwrap();
    
    // PART 1
    let steps = dig_plan.iter().map(|(dir, dist, _)| (dir.as_str(), *dist));
    println!("Result part 1: {}", solve(steps).unwrap());

    // PART 2
    let steps = dig_plan.iter().map(
        |(_, _, colour)| -> (&str, i64) {
            let chars: Vec<char> = colour.chars().collect();
            let dist = i64::from_str_radix(&String::from_iter(chars[2..7].iter()), 16).unwrap();
            let dir = match chars[7] {
                '0' => "R",
                '1' => "D",
                '2' => "L",
                '3' | _ => "U",
            };
            return (dir, dist)
        }
    );
    println!("Result part 2: {}", solve(steps).unwrap());
}

fn solve<'a, I>(steps: I) -> Option<i64> where I: Iterator<Item = (&'a str, i64)>{
    let mut integral = 0;
    let mut path = 0;

    let mut y = 0;
    for (dir, dist) in steps { // Walk through path
        // Take step
        path += dist;
        match dir {
            "R" => integral += y*dist,
            "L" => integral -= y*dist,
            "U" => y += dist,
            "D" => y -= dist,
            _ => return None,
        }
    }

    return Some(integral + (path-4)/2 + 3)
}

fn parse_file(filepath: &str) -> Result<DataStruct, std::io::Error> {
    Ok(
        read_to_string(filepath)?.trim().split('\n').map(
            |x| -> Step {
                let args: Vec<&str> = x.split(' ').collect();
                let direction = args[0].to_string();
                let distance = args[1].parse().unwrap();
                let colour = args[2].to_string();
                (direction, distance, colour)
            }
        ).collect()
    )
}