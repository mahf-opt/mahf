use crate::{
    framework::{self, components::Configuration},
    problems::{coco_bound::CocoInstance, HasKnownTarget},
    random::Random,
    threads::SyncThreadPool,
    tracking::Log,
};
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
                    let log_file = data_dir.join(format!("{}.json", experiment_desc));

                    let state = framework::run(&problem, &configuration, Some(Random::default()));
                    let log = state.get::<Log>();
                    serde_json::to_writer_pretty(BufWriter::new(File::create(log_file)?), log)?;
                    let target_hit = problem.target_hit(state.best_fitness());

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
