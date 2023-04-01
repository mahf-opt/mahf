use crate::state::{common, CustomState, State};
use better_any::Tid;
use erased_serde::Serialize as DynSerialize;
use serde::Serialize;
use std::any::type_name;

/// A log tracking state throughout the run.
///
/// [Log] implements [CustomState] and will be
/// automatically inserted for every run.
#[derive(Default, Serialize, Tid)]
#[serde(transparent)]
pub struct Log {
    steps: Vec<Step>,
}

impl CustomState<'_> for Log {}

impl Log {
    /// Creates a new, empty log.
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes a new [Step] to the log.
    ///
    /// There should be at most one [Step] per iteration.
    pub fn push(&mut self, entry: Step) {
        self.steps.push(entry);
    }

    /// Returns the currently recorded steps.
    pub fn steps(&self) -> &[Step] {
        &self.steps
    }
}

/// A step (usually an interation).
#[derive(Default, Serialize)]
#[serde(transparent)]
pub struct Step {
    entries: Vec<Entry>,
}

impl Step {
    /// Checks whether an entry with the given name already exists.
    pub fn contains(&self, name: &str) -> bool {
        self.entries.iter().any(|entry| entry.name == name)
    }

    /// Logs a new [Entry] at this [Step].
    pub fn push(&mut self, entry: Entry) {
        if !self.contains(entry.name) {
            self.entries.push(entry);
        }
    }

    /// Pushes the current iteration if it has not been logged yet.
    ///
    /// Will also ensure that the iteration is at index 0.
    pub(crate) fn push_iteration(&mut self, state: &State) {
        let name = type_name::<common::Iterations>();

        if !self.contains(name) {
            let value = Box::new(state.iterations());
            self.entries.insert(0, Entry { name, value });
        }
    }

    /// Returns all entries.
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }
}

/// A single log entry.
#[derive(Serialize)]
pub struct Entry {
    pub name: &'static str,
    pub value: Box<dyn DynSerialize + Send>,
}
