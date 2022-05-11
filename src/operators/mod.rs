//! Implementations of [crate::framework::components].

#![allow(clippy::new_ret_no_self)]

use crate::framework::components::Component;
use crate::framework::State;
use crate::problems::Problem;

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
