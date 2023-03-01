use crate::{
    problems::Problem,
    state::{common, CustomState, State},
    tracking::{
        functions::{self, LogFn},
        log::Step,
        trigger::Trigger,
    },
};
use better_any::Tid;

/// A combination of [Trigger] and [LogFn].
#[derive(Default, Tid)]
pub struct LogSet<'a, P: 'static> {
    pub(crate) entries: Vec<(Box<dyn Trigger<'a, P>>, LogFn<'a>)>,
}

impl<'a, P> CustomState<'a> for LogSet<'a, P> {}

impl<'a, P: Problem + 'static> LogSet<'a, P> {
    /// Creates a new, empty instance.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn with(mut self, trigger: Box<dyn Trigger<'a, P>>, extractor: LogFn<'a>) -> Self {
        self.entries.push((trigger, extractor));
        self
    }

    /// Returns a common log set.
    ///
    /// Every 10 [Iteration][common::Iterations], [common::Evaluations] and [common::Progress] are logged.
    pub fn with_common_extractors(trigger: Box<dyn Trigger<'a, P>>) -> Self {
        Self::new()
            .with(
                dyn_clone::clone_box(&*trigger),
                functions::auto::<common::Evaluations>,
            )
            .with(
                dyn_clone::clone_box(&*trigger),
                functions::auto::<common::Progress>,
            )
    }

    pub(crate) fn execute(&self, problem: &P, state: &mut State<'a>, step: &mut Step) {
        for (trigger, extractor) in &self.entries {
            if trigger.evaluate(problem, state) {
                step.push((extractor)(state));
            }
        }
    }
}
