#![doc = include_str!("../../docs/framework.md")]

use trait_set::trait_set;

mod configuration;
pub use configuration::{Configuration, ConfigurationBuilder};

pub use crate::problems::objective;
pub use objective::{IllegalObjective, MultiObjective, Objective, SingleObjective};

pub use crate::problems::individual;
pub use individual::Individual;

trait_set! {
    /// Collection of traits required by every component.
    pub trait AnyComponent = erased_serde::Serialize + std::any::Any + Send + Sync + dyn_clone::DynClone;
}
