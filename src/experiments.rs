use std::{fmt::Debug, fs, fs::File, path::Path, sync::Arc};

use eyre::WrapErr;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    problems::{KnownOptimumProblem, SingleObjectiveProblem},
    state::random::Random,
    Configuration, ExecResult, State,
};

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
    ron::ser::to_writer_pretty(
        std::io::BufWriter::new(
            File::create(config_log_file).context("failed to create configuration file")?,
        ),
        config.heuristic(),
        ron::ser::PrettyConfig::default().struct_names(true),
    )
    .wrap_err("failed to serialize configuration")?;

    let bar = ProgressBar::new(runs).with_message("Performing Experiment.");
    bar.set_style(ProgressStyle::with_template("{percent}% |{bar:40.white/green}| {pos:>7}/{len:7} [{elapsed_precise}<{eta_precise}, {per_sec}] {msg}").unwrap());

    (0..runs)
        .into_par_iter()
        .progress_with(bar)
        .map(|run| {
            for problem in problems {
                let state = config.optimize_with(problem, |state| {
                    state.insert(Random::new(run));
                    setup(state)
                })?;
                if log {
                    let log_file = data_dir.join(format!("{}_{run}.cbor", problem.name()));
                    state.log().to_cbor(log_file)?;
                }
            }
            Ok(())
        })
        .collect::<ExecResult<()>>()?;

    println!("All runs finished.");
    Ok(())
}
