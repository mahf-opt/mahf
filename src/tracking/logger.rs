use std::ops::{Deref, Sub};

use crate::{
    framework::{
        components::Component,
        state::{common, CustomState, State},
    },
    problems::Problem,
    tracking::{log::Step, set::LogSet, trigger, Log},
};
use serde::Serialize;

/// A collection of [LogSet]s.
///
/// Can be created using [Logger::builder].
///
/// Implements [Component] and should be added to the end
/// of an algorithms main loop.
#[derive(Serialize)]
#[serde(bound = "")]
pub struct Logger<P: Problem> {
    #[serde(skip)]
    sets: Vec<LogSet<P>>,
}

impl<P: Problem + 'static> Logger<P> {
    /// Creates a new logger.
    ///
    /// Can be finalized using [Logger::build].
    pub fn builder() -> Self {
        Logger { sets: Vec::new() }
    }

    /// Log state `S` when `S` changes by `delta` or more.
    pub fn log_on_change<S>(self, delta: S::Target) -> Self
    where
        S: CustomState + Clone + Serialize + Deref,
        S::Target: Clone + Sub<Output = S::Target> + Ord + Send + Sync + 'static,
    {
        self.log_set(
            LogSet::new()
                .with_trigger(trigger::Change::<S>::new(delta))
                .with_auto_logger::<S>(),
        )
    }

    /// Add a custom [LogSet].
    pub fn log_set(mut self, set: LogSet<P>) -> Self {
        self.sets.push(set);
        self
    }

    /// Add the most common sets.
    ///
    /// Currently encompases:
    /// - [common::Evaluations]
    /// - [common::Progress]
    /// - [common::BestFitness]
    pub fn log_common_sets(self) -> Self {
        self.log_set(
            LogSet::new()
                .with_trigger(trigger::Iteration::new(10))
                .with_auto_logger::<common::Evaluations>()
                .with_auto_logger::<common::Progress>()
                .with_auto_logger::<common::BestObjectiveValue>(),
        )
    }

    /// Turns the logger into a [Component].
    pub fn build(self) -> Box<dyn Component<P>> {
        Box::new(self)
    }

    /// Creates an empty [Logger] [Component].
    pub fn default() -> Box<dyn Component<P>> {
        Logger::builder().build()
    }
}

impl<P: Problem + 'static> Component<P> for Logger<P> {
    fn initialize(&self, problem: &P, state: &mut State) {
        for set in &self.sets {
            for criteria in &set.criteria {
                criteria.initialize(problem, state);
            }
        }
    }

    fn execute(&self, problem: &P, state: &mut State) {
        let mut step = Step::default();

        for set in &self.sets {
            set.execute(problem, state, &mut step);
        }

        if !step.entries().is_empty() {
            step.push_iteration(state);
            state.get_mut::<Log>().push(step);
        }
    }
}
