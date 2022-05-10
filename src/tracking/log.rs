use erased_serde::Serialize as DynSerialize;
use serde::Serialize;

use crate::framework::{CustomState, State};

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
}

#[derive(Default, Serialize)]
#[serde(transparent)]
pub struct LogEntry {
    state: Vec<LoggedState>,
}

#[derive(Serialize)]
pub struct LoggedState {
    name: &'static str,
    value: Box<dyn DynSerialize>,
}

#[derive(Default)]
pub struct LogConfig {
    loggers: Vec<Logger>,
}

impl LogConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn log(&self, entry: &mut LogEntry, state: &State) {
        for logger in &self.loggers {
            entry.state.push((logger.log_fn)(state));
        }
    }

    pub fn with_logger(mut self, logger: Logger) -> Self {
        self.add_logger(logger);
        self
    }

    pub fn add_logger(&mut self, logger: Logger) {
        // TODO: check that the logger is unique
        self.loggers.push(logger);
    }
}

pub struct Logger {
    log_fn: fn(&State) -> LoggedState,
}

impl Logger {
    pub fn new_for<T: CustomState + Clone + Serialize>() -> Logger {
        fn log_fn<T: CustomState + Clone + Serialize>(state: &State) -> LoggedState {
            let instance = state.get::<T>();
            let value = Box::new(instance.clone());
            let name = std::any::type_name::<T>();
            LoggedState { name, value }
        }

        Logger {
            log_fn: log_fn::<T>,
        }
    }
}
