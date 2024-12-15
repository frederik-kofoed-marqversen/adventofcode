use aoc::complex::Complex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

const I: Complex<i32> = Complex { real: 0, imag: 1 };

fn main() {
    // PARSING
    let input = read_to_string("./input.data").unwrap();
    let input = input.split_once("\n\n").unwrap();

    let movements: Vec<char> = input.1.replace('\n', "").chars().collect();

    /* // PART 1
    let mut robot = 0 * I;
    let mut map = HashMap::new();
    for (y, line) in input.0.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = x as i32 + I * y as i32;
            if c == '@' {
                robot = pos;
            } else if c == '.' {
                continue;
            } else {
                map.insert(pos, c);
            };
        }
    }

    // println!("{}", map_to_string(&map, &robot, &[10, 10]));

    'outer: for movement in &movements {
        let dir = [('^', -I), ('>', 1 + 0 * I), ('v', I), ('<', -1 + 0 * I)]
            .iter()
            .find(|(c, _)| c == movement)
            .map(|(_, num)| *num)
            .unwrap();

        // Find first empty space in front of robot
        let mut pos = robot + dir;
        while let Some(c) = map.get(&pos) {
            if c == &'#' {
                // Robot cannot move in the current direction
                // => go to next movement
                continue 'outer;
            }
            pos = pos + dir;
        }

        // Move the robot
        robot = robot + dir;
        // If boxes in front of robot, move them
        if let Some(c) = map.remove(&robot) {
            map.insert(pos, c);
        }
    }

    // println!("{}", map_to_string(&map, &robot, &[10, 10]));

    let obj_coords: Vec<i32> = map
        .iter()
        .filter(|(_, c)| c == &&'O')
        .map(|(pos, _)| pos.real + 100 * pos.imag)
        .collect();
    println!("Result part 1: {}", obj_coords.iter().sum::<i32>()); */

    // PART 2
    let modified_input = input
        .0
        .replace('.', "..")
        .replace('@', "@.")
        .replace('#', "##")
        .replace('O', "[]");
    let mut robot = 0 * I;
    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();
    for (y, line) in modified_input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = x as i32 + I * y as i32;
            match c {
                '@' => robot = pos,
                '#' => {
                    walls.insert(pos);
                }
                '[' => {
                    boxes.insert(pos);
                }
                '.' | ']' | _ => continue,
            }
        }
    }

    // println!("{}", map_to_string(&robot, &boxes, &walls, &[20, 10]));
    'outer: for movement in &movements {
        let dir = [('^', -I), ('>', 1 + 0 * I), ('v', I), ('<', -1 + 0 * I)]
            .iter()
            .find(|(c, _)| c == movement)
            .map(|(_, num)| *num)
            .unwrap();

        let mut are_pushed = Vec::new();
        let mut box_queue = VecDeque::new();

        if walls.contains(&(robot + dir)) {
            continue;
        }
        if boxes.contains(&(robot + dir)) {
            box_queue.push_back(robot + dir);
        }
        if boxes.contains(&(robot + dir - 1)) {
            box_queue.push_back(robot + dir - 1);
        }

        while let Some(pos) = box_queue.pop_front() {
            are_pushed.push(pos);
           
            match movement {
                '>' => {
                    if walls.contains(&(pos + 2 * dir)) {
                        continue 'outer;
                    }
                    if boxes.contains(&(pos + 2 * dir)) {
                        box_queue.push_back(pos + 2 * dir);
                    }
                },
                '<' => {
                    if walls.contains(&(pos + dir)) {
                        continue 'outer;
                    }
                    if boxes.contains(&(pos + 2 * dir)) {
                        box_queue.push_back(pos + 2 * dir);
                    }
                }
                '^' | 'v' => {
                    if walls.contains(&(pos + dir)) || walls.contains(&(pos + 1 + dir)) {
                        continue 'outer;
                    }
                    for step in [dir - 1, dir, dir + 1] {
                        if boxes.contains(&(pos + step)) {
                            box_queue.push_back(pos + step)
                        }
                    }
                }
                _ => panic!("Something went wrong!"),
            }
        }
        
        robot = robot + dir;
        for pos in are_pushed.iter().rev() {
            boxes.remove(pos);
            boxes.insert(*pos + dir);
        }
    }
    // println!("{}", map_to_string(&robot, &boxes, &walls, &[20, 10]));
    
    let box_coords: Vec<i32> = boxes
        .iter()
        .map(|pos| pos.real + 100 * pos.imag)
        .collect();
    println!("Result part 2: {}", box_coords.iter().sum::<i32>());
}

#[allow(dead_code)]
fn map_to_string(
    robot: &Complex<i32>,
    boxes: &HashSet<Complex<i32>>,
    walls: &HashSet<Complex<i32>>,
    bounds: &[usize],
) -> String {
    let mut map = vec![vec!['.'; bounds[0]]; bounds[1]];
    
    map[robot.imag as usize][robot.real as usize] = '@';
    for pos in boxes {
        let (i, j) = (pos.imag as usize, pos.real as usize);
        map[i][j] = '[';
        map[i][j + 1] = ']';
    }
    for pos in walls {
        let (i, j) = (pos.imag as usize, pos.real as usize);
        map[i][j] = '#';
    }

    map.iter().map(|row| row.iter().collect::<String>() + "\n").collect()
}
