use serde::Serialize;

use crate::{component::ExecResult, components::Component, Problem, State};

pub mod debug;
pub mod improvement;
pub mod populations;

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
