//! Implementations of [crate::framework::components].

#![allow(clippy::new_ret_no_self)]

use serde::Serialize;

use crate::{
    framework::{components::Component, State},
    problems::Problem,
};

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

/// Doesn't do anything.
///
/// Note that this component is different from [initialization::Empty] as it doesn't modify
/// the state at all, while [Empty][initialization::Empty] pushes an empty population on the stack.
#[derive(Serialize)]
pub struct Noop;
impl Noop {
    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: Problem,
    {
        Box::new(Self)
    }
}
impl<P: Problem> Component<P> for Noop {
    fn execute(&self, _problem: &P, _state: &mut State) {
        // Noop
    }
}
