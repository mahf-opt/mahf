# MAHF

A framework for modular construction and evaluation of metaheuristics.

# Purpose and Features

MAHF aims to make construction and modification of metaheuristics as simple and reliable as possible. In addition to
construction it also provides utilities for logging, evaluation and comparison of those heuristics.

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
- Variety of examples and clear guidelines
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

Examples on how to use MAHF for evaluation can be found in the [examples](examples) directory.

Examples of heuristics can be found under [heuristics](src/heuristics) and components
under [components](src/components).

# Additional Resources

None yet.

# Publications

If you use MAHF in a scientific publication, we would appreciate citations to the following paper:

Helena Stegherr, Leopold Luley, Jonathan Wurth, Michael Heider, and Jörg Hähner. 2023. A framework for modular
construction and evaluation of metaheuristics. Fakultät für Angewandte
Informatik. https://opus.bibliothek.uni-augsburg.de/opus4/103452

Bibtex entry:

```bibtex
@techreport{stegherr2023,
  author    = {Helena Stegherr and Leopold Luley and Jonathan Wurth and Michael Heider and J{\"o}rg H{\"a}hner},
  title     = {A framework for modular construction and evaluation of metaheuristics},
  institution = {Fakult{\"a}t f{\"u}r Angewandte Informatik},
  series    = {Reports / Technische Berichte der Fakult{\"a}t f{\"u}r Angewandte Informatik der Universit{\"a}t Augsburg},
  number    = {2023-01},
  pages     = {25},
  year      = {2023},
}
```