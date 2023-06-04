//! A framework for the modular construction and evaluation of metaheuristics.

pub mod component;
pub mod components;
pub mod conditions;
pub mod configuration;
pub mod encoding;
pub mod experiments;
pub mod heuristics;
pub mod identifier;
pub mod individual;
pub mod logging;
pub mod objective;
pub mod population;
pub mod prelude;
pub mod problems;
pub mod state;
pub(crate) mod testing;
pub(crate) mod utils;

pub use component::ExecResult;
pub use components::Component;
pub use conditions::Condition;
pub use configuration::Configuration;
pub use individual::Individual;
pub use objective::{MultiObjective, SingleObjective};
pub use problems::Problem;
pub use state::{
    extract::{IdFn, ValueOf},
    CustomState, State, StateError,
};
