//! A framework for the modular construction and evaluation of metaheuristics.
//!
//! TODO

// TODO: #![deny(missing_docs)]

pub use derive_more;
pub use float_eq;
pub use rand;
pub use rand_distr;
pub use serde;

pub mod component;
pub mod components;
pub mod conditions;
pub mod configuration;
pub mod experiments;
pub mod heuristics;
pub mod identifier;
pub mod lens;
pub mod logging;
pub mod population;
pub mod prelude;
pub mod problems;
pub mod state;
pub(crate) mod testing;
pub mod utils;

pub use component::ExecResult;
pub use components::Component;
pub use conditions::Condition;
pub use configuration::Configuration;
pub use lens::{IdLens, ValueOf};
pub use problems::{
    individual::Individual,
    objective::{MultiObjective, Objective, SingleObjective},
    MultiObjectiveProblem, Problem, SingleObjectiveProblem,
};
pub use state::{CustomState, Random, State, StateError, StateRegistry};
