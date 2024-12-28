use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::Write;

pub fn run(use_test_input: bool) {
    let input = super::read_input(14, use_test_input);
    let bounds = if use_test_input { [11, 7] } else { [101, 103] };

    let re = Regex::new(r"-*\d+").unwrap();
    let robots: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            re.find_iter(line)
                .map(|num| num.as_str().parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    /*We use the safety factor as a measure for how likely a room is to be the easter
    egg. A lower safety factor corresponds to higher likelyhood.

    The picture will loop every 101 * 103 = 10_403 time step, since this is the LCM
    of the two dimensions of the room (these are twin primes). Therefore we seach all
    boards up until this time

    For my input the most likely times in prioritised order turn out to be:
    [7055, 5843, 793, ...]
    Indeed the most likely candidate t=7055 is the easter egg (The christmass tree
    is drawn entirely in a single quadrant).*/

    let safety_factors: Vec<i32> = (0..bounds[0] * bounds[1])
        .map(|dt| compute_safety_factor(&step_robots(&robots, &bounds, dt), &bounds))
        .collect();

    // PART 1
    println!("Result part 1: {}", safety_factors[100]);

    // PART 2
    let mut prioritized_list: Vec<(usize, &i32)> = safety_factors.iter().enumerate().collect();
    prioritized_list.sort_by_key(|(_, &val)| val);
    println!("Result part 2: {}", prioritized_list[0].0);
    println!(
        "8 most likely times for easter egg in prioritized order:\n{:?}",
        &prioritized_list[..8]
            .iter()
            .map(|item| item.0)
            .collect::<Vec<usize>>()
    );

    // Small terminal application to visually search the most likely times for the easter egg
    let mut robots = robots;
    let mut time = 0;
    let mut stdout = std::io::stdout();
    let stdin = std::io::stdin();
    loop {
        println!("{}", robots_to_string(&robots, &bounds));
        println!("Current time: {}", time);
        loop {
            print!("Supply amount to time warp (negatives allowed): ");
            stdout.flush().expect("Flush problem");
            let mut user_input = String::new();
            stdin.read_line(&mut user_input).expect("Read problem");
            if let Ok(dt) = user_input.trim().parse::<i32>() {
                time += dt;
                robots = step_robots(&robots, &bounds, time);
                println!("\nTime warping by {} time steps...\n", dt);
                break;
            } else {
                println!("Bad input. Try something else.");
            }
        }
    }
}

fn step_robots(robots: &Vec<Vec<i32>>, bounds: &[i32], time: i32) -> Vec<Vec<i32>> {
    robots
        .iter()
        .map(|robot| step_robot(&robot, &bounds, time))
        .collect()
}

fn step_robot(robot: &Vec<i32>, bounds: &[i32], time: i32) -> Vec<i32> {
    let mut end = [
        (robot[0] + robot[2] * time) % bounds[0],
        (robot[1] + robot[3] * time) % bounds[1],
    ];
    if end[0] < 0 {
        end[0] += bounds[0];
    }
    if end[1] < 0 {
        end[1] += bounds[1];
    }
    return vec![end[0], end[1], robot[2], robot[3]];
}

fn robots_to_string(robots: &Vec<Vec<i32>>, bounds: &[i32]) -> String {
    let positions: HashSet<(i32, i32)> =
        HashSet::from_iter(robots.iter().map(|robot| (robot[0], robot[1])));

    (0..bounds[1])
        .map(|j| {
            (0..bounds[0])
                .map(|i| {
                    if positions.contains(&(i, j)) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
                + "\n"
        })
        .collect()
}

fn compute_safety_factor(robots: &Vec<Vec<i32>>, bounds: &[i32]) -> i32 {
    let mut quadrant_counts = [0, 0, 0, 0];
    for robot in robots {
        match (
            robot[0].cmp(&(bounds[0] / 2)),
            robot[1].cmp(&(bounds[1] / 2)),
        ) {
            (Ordering::Less, Ordering::Less) => quadrant_counts[0] += 1,
            (Ordering::Less, Ordering::Greater) => quadrant_counts[1] += 1,
            (Ordering::Greater, Ordering::Less) => quadrant_counts[2] += 1,
            (Ordering::Greater, Ordering::Greater) => quadrant_counts[3] += 1,
            _ => {}
        }
    }
    quadrant_counts.iter().fold(1, |acc, num| acc * num)
}
