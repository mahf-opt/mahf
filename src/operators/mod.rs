//! Implementations of [crate::framework::components].

pub mod archive;
pub mod custom_state;
pub mod generation;
pub mod initialization;
pub mod postprocess;
pub mod replacement;
pub mod schedulers;
pub mod selection;
pub mod termination;

#[cfg(test)]
pub mod testing;
