use crate::{
    framework::{components::Condition, CustomState, State},
    problems::Problem,
    tracking::{
        function::{self, LogFn},
        log::Step,
    },
};
use serde::Serialize;

#[derive(Default)]
pub struct LogSet<P> {
    pub(crate) criteria: Vec<Box<dyn Condition<P>>>,
    pub(crate) loggers: Vec<LogFn>,
}

impl<P: Problem + 'static> LogSet<P> {
    pub fn new() -> Self {
        Self {
            criteria: Vec::new(),
            loggers: Vec::new(),
        }
    }

    pub fn with_trigger(mut self, trigger: Box<dyn Condition<P>>) -> Self {
        self.criteria.push(trigger);
        self
    }

    pub fn with_logger(mut self, logger: LogFn) -> Self {
        self.loggers.push(logger);
        self
    }

    pub fn with_auto_logger<T: CustomState + Clone + Serialize>(self) -> Self {
        self.with_logger(function::auto::<T>)
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
