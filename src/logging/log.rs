//! A serializable log for arbitrary data.

use std::{any::type_name, collections::HashMap, fs::File, io::BufWriter, path::Path};

use better_any::{Tid, TidAble};
use erased_serde::Serialize as DynSerialize;
use eyre::WrapErr;
use serde::Serialize;

use crate::{component::ExecResult, state::common, CustomState, Problem, State};

/// A serializable log for arbitrary state throughout the execution of a [`Configuration`].
///
/// The `Log` is written to by the [`Logger`] component.
///
/// [`Logger`]: crate::logging::Logger
///
/// # Usages
///
/// This state is automatically inserted into the [`State`] by the [`Configuration`].
///
/// It can be retrieved from the [`State`] using the [`State::log`] method.
///
/// [`Configuration`]: crate::Configuration
///
/// # Serialization
///
/// The `Log` currently supports serialization into the `cbor` and `json` file formats using
/// the [`ciborium::ser`] and [`serde_json`] crates, respectively.
///
/// Independent tools can then be used to analyze the collected runtime data.
///
/// # Examples
///
/// Serializing the log into a `json` file:
///
/// ```
/// # use mahf::{ExecResult, Problem, State};
/// # fn example<P: Problem>(state: &mut State<P>) -> ExecResult<()> {
/// state.log().to_json("path/to/log.json")?;
/// # Ok(())
/// # }
/// ```
#[derive(Default, Serialize, Tid)]
#[serde(transparent)]
pub struct Log {
    steps: Vec<Step>,
}

impl CustomState<'_> for Log {}

impl Log {
    /// Creates a new, empty `Log`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes a new [`Step`] to the `Log`.
    ///
    /// There should be at most one [`Step`] per iteration.
    pub(crate) fn push(&mut self, entry: Step) {
        self.steps.push(entry);
    }

    /// Returns the currently recorded steps.
    pub(crate) fn steps(&self) -> &[Step] {
        &self.steps
    }
}

/// A step (usually an interaction).
#[derive(Default, Serialize)]
#[serde(transparent)]
pub(crate) struct Step {
    entries: Vec<Entry>,
}

impl Step {
    /// Checks whether an [`Entry`] with the given name already exists.
    pub(crate) fn contains(&self, name: &str) -> bool {
        self.entries.iter().any(|entry| entry.name == name)
    }

    /// Logs a new [`Entry`] at this [`Step`].
    pub(crate) fn push(&mut self, entry: Entry) {
        if !self.contains(entry.name) {
            self.entries.push(entry);
        }
    }

    /// Pushes the current number of [`Iterations`] if it has not been logged yet.
    ///
    /// Will also ensure that the key for [`Iterations`] is at index 0.
    ///
    /// [`Iterations`]: common::Iterations
    pub(crate) fn push_iteration<P: Problem>(&mut self, state: &State<P>) {
        let name = type_name::<common::Iterations>();

        if !self.contains(name) {
            let value = Box::new(state.iterations());
            self.entries.insert(0, Entry { name, value });
        }
    }

    /// Returns all entries.
    pub(crate) fn entries(&self) -> &[Entry] {
        &self.entries
    }
}

/// A single log entry.
#[derive(Serialize)]
pub struct Entry {
    pub name: &'static str,
    pub value: Box<dyn DynSerialize + Send>,
}

/// A compressed [`Log`] representation.
#[derive(Default, Serialize)]
pub struct CompressedLog<'a> {
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

/// Creates a writer for `path`.
fn create_writer(path: impl AsRef<Path>) -> ExecResult<impl std::io::Write> {
    let file = File::create(path.as_ref()).wrap_err("failed to create log file")?;
    Ok(BufWriter::new(file))
}

impl Log {
    pub fn as_compressed(&self) -> CompressedLog {
        self.into()
    }

    pub fn to_json(&self, path: impl AsRef<Path>) -> ExecResult<()> {
        let writer = create_writer(path)?;
        serde_json::to_writer_pretty(writer, &self.as_compressed())
            .wrap_err("failed to write json log")?;
        Ok(())
    }

    pub fn to_cbor(&self, path: impl AsRef<Path>) -> ExecResult<()> {
        let writer = create_writer(path)?;
        ciborium::ser::into_writer(&self.as_compressed(), writer)
            .wrap_err("failed to write cbor log")?;
        Ok(())
    }
}
