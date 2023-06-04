use dyn_clone::DynClone;
use serde::{Serialize, Serializer};
use trait_set::trait_set;

use crate::{component::ExecResult, components::Component, Problem, State};

trait_set! {
    /// Helper trait to allow cloning of debug functions.
    pub trait DebugFn<P: Problem> = Fn(&P, &mut State<P>) + Send + Sync + DynClone + 'static;
}

dyn_clone::clone_trait_object!(<P: Problem> DebugFn<P>);

#[derive(derivative::Derivative)]
#[derivative(Clone(bound = ""))]
pub struct Debug<P: Problem>(Box<dyn DebugFn<P, Output = ()>>);

impl<P: Problem> Debug<P> {
    pub fn from_params(debug: impl DebugFn<P>) -> Self {
        Self(Box::new(debug))
    }

    pub fn new(debug: impl DebugFn<P>) -> Box<dyn Component<P>> {
        Box::new(Self::from_params(debug))
    }
}

impl<P: Problem> Component<P> for Debug<P> {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        self.0(problem, state);
        Ok(())
    }
}

impl<P: Problem> Serialize for Debug<P> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_unit_struct("Debug")
    }
}
