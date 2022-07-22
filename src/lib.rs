#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::new_ret_no_self
)]
#![doc = include_str!("../docs/overview.md")]

pub mod framework;
pub mod heuristics;
pub mod operators;
pub mod problems;
pub mod random;
pub mod tracking;
pub mod utils;

// re-exports
pub use float_eq;
pub use rand;
pub use rand_distr;
