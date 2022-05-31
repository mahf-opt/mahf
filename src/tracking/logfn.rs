use std::any::type_name;

use crate::{
    framework::{
        common_state, common_state::BestIndividual, components::Condition, CustomState, State,
    },
    problems::Problem,
    tracking::{
        log::{LogEntry, LoggedState},
        Log,
    },
};
use serde::Serialize;

#[derive(Default)]
pub struct LogSet<P> {
    pub(crate) criteria: Vec<Box<dyn Condition<P>>>,
    pub(crate) loggers: Vec<LoggerFunction>,
}

impl<P: Problem + 'static> LogSet<P> {
    pub fn new() -> Self {
        Self {
            criteria: Vec::new(),
            loggers: Vec::new(),
        }
    }

    pub fn with_criteria(mut self, criteria: Box<dyn Condition<P>>) -> Self {
        self.criteria.push(criteria);
        self
    }

    pub fn with_logger(mut self, logger: LoggerFunction) -> Self {
        self.loggers.push(logger);
        self
    }

    pub fn with_auto_logger<T: CustomState + Clone + Serialize>(self) -> Self {
        self.with_logger(LoggerFunction::auto::<T>())
    }

    pub fn with_common_loggers(self) -> Self {
        self.with_auto_logger::<common_state::Iterations>()
            .with_auto_logger::<common_state::Evaluations>()
            .with_auto_logger::<common_state::BestFitness>()
            .with_auto_logger::<common_state::Progress>()
    }

    pub(crate) fn execute(&self, problem: &P, state: &mut State) {
        let criteria = self
            .criteria
            .iter()
            .map(|c| c.evaluate(problem, state))
            .any(|b| b); // normal any would be short-circuiting

        if criteria {
            let mut entry = LogEntry::default();

            for logger in &self.loggers {
                entry.state.push((logger.function)(state));
            }

            state.get_mut::<Log>().push(entry);
        }
    }
}

pub struct LoggerFunction {
    pub(crate) function: fn(&State) -> LoggedState,
}

impl LoggerFunction {
    pub fn auto<T: CustomState + Clone + Serialize>() -> LoggerFunction {
        fn log_fn<T: CustomState + Clone + Serialize>(state: &State) -> LoggedState {
            debug_assert!(state.has::<T>(), "missing state: {}", type_name::<T>());

            let instance = state.get::<T>();
            let value = Box::new(instance.clone());
            let name = std::any::type_name::<T>();
            LoggedState { name, value }
        }

        LoggerFunction {
            function: log_fn::<T>,
        }
    }

    pub fn best_individual<E: Clone + Serialize + Sized + 'static>() -> LoggerFunction {
        fn log_fn<E: Clone + Serialize + Sized + 'static>(state: &State) -> LoggedState {
            debug_assert!(
                state.has::<BestIndividual>(),
                "missing state: {}",
                type_name::<BestIndividual>()
            );

            let instance = state.get::<BestIndividual>();
            let individual = instance.solution::<E>().clone();
            let value = Box::new(individual);
            let name = std::any::type_name::<BestIndividual>();
            LoggedState { name, value }
        }

        LoggerFunction {
            function: log_fn::<E>,
        }
    }

    pub fn custom(function: fn(&State) -> LoggedState) -> LoggerFunction {
        Self { function }
    }
}
