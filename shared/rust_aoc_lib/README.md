# `rust_aoc_lib` Crate for Rust

The `rust_aoc_lib` crate is a library of helpful functionality for solving Advent of Code (AOC) problems in Rust. It was initially developed during the 2023 season and is intended to evolve over time as more functionality is added during future seasons of participation.

## Current Modules

### 1. `rust_aoc_lib::graph`

This module provides utilities for graph manipulation and computation. It includes:

- **`graph::Graph`**: A simple graph structure designed to mimic Python's `networkx`.
- **`graph::algorithms` (alias: `graph::algs`)**: Implements generic graph algorithms, such as:
  - **`a_star`**: A* pathfinding algorithm.
  - **`max_flow`**: Implementation of the Edmonds-Karp algorithm for computing maximum flow in a flow network.
  - **`min_cut`**: Functionality for finding the minimum cut of a graph.

### 2. `rust_aoc_lib::complex`

This module implements a very minimal complex number struct. It is particularly useful for:
- Complex arithmetic.
- Performing 2D vector operations.

### 3. `rust_aoc_lib::multivec`

This module implements 2D multivectors (`multivec::Multivec2D`) inspired by geometric algebra. It is, for the most part, replacable by the module for complex numbers.

### 4. `rust_aoc_lib::utility_functions` (alias: `rust_aoc_lib::utils`)

A collection of general-purpose utility functions with broad applicability, including:
- **`gcd`**: Computes the greatest common divisor using the Euclidean algorithm.
- **`crt_solve`**: Solves systems of congruence relations using the Chinese Remainder Theorem.
- **`extrapolate`**: Performs polynomial extrapolation using Newton's forward difference formula.

---

This crate is a work in progress, and contributions or suggestions for additional functionality are always welcome.
