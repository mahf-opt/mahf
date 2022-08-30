# MAHF
A framework for modular construction and evaluation of meta-heuristics.

This document provides an overview of what MAHF provides and where those features can be found.
The individual module's documentation then describes how it can be used in practice.

## Primary Features

- Simple modular construction of metaheuristics
- State management and state tracking
- Collection of common operators
- Templates for common heuristics
- A basic set of common benchmark problems
- Utilities for parameter optimization
- Utilities for efficient evaluation and comparison

## Crate Structure

### The `framework` module

The [framework] module is the heart of MAHF and provides everything for constructing and running modular heuristics.
Most importantly [framework::Configuration] which describes a heuristic and [framework::run] which will run the given heuristic on a benchmark problem.

### The `heuristics` module

The [heuristics] module contains many common heuristics implemented in a modular fashion.
It serves both as a collection of directly usable heuristics and as a template library for constructing your own.
If you are in doubt, how you should model your heuristic with MAHF, taking a look at the source code of the heuristics in this module is a great starting point.

### The `operators` module

The [operators] module collects implementations of operators which can then be combined to form a heuristic.

### The `problems` module

The [problems] module contains many common benchmark problems used to evaluate a heuristics performance.

### The `tracking` module

The [tracking] module provides facilities for tracking state during the evaluation, as well as utilities to store these logs on disk.
