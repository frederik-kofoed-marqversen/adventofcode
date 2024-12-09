use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs::read_to_string;

type DiskMap = Vec<(Option<u32>, u32)>;

fn main() {
    // Parsing
    let input = read_to_string("./input.data").unwrap().replace(':', "");
    let disk_map: DiskMap = input
        .chars()
        .enumerate()
        .map(|(i, digit)| {
            (
                if i % 2 == 0 { Some(i as u32 / 2) } else { None },
                digit.to_digit(10).unwrap(),
            )
        })
        .collect();

    // PART 1
    let mut fracmented_disk_map: DiskMap = Vec::new();
    let mut file_queue = VecDeque::from(disk_map.clone());
    while let Some((data, length)) = file_queue.pop_front() {
        if data.is_some() {
            // Simply put the data on the end of the disk
            fracmented_disk_map.push((data, length));
            continue;
        }

        let free_disk_space = length;
        if let Some((data, length)) = file_queue.pop_back() {
            if data.is_none() {
                file_queue.push_front((None, free_disk_space));
                continue;
            }

            match free_disk_space.cmp(&length) {
                Ordering::Less => {
                    fracmented_disk_map.push((data, free_disk_space));
                    file_queue.push_back((data, length - free_disk_space));
                }
                Ordering::Greater => {
                    fracmented_disk_map.push((data, length));
                    file_queue.push_front((None, free_disk_space - length));
                }
                Ordering::Equal => {
                    fracmented_disk_map.push((data, length));
                }
            }
        }
    }
    // dbg!(disk_layout(&fracmented_disk_map));
    println!("Result part 1: {}", checksum(&fracmented_disk_map));

    // PART 2
    let mut compacted_disk_map: Vec<(Option<u32>, u32)> = disk_map.clone();
    for (data, length) in disk_map.iter().rev() {
        if data.is_none() {
            // No file to be moved
            continue;
        }

        let index = compacted_disk_map
            .iter()
            .position(|(other, _)| other == data)
            .unwrap();

        let disk_space = compacted_disk_map[..index]
            .iter()
            .position(|(data, space)| data.is_none() && space >= length);

        if let Some(disk_space_index) = disk_space {
            // Disk space found that fits data => move data here
            let free_disk_space = compacted_disk_map[disk_space_index].1;

            compacted_disk_map.remove(index);

            match free_disk_space.cmp(&length) {
                Ordering::Less => {
                    // Should never enter here
                    panic!("Err")
                }
                Ordering::Greater => {
                    compacted_disk_map[disk_space_index] = (*data, *length);
                    compacted_disk_map
                        .insert(disk_space_index + 1, (None, free_disk_space - length));
                }
                Ordering::Equal => {
                    compacted_disk_map[disk_space_index] = (*data, *length);
                }
            }
            
            // should combine now consecutive empty disk spaces if present
        }
    }

    // dbg!(disk_layout(&compacted_disk_map));
    println!("Result part 2: {}", checksum(&compacted_disk_map));
}

#[allow(dead_code)]
fn disk_layout(disk_map: &Vec<(Option<u32>, u32)>) -> String {
    disk_map
        .iter()
        .map(|(data, length)| {
            data.and_then(|num| Some(num.to_string()))
                .unwrap_or(".".to_string())
                .repeat(*length as usize)
        })
        .fold("".to_string(), |acc, s| acc + &s)
}

fn checksum(disk_map: &Vec<(Option<u32>, u32)>) -> u64 {
    let mut result: u64 = 0;
    let mut index: u32 = 0;
    for (data, length) in disk_map {
        index += length;
        if data.is_some() {
            result += (data.unwrap() * (index - length..index).sum::<u32>()) as u64;
        }
    }
    return result;
}
