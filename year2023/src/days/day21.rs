use std::collections::HashSet;

type DataStruct = Vec<Vec<char>>;

pub fn run(use_test_input: bool) {
    let input = super::read_input(21, use_test_input);
    let map: DataStruct = input.trim().lines().map(|line| line.chars().collect()).collect();
    let n = map.len() as i32; // Maps are square

    // Find starting position
    let mut start = None;
    for (i, row) in map.iter().enumerate() {
        if row.contains(&'S') {
            let j = row.iter().position(|x| x == &'S').unwrap();
            start = Some((i as i32, j as i32));
            break;
        }
    }
    let start = match start {
        Some(position) => position,
        None => panic!("Starting position not found"),
    };
    // dbg!(&start, &n);

    // PART 1 + PART 2

    // Observations (not all apply to the test case):
    // The map is an odd sized square (131 x 131)
    // The start position 'S' is at the center of the map (66, 66) which is 65 steps from the edges
    // There are no stones in neither row nor collumn of the start position

    // There is a wide open path whose centerline coincide with the 90degree rotated inscribed square.

    // Positions are subject to a parity condition:
    //     A position is reachable by either an even or an odd number of steps
    //     Asuming no obstructions, the number of steps to reach a square is the Manhattan distance
    //     The parity of a position (n, m) is thus equal to the parity of the number n+m

    // The number of steps `N` = 26501365 is chosen carefully: (26501365 - 65) % 131 = 0

    // Conclusions:
    // Once the number of steps taken equals 65, the distance to the edge of the map, all reacable positions
    //     must lie within a Manhattan distance of 65. Due to the wide passage, there will be no positions
    //     that are behind some "wall". Thus, all squares within that distance with odd parity ARE reachable!
    // Due to the carefully chosen number of steps `N`, after the last step has been taken, we will have
    //     exactly reached the edge of some unit cell of the periodic map. Notice `N` is odd.

    // ---------------------------------------------
    // I started deriving the closed-form formula for the number of positions for each stepnum given by
    //     65 + n*133. After struggeling for a long time i visited the Reddit where people were talking
    //     about fitting. From the above observations it is clear that the formula will be a simple
    //     polynomial. It must be at most degree 2! due to the total number scaling with n^2
    // ---------------------------------------------

    let to_index = |i: i32| {
        let res = i % n;
        if res >= 0 {
            res as usize
        } else {
            (res + n) as usize
        }
    };

    let mut numbers = Vec::new();
    let mut positions = HashSet::from([start]);
    let mut num_steps = 0;
    while numbers.len() < 3 {
        // Need at least three points to determine second order polynomial
        num_steps += 1;
        let mut new_positions = HashSet::new();
        for pos in positions {
            for step in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (i, j) = (pos.0 + step.0, pos.1 + step.1);
                if map[to_index(i)][to_index(j)] != '#' {
                    new_positions.insert((i, j));
                }
            }
        }
        positions = new_positions;

        // Save the numbers
        if (num_steps - n / 2) % n == 0 {
            numbers.push(positions.len());
        }
        if num_steps == 64 {
            println!("Result part 1: {}", positions.len());
        }
    }

    // Using Newtons finite difference method to determine polynomial
    // a1  a2  a3  a4... ai
    //   b1  b2  b3...
    //     c1  c1...
    // p(i) = (i choose 0) a1 + (i choose 1) b1 + (i choose 2) c1
    // (n chose k+1) = (n-k)/(k+1) * (i choose k)

    let num_steps = 26501365;
    let i = (num_steps / n) as usize;

    let coeffs = [
        numbers[0],
        numbers[1] - numbers[0],
        numbers[2] - 2 * numbers[1] + numbers[0],
    ];
    let binoms = [1, i, (i * (i - 1)) / 2];

    let result = coeffs.iter().zip(binoms).fold(0, |res, (a, b)| res + a * b);
    println!("Result part 2: {result}");

    // ---------------------------------------------
    // Thoughts related to deriving the formula
    // ---------------------------------------------
    // After 131 steps, the center cell is guarantied to be a filled cell. This cell will contribute all
    //     it's odd parity reachable positions.
    // It takes 131 steps to go from the center of one cell to the center of a neighbour. This the
    //     parity positions in neighbouring cells is flipped
    // For each additional 131 steps another layer of cells will be filled. These cells will contribute
    //     their even parity reachable positions.
    // The size of an added layer after n number of 131 steps is 4*n (easily seen from drawing)

    // For a unit cell we cannot simply count the number of non-stone positions with some parity
    //     due to unreachable positions such as the center of the following pattern
    //     .....
    //     ..#..
    //     .#.#.
    //     ..#..
    //     .....
    // These numbers must therefore be computed explicitly by running the steps on a unit cell
}
