# Advent of Code 2025 Solutions

This is the first year with only 12 problems. I used this installment of AoC as an excuse to get started with C++.

## Structure

- **`days/`**: Solution files for each day (`day01.cpp`, `day02.cpp`, etc.).
- **`input/`**: Git-ignored puzzle inputs.
  - `dayNN.txt` (real input)
  - `dayNN_test.txt` (test input)
- **`bin/`**: Compiled executables (git-ignored).

## Highlights

- **Day 08**: First problem this year that had some interesting strategies. After my initial solve, I read about DSU (Disjoint Set Union) which significantly sped up the solution.
- **Day 09**: Particularly interesting to me since the problem can be reduced it to an intersection test between an axis-aligned bounding box and a line segment which has direct associations to my ray-tracing project.
- **Day 10**: 
  Was not able to solve part 2. Simply applying BFS as in part 1 did not work. Then realised that the problem could be stated as solving the system of equations Ax = b. However, the columns of A (the buttons) are not linearly independent and thus the problem becomes an optimisation with objective that is the sum of x and constraint Ax = b. In other words, this is an integer linear programming problem that could be solved with e.g. Z3. Not wanting to bring in a large third party library, I consulted the Solution Megathread, only to find that this is the common solution. However, a few people did manage to optimise a DFS to a point where a solution is reached in reasonable time. Maybe I will return to this problem at some point.

  Also learned that RegEx with C++ adds A LOT of compilation time. It is also quite verbose and thus does not lead to a significantly nicer solution compared to simply string parsing.

## Adding a New Solution

To add a new solution, simply run `./setup_day.sh NN` with `NN` the number of the day:

This will create:
- `days/dayNN.cpp` with a template
- `input/dayNN.txt` (empty, for your puzzle input)
- `input/dayNN_test.txt` (empty, for test input)

## Building and Running Solutions

1. Place inputs in `input/dayNN.txt` and `input/dayNN_test.txt`.
2. Run the solution:

```bash
make run DAY=01 [TEST=1]
```