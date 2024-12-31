mod days;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (day, use_test_input) = parse_args(args);

    let day_map = initialize_day_map();

    match day_map.get(&day) {
        Some(&func) => func(use_test_input),
        None => eprintln!("Day {} not implemented!", day),
    }
}

fn parse_args(args: Vec<String>) -> (u32, bool) {
    let usage = "Usage: cargo run <day> [-t | --test]";
    if args.len() < 2 {
        eprintln!("{usage}");
        std::process::exit(1);
    }
    
    let day = args[1]
        .replace("day", "")
        .parse()
        .expect("Day must be an integer");
    
    if args.len() == 2 {
        (day, false)
    } else if args.len() == 3 && ["-t", "--test"].contains(&args[2].as_str()) {
        (day, true)
    } else {
        eprintln!("{usage}");
        std::process::exit(1);
    }
}

fn initialize_day_map() -> HashMap<u32, fn(bool)> {
    let mut day_map: HashMap<u32, fn(bool)> = HashMap::new();
    day_map.insert(1, days::day01::run);
    day_map.insert(2, days::day02::run);
    day_map.insert(3, days::day03::run);
    day_map.insert(4, days::day04::run);
    day_map.insert(5, days::day05::run);
    day_map.insert(6, days::day06::run);
    day_map.insert(7, days::day07::run);
    day_map.insert(8, days::day08::run);
    day_map.insert(9, days::day09::run);
    day_map.insert(10, days::day10::run);
    day_map.insert(11, days::day11::run);
    day_map.insert(12, days::day12::run);
    day_map.insert(13, days::day13::run);
    day_map.insert(14, days::day14::run);
    day_map.insert(15, days::day15::run);
    day_map.insert(16, days::day16::run);
    day_map.insert(17, days::day17::run);
    day_map.insert(18, days::day18::run);
    day_map.insert(19, days::day19::run);
    day_map.insert(20, days::day20::run);
    day_map.insert(21, days::day21::run);
    day_map.insert(22, days::day22::run);
    day_map.insert(23, days::day23::run);
    day_map.insert(24, days::day24::run);
    day_map.insert(25, days::day25::run);
    day_map
}
