use std::{
    fs::File,
    io::{self, BufWriter},
    path::Path,
};

use anyhow::Context;

use crate::tracking::Log;

pub fn write_log(output: &mut impl io::Write, log: &Log) -> Result<(), rmp_serde::encode::Error> {
    rmp_serde::encode::write(output, log)
}

pub fn write_log_file(output: impl AsRef<Path>, log: &Log) -> anyhow::Result<()> {
    let file = File::create(output.as_ref()).context("failed to create log file")?;
    let writer = &mut BufWriter::new(file);
    rmp_serde::encode::write(writer, log).context("failed to serialize log")
}
