pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

/// Reads the input for a given day, optionally using the test input.
/// `day` is the day number (e.g. 1 for day01).
/// `read_test_input` is a boolean indicating whether to use the test input (true) or real input (false).
pub fn read_input(day: u32, read_test_input: bool) -> String {
    let project_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let folder = if read_test_input { "test" } else { "real" };
    let name_extention = if read_test_input { "_test" } else { "" };
    let filename = format!("{project_dir}/input/{folder}/day{day:02}{name_extention}.txt");
    std::fs::read_to_string(&filename).expect(&format!("Failed to read input: {filename}"))
}