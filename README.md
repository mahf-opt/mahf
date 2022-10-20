# MAHF

A framework for modular construction and evaluation of meta-heuristics.

# Purpose and Features

MAHF aims to make construction and modification of metaheuristics as simple and reliable as possible. In addition to construction it also provides utilities for tracking, evaluation and comparison of those heuristics.

- Simple modular construction of metaheuristics
- State management and state tracking
- Collection of common operators
- Templates for common heuristics
- A basic set of common benchmark problems

# Planned Features

- Extensive collection of components and heuristic templates
- Standardized benchmarking and comparison of heuristics
- First class support for constructive heuristics
- Utilities for parameter optimization
- Utilities for efficient evaluation and comparison
- Variety of examples and clear guidlines
- Documentation for users with no Rust experience

# Requirements

- [The Rust Programming Language](https://rust-lang.org)
- Either `gcc` or `clang`

# Documentation

MAHF has extensive documentation which should make it easy to get started.

Just run
```sh
$ cargo doc --open
```
to build and open the documentation.

# Examples

Examples on how to use MAHF for evaluation can be found in the [examples](/examples) directory.

Examples of heuristics can be found under [heuristics](/src/heuristics/) and components under [operators](/src/operators/).

# Additional Resources

- [Parameter Optimization](/param-study/)

# Papers and Projects using MAHF

None yet.
