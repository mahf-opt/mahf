#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::new_ret_no_self
)]
#![doc = include_str!("../docs/overview.md")]

pub mod components;
pub use components::Component;

pub mod conditions;
pub use conditions::Condition;

pub mod framework;
pub use framework::{Configuration, Individual};

pub mod heuristics;
pub mod prelude;
pub mod problems;
pub mod state;
pub use state::{CustomState, Random, State};

pub mod tracking;
pub mod utils;

#[cfg(test)]
pub mod testing;

// re-exports
pub use derive_more;
pub use float_eq;
pub use rand;
pub use rand_distr;
pub use serde;
