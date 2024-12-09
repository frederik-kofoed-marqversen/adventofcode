use std::fs::read_to_string;

type DiskMap = Vec<(u32, u32, u32)>; // (filename, start, stop)

fn main() {
    // Parsing
    let input = read_to_string("./input.data").unwrap().replace(':', "");

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

    // PART 1
    let mut compacted_disk_map = disk_map.clone();

    loop {
        // Find free disk space between files
        if let Some(index) = compacted_disk_map
            .iter()
            .zip(&compacted_disk_map[1..])
            .position(|(a, b)| b.1 - a.2 > 0)
        {
            // Found free disk space
            let space_start = compacted_disk_map[index].2;
            let space_stop = compacted_disk_map[index + 1].1;
            let free_disk_space = space_stop - space_start;
            
            // Fetch rightmost file
            let (file, start, stop) = compacted_disk_map.pop().unwrap();
            let file_length = stop - start;
            
            // Move data from file to free disk space
            let moved_data = u32::min(free_disk_space, file_length);
            compacted_disk_map.insert(index + 1, (file, space_start, space_start + moved_data));
            if moved_data < file_length {
                // No space for the entire file. Reinsert remaining data from file.
                compacted_disk_map.push((file, start, start + file_length - moved_data))
            }
        } else {
            // No more roome for moving files
            break;
        }
    }
    // dbg!(disk_layout(&compacted_disk_map));
    println!("Result part 1: {}", checksum(&compacted_disk_map));

    // PART 2
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
    println!("Result part 2: {}", checksum(&compacted_disk_map));
}

fn checksum(disk_map: &DiskMap) -> u64 {
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
