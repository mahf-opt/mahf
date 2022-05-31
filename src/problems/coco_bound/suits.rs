use crate::{
    framework::{self, components::Configuration},
    problems::coco_bound::CocoInstance,
    random::Random,
    threads::SyncThreadPool,
};
use coco_rs::{Suite, SuiteName};
use std::{
    fs,
    io::Write,
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

    let runs = 1;
    let total_runs = suite.number_of_problems() * (runs as usize);
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
                #[allow(clippy::redundant_closure_call)]
                let result: anyhow::Result<()> = (|| {
                    let experiment_desc = problem.format_name();
                    let _data_dir = data_dir.join(experiment_desc);

                    for _ in 0..runs {
                        framework::run(&problem, &configuration, Some(Random::default()));
                        let _ = tx.send(Ok(()));
                    }

                    Ok(())
                })();

                if result.is_err() {
                    let _ = tx.send(result);
                }
            });
        }
    });

    let mut finished_runs = 0;
    while finished_runs < total_runs {
        rx.recv().unwrap()?;
        finished_runs += 1;
        print!("Runs: {}/{}\r", finished_runs, total_runs);
        std::io::stdout().flush().unwrap();
    }
    println!("\nDone.");

    Ok(())
}
