use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufWriter},
    path::Path,
};

use anyhow::Context;
use erased_serde::Serialize as DynSerialize;
use serde::Serialize;

use crate::tracking::Log;

#[derive(Default, Serialize)]
struct CompressedLog<'a> {
    names: HashMap<usize, &'static str>,
    entries: Vec<Vec<CompressedEntry<'a>>>,
}

#[derive(Serialize)]
struct CompressedEntry<'a> {
    key: usize,
    value: &'a dyn DynSerialize,
}

impl<'a> From<&'a Log> for CompressedLog<'a> {
    fn from(log: &'a Log) -> Self {
        let mut clog = CompressedLog::default();

        let mut next_key = 0;
        let mut keys: HashMap<&'static str, usize> = HashMap::new();

        for step in log.steps() {
            let mut cstep = Vec::with_capacity(step.entries().len());

            for entry in step.entries() {
                let key = *keys.entry(entry.name).or_insert_with(|| {
                    next_key += 1;
                    clog.names.insert(next_key, entry.name);
                    next_key
                });
                let value = &entry.value;

                cstep.push(CompressedEntry { key, value });
            }

            clog.entries.push(cstep);
        }

        clog
    }
}

pub fn write_log(output: &mut impl io::Write, log: &Log) -> Result<(), rmp_serde::encode::Error> {
    rmp_serde::encode::write(output, &CompressedLog::from(log))
}

pub fn write_log_file(output: impl AsRef<Path>, log: &Log) -> anyhow::Result<()> {
    let file = File::create(output.as_ref()).context("failed to create log file")?;
    let writer = &mut BufWriter::new(file);
    write_log(writer, log).context("failed to serialize log")
}
