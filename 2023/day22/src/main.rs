use std::fs::read_to_string;
use std::collections::HashSet;

type Brick = [Interval; 3];

fn main() {
    let mut snapshot = parse_file("./input.data").unwrap();
    // dbg!(&snapshot);

    // Sort bricks from lowest to highest
    snapshot.sort_by_key(|brick| brick[2].lower);

    // Compute pile of bricks after falling to ground at z=0;
    let mut brick_pile: Vec<Brick> = Vec::new(); // Bricks after falling
    let mut supports: Vec<Vec<usize>> = Vec::new(); // `supports[i]` are the indices of bricks supported by i
    let mut supported_by : Vec<Vec<usize>> = Vec::new(); // `supported_by[i]` are the indices of bricks that support i
    for brick in &snapshot {
        // Index of bricks in fall path of current brick
        let mut hits = Vec::new();
        for (i, other) in brick_pile.iter().enumerate() {
            if brick[0].intersection(&other[0]).volume() > 0 && brick[1].intersection(&other[1]).volume() > 0 {
                hits.push(i);
            }
        }

        // If none below, brick falls to the ground
        let final_height = hits.iter().map(|&i| brick_pile[i][2].upper + 1).max().unwrap_or(1);
        
        // Current brick is supported by bricks one below its final height
        let support: Vec<usize> = hits.into_iter().filter(|&i| brick_pile[i][2].upper == final_height-1).collect();
        supported_by.push(support.clone());
        
        // Current brick does not support any of the previous bricks
        supports.push(Vec::new());
        // Add current brick to list of supported bricks
        for i in support {
            supports[i].push(brick_pile.len());
        }
    
        brick_pile.push([
            brick[0],
            brick[1],
            Interval {lower: final_height, upper: final_height + brick[2].volume() - 1}
        ]);
    }
    // dbg!(&brick_pile, &supports, &supported_by);    
    
    // PART 1
    let mut disintegratable = Vec::new();
    for i in 0..brick_pile.len() {
        if supports[i].iter().all(|&j| supported_by[j].len() > 1) {
            disintegratable.push(i);
        }
    }
    // dbg!(&disintegratable);

    println!("Result part 1: {}", disintegratable.len());


    // PART 2
    let mut will_fall_if_disintegrated = Vec::new();
    for i in 0..brick_pile.len() {
        let mut will_fall = HashSet::new();
        will_fall.insert(i);

        for j in i+1..brick_pile.len() { // check if j will fall
            if supported_by[j].len() == 0 {continue;} // brick lies on ground and will not fall
            if supported_by[j].iter().all(|k| will_fall.contains(k)) {
                will_fall.insert(j);
            }
        }

        will_fall.remove(&i);
        will_fall_if_disintegrated.push(will_fall);
    }
    // dbg!(&will_fall_if_disintegrated);

    println!("Result part 2: {}", will_fall_if_disintegrated.iter().fold(0, |sum, list| sum + list.len()));

}

#[derive(Debug, Clone, Copy)]
struct Interval {
    lower: u32,
    upper: u32,
}

impl Interval {
    fn new(a: u32, b: u32) -> Self {
        if a <= b {
            Self {lower: a, upper: b}
        } else {
            Self {lower: b, upper: b}
        }
    }

    fn intersection(&self, other: &Self) -> Self {
        return Self {
            lower: self.lower.max(other.lower),
            upper: self.upper.min(other.upper),
        }
    }

    fn volume(&self) -> u32 {
        if self.lower > self.upper {
            0
        } else {
            self.upper - self.lower + 1
        }
    }
}

fn parse_file(filepath: &str) -> Result<Vec<Brick>, std::io::Error> {
    let input = read_to_string(filepath)?;
    let mut result = Vec::new();
    for line in input.trim().split('\n') {
        let endpoints: Vec<Vec<u32>> = line.split('~').map(
            |coords| coords.split(',').map(|n| n.parse().unwrap()).collect()
        ).collect();
        let brick = [
            Interval::new(endpoints[0][0], endpoints[1][0]),
            Interval::new(endpoints[0][1], endpoints[1][1]),
            Interval::new(endpoints[0][2], endpoints[1][2]),
        ];
        result.push(brick);
    }
    
    return Ok(result)
}