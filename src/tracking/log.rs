use erased_serde::Serialize as DynSerialize;
use serde::Serialize;

use crate::framework::{common_state::BestIndividual, CustomState, State};

#[derive(Default, Serialize)]
#[serde(transparent)]
pub struct Log {
    entries: Vec<LogEntry>,
}

impl CustomState for Log {}

impl Log {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, entry: LogEntry) {
        self.entries.push(entry);
    }
}

#[derive(Default, Serialize)]
#[serde(transparent)]
pub struct LogEntry {
    pub(crate) state: Vec<LoggedState>,
}

#[derive(Serialize)]
pub struct LoggedState {
    name: &'static str,
    value: Box<dyn DynSerialize>,
}

#[derive(Default)]
pub struct LoggerSet {
    pub(crate) loggers: Vec<LoggerFunction>,
}

impl CustomState for LoggerSet {}

impl LoggerSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_logger(mut self, logger: LoggerFunction) -> Self {
        self.add_logger(logger);
        self
    }

    pub fn add_logger(&mut self, logger: LoggerFunction) {
        // TODO: check that the logger is unique
        self.loggers.push(logger);
    }
}

pub struct LoggerFunction {
    pub(crate) function: fn(&State) -> LoggedState,
}

impl LoggerFunction {
    pub fn auto<T: CustomState + Clone + Serialize>() -> LoggerFunction {
        fn log_fn<T: CustomState + Clone + Serialize>(state: &State) -> LoggedState {
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
