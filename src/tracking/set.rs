use crate::{
    problems::Problem,
    state::{common, CustomState, State},
    tracking::{
        functions::{self, LogFn},
        log::Step,
        trigger::{self, Trigger},
    },
};
use serde::Serialize;

/// A combination of [Trigger] and [LogFn].
#[derive(Default)]
pub struct LogSet<P> {
    pub(crate) criteria: Vec<Box<dyn Trigger<P>>>,
    pub(crate) loggers: Vec<LogFn>,
}

impl<P: Problem + 'static> LogSet<P> {
    /// Creates a new, empty instance.
    pub fn new() -> Self {
        Self {
            criteria: Vec::new(),
            loggers: Vec::new(),
        }
    }

    /// Returns a common log set.
    ///
    /// Every 10 [Iteration][common::Iterations], [common::Evaluations] and [common::Progress] are logged.
    pub fn common() -> Self {
        Self::new()
            .with_trigger(trigger::Iteration::new(10))
            .with_auto_logger::<common::Evaluations>()
            .with_auto_logger::<common::Progress>()
    }

    /// Adds a [Trigger].
    ///
    /// The logset will be executed whenever one of the [Trigger]s fires.
    pub fn with_trigger(mut self, trigger: Box<dyn Trigger<P>>) -> Self {
        self.criteria.push(trigger);
        self
    }

    /// Adds a [LogFn].
    ///
    /// When a trigger fires the [LogFn] will be called.
    pub fn with_logger(mut self, logger: LogFn) -> Self {
        self.loggers.push(logger);
        self
    }

    /// Adds a generated [LogFn] for the [CustomState] `T`.
    ///
    /// Works for any `T` implementing [CustomState] and [Clone] + [Serialize].
    pub fn with_auto_logger<T: CustomState + Clone + Serialize>(self) -> Self {
        self.with_logger(functions::auto::<T>)
    }

    pub(crate) fn execute(&self, problem: &P, state: &mut State, step: &mut Step) {
        let trigger = self
            .criteria
            .iter()
            .map(|c| c.evaluate(problem, state))
            .any(|b| b); // normal any would be short-circuiting

        if trigger {
            for logger in &self.loggers {
                step.push((logger)(state));
            }
        }
    }
}
