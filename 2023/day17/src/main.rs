use std::fs::read_to_string;
use std::collections::HashMap;

use aoc::graph::algorithms::{a_star, Traversible};

fn main() {
    let data = parse_file("./input.data").unwrap();
    let n = data.len(); // Maps are square.
    let map = Map { data, n };
    
    let start_pos = (0, 0);
    let end_pos = (n-1, n-1);
    
    let mut start = State {position: start_pos, direction: "None".to_string(), step_num: 0, part2: false};
    let end_condition = |state: &State| state.position == end_pos;
    let manhattan = |state: &State| (state.position.0.abs_diff(end_pos.0) + state.position.1.abs_diff(end_pos.1)) as u64;

    // PART 1
    start.part2 = false;
    println!("Result part 1: {}", a_star(&map, &start, &end_condition, Some(&manhattan)).unwrap().0);

    // PART 2
    start.part2 = true;
    println!("Result part 2: {}", a_star(&map, &start, &end_condition, Some(&manhattan)).unwrap().0);

}

fn parse_file(filepath: &str) -> Result<Vec<Vec<u64>>, std::io::Error> {
    Ok(read_to_string(filepath)?.trim().split('\n').map(
        |x| x.chars().map(
            |c| c.to_digit(10).unwrap() as u64
        ).collect()
    ).collect())
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    position: (usize, usize),
    direction: String,
    step_num: u8,
    part2: bool,
}

struct Map {
    data: Vec<Vec<u64>>,
    n: usize,
}

impl Traversible<State, u64> for Map {
    fn connections(&self, state: &State) -> HashMap<State, u64> {
        let mut result = HashMap::new();

        let (position, direction) = (&state.position, &state.direction);
        let step_num = state.step_num;
        let part2 = state.part2;
        for new_direction in ["north", "south", "east", "west"] {
            let new_step_num = if new_direction == direction {step_num + 1} else {1};
            
            if part2 && step_num < 4 && new_direction != direction && direction != "None" {continue;}
            if (!part2 && new_step_num > 3) || (part2 && new_step_num > 10) {continue;}
            
            // must stay on map and no turn arounds
            let new_position = match new_direction {
                "north" => {
                    if position.0 == 0 || direction == "south" {continue;}
                    (position.0 - 1, position.1)
                },
                "south" => {
                    if position.0 == self.n-1  || direction == "north" {continue;}
                    (position.0 + 1, position.1)
                },
                "west" => {
                    if position.1 == 0  || direction == "east" {continue;}
                    (position.0, position.1 - 1)
                },
                "east" => {
                    if position.1 == self.n-1  || direction == "west" {continue;}
                    (position.0, position.1 + 1)
                },
                _ => {return result}
            };
            
            let new_state = State {
                position: new_position, 
                direction: new_direction.to_string(), 
                step_num: new_step_num,
                part2: part2,
            };

            result.insert(new_state, self.data[new_position.0][new_position.1]);
        }

        return result
    }
}