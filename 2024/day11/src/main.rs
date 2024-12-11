use cached::proc_macro::cached;

fn main() {
    let input = vec![0, 44, 175060, 3442, 593, 54398, 9, 8101095];

    // dbg!(num_stones(&vec![125, 17], 25));
    println!("Result part 1: {}", num_stones(&input, 25));
    println!("Result part 2: {}", num_stones(&input, 75));
}

fn num_stones(stones: &Vec<u64>, steps: usize) -> usize {
    stones.iter().map(|&stone| num_children(stone, steps)).sum()
}

#[cached]
fn num_children(stone: u64, steps: usize) -> usize {
    if steps == 0 {
        return 1;
    } else {
        return update(stone)
            .into_iter()
            .map(|child| num_children(child, steps - 1))
            .sum();
    }
}

fn update(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }

    let num_digits = stone.ilog10() + 1;
    if num_digits % 2 == 0 {
        return vec![
            stone / 10_u64.pow(num_digits / 2),
            stone % 10_u64.pow(num_digits / 2),
        ];
    } else {
        return vec![stone * 2024];
    }
}
