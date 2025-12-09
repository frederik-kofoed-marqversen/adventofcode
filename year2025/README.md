# Advent of Code 2025 Solutions

This is the first year with only 12 problems. This year I used AoC as an excuse to get started with C++.

## Structure

- **`days/`**: Solution files for each day (`day01.cpp`, `day02.cpp`, etc.).
- **`input/`**: Git-ignored puzzle inputs.
  - `dayNN.txt` (real input)
  - `dayNN_test.txt` (test input)
- **`bin/`**: Compiled executables (git-ignored).

## Highlights

- **Day 08**: First problem this year that had some interesting strategies. After my initial solve, I learned about DSU (Disjoint Set Union) which significantly speeds up the solution.
- **Day 09**: Particularly interesting to me since the problem can be reduced it to an intersection test between an axis-aligned bounding box and a line segment which I am very familiar with via my ray-tracing project.

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