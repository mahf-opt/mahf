use better_any::Tid;
use serde::Serialize;

use crate::state::common::Iterations;
use crate::{
    problems::Problem,
    state::{common, CustomState, State},
    tracking::{
        functions::{self, Extractor},
        log::Step,
        trigger::Trigger,
    },
};

/// A combination of [Trigger] and [LogFn].
#[derive(Default, Tid)]
pub struct LogSet<'a, P: 'static> {
    pub(crate) entries: Vec<(Box<dyn Trigger<'a, P> + 'a>, Extractor<'a, P>)>,
}

impl<'a, P> Clone for LogSet<'a, P> {
    fn clone(&self) -> Self {
        let mut entries = Vec::with_capacity(self.entries.len());

        for (trigger, logfn) in &self.entries {
            entries.push((dyn_clone::clone_box(&**trigger), *logfn));
        }

        LogSet { entries }
    }
}

impl<'a, P> CustomState<'a> for LogSet<'a, P> {}

impl<'a, P: Problem + 'static> LogSet<'a, P> {
    /// Creates a new, empty instance.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn with(
        mut self,
        trigger: Box<dyn Trigger<'a, P> + 'a>,
        extractor: Extractor<'a, P>,
    ) -> Self {
        self.entries.push((trigger, extractor));
        self
    }

    pub fn with_many(
        mut self,
        trigger: Box<dyn Trigger<'a, P> + 'a>,
        extractors: impl IntoIterator<Item = Extractor<'a, P>>,
    ) -> Self {
        for extractor in extractors {
            self.entries.push((trigger.clone(), extractor));
        }
        self
    }

    pub fn with_auto_extractor<T>(mut self, trigger: Box<dyn Trigger<'a, P> + 'a>) -> Self
    where
        T: CustomState<'a> + Clone + Serialize + 'static,
    {
        self.entries.push((trigger, functions::auto::<T, P>));
        self
    }

    /// Returns a common log set.
    ///
    /// Every 10 [Iteration][common::Iterations], [common::Evaluations] and [common::Progress] are logged.
    pub fn with_common_extractors(self, trigger: Box<dyn Trigger<'a, P> + 'a>) -> Self {
        self.with(trigger.clone(), functions::auto::<common::Evaluations, _>)
            .with(
                trigger.clone(),
                functions::auto::<common::Progress<Iterations>, _>,
            )
    }

    pub(crate) fn execute(&self, problem: &P, state: &mut State<'a, P>, step: &mut Step) {
        for (trigger, extractor) in &self.entries {
            if trigger.evaluate(problem, state) {
                step.push((extractor)(state));
            }
        }
    }
}
