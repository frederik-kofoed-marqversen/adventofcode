## Crate [`aoc`](/aoc) for Rust
During the season of 2023 I started building a library with helpful functionality for solving AOC problems in Rust. It is my intention to keep adding to this as I participate in more seasons. 

### Current modules:

1. `aoc::graph`

This module is for manipulations and computations with graphs. It includes a simple graph struct `graph::Graph` implemented such that it mimics that of pythons networkx. The module includes submodule `graph::algorithms` or `graph::algs` which implements genereic algorithms. This currently includes `a_star` for pathfinding, implementation of Edmond-Karp `max_flow`, and `min_cut` for computing minimum cut.

2. `aoc::multivec`

A module implementing 2D multivecors `multivec::Multivec2D` as known from geometric algebra. This is useful for performing 2D vector operations and also complex arithmetic.

3. `aoc::utility_functions` or simply `aoc::utils`

This is a collection of useful functions with wide applicability. It includes implementations like the Euclidean algorithm `gcd` for computing greatest common divisor, a solver for systems of congruence relations `crt_solve` using the Chinese remainder theorem, and a function for polynomial extrapolation `extrapolate` using Newton's forward difference formula.
