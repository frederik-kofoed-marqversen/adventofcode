use std::collections::{HashMap, VecDeque};

pub fn run(use_test_input: bool) {
    let input = super::read_input(18, use_test_input);
    let (bytes, size) = if use_test_input { (12, 6) } else { (1024, 70) };

    let mut coords = Vec::new();
    for line in input.lines() {
        let args = line.split(',').map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        coords.push((args[0], args[1]))
    }

    let start = (0, 0);
    let end = (size, size);

    // PART 1
    println!("Result part 1: {}", find_path(start, end, &coords[..bytes]).unwrap());

    // PART 2
    // Binary search
    let mut l = bytes;
    let mut r = coords.len();
    while l < r {
        let m = (l + r) / 2;
        if find_path(start, end, &coords[..m]).is_some() {
            l = m + 1;
        } else {
            r = m
        }
    }

    let coord = coords[l as usize - 1];
    println!("Result part 2: {},{}", coord.0, coord.1);
}

fn find_path(start: (i32, i32), end: (i32, i32), corrupted: &[(i32, i32)]) -> Option<i32> {
    let mut data = HashMap::new();
    let mut queue = VecDeque::from(vec![(start, 0)]);
    while let Some((pos, cost)) = queue.pop_front() {
        if pos == end {
            return Some(cost);
        }

        data.entry(pos).or_insert(i32::MAX);
        if data[&pos] > cost {
            data.insert(pos, cost);

            for step in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let next = (pos.0 + step.0, pos.1 + step.1);
                if next.0 > end.0
                    || next.0 < 0
                    || next.1 > end.1
                    || next.1 < 0
                    || corrupted.contains(&next)
                {
                    continue;
                } else {
                    queue.push_back((next, cost + 1));
                }
            }
        }
    }
    return None
}
