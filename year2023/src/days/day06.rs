pub fn run(_: bool) {
    // PART 1
    let times = [46, 85, 75, 82];
    let distances = [208, 1412, 1257, 1410];
    let mut result = Vec::new();
    for (t, l) in times.iter().zip(&distances) {
        result.push(num_possible_times(*t, *l));
    }
    // dbg!(&result);
    println!(
        "Result part 1: {}",
        result.iter().fold(1, |prod, a| prod * a)
    );

    // PART 2
    println!(
        "Result part 1: {}",
        num_possible_times(46857582, 208141212571410)
    );
}

fn num_possible_times(t: u64, l: u64) -> u64 {
    let d = f64::sqrt(1.0 - 4.0 * l as f64 / ((t * t) as f64));
    let mut min = f64::ceil(t as f64 / 2.0 * (1.0 - d)) as u64;
    let mut max = f64::floor(t as f64 / 2.0 * (1.0 + d)) as u64;
    if (t - min) * min == l {
        min += 1;
    }
    if (t - max) * max == l {
        max -= 1;
    }
    return max - min + 1;
}
