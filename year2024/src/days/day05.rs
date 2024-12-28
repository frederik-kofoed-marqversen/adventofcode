use std::cmp::Ordering;
use std::collections::HashMap;

pub fn run(use_test_input: bool) {
    let input = super::read_input(5, use_test_input);
    let input = input.split_once("\n\n").unwrap();

    let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();
    for rule in input.0.lines() {
        let (key, value) = rule.split_once('|').unwrap();
        rules
            .entry(key)
            .and_modify(|vec| vec.push(value))
            .or_insert(vec![value]);
    }
    // dbg!(rules);

    // Comparator of page numbers
    let compare = |a: &&str, b: &&str| -> Ordering {
        if rules.get(a).is_some_and(|vec| vec.contains(b)) {
            return Ordering::Less;
        }
        if rules.get(b).is_some_and(|vec| vec.contains(a)) {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    };

    // Sort all updates
    let mut correct_updates: Vec<Vec<&str>> = Vec::new();
    let mut incorrect_updates: Vec<Vec<&str>> = Vec::new();
    for pages in input.1.lines() {
        let numbers: Vec<&str> = pages.split(',').collect();
        let mut sorted = numbers.clone();
        sorted.sort_by(compare);

        if numbers == sorted {
            correct_updates.push(numbers);
        } else {
            incorrect_updates.push(sorted);
        }
    }

    // PART 1
    let part1: u32 = correct_updates
        .iter()
        .map(|vec| vec[vec.len() / 2])
        .map(|s| s.parse::<u32>().unwrap())
        .sum();
    println!("Result part 1: {part1}");

    // PART 2
    let part2: u32 = incorrect_updates
        .iter()
        .map(|vec| vec[vec.len() / 2])
        .map(|s| s.parse::<u32>().unwrap())
        .sum();
    println!("Result part 2: {part2}");
}
