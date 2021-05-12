//! Logging for Parameter Studies

use crate::{
    heuristic::Configuration,
    tracking::{
        serialize::{serialize_config, SerializedConfiguration},
        Log,
    },
};
use anyhow::{bail, Context};
use std::{
    any::TypeId,
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::Path,
};

#[derive(PartialEq)]
struct ConfigurationType {
    initialization: TypeId,
    selection: TypeId,
    generation: TypeId,
    replacement: TypeId,
    termination: TypeId,
}

impl<P> From<&Configuration<P>> for ConfigurationType {
    fn from(config: &Configuration<P>) -> Self {
        ConfigurationType {
            initialization: config.initialization.type_id(),
            selection: config.selection.type_id(),
            generation: config.generation.type_id(),
            replacement: config.replacement.type_id(),
            termination: config.termination.type_id(),
        }
    }
}

pub struct Study {
    config_type: ConfigurationType,
    output: BufWriter<File>,
}

impl Study {
    pub fn new<P, Out>(output: Out, sample: &Configuration<P>) -> anyhow::Result<Self>
    where
        Out: AsRef<Path>,
    {
        if output.as_ref().exists() {
            bail!("output file already exists");
        }

        let file = fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(output)
            .context("opening output file")?;

        let mut output = BufWriter::new(file);
        let config_type = ConfigurationType::from(sample);

        let config = serialize_config(sample).context("serializing sample config")?;
        write_header(&mut output, &config).context("writing header")?;

        Ok(Study {
            config_type,
            output,
        })
    }

    pub fn log_run<P>(
        &mut self,
        config: &Configuration<P>,
        summary: &Summary,
    ) -> anyhow::Result<()> {
        if self.config_type != ConfigurationType::from(config) {
            bail!("tried to log a run with different configuration type");
        }
        let config = serialize_config(config)?;
        self.log_serialized_run(&config, summary)
    }

    pub fn log_serialized_run(
        &mut self,
        config: &SerializedConfiguration,
        summary: &Summary,
    ) -> anyhow::Result<()> {
        let fitness = summary.average_best();
        let evaluations = summary.average_evaluations();

        write!(self.output, "{},{}", fitness, evaluations)?;

        let values = collect_values(&config);
        for value in values {
            write!(self.output, ",{}", value)?;
        }

        writeln!(self.output)?;
        Ok(())
    }
}

fn write_header(output: &mut impl Write, config: &SerializedConfiguration) -> io::Result<()> {
    let headers = collect_headers(&config);

    write!(output, "fitness,evaluations")?;

    for header in headers {
        write!(output, ",{}", header)?;
    }

    writeln!(output)
}

fn collect_headers(c: &SerializedConfiguration) -> Vec<String> {
    let mut headers = Vec::new();

    headers.extend(c.initialization.fields.keys().map(|k| format!("i.{}", k)));
    headers.extend(c.selection.fields.keys().map(|k| format!("s.{}", k)));
    headers.extend(c.generation.fields.keys().map(|k| format!("g.{}", k)));
    headers.extend(c.replacement.fields.keys().map(|k| format!("r.{}", k)));
    headers.extend(c.termination.fields.keys().map(|k| format!("t.{}", k)));

    headers
}

fn collect_values(c: &SerializedConfiguration) -> Vec<&str> {
    let mut values = Vec::new();

    values.extend(c.initialization.fields.values().map(|v| v.as_str()));
    values.extend(c.selection.fields.values().map(|v| v.as_str()));
    values.extend(c.generation.fields.values().map(|v| v.as_str()));
    values.extend(c.replacement.fields.values().map(|v| v.as_str()));
    values.extend(c.termination.fields.values().map(|v| v.as_str()));

    values
}

#[derive(Default)]
pub struct Summary {
    entries: Vec<SummaryEntry>,
}

struct SummaryEntry {
    best: f64,
    evaluations: usize,
}

impl Summary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_run(&mut self, log: &Log) {
        let last = log.evaluations.last().unwrap();
        let best = last.best_fx;
        let evaluations = last.evaluation as usize;
        self.entries.push(SummaryEntry { best, evaluations });
    }

    pub fn average_best(&self) -> f64 {
        self.entries.iter().map(|e| e.best).sum::<f64>() / self.entries.len() as f64
    }

    pub fn average_evaluations(&self) -> usize {
        self.entries.iter().map(|e| e.evaluations).sum::<usize>() / self.entries.len()
    }
}
