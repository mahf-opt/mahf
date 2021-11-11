#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![doc = include_str!("../docs/overview.md")]

pub mod fitness;
pub mod heuristic;
pub mod heuristics;
pub mod operators;
pub mod problems;
pub mod prompt;
pub mod random;
pub mod threads;
pub mod tracking;

// re-exports
pub use float_eq;
pub use rand;
pub use rand_distr;
