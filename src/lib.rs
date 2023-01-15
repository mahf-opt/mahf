#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::new_ret_no_self
)]
#![doc = include_str!("../docs/overview.md")]

pub mod components;
pub mod conditions;
pub mod framework;
// pub mod heuristics;
pub mod prelude;
pub mod problems;
pub mod state;
pub mod tracking;
pub mod utils;

#[cfg(test)]
pub mod testing;

// re-exports
pub use derive_deref;
pub use float_eq;
pub use rand;
pub use rand_distr;
pub use serde;
