use crate::{
    framework::{self, Configuration},
    problems::coco_bound::CocoInstance,
    random::Random,
    threads::SyncThreadPool,
    tracking::{
        runtime_analysis::Experiment,
        trigger::{EvalTrigger, IterTrigger},
        Log,
    },
};
use anyhow::Context;
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

    let eval_trigger = EvalTrigger {
        improvement: true,
        interval: None,
    };
    let iter_trigger = IterTrigger {
        improvement: false,
        interval: Some(10),
    };

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
                let result: anyhow::Result<()> = (|| {
                    let logger = &mut Log::new(eval_trigger, iter_trigger);

                    let experiment_desc = problem.format_name();
                    let data_dir = data_dir.join(experiment_desc);

                    let random = Random::default();
                    let experiment =
                        &mut Experiment::create(data_dir, &problem, &random, &configuration)
                            .context("creating experiment")?;

                    for _ in 0..runs {
                        framework::run(&problem, logger, &configuration, None, None);
                        experiment.log_run(logger)?;
                        logger.clear();
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
