# Advent of Code 2025 Solutions

First year with only 12 problems; used it to learn C++.

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
  Finally got arround to trying Z3. Part 1 was a straightforward BFS, but part 2 was best expressed as an integer linear program (minimise sum(x) subject to Ax = b, x ≥ 0). Across instances the systems were both over- and under‑constrained, so solving over the reals and rounding wasn’t reliable; Z3 handled it cleanly. Also learned that using regex in C++ adds noticeable compile time without a big readability gain over simple string parsing.

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