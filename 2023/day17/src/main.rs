use std::fs::read_to_string;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

type DataStruct = Vec<Vec<usize>>;

fn main() {
    let map = parse_file("./input.data").unwrap();
    // dbg!(map.len(), map[0].len());
    // dbg!(&map);

    // PART 1
    println!("Result part 1: {}", solve(&map, false).unwrap());

    // PART 2
    println!("Result part 2: {}", solve(&map, true).unwrap());

}

fn solve(map: &DataStruct, part2: bool) -> Option<usize> {
    // BinaryHeap is a max-heap wrt. the ordering of State.
    // Here we have defined State such that it is a min-heap in the distance
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    let mut min_dist: HashMap<((usize, usize), &str, u8), usize> = HashMap::new();
    
    let start = State{distance: 0, position: (0, 0), direction: "None", step_num: 0};
    queue.push(start);
    min_dist.insert(((0, 0), "None", 0), 0);
    
    let n = map.len(); // map is square
    while let Some(state) = queue.pop() {
        let (distance, position, direction, step_num) = (state.distance, state.position, state.direction, state.step_num);
        if position == (n-1, n-1) {
            // `queue`` is a min-heap => the first time we encounter end `distance` will be the minimum
            return Some(distance)
        }
        if &distance > min_dist.get(&(position, direction, step_num)).unwrap_or(&usize::MAX) {
            // have found a shorter path since item was added to queue
            continue;
        }
        
        // add all possible next states to queue
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
                    if position.0 == n-1  || direction == "north" {continue;}
                    (position.0 + 1, position.1)
                },
                "west" => {
                    if position.1 == 0  || direction == "east" {continue;}
                    (position.0, position.1 - 1)
                },
                "east" => {
                    if position.1 == n-1  || direction == "west" {continue;}
                    (position.0, position.1 + 1)
                },
                _ => {return None}
            };
            
            let new_distance = distance + map[new_position.0][new_position.1];
            let new_state = State{
                distance: new_distance, 
                position: new_position, 
                direction: new_direction, 
                step_num: new_step_num
            };
            
            if &new_distance < min_dist.get(&(new_position, new_direction, new_step_num)).unwrap_or(&usize::MAX) {
                min_dist.insert((new_position, new_direction, new_step_num), new_distance);
                queue.push(new_state);
            }
        }
    }

    return None
}

fn parse_file(filepath: &str) -> Result<DataStruct, std::io::Error> {
    Ok(read_to_string(filepath)?.trim().split('\n').map(
        |x| x.chars().map(
            |c| c.to_digit(10).unwrap() as usize
        ).collect()
    ).collect())
}

#[derive(Clone, Eq, PartialEq)]
struct State<'a> {
    distance: usize,
    position: (usize, usize),
    direction: &'a str,
    step_num: u8,
}

impl Ord for State<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare other way arround to get a min heap!
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for State<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}