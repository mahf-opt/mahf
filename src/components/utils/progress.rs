use better_any::{Tid, TidAble};
use derive_more::{Deref, DerefMut};
use indicatif::{ProgressBar, ProgressStyle};
use serde::Serialize;

use crate::{Component, CustomState, ExecResult, Problem, State};

#[derive(Deref, DerefMut, Tid)]
struct LoopProgress(pub ProgressBar);

impl CustomState<'_> for LoopProgress {}

/// Increments a [`ProgressBar`].
#[derive(Clone, Serialize)]
pub struct ProgressBarIncrement;
impl ProgressBarIncrement {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Self::from_params())
    }
}

impl<P: Problem> Component<P> for ProgressBarIncrement {
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(LoopProgress(ProgressBar::new_spinner().with_style(
            ProgressStyle::with_template("{spinner} {pos} {message}")?,
        )));
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.borrow_value_mut::<LoopProgress>().inc(1);
        Ok(())
    }
}
