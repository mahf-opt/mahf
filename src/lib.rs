//! A framework for the modular construction and evaluation of metaheuristics.

#![allow(clippy::new_ret_no_self)]

pub mod components;
pub mod conditions;
pub mod framework;
pub mod heuristics;
pub mod population;
pub mod prelude;
pub mod problems;
pub mod state;
pub(crate) mod testing;
pub mod tracking;
pub mod utils;

// Re-exports from modules
pub use components::Component;
pub use conditions::Condition;
pub use framework::Configuration;
pub use problems::{
    individual::Individual,
    objective::{MultiObjective, Objective, SingleObjective},
    MultiObjectiveProblem, Problem, SingleObjectiveProblem,
};
pub use state::{CustomState, Random, State};

// Crate re-exports
pub use derive_more;
pub use float_eq;
pub use rand;
pub use rand_distr;
pub use serde;
