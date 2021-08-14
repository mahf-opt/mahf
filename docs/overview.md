# MAHF
A framework for modular construction and evaluation of meta-heuristics.

Take a look at the [heuristic] module for a description of how MAHF's module system works.

## Primary Features

- Simple modular costruction of metaheuristics
- Collection of common operators
- Collection of common benchmark problems
- Utilities for parameter optimization
- Utilities for efficient evaluation and tracking

## Crate Structure

### heuristic

The [heuristic] module contains the utilities for constructing and running modular heuristics. Most importantly [heuristic::Configuration] which describes a heuristic and [heuristic::run] which will apply the provided config to a given problem. Also see the [heuristic] module for a description of how MAHF's module system works.

### operators

The [operators] module collects implementations of operators. There is a module for each of the [heuristic::components] providing a solid foundation of reusable operators.

### heuristics

The [heuristics] module contains many of the common heuristics implemented in a modular fashion.

### problems

The [problems] module contains many of the common benchmark problems used when evaluation a heuristics performance.