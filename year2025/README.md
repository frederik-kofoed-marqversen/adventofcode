# Advent of Code 2025 Solutions

## Notes
This is the first year with only 12 problems. I used this year as an excuse to get started with C++.

## Structure

- **`days/`**: Solution files for each day (`day01.cpp`, `day02.cpp`, etc.).
- **`input/`**: Git-ignored puzzle inputs.
  - `dayNN.txt` (real input)
  - `dayNN_test.txt` (test input)
- **`bin/`**: Compiled executables (git-ignored).

## Highlights

- **Day NN**: Something interesting about the problem

## Adding a New Solution

To add a new solution, simply run `./setup_day.sh NN` with `NN` the number of the day:

This will create:
- `days/day03.cpp` with a template
- `input/day03.txt` (empty, for your puzzle input)
- `input/day03_test.txt` (empty, for test input)

## Building and Running Solutions

1. Place inputs in `input/dayNN.txt` and `input/dayNN_test.txt`.
2. Run the solution:

```bash
# Run with real input
make run DAY=01 [TEST=1]
```