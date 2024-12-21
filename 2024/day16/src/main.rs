use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    pos: (i32, i32),
    dir: (i32, i32),
}

struct Maze {
    data: Vec<Vec<char>>,
}

fn main() {
    // PARSING
    let input = read_to_string("./input.data").unwrap();
    let maze = Maze {
        data: input.lines().map(|line| line.chars().collect()).collect(),
    };

    // Start and end turn out to be in the corners for both input and test data. Also, 
    // for input the end can only be reached by a single direction => single end state!
    // This simplifies matters a bit.
    let start = State {
        pos: (maze.data.len() as i32 - 2, 1),
        dir: (0, 1),
    };
    let end = State {
        pos: (1, maze.data[0].len() as i32 - 2),
        dir: (-1, 0),
    };

    // Label all reachable states by min score to going there, together with all possible
    // predecessors resulting in that score.
    // Thus, data holds tuples (shortest_distance, vec![predecessors]) for each state.
    let data = compute_scores(&maze, start, end);

    // PART 1
    println!("Result part 1: {}", data[&end].0);

    // PART 2
    let mut queue: Vec<State> = vec![end];
    // For each state in the queue, add the position to visited, and add its predecessors
    // to the queue to be handled later.
    let mut visited = HashSet::new();
    while let Some(state) = queue.pop() {
        visited.insert(state.pos);
        let mut predecessors = data[&state].1.clone();
        queue.append(&mut predecessors);
    }

    println!("Result part 2: {}", visited.len());
}

impl Maze {
    fn connections(&self, state: &State) -> Vec<(State, i32)> {
        // Takes State and returns all possible next positions together with the direction,
        // and the cost of going there.
        let mut result = Vec::new();
        for step in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let next_pos = (state.pos.0 + step.0, state.pos.1 + step.1);
            let next_state = State {
                pos: next_pos,
                dir: step,
            };

            if self.data[next_pos.0 as usize][next_pos.1 as usize] != '#' {
                let cost = if next_state.dir == state.dir { 1 } else { 1001 };
                result.push((next_state, cost));
            }
        }
        return result;
    }
}

fn compute_scores(maze: &Maze, start: State, end: State) -> HashMap<State, (i32, Vec<State>)> {
    let mut data = HashMap::<State, (i32, Vec<State>)>::new();
    data.insert(start, (0, vec![]));

    let mut queue = VecDeque::<State>::from(vec![start]);
    while let Some(state) = queue.pop_front() {
        if state == end {
            continue;
        }
        for (next_state, cost) in maze.connections(&state) {
            // Score at next state through current state.
            let s = data[&state].0 + cost;
            // If data entry does not exist, fill with default value.
            let entry = data.entry(next_state).or_insert((i32::MAX, vec![]));

            if s == entry.0 {
                // This path is a new equally cheap path. Add predecessor.
                entry.1.push(state);
            } else if s < entry.0 {
                // This path is shorter than previous best.
                entry.0 = s;
                entry.1 = vec![state];
                queue.push_back(next_state);
            } else {
                // Path is longer, so stop here
                continue;
            }
        }
    }

    return data;
}
