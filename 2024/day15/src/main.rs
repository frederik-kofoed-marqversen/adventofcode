use aoc::complex::Complex;
use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

const I: Complex<i32> = Complex { real: 0, imag: 1 };

fn main() {
    // PARSING
    let input = read_to_string("./input.data").unwrap();
    let input = input.split_once("\n\n").unwrap();
    
    let map_str = input.0;
    let movements = input.1.replace('\n', "");
    
    // PART 1
    println!("Result part 1: {}", solve(map_str, &movements, false));

    // PART 2
    let modified_map_str = map_str
        .replace('.', "..")
        .replace('@', "@.")
        .replace('#', "##")
        .replace('O', "[]");
    println!("Result part 2: {}", solve(&modified_map_str, &movements, false));
}

fn solve(map_str: &str, movements: &str, print_map: bool) -> i32 {
    // Parse input strings
    let movements = movements.chars().collect();
    let bounds = [
        map_str.split_once('\n').unwrap().0.chars().count(),
        map_str.split('\n').count(),
    ];
    let mut robot = 0 * I;
    let mut map = HashMap::new();
    for (y, line) in map_str.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = x as i32 + I * y as i32;
            match c {
                '@' => robot = pos,
                '.' => continue,
                _ => {
                    map.insert(pos, c);
                }
            }
        }
    }

    if print_map {
        println!("Initial map");
        println!("{}", map_to_string(&map, &robot, &bounds));
    }
    // Simulate movements
    (map, robot) = simulate(map, robot, &movements);
    if print_map {
        println!("Final map");
        println!("{}", map_to_string(&map, &robot, &bounds));
    }

    // Find boxes and convert position to GPS (Goods Positioning System) coord
    let box_coords: Vec<i32> = map
        .iter()
        .filter(|(_, c)| c == &&'O' || c == &&'[')
        .map(|(pos, _)| pos.real + 100 * pos.imag)
        .collect();
    return box_coords.iter().sum::<i32>()
}

fn simulate(
    mut map: HashMap<Complex<i32>, char>,
    mut robot: Complex<i32>,
    movements: &Vec<char>,
) -> (HashMap<Complex<i32>, char>, Complex<i32>) {
    'outer: for movement in movements {
        let dir = [('^', -I), ('>', 1 + 0 * I), ('v', I), ('<', -1 + 0 * I)]
            .iter()
            .find(|(c, _)| c == movement)
            .map(|(_, num)| *num)
            .unwrap();

        let mut are_pushed = Vec::new();
        let mut queue = VecDeque::new();

        match map.get(&(robot + dir)) {
            None => {}
            Some('#') => {
                continue;
            }
            Some('[') => {
                queue.push_back(robot + dir);
                queue.push_back(robot + dir + 1);
            }
            Some(']') => {
                queue.push_back(robot + dir);
                queue.push_back(robot + dir - 1);
            }
            Some('O') => {
                queue.push_back(robot + dir);
            }
            _ => panic!("Unexpected character!"),
        }

        while let Some(pos) = queue.pop_front() {
            are_pushed.push(pos);

            match (movement, map.get(&(pos + dir))) {
                (_, None) => {}
                (_, Some('#')) => continue 'outer,
                (_, Some('O')) => queue.push_back(pos + dir),
                ('<' | '>', _) => queue.push_back(pos + dir),
                ('^' | 'v', Some('[')) => {
                    queue.push_back(pos + dir);
                    queue.push_back(pos + dir + 1);
                }
                ('^' | 'v', Some(']')) => {
                    queue.push_back(pos + dir);
                    queue.push_back(pos + dir - 1);
                }
                _ => panic!("Something has gone wrong!"),
            }
        }

        robot = robot + dir;

        are_pushed.sort_by_key(|pos| pos.real * dir.real + pos.imag * dir.imag);
        are_pushed.dedup();
        for pos in are_pushed.iter().rev() {
            if let Some(c) = map.remove(pos) {
                map.insert(*pos + dir, c);
            }
        }
    }

    return (map, robot);
}

fn map_to_string(
    map: &HashMap<Complex<i32>, char>,
    robot: &Complex<i32>,
    bounds: &[usize],
) -> String {
    (0..bounds[1])
        .map(|y| {
            (0..bounds[0])
                .map(|x| {
                    let pos = x as i32 + I * y as i32;
                    if let Some(&c) = map.get(&pos) {
                        c
                    } else if &pos == robot {
                        '@'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
                + "\n"
        })
        .collect()
}
