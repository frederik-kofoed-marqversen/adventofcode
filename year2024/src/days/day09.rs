type DiskMap = Vec<(u32, u32, u32)>; // (filename, start, stop)

pub fn run(use_test_input: bool) {
    let input = super::read_input(9, use_test_input);

    // PART 1
    // Compute the data on each disk location from input
    let mut disk = Vec::new();
    for (i, digit) in input.chars().enumerate() {
        let length = digit.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            disk.append(&mut vec![Some(i / 2); length]);
        } else {
            disk.append(&mut vec![None; length]);
        }
    }

    // Compute compacted disk
    let mut index = 0;
    while index < disk.len() {
        if disk[index].is_some() {
            index += 1;
        } else if let Some(Some(file)) = disk.pop() {
            disk[index] = Some(file);
        }
    }
    
    let checksum: usize = disk
        .iter()
        .enumerate()
        .map(|(i, file)| i * file.unwrap())
        .sum();
    println!("Result part 1: {checksum}");

    // PART 2
    // Parse input into a map of non-empty disk space: (filename, start, stop)
    let mut disk_map: DiskMap = DiskMap::new();
    let mut index = 0;
    for (i, digit) in input.chars().enumerate() {
        let length = digit.to_digit(10).unwrap();
        if i % 2 == 0 {
            disk_map.push((i as u32 / 2, index, index + length));
        }
        index += length;
    }
    // dbg!(disk_layout(&disk_map));

    // Compute compacted disk
    let mut compacted_disk_map: DiskMap = disk_map.clone();
    for file in (0..disk_map.len() as u32).rev() {
        // Find file
        let file_index = compacted_disk_map
            .iter()
            .position(|(other, _, _)| other == &file)
            .unwrap();
        let (_, start, stop) = compacted_disk_map[file_index];
        let file_length = stop - start;

        // Look for empty space to the left of the given file
        let empty_disk_space = compacted_disk_map[..file_index]
            .iter()
            .zip(&compacted_disk_map[1..])
            .position(|(a, b)| b.1 - a.2 >= file_length);

        // If found, move file
        if let Some(pre_index) = empty_disk_space {
            let new_start = compacted_disk_map[pre_index].2;
            compacted_disk_map.remove(file_index);
            compacted_disk_map.insert(pre_index + 1, (file, new_start, new_start + file_length));
        }
    }

    // dbg!(disk_layout(&compacted_disk_map));
    println!("Result part 2: {}", compute_checksum(&compacted_disk_map));
}

fn compute_checksum(disk_map: &DiskMap) -> u64 {
    disk_map
        .iter()
        .map(|&(file, start, stop)| (file * (start..stop).sum::<u32>()) as u64)
        .sum()
}

#[allow(dead_code)]
fn disk_layout(disk_map: &DiskMap) -> String {
    let mut result = "".to_string();
    let mut index = 0;
    for &(file, start, stop) in disk_map {
        result += &".".repeat(start as usize - index);
        result += &file.to_string().repeat((stop - start) as usize);
        index = stop as usize;
    }
    return result;
}
