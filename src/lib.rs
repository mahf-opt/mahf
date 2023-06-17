//! A framework for the modular construction and evaluation of metaheuristics.

// TODO: #![deny(missing_docs)]

pub mod component;
pub mod components;
pub mod conditions;
pub mod configuration;
pub mod experiments;
pub mod heuristics;
pub mod identifier;
pub mod logging;
pub mod population;
pub mod prelude;
pub mod problems;
pub mod state;
pub(crate) mod testing;
pub mod utils;

// Re-exports from modules
pub use component::ExecResult;
pub use components::Component;
pub use conditions::Condition;
pub use configuration::Configuration;
// Crate re-exports
pub use derive_more;
pub use float_eq;
pub use problems::{
    individual::Individual,
    objective::{MultiObjective, Objective, SingleObjective},
    MultiObjectiveProblem, Problem, SingleObjectiveProblem,
};
pub use rand;
pub use rand_distr;
pub use serde;
pub use state::{
    lens::{IdLens, ValueOf},
    CustomState, Random, State, StateError, StateRegistry,
};
