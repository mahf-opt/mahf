//! A serializable log for arbitrary data.

use std::{any::type_name, collections::HashMap, fs::File, io::BufWriter, path::Path};

use better_any::{Tid, TidAble};
use erased_serde::Serialize as DynSerialize;
use eyre::WrapErr;
use serde::Serialize;

use crate::{component::ExecResult, state::common, CustomState, Problem, State};

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

/// A step (usually an interaction).
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
    pub(crate) fn push_iteration<P: Problem>(&mut self, state: &State<P>) {
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

#[derive(Default, Serialize)]
struct CompressedLog<'a> {
    names: Vec<&'static str>,
    entries: Vec<HashMap<usize, &'a dyn DynSerialize>>,
}

impl<'a> From<&'a Log> for CompressedLog<'a> {
    fn from(log: &'a Log) -> Self {
        let mut clog = CompressedLog::default();

        let mut next_key = 0;
        let mut keys: HashMap<&'static str, usize> = HashMap::new();

        for step in log.steps() {
            let mut cstep = HashMap::with_capacity(step.entries().len());

            for entry in step.entries() {
                let key = *keys.entry(entry.name).or_insert_with(|| {
                    clog.names.push(entry.name);
                    let key = next_key;
                    next_key += 1;
                    key
                });

                let value: &'a dyn DynSerialize = &entry.value;

                cstep.insert(key, value);
            }

            clog.entries.push(cstep);
        }

        clog
    }
}

fn path_into_writer(path: impl AsRef<Path>) -> ExecResult<impl std::io::Write> {
    let file = File::create(path.as_ref()).wrap_err("failed to create log file")?;
    Ok(BufWriter::new(file))
}

impl Log {
    fn as_compressed(&self) -> CompressedLog {
        self.into()
    }

    pub fn to_json(&self, path: impl AsRef<Path>) -> ExecResult<()> {
        let writer = path_into_writer(path)?;
        serde_json::to_writer_pretty(writer, &self.as_compressed())
            .wrap_err("failed to write json log")?;
        Ok(())
    }

    pub fn to_cbor(&self, path: impl AsRef<Path>) -> ExecResult<()> {
        let writer = path_into_writer(path)?;
        ciborium::ser::into_writer(&self.as_compressed(), writer)
            .wrap_err("failed to write cbor log")?;
        Ok(())
    }
}
