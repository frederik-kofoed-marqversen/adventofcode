use regex::Regex;

pub fn run(use_test_input: bool) {
    // For this problem we only need to solve 2x2 linear equations. So we implement
    // functions for this since ndarray is very much overkill here.
    
    let input = super::read_input(13, use_test_input);
    let re = Regex::new(r"\d+").unwrap();
    let machines: Vec<Vec<f64>> = input
        .split("\n\n")
        .map(|block| {
            re.find_iter(block)
                .map(|num| num.as_str().parse::<f64>().unwrap())
                .collect()
        })
        .collect();

    // PART 1
    println!(
        "Result part 1: {}",
        machines
            .iter()
            .map(|machine| num_tokens(machine, 0.0))
            .sum::<u64>()
    );

    // PART 2
    println!(
        "Result part 2: {}",
        machines
            .iter()
            .map(|machine| num_tokens(machine, 10_000_000_000_000.0))
            .sum::<u64>()
    );
}

fn num_tokens(machine: &Vec<f64>, offset: f64) -> u64 {
    let a = [[machine[0], machine[2]], [machine[1], machine[3]]];
    let b = [machine[4] + offset, machine[5] + offset];
    // Assume all matrices are full rank
    let button_presses = solve_2x2_matrix_equation(&a, &b).unwrap();

    let button_presses = button_presses.map(|val| val.round());

    if matrix_multiply(&a, &button_presses) == b {
        return 3 * button_presses[0].round() as u64 + button_presses[1].round() as u64;
    } else {
        return 0;
    }
}

fn matrix_multiply(a: &[[f64; 2]; 2], b: &[f64; 2]) -> [f64; 2] {
    [
        (b[0] * a[0][0] + b[1] * a[0][1]),
        (b[0] * a[1][0] + b[1] * a[1][1]),
    ]
}

fn solve_2x2_matrix_equation(a: &[[f64; 2]; 2], b: &[f64; 2]) -> Option<[f64; 2]> {
    let det = a[0][0] * a[1][1] - a[0][1] * a[1][0];

    if det.abs() < f64::EPSILON {
        return None;
    }

    let a_adj = [[a[1][1], -a[0][1]], [-a[1][0], a[0][0]]];
    let x = matrix_multiply(&a_adj, b);
    return Some([x[0] / det, x[1] / det]);
}
