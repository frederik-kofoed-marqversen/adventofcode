use cached::proc_macro::cached;
use std::fs::read_to_string;

fn main() {
    // PARSING
    let input = read_to_string("./input.data").unwrap();

    let args = input.split_once("\n\n").unwrap();
    let towels: Vec<&str> = args.0.split(", ").collect();
    let designs: Vec<&str> = args.1.lines().collect();

    // PART 1
    println!(
        "Result part 1: {}",
        designs
            .iter()
            .filter(|d| num_factorisations(d, &towels) > 0)
            .count()
    );

    // PART 2
    println!(
        "Result part 2: {}",
        designs
            .iter()
            .map(|d| num_factorisations(d, &towels))
            .sum::<u64>()
    );
}

#[cached(key = "String", convert = r#"{ String::from(design) }"#)]
fn num_factorisations<'a>(design: &str, towels: &Vec<&'a str>) -> u64 {
    // println!("{design}");
    if design == "" {
        return 1;
    }

    let mut total = 0;
    for towel in towels {
        if design.starts_with(towel) {
            total += num_factorisations(&design[towel.chars().count()..], towels);
        }
    }
    return total;
}
