#![allow(clippy::too_many_arguments, clippy::type_complexity)]

//! MAHF
//!
//! A framework for modular construction and evaluation of meta-heuristics.

pub mod fitness;
pub mod heuristic;
pub mod heuristics;
pub mod operators;
pub mod problem;
pub mod problems;
pub mod prompt;
pub mod random;
pub mod threads;
pub mod tracking;

// re-exports
pub use rand;
pub use rand_distr;
