use aoc::complex::Complex;
use std::collections::{HashMap, HashSet, VecDeque};
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
    println!(
        "Result part 2: {}",
        solve(&modified_map_str, &movements, false)
    );
}

fn solve(map_str: &str, movements: &str, print_map: bool) -> i32 {
    // Parse input strings
    let movements = movements.chars().collect();
    let bounds = [
        map_str.split_once('\n').unwrap().0.chars().count(),
        map_str.split('\n').count(),
    ];

    let map: HashMap<Complex<i32>, char> = map_str
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (x as i32 + I * y as i32, c))
        })
        .flatten()
        .filter(|(_, c)| c != &'.')
        .collect();

    if print_map {
        println!("Initial map:");
        println!("{}", map_to_string(&map, &bounds));
    }
    // Simulate movements
    let map = simulate(map, &movements);
    if print_map {
        println!("Final map:");
        println!("{}", map_to_string(&map, &bounds));
    }

    // Find boxes and convert their positions to GPS (Goods Positioning System) coord
    let box_coords: Vec<i32> = map
        .iter()
        .filter(|(_, c)| c == &&'O' || c == &&'[')
        .map(|(pos, _)| pos.real + 100 * pos.imag)
        .collect();
    return box_coords.iter().sum::<i32>();
}

fn simulate(
    mut map: HashMap<Complex<i32>, char>,
    movements: &Vec<char>,
) -> HashMap<Complex<i32>, char> {
    // Keep the position of the robot global for efficiency.
    let mut robot = map.iter().find(|(_, c)| c == &&'@').unwrap().0.clone();

    'outer: for movement in movements {
        let dir = [('^', -I), ('>', 1 + 0 * I), ('v', I), ('<', -1 + 0 * I)]
            .iter()
            .find(|(c, _)| c == movement)
            .map(|(_, num)| *num)
            .unwrap();

        let mut will_move = HashSet::new();
        let mut queue = VecDeque::from(vec![robot]);

        // Check if boxes on the queue can be pushed. Early escape if not possible.
        while let Some(pos) = queue.pop_front() {
            if !will_move.insert(pos) {
                // This object has already been checked
                continue;
            }

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

        // Move the global record of robot position
        robot = robot + dir;
        // Move the affected objects (which includes the robot in the map). To avoid
        // overwriting: Move objects in order of furthest to closest in direction of movement.
        let mut will_move: Vec<Complex<i32>> = will_move.into_iter().collect();
        will_move.sort_by_key(|pos| -(pos.real * dir.real + pos.imag * dir.imag));

        for pos in will_move {
            let c = map.remove(&pos).unwrap();
            map.insert(pos + dir, c);
        }
    }

    return map;
}

fn map_to_string(map: &HashMap<Complex<i32>, char>, bounds: &[usize]) -> String {
    (0..bounds[1])
        .map(|y| {
            (0..bounds[0])
                .map(|x| {
                    if let Some(&c) = map.get(&(x as i32 + I * y as i32)) {
                        c
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
                + "\n"
        })
        .collect::<String>()
        .trim()
        .to_string()
}
