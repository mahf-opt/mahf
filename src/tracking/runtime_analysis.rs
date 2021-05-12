//! Logging for Runtime Analysis
//!
//! # Format
//! - data_dir
//!   - evaluations.csv
//!   - iterations.csv
//!   - summary.csv
//!   - configuration.ron
//!

use anyhow::Context;
use serde::Serialize;

use crate::{
    heuristic::Configuration,
    problem::Problem,
    tracking::{EvaluationEntry, IterationEntry, Log},
};
use std::{
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::Path,
};

pub struct Experiment {
    logged_runs: u32,
    evaluations_file: File,
    iterations_file: File,
    summary_file: File,
}

impl Experiment {
    pub fn create<P: Problem + Serialize>(
        data_dir: impl AsRef<Path>,
        problem: &P,
        config: &Configuration<P>,
    ) -> Result<Self, anyhow::Error> {
        if data_dir.as_ref().exists() {
            anyhow::bail!("experiment already exists");
        }

        fs::create_dir_all(&data_dir).context("creating data directory")?;

        write_configuration(&data_dir, problem, config).context("writing configuration file")?;

        let dir = data_dir.as_ref();
        let mut evaluations_file =
            open_new_file(dir.join("evaluations.csv")).context("opening evaluations file")?;
        let mut iterations_file =
            open_new_file(dir.join("iterations.csv")).context("opening iterations file")?;
        let mut summary_file =
            open_new_file(dir.join("summary.csv")).context("opening summary file")?;

        writeln!(evaluations_file, "evaluation,current_fx,best_fx")
            .context("writing evaluations header")?;
        writeln!(iterations_file, "iteration,best_fx,diversity")
            .context("writing iterations header")?;
        writeln!(summary_file, "run,iterations,evaluations,best")
            .context("writing summary header")?;

        Ok(Experiment {
            logged_runs: 0,
            evaluations_file,
            iterations_file,
            summary_file,
        })
    }

    pub fn log_run(&mut self, log: &Log) -> anyhow::Result<()> {
        self.logged_runs += 1;

        let eval_buf = &mut BufWriter::new(&mut self.evaluations_file);
        let iter_buf = &mut BufWriter::new(&mut self.iterations_file);
        let summ_buf = &mut self.summary_file;

        write_evaluations(eval_buf, &log.evaluations).context("writing evaluations")?;
        write_iterations(iter_buf, &log.iterations).context("writing iterations")?;
        write_summary(summ_buf, self.logged_runs, log).context("writing summary")?;

        Ok(())
    }
}

#[derive(Serialize)]
#[serde(rename = "Experiment")]
struct ExperimentConfiguration<'a, P: Serialize + 'static> {
    problem: &'a P,
    heuristic: &'a Configuration<P>,
}

fn write_configuration<P: Serialize>(
    data_dir: impl AsRef<Path>,
    problem: &P,
    config: &Configuration<P>,
) -> anyhow::Result<()> {
    let cfg = ExperimentConfiguration {
        problem,
        heuristic: config,
    };
    let cfg_string = to_pretty_ron(&cfg).context("serializing configuration")?;
    let cfg_path = data_dir.as_ref().join("configuration.ron");
    let mut cfg_file = open_new_file(cfg_path).context("opening configuration file")?;
    cfg_file
        .write_all(cfg_string.as_bytes())
        .context("writing configuration to file")?;
    Ok(())
}

fn write_evaluations(output: &mut impl Write, log: &[EvaluationEntry]) -> io::Result<()> {
    for entry in log {
        let &EvaluationEntry {
            evaluation,
            current_fx,
            best_fx,
        } = entry;
        writeln!(
            output,
            "{},{:+1.5e},{:+1.5e}",
            evaluation, current_fx, best_fx
        )?;
    }

    Ok(())
}

fn write_iterations(output: &mut impl Write, log: &[IterationEntry]) -> io::Result<()> {
    for entry in log {
        let &IterationEntry {
            iteration,
            best_fx,
            diversity,
        } = entry;
        writeln!(
            output,
            "{},{:+1.5e},{:+1.5e}",
            iteration, best_fx, diversity
        )?;
    }

    Ok(())
}

fn write_summary(output: &mut impl Write, run: u32, log: &Log) -> io::Result<()> {
    let iterations = log.iterations.len();
    let evaluations = log.evaluations.len();
    let best = log.evaluations.last().unwrap().best_fx;

    writeln!(output, "{},{},{},{}", run, iterations, evaluations, best)
}

fn open_new_file(path: impl AsRef<Path>) -> io::Result<File> {
    fs::OpenOptions::new().create(true).write(true).open(path)
}

pub fn to_pretty_ron<T>(value: &T) -> ron::Result<String>
where
    T: serde::Serialize,
{
    let mut buf = Vec::new();
    let config = ron::ser::PrettyConfig::default();
    let mut s = ron::ser::Serializer::new(&mut buf, Some(config), true)?;
    value.serialize(&mut s)?;
    Ok(String::from_utf8(buf).expect("Ron should be utf-8"))
}
