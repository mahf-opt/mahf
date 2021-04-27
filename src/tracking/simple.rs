//! Custom Log Format
use crate::tracking::*;
use std::{
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::Path,
};

pub fn write_log(
    data_dir: impl AsRef<Path>,
    heuristic: &str,
    function: &str,
    dimension: usize,
    run: u32,
    log: &mut Log,
) -> io::Result<()> {
    log.finalize();

    if log.evaluations.is_empty() && log.iterations.is_empty() {
        return Ok(());
    }

    let dir_name = format!("{}_{}_{}", heuristic, function, dimension);
    let eval_file_name = format!("{}_evaluations.csv", run);
    let iter_file_name = format!("{}_iterations.csv", run);

    let dir_path = data_dir.as_ref().join(&dir_name);
    let eval_file_path = dir_path.join(&eval_file_name);
    let iter_file_path = dir_path.join(&iter_file_name);

    fs::create_dir_all(&dir_path)?;

    let eval_file = &mut BufWriter::new(open_new(&eval_file_path)?);
    write_evaluations(eval_file, &log.evaluations)?;

    let iter_file = &mut BufWriter::new(open_new(&iter_file_path)?);
    write_iterations(iter_file, &log.iterations)?;

    log.clear();
    Ok(())
}

fn write_evaluations(output: &mut impl Write, log: &[EvaluationEntry]) -> io::Result<()> {
    writeln!(output, "evaluation,current_fx,best_fx")?;

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
    writeln!(output, "iteration,best_fx,diversity")?;

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

fn open_new(path: impl AsRef<Path>) -> io::Result<File> {
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
}
