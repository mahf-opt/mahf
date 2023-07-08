//! A framework for the modular construction and evaluation of metaheuristics.
//!
//! MAHF enables easy construction and experimental analysis of metaheuristics by
//! breaking them down into their fundamental components.
//!
//! The framework supports not only evolutionary algorithms, but also any other
//! metaheuristic framework, including non-population-based,
//! constructive, and specifically hybrid approaches.
//!
//! # Overview
//!
//! MAHF aims to make construction and modification of metaheuristics as simple and reliable as possible.
//! In addition to construction it also provides utilities for logging, evaluation, and comparison of those heuristics:
//!
//! - Simple modular construction of metaheuristics
//! - State management and state tracking
//! - Collection of common operators
//! - Templates for common heuristics
//! - Flexible logging of runtime information

// TODO: #![warn(missing_docs)]

#[doc(hidden)]
pub use derive_more;
#[doc(hidden)]
pub use float_eq;
#[doc(hidden)]
pub use rand;
#[doc(hidden)]
pub use rand_distr;
#[doc(hidden)]
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

#[doc(hidden)]
pub use component::ExecResult;
#[doc(hidden)]
pub use components::Component;
#[doc(hidden)]
pub use conditions::Condition;
#[doc(hidden)]
pub use configuration::Configuration;
#[doc(hidden)]
pub use problems::{
    individual::Individual,
    objective::{MultiObjective, Objective, SingleObjective},
    MultiObjectiveProblem, Problem, SingleObjectiveProblem,
};
#[doc(hidden)]
pub use state::{CustomState, Random, State, StateError, StateRegistry};
