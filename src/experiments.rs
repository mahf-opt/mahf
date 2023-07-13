//! Utilities for performing experiments.

use std::{fmt::Debug, fs, path::Path, sync::Arc};

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{
    problems::{KnownOptimumProblem, SingleObjectiveProblem},
    state::random::Random,
    Configuration, ExecResult, State,
};

/// Execute the `config` on all `problems` `runs` times in parallel.
///
/// Execution is parallelized using [`rayon`].
/// Progress is displayed using [`indicatif`].
///
/// # Setup
///
/// The `setup` function can be used to initialize the [`State`] before each run.
///
/// Note that the [`Random`] seed is automatically set to the run number.
///
/// # Saving the results
///
/// The `config` is serialized into the `folder`.
///
/// If `log` is true, the [`Log`] is serialized into the `folder` using the [`Log::to_cbor`] method.
///
/// [`Log`]: crate::logging::Log
/// [`Log::to_cbor`]: crate::logging::Log::to_cbor
pub fn par_experiment<P>(
    config: &Configuration<P>,
    setup: impl Fn(&mut State<P>) -> ExecResult<()> + Send + Sync,
    problems: &[P],
    runs: u64,
    folder: impl AsRef<Path>,
    log: bool,
) -> ExecResult<()>
where
    P: SingleObjectiveProblem + KnownOptimumProblem + Send + Sync,
    P::Encoding: Debug,
{
    // Create data dir
    let data_dir = Arc::new(folder.as_ref());
    fs::create_dir_all(data_dir.as_ref())?;

    // Write configuration RON file
    let config_log_file = data_dir.join("configuration.ron");
    config.to_ron(config_log_file)?;

    let bar = ProgressBar::new(runs).with_message("Performing Experiment.");
    bar.set_style(ProgressStyle::with_template("{percent}% |{bar:40.white/green}| {pos:>7}/{len:7} [{elapsed_precise}<{eta_precise}, {per_sec}] {msg}").unwrap());

    (0..runs)
        .cartesian_product(problems)
        .par_bridge()
        .progress_with(bar)
        .map(|(run, problem)| {
            let state = config.optimize_with(problem, |state| {
                state.insert(Random::new(run));
                setup(state)
            })?;
            if log {
                let log_file = data_dir.join(format!("{}_{run}.cbor", problem.name()));
                state.log().to_cbor(log_file)?;
            }
            Ok(())
        })
        .collect::<ExecResult<()>>()?;

    println!("All runs finished.");
    Ok(())
}
