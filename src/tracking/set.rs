use std::{any::type_name, ops::Deref};

use crate::{
    framework::{common_state, components::Condition, CustomState, State},
    problems::Problem,
    tracking::log::{Entry, Step},
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

    pub fn with_trigger(mut self, criteria: Box<dyn Condition<P>>) -> Self {
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

    pub(crate) fn execute(&self, problem: &P, state: &mut State, step: &mut Step) {
        let criteria = self
            .criteria
            .iter()
            .map(|c| c.evaluate(problem, state))
            .any(|b| b); // normal any would be short-circuiting

        if criteria {
            for logger in &self.loggers {
                step.push((logger.function)(state));
            }
        }
    }
}

pub struct LoggerFunction {
    pub(crate) function: fn(&State) -> Entry,
}

impl LoggerFunction {
    pub fn auto<T: CustomState + Clone + Serialize>() -> LoggerFunction {
        fn log_fn<T: CustomState + Clone + Serialize>(state: &State) -> Entry {
            debug_assert!(state.has::<T>(), "missing state: {}", type_name::<T>());

            let instance = state.get::<T>();
            let value = Box::new(instance.clone());
            let name = std::any::type_name::<T>();
            Entry { name, value }
        }

        LoggerFunction {
            function: log_fn::<T>,
        }
    }

    pub fn best_individual<P, E>() -> LoggerFunction
    where
        P: Problem<Encoding = E>,
        E: Clone + Serialize + Sized + 'static,
    {
        fn log_fn<E: Clone + Serialize + Sized + 'static>(state: &State) -> Entry {
            debug_assert!(
                state.has::<common_state::BestIndividual>(),
                "missing state: {}",
                type_name::<common_state::BestIndividual>()
            );

            let instance = state.get::<common_state::BestIndividual>();
            let value = Box::new(if let Some(instance) = instance.deref() {
                let individual = instance.solution::<E>().clone();
                Some(individual)
            } else {
                None
            });

            let name = std::any::type_name::<common_state::BestIndividual>();
            Entry { name, value }
        }

        LoggerFunction {
            function: log_fn::<E>,
        }
    }

    pub fn custom(function: fn(&State) -> Entry) -> LoggerFunction {
        Self { function }
    }
}
