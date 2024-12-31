use regex::Regex;
use rust_aoc_lib::utils::crt_solve;
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

    /*My initial solution was to use the safety factor of part 1 as an entropy measure,
    and then look for the the lowest entropy frame between the first 101 * 103 = 10_403
    frames. This number suffices since 10_403 is the LCM of the two dimensions, and so,
    the state of the robots will loop with that period. This turned out to be the right
    idea and solved the problem since the easter egg image is drawn entirely in a single
    quadrant.

    However, this is not very applicable to other problems since the safety factor is a
    pretty bad entropy measure in general. After consulting the Megathread I we get the
    following much faster, and generalisable solution:

    Alternative and much faster solution:
    We will search for the frame which has minimal entropy which is higly likely to be the
    easter egg image.

    We will compute the entropy of the state in each dimension separately. The state
    of each dimension is simply the histogram of coordinates. The frame which will have
    minimal overall entropy will be the frame which has minimal combined entropy between
    both x and y.

    Now note that the x-coordinate for all robots will loop every 101 frames, and the
    y-coordinate every 103 frames. Thus, it suffices to compute the entropies of only
    the first 103 frames.
    Since the coordinates loop, the entropies loop as well. This means that we will have
    minimal entropy in each dimension for every frame x
        x = n mod 101 (in x)
        x = m mod 103 (in y)
    Since the two periods 101 and 103 are clearly pairwise coprime, (they are both individually
    prime) the Chinese Remainder Theorem guaranties a solution to the above simultaneus
    congruence relations. That is, a frame x which has minimal entropy in x and y simultaneously.

    -----
    Short on entropy of a histogram:

    Since the robots are indistinguishable, and assuming that for a given frame and a given robot,
    every position is equally likely, the entropy S of a state is given by (up to scaling, and
    choice of logarithm)
        S = log(Ω),
    where Ω is the multiplicity of the state. The multiplicity of a histogram is simply the
    multinomial coefficient. Let there be N robots and let the number of robots with position i
    be n_i, such that
        Σ_i n_i = N.
    Then the multiplicity is
        Ω = N chose (n_1, n_2, ...)

    However, the entropy can be estimated much simpler. Using Stirlings approximation and reducing
    one finds the much simpler:
        S ≈ N Σ_i p_i log(p_i)    with    p_i = n_i/N
    */

    let mut part1 = 0;
    let mut x_entropies = Vec::new();
    let mut y_entropies = Vec::new();
    for i in 0..*bounds.iter().max().unwrap() {
        let state = step_robots(&robots, &bounds, i);
        let x_state = (0..bounds[0])
            .map(|pos| state.iter().filter(|robot| robot[0] == pos).count())
            .collect();
        let y_state = (0..bounds[1])
            .map(|pos| state.iter().filter(|robot| robot[1] == pos).count())
            .collect();
        x_entropies.push(entropy_of_histogram(&x_state));
        y_entropies.push(entropy_of_histogram(&y_state));

        if i == 100 {
            // Record safety factor of the 100th frame
            part1 = compute_safety_factor(&state, &bounds);
        }
    }

    // A bit cumbersome due to f32 not having an implementation of Ord
    let index_of_min = |vec: &Vec<f32>| -> i64 {
        vec.iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0 as i64
    };

    // Frame of minimal entropy
    let a_vec = vec![index_of_min(&x_entropies), index_of_min(&y_entropies)];
    let image_frame = crt_solve(&vec![bounds[0] as i64, bounds[1] as i64], &a_vec)
        .unwrap()
        .0 as i32;

    println!("Result part 1: {part1}");
    println!("Result part 2: {image_frame}");

    // Small terminal application to step in time. Starts at the frame with the easter egg.
    let mut time = image_frame;
    let mut robots = step_robots(&robots, &bounds, time);
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

fn entropy_of_histogram(hist: &Vec<usize>) -> f32 {
    let n = hist.iter().sum::<usize>() as f32;
    hist.iter()
        .map(|&ni| ni as f32 / n)
        .map(|pi| {
            if pi < f32::EPSILON {
                0.0
            } else {
                pi * f32::log2(pi)
            }
        })
        .sum::<f32>()
        * (-1.0)
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
