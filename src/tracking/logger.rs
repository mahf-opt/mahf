use serde::Serialize;

use crate::{
    framework::components::Component,
    problems::Problem,
    state::State,
    tracking::{log::Step, Log, LogSet},
};

/// A collection of [LogSet]s.
///
/// Can be created using [Logger::builder].
///
/// Implements [Component] and should be added to the end
/// of an algorithms main loop.
///
/// # Cloning
///
/// Note that [Clone]ing does **NOT** preserve existing [LogSet]'s.
#[derive(Clone, Default, Serialize)]
pub struct Logger;

impl Logger {
    /// Creates an empty [Logger] [Component].
    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Logger)
    }
}

impl<P: Problem> Component<P> for Logger {
    fn initialize(&self, problem: &P, state: &mut State) {
        state.holding::<LogSet<P>>(|sets, state| {
            for (trigger, _) in &sets.entries {
                trigger.initialize(problem, state);
            }
        });
    }

    fn execute(&self, problem: &P, state: &mut State) {
        state.holding::<LogSet<P>>(|sets, state| {
            let mut step = Step::default();

            sets.execute(problem, state, &mut step);

            if !step.entries().is_empty() {
                step.push_iteration(state);
                state.get_mut::<Log>().push(step);
            }
        });
    }
}
