# MAHF

![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/mahf-opt/mahf/ci.yml?logo=github)
![GitHub](https://img.shields.io/github/license/mahf-opt/mahf)

A framework for modular construction and evaluation of metaheuristics.

MAHF enables easy construction and experimental analysis of metaheuristics by breaking them down into their fundamental components.

The framework supports not only evolutionary algorithms, but also any other metaheuristic framework, including non-population-based, constructive, and specifically hybrid approaches.

# Overview

MAHF aims to make construction and modification of metaheuristics as simple and reliable as possible. 
In addition to construction it also provides utilities for logging, evaluation and comparison of those heuristics:

- Simple modular construction of metaheuristics
- State management and state tracking
- Collection of common operators
- Templates for common heuristics
- Flexible logging of runtime information

# Getting Started

## Requirements

- [The Rust Programming Language](https://rust-lang.org)
- Either `gcc` or `clang`

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
mahf = { git = "https://github.com/mahf-opt/mahf" }
```

## Example

A simple genetic algorithm for real-valued black box optimization.

The example uses the [common benchmark functions](https://github.com/mahf-opt/mahf-bmf) for MAHF.

```rust
use mahf::prelude::*;
use mahf_bmf::BenchmarkFunction;

let problem = BenchmarkFunction::sphere(/*dim: */ 30);

let ga = Configuration::builder()
    .do_(initialization::RandomSpread::new(population_size))
    .evaluate()
    .update_best_individual()
    .while_(conditions::LessThanN::iterations(n), |builder| {
       builder
           .do_(selection::Tournament::new(num_selected, size))
           .do_(recombination::ArithmeticCrossover::new_insert_both(pc))
           .do_(mutation::NormalMutation::new(std_dev, rm))
           .evaluate()
           .update_best_individual()
           .do_(replacement::MuPlusLambda::new(max_population_size))
    })
    .build();

let state = ga.optimize(&problem, evaluate::Sequential::new())?;
println!("Best solution found: {:?}", state.best_individual());
```

More examples  can be found in the [examples](examples) directory.

Examples of heuristic templates can be found under [heuristics](src/heuristics).

For component implementations, see [components](src/components).

# Documentation

MAHF has extensive documentation, which should make it easy to get started.

Just run

```sh
$ cargo doc --open
```

to build and open the documentation.

# Related Projects

- [mahf-bmf](https://github.com/mahf-opt/mahf-bmf): Common continuous benchmark functions
- [mahf-coco](https://github.com/mahf-opt/mahf-coco): Bindings to the [COCO](https://github.com/numbbo/coco) benchmarking framework
- [mahf-tsplib](https://github.com/mahf-opt/mahf-tsplib): Bindings to the [TSPLIB](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/) library

# Contributing

We welcome contributions from the community and appreciate your interest in improving this project.
A contribution guide will follow shortly.

# License

This project is licensed under the [GNU General Public License v3.0](https://github.com/mahf-opt/mahf/blob/master/LICENSE).

# Publications

## Citing MAHF

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