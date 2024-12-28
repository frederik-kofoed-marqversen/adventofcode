pub fn run(use_test_input: bool) {
    let input = super::read_input(7, use_test_input).replace(':', "");

    let equations: Vec<Vec<u64>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|digits| digits.parse().unwrap())
                .collect()
        })
        .collect();

    let solve = |part2: bool| -> u64 {
        equations
            .iter()
            .filter(|equation| check_equation(equation[0], equation[1], &equation[2..], part2))
            .map(|equation| equation[0])
            .sum()
    };

    // PART 1
    println!("Result part 1: {}", solve(false));
    // PART 2
    println!("Result part 2: {}", solve(true));
}

fn concat(a: u64, b: u64) -> u64 {
    (a.to_string() + &b.to_string()).parse().unwrap()
}

fn check_equation(goal: u64, current: u64, remaining: &[u64], part2: bool) -> bool {
    if current > goal {
        return false;
    }
    if remaining.len() == 0 {
        return goal == current;
    }

    return check_equation(goal, current + remaining[0], &remaining[1..], part2)
        || check_equation(goal, current * remaining[0], &remaining[1..], part2)
        || (part2 && check_equation(goal, concat(current, remaining[0]), &remaining[1..], part2));
}
