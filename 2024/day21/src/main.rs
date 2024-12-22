use cached::proc_macro::cached;
use std::collections::HashMap;

fn main() {
    let test = false;
    let codes = if test {
        ["029A", "980A", "179A", "456A", "379A"]
    } else {
        ["540A", "582A", "169A", "593A", "579A"]
    };

    /*
    The layout of the numeric keypad
    +---+---+---+
    | 7 | 8 | 9 |
    +---+---+---+
    | 4 | 5 | 6 |
    +---+---+---+
    | 1 | 2 | 3 |
    +---+---+---+
        | 0 | A |
        +---+---+
    and WLOG that of the directional keypad:
    +---+---+---+
    | < | v | > |
    +---+---+---+
        | ^ | A |
        +---+---+
    Notice in this form, the two are equivalent. Thus, the same function
    can handle both cases. However, vertical moves on a directional keypad
    must be flipped to reflect the physical layout.

    If we choose origin at 0 and ^ respectively, we get the key property
    that the coordinate that has to be avoided is the same in both cases!
    */

    let mut keypad_map: HashMap<char, (i32, i32)> = HashMap::new();
    // Directional keys
    keypad_map.extend(vec![
        ('^', (0, 0)),
        ('A', (1, 0)),
        ('<', (-1, 1)),
        ('v', (0, 1)),
        ('>', (1, 1)),
    ]);
    // Numeric keys
    keypad_map.insert('0', (0, 0));
    for n in 1..=9 {
        keypad_map.insert(
            char::from_digit(n as u32, 10).unwrap(),
            ((n - 1) % 3 - 1, (n - 1) / 3 + 1),
        );
    }

    // Numeric part of codes
    let numbers: Vec<u32> = codes
        .iter()
        .map(|keys| keys[..3].parse().unwrap())
        .collect();

    // PART 1
    let lengths: Vec<u32> = codes
        .iter()
        .map(|keys| min_sequence_length(keys, &[false, true, true], &keypad_map) as u32)
        .collect();

    println!(
        "Result part 1: {}",
        std::iter::zip(&lengths, &numbers)
            .map(|(a, b)| a * b)
            .sum::<u32>()
    );

    // PART 2
    let mut keypads = vec![true; 26];
    keypads[0] = false;

    let lengths: Vec<u64> = codes
        .iter()
        .map(|keys| min_sequence_length(keys, &keypads, &keypad_map) as u64)
        .collect();

    println!(
        "Result part 2: {}",
        std::iter::zip(&lengths, &numbers)
            .map(|(a, &b)| a * b as u64)
            .sum::<u64>()
    );
}

#[cached(
    key = "(String, usize)",
    convert = r#"{ (String::from(keys), is_dir_kp.len())}"#
)]
fn min_sequence_length(
    keys: &str,
    is_dir_kp: &[bool],
    keypad_map: &HashMap<char, (i32, i32)>,
) -> usize {
    if is_dir_kp.len() == 0 {
        return keys.chars().count();
    }

    let mut result = 0;
    let mut current_key = 'A';
    for next_key in keys.chars() {
        if current_key == next_key {
            // No moves to take, just press the current key
            result += 1;
            continue;
        }

        let (start, end) = (keypad_map[&current_key], keypad_map[&next_key]);
        let dx = end.0 - start.0;
        let dy = end.1 - start.1;
        let xmove = (if dx >= 0 { ">" } else { "<" }).repeat(dx.abs() as usize);
        let ymove = (if (dy >= 0) ^ is_dir_kp[0] { "^" } else { "v" }).repeat(dy.abs() as usize);

        // Find possible ways of reaching the next key (avoiding (-1, 1)), and press it.
        let mut sub_seqs = Vec::new();
        if !(start.1 == 0 && end.0 == -1) {
            let moves = String::new() + &xmove + &ymove + "A";
            sub_seqs.push(min_sequence_length(&moves, &is_dir_kp[1..], keypad_map));
        }
        if !(start.0 == -1 && end.1 == 0) {
            let moves = String::new() + &ymove + &xmove + "A";
            sub_seqs.push(min_sequence_length(&moves, &is_dir_kp[1..], keypad_map));
        }

        // Add the shortest sub_sequence
        result += sub_seqs.iter().min().unwrap();
        // Update current key
        current_key = next_key;
    }

    return result;
}

// This function was my first solution to part 1. It computes an actual minimum
// keypress sequence, not just the length. The above function is nothing but a
// cached copy of this one, reduced to returning only the length, not the contents
// of a minimum keypress sequence.
#[allow(dead_code)]
fn min_sequence(keys: &str, is_dir_kp: &[bool], keypad_map: &HashMap<char, (i32, i32)>) -> String {
    if is_dir_kp.len() == 0 {
        return keys.to_string();
    }

    let mut result = String::new();
    let mut current_key = 'A';
    for next_key in keys.chars() {
        if current_key == next_key {
            // No moves to take, just press the current key
            result.push('A');
            continue;
        }

        let (start, end) = (keypad_map[&current_key], keypad_map[&next_key]);
        let dx = end.0 - start.0;
        let dy = end.1 - start.1;
        let xmove = (if dx >= 0 { ">" } else { "<" }).repeat(dx.abs() as usize);
        let ymove = (if (dy >= 0) ^ is_dir_kp[0] { "^" } else { "v" }).repeat(dy.abs() as usize);

        // Find possible ways of reaching the next key (avoiding (-1, 1)), and press it.
        let mut sub_seqs = Vec::new();
        if !(start.1 == 0 && end.0 == -1) {
            let moves = String::new() + &xmove + &ymove + "A";
            sub_seqs.push(min_sequence(&moves, &is_dir_kp[1..], keypad_map));
        }
        if !(start.0 == -1 && end.1 == 0) {
            let moves = String::new() + &ymove + &xmove + "A";
            sub_seqs.push(min_sequence(&moves, &is_dir_kp[1..], keypad_map));
        }

        // Add the shortest sub_sequence
        result += sub_seqs
            .iter()
            .min_by_key(|moves| moves.chars().count())
            .unwrap();
        // Update current key
        current_key = next_key;
    }

    return result;
}
