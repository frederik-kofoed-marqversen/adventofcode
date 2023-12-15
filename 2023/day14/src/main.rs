use std::fs::read_to_string;

type Datastruct = Vec<Vec<char>>;

fn main() {
    // Turns out our input maps are all square
    let filepath = "./input.data";
    
    // PART 1
    let mut map = parse_file(filepath).unwrap();
    tilt_north(&mut map);
    println!("Result part 1: {}", total_load(&map));
    
    // PART 2
    // Solution is to notice that at some point the process loops!
    let mut map = parse_file(filepath).unwrap();
    let mut prev_states: Vec<Datastruct> = vec![map.clone()];
    let cycle_start: usize;
    loop {
        cycle(&mut map);
        match prev_states.iter().position(|x| x==&map) {
            Some(index) => {
                cycle_start = index;
                break;
            },
            None => prev_states.push(map.clone()),
        }
    }
    let cycle_period = prev_states.len() - cycle_start;
    
    let final_state = &prev_states[cycle_start + (1000000000 - cycle_start) % cycle_period];
    println!("Result part 2: {}", total_load(final_state));
}

fn total_load(map: &Datastruct) -> usize {
    let mut result = 0;
    for (i, line) in map.iter().enumerate() {
        for obj in line {
            if obj == &'O' {
                result += map.len() - i;
            }
        }
    }
    return result
}

fn tilt_north(map: &mut Datastruct) -> &mut Datastruct {
    let mut empty_spaces = vec![0_usize; map[0].len()];
    for i in 0..map.len() {
        for j in 0..map.len() {
            match map[i][j] {
                'O' => {
                    map[i][j] = '.';
                    map[i-empty_spaces[j]][j] = 'O';
                },
                '#' => empty_spaces[j] = 0,
                '.' | _ => empty_spaces[j] += 1, 
            }
        }
    }
    return map;
}

fn tilt_south(map: &mut Datastruct) -> &mut Datastruct {
    let mut empty_spaces = vec![0_usize; map[0].len()];
    for i in (0..map.len()).rev() {
        for j in 0..map.len() {
            match map[i][j] {
                'O' => {
                    map[i][j] = '.';
                    map[i+empty_spaces[j]][j] = 'O';
                },
                '#' => empty_spaces[j] = 0,
                '.' | _ => empty_spaces[j] += 1, 
            }
        }
    }
    return map;
}

fn tilt_west(map: &mut Datastruct) -> &mut Datastruct {
    let mut empty_spaces = vec![0_usize; map[0].len()];
    for j in 0..map.len() {
        for i in 0..map.len() {
            match map[i][j] {
                'O' => {
                    map[i][j] = '.';
                    map[i][j-empty_spaces[i]] = 'O';
                },
                '#' => empty_spaces[i] = 0,
                '.' | _ => empty_spaces[i] += 1, 
            }
        }
    }
    return map;
}

fn tilt_east(map: &mut Datastruct) -> &mut Datastruct {
    let mut empty_spaces = vec![0_usize; map[0].len()];
    for j in (0..map.len()).rev() {
        for i in 0..map.len() {
            match map[i][j] {
                'O' => {
                    map[i][j] = '.';
                    map[i][j+empty_spaces[i]] = 'O';
                },
                '#' => empty_spaces[i] = 0,
                '.' | _ => empty_spaces[i] += 1, 
            }
        }
    }
    return map;
}

fn cycle(map: &mut Datastruct) -> &mut Datastruct {
    // tilt(map, "north");
    // tilt(map, "west");
    // tilt(map, "sout");
    // tilt(map, "east");
    
    tilt_north(map);
    tilt_west(map);
    tilt_south(map);
    tilt_east(map);
    return map
}

fn parse_file(filepath: &str) -> Result<Datastruct, std::io::Error> {
    Ok(read_to_string(filepath)?.trim().split('\n').map(
        |line| line.chars().collect()
    ).collect())
}