# Advent of Code 2024 Solutions

## Highlights

- **Day 03**: Leveraged clever regular expressions for parsing tricky inputs.
- **Day 05**: A simple problem, solved using `Vec::sort_by()` with a custom comparator.
- **Day 06**: Part 2 is solved simply by brute-force and is quite slow; taking over 29 seconds in debug mode and around 2 seconds in release mode.
- **Day 13**: Used `Regex::new(r"\d+")` to easily extract numbers from the input file.
- **Day 14**: This puzzle was particularly interesting. Part two involved detecting an image among multiple noisy frames by evaluating an entropy measure.
- **Day 19**: Proud of this very short solution, involving a custom cache key that ignores certain arguments.
- **Day 23**: The NP-hard maximum clique problem solved with a brute force algorithm, now part of the graph module in the shared library.
- **Day 24**: This one stumped me for a while. My solution requires manual inspection. I plan to revisit it once I learn how to use Z3.

## Running a Solution

1. Create the directory `year2024/input/real/` and place the input files there, named `dayXX.txt`.
2. To run a specific day's solution, use the following command, specifying the day and the test flag if needed:

    ```bash
    cargo run [--release] <day> [-t | --test]
    ```
Replace <day> with the day number (e.g., 1 for Day 1) and use the -t or --test flag to run with test input instead of the real input.