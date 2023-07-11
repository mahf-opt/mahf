//! Utility components.

use serde::Serialize;

use crate::{component::ExecResult, components::Component, Problem, State};

pub mod debug;
pub mod improvement;
pub mod populations;

/// Doesn't do anything.
///
/// Can be used as a placeholder in e.g. [heuristic templates].
///
/// [heuristic templates]: crate::heuristics
#[derive(Clone, Serialize)]
pub struct Noop;

impl Noop {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Self::from_params())
    }
}

impl<P: Problem> Component<P> for Noop {
    fn execute(&self, _problem: &P, _state: &mut State<P>) -> ExecResult<()> {
        Ok(())
    }
}
