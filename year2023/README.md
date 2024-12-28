# Advent of Code 2024 Solutions

## Highlights

- **Day 06**: Easy when noticing that the quadratic equation of motion can be solved analytically.
- **Day 08**: Standard cycle detection puzzle. Then using the least common multiple (LCM) to find when they coincide.
- **Day 10**: Very proud of how I handled this puzzle. I wanted to calculate the internal area as a line integral while walking along the perimiter. Initially I could not figure out how to take into acount the width of the pipes, so I had the alternative and also great idea to use the Jordan Curve lemma: Rays eminating from internal points will intersect the path an even number of times, then simply checking every point individually. However, the day after I got the line integral method to work.
- **Day 12**: Learned about caching for solving this type of problems after reviewing the Reddit Solution Megathread.
- **Day 14**: Tilting tables. Yet another cycle detection puzzle. Solution could be much nicer if tilting each direction was handled by a single generalised function, instead of four separate functions.
- **Day 17**: A*-algorithm.
- **Day 18**: Almost free after having done all the clever thinking on day 10.
- **Day 20**: An exercise in studying the details of the input, using that knowledge to solve the puzzle.
- **Day 21**: I spend way too long trying to solve the problem semi-analytically, before consulting the Megathread finding out that the solution was fitting and extrapolating.
- **Day 24**: Got to learn how to use the crate ndarray for linear algebra.

## Running a Solution

1. Create the directory `year2024/input/real/` and place the input files there, named `dayXX.txt`.
2. To run a specific day's solution, use the following command, specifying the day and the test flag if needed:

    ```bash
    cargo run [--release] <day> [-t | --test]
    ```
Replace <day> with the day number (e.g., 1 for Day 1) and use the -t or --test flag to run with test input instead of the real input.