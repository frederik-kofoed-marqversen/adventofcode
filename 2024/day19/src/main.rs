use cached::proc_macro::cached;
use std::fs::read_to_string;

fn main() {
    // PARSING
    let input = read_to_string("./input.data").unwrap();

    let args = input.split_once("\n\n").unwrap();
    let towels: Vec<&str> = args.0.split(", ").collect();
    let designs: Vec<&str> = args.1.lines().collect();

    let num_factorisations: Vec<u64> = designs
        .iter()
        .map(|d| count_factorisations(d, &towels))
        .collect();

    // PART 1
    println!(
        "Result part 1: {}",
        num_factorisations.iter().filter(|&&num| num > 0).count()
    );

    // PART 2
    println!(
        "Result part 2: {}",
        num_factorisations.iter().sum::<u64>()
    );
}

#[cached(key = "String", convert = r#"{ String::from(design) }"#)]
fn count_factorisations<'a>(design: &str, towels: &Vec<&'a str>) -> u64 {
    if design == "" {
        return 1;
    }

    return towels
        .iter()
        .filter(|&towel| design.starts_with(towel))
        .map(|towel| count_factorisations(&design[towel.chars().count()..], towels))
        .sum();
}
