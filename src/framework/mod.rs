#![doc = include_str!("../../docs/framework.md")]

pub mod components;
pub mod conditions;

mod configuration;
pub use configuration::{Configuration, ConfigurationBuilder};

mod objective;
pub use objective::{IllegalObjective, MultiObjective, Objective, SingleObjective};

mod individual;
pub use individual::Individual;

mod random;
pub use random::{Random, RandomConfig};
