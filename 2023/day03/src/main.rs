use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools as it;

fn main() {    
    let schematic = parse_schematic("./input.data").unwrap();
    let shape = (schematic.len(), schematic[0].len());

    // PART 1
    let mut part_nums: Vec<u32> = Vec::new();
    for (i, row) in schematic.iter().enumerate() {
        let mut is_part = false;
        let mut num = String::new();
        for (j, char) in row.iter().enumerate() {
            if char.is_digit(10) {
                num.push(*char);
                
                for index in neightbours((i, j), shape) {
                    let neighbour = schematic[index.0][index.1];
                    if is_symbol(neighbour) {
                        is_part = true;
                    }
                }
            } else {
                if is_part {
                    part_nums.push(num.parse().unwrap());
                }
                is_part = false;
                num = String::new();
            }
        }
        if is_part { // the line ends with a part number
            part_nums.push(num.parse().unwrap());
        }
    }

    println!("Result part 1: {}", part_nums.iter().sum::<u32>());

    // PART 2
    let mut gear_ratios: Vec<u32> = Vec::new();
    for (i, row) in schematic.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if char == &'*' {
                let mut numbers = Vec::new();

                if j > 0 && schematic[i][j-1].is_digit(10) {
                    numbers.push(get_num(&schematic, (i, j-1)));
                }
                if j < schematic[i].len()-1 && schematic[i][j+1].is_digit(10) {
                    numbers.push(get_num(&schematic, (i, j+1)));
                }
                if i > 0 {
                    if schematic[i-1][j].is_digit(10) {
                        numbers.push(get_num(&schematic, (i-1, j)));
                    } else {
                        if schematic[i-1][j-1].is_digit(10) {
                            numbers.push(get_num(&schematic, (i-1, j-1)));
                        }
                        if schematic[i-1][j+1].is_digit(10) {
                            numbers.push(get_num(&schematic, (i-1, j+1)));
                        }
                    }
                }
                if i < schematic.len()-1 {
                    if schematic[i+1][j].is_digit(10) {
                        numbers.push(get_num(&schematic, (i+1, j)));
                    } else {
                        if schematic[i+1][j-1].is_digit(10) {
                            numbers.push(get_num(&schematic, (i+1, j-1)));
                        }
                        if schematic[i+1][j+1].is_digit(10) {
                            numbers.push(get_num(&schematic, (i+1, j+1)));
                        }
                    }
                }

                if numbers.len() == 2 {
                    gear_ratios.push(numbers[0].unwrap() * numbers[1].unwrap());
                }
            }
        }

    }

    println!("Result part 2: {}", gear_ratios.iter().sum::<u32>());
}

fn neightbours(index: (usize, usize), shape: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let (i, j) = index;
    let mut i_steps = vec![(i as i32 - 1) as usize, i, i + 1];
    let mut j_steps = vec![(j as i32 - 1) as usize, j, j + 1];

    if i == 0 {
        i_steps.remove(0);
    } else if i == shape.0 - 1 {
        i_steps.remove(2);
    }
    if j == 0 {
        j_steps.remove(0);
    } else if j == shape.1 - 1 {
        j_steps.remove(2);
    }

    return it::iproduct!(i_steps, j_steps);
}

fn get_num(schematic: &Vec<Vec<char>>, index: (usize, usize)) -> Option<u32> {
    let row = &schematic[index.0];
    let mut j = index.1;
    if !row[j].is_digit(10) {
        return None;
    }
    
    let mut string = String::new();
    while j > 0 && row[j - 1].is_digit(10) {
        j -= 1;
    }
    while j < row.len() && row[j].is_digit(10) {
        string.push(row[j]);
        j += 1;
    }

    let num: u32 = string.parse().unwrap();
    return Some(num);
}

fn is_symbol(char: char) -> bool {
    if char.is_digit(10) || char == '.' {
        return false;
    } else  {
        return true
    }
}

fn parse_schematic(filepath: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
    let file = File::open(filepath)?;
    let lines = BufReader::new(file).lines();
    
    let mut result = Vec::new();
    for line in lines {
        let string = line?;
        let chars = string.chars().collect();
        result.push(chars);
    }

    return Ok(result);
}