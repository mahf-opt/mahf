use crate::{
    framework::{self, Configuration, Random},
    problems::{coco_bound::CocoInstance, HasKnownTarget},
    tracking::{files, Log},
    utils::threads::SyncThreadPool,
};
use anyhow::Context;
use coco_rs::{Suite, SuiteName};
use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::PathBuf,
    sync::{mpsc, Arc},
    thread,
};

pub fn toy() -> Suite {
    Suite::new(SuiteName::Toy, "", "").unwrap()
}

pub fn bbob() -> Suite {
    Suite::new(SuiteName::Bbob, "", "").unwrap()
}

pub fn largescale() -> Suite {
    Suite::new(SuiteName::BbobLargescale, "", "").unwrap()
}

pub fn evaluate_suite(
    mut suite: Suite,
    configuration: Configuration<CocoInstance>,
    output_dir: &str,
) -> anyhow::Result<()> {
    let data_dir = Arc::new(PathBuf::from(output_dir));
    fs::create_dir_all(data_dir.as_ref())?;

    let config_log_file = data_dir.join("configuration.ron");
    ron::ser::to_writer_pretty(
        BufWriter::new(
            File::create(config_log_file).context("failed to create configuration file")?,
        ),
        configuration.heuristic(),
        ron::ser::PrettyConfig::default().struct_names(true),
    )
    .context("failed to serialize configuration")?;

    let total_runs = suite.number_of_problems();
    let (tx, rx) = mpsc::channel();

    coco_rs::set_log_level(coco_rs::LogLevel::Warning);

    let configuration = Arc::new(configuration);
    thread::spawn(move || {
        let mut pool = SyncThreadPool::default();
        while let Some(problem) = suite.next_problem(None) {
            let tx = tx.clone();
            let data_dir = data_dir.clone();
            let configuration = configuration.clone();
            let problem: CocoInstance = problem.into();
            pool.enqueue(move || {
                let result: anyhow::Result<_> = (|| {
                    let experiment_desc = problem.format_name();
                    let log_file = data_dir.join(format!("{}.log", experiment_desc));

                    let state = framework::run(&problem, &configuration, Some(Random::default()));
                    let log = state.get::<Log>();
                    files::write_log_file(log_file, log)?;

                    let target_hit =
                        if let Some(fitness) = state.best_objective_value::<CocoInstance>() {
                            problem.target_hit(*fitness)
                        } else {
                            false
                        };
                    Ok(target_hit)
                })();

                let _ = tx.send(result);
            });
        }
    });

    let mut finished_runs = 0;
    let mut successful_runs = 0;
    while finished_runs < total_runs {
        let hit = rx.recv().unwrap()?;
        finished_runs += 1;
        successful_runs += if hit { 1 } else { 0 };
        print!(
            "Runs: {}/{}/{}\r",
            successful_runs, finished_runs, total_runs
        );
        std::io::stdout().flush().unwrap();
    }
    println!("\nDone.");

    Ok(())
}
