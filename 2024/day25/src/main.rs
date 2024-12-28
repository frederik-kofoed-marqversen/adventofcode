use std::fs::read_to_string;

fn main() {
    // PARSING
    let input = read_to_string("./input.data").unwrap();

    let size = (5, 7);
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for schematic in input.split("\n\n") {
        let numbers: Vec<usize> = (0..size.0)
            .map(|i| {
                schematic.replace('\n', "")[i..]
                    .chars()
                    .step_by(size.0)
                    .filter(|c| c == &'#')
                    .count()
            })
            .collect();
        if schematic.starts_with('#') {
            locks.push(numbers);
        } else {
            keys.push(numbers);
        }
    }
    // dbg!(&keys, &locks);

    // PART 1
    let pairs: usize = keys
        .iter()
        .map(|key| {
            locks
                .iter()
                .filter(|&lock| std::iter::zip(key, lock).all(|(a, b)| a + b <= size.1))
                .count()
        })
        .sum();

    println!("Result part 1: {pairs}");

    // PART 2
    println!("Result part 2: {}", 0);
}
