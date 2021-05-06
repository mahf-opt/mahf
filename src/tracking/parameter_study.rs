//! Logging for Parameter Studies

use crate::{fitness::Fitness, heuristic::Configuration, tracking::Log};
use anyhow::{bail, Context};
use std::{
    any::TypeId,
    fs::{self, File},
    io::{self, BufWriter},
    path::{Path, PathBuf},
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

        let output = BufWriter::new(file);
        let config_type = ConfigurationType::from(sample);

        // TODO: write header
        // requires extracting names of the components parameters

        Ok(Study {
            config_type,
            output,
        })
    }

    pub fn log_run<P>(&mut self, config: Configuration<P>, summary: Summary) -> io::Result<()> {
        // TODO: write entry
        // requires extracting the components parameters
        Ok(())
    }
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
        let best = log.evaluations.last().unwrap().best_fx;
        let evaluations = log.evaluations.len();
        self.entries.push(SummaryEntry { best, evaluations });
    }

    pub fn average_best(&self) -> f64 {
        self.entries.iter().map(|e| e.best).sum::<f64>() / self.entries.len() as f64
    }

    pub fn average_evaluations(&self) -> usize {
        self.entries.iter().map(|e| e.evaluations).sum::<usize>() / self.entries.len()
    }
}
