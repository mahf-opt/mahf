use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufWriter},
    path::Path,
};

use erased_serde::Serialize as DynSerialize;
use eyre::WrapErr;
use serde::Serialize;

use crate::tracking::Log;

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

/// Write the [Log] to the output.
///
/// When writing to files, [write_log_file] is more convenient.
pub fn write_log(
    output: &mut impl io::Write,
    log: &Log,
) -> Result<(), ciborium::ser::Error<std::io::Error>> {
    ciborium::ser::into_writer(&CompressedLog::from(log), output)
}

/// Write the [Log] to a file.
pub fn write_log_file(output: impl AsRef<Path>, log: &Log) -> eyre::Result<()> {
    let file = File::create(output.as_ref()).wrap_err("failed to create log file")?;
    let writer = &mut BufWriter::new(file);
    write_log(writer, log).context("failed to serialize log")
}
