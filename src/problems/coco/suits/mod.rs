use std::{
    fs,
    io::Write,
    path::PathBuf,
    sync::{mpsc, Arc},
    thread,
};

use anyhow::Context;

use crate::{
    framework::{self, legacy::Configuration},
    problems::coco::CocoInstance,
    random::Random,
    threads::SyncThreadPool,
    tracking::{
        runtime_analysis::Experiment,
        trigger::{EvalTrigger, IterTrigger},
        Log,
    },
};

mod toy;
pub use toy::new as toy;

mod bbob;
pub use bbob::new as bbob;

pub type SuiteGenerator = fn(function: usize, instance: usize, dimension: usize) -> CocoInstance;

pub struct Suite {
    functions: Vec<usize>,
    next_function: usize,
    instances: Vec<usize>,
    next_instance: usize,
    dimensions: Vec<usize>,
    next_dimension: usize,
    generator: SuiteGenerator,
}

impl Suite {
    pub fn new(
        functions: Vec<usize>,
        instances: Vec<usize>,
        dimensions: Vec<usize>,
        generator: SuiteGenerator,
    ) -> Self {
        Suite {
            functions,
            next_function: 0,
            instances,
            next_instance: 0,
            dimensions,
            next_dimension: 0,
            generator,
        }
    }

    pub fn functions(&self) -> &[usize] {
        &self.functions
    }

    pub fn instances(&self) -> &[usize] {
        &self.instances
    }

    pub fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }

    pub fn total_instances(&self) -> usize {
        self.functions.len() * self.instances.len() * self.dimensions.len()
    }

    fn current_instance(&self) -> CocoInstance {
        (self.generator)(
            self.functions[self.next_function],
            self.instances[self.next_instance],
            self.dimensions[self.next_dimension],
        )
    }

    pub fn next_instance(&mut self) -> Option<CocoInstance> {
        if self.next_function == self.functions.len()
            || self.next_instance == self.instances.len()
            || self.next_dimension == self.dimensions.len()
        {
            return None;
        }

        let instance = self.current_instance();

        self.next_instance += 1;
        if self.next_instance == self.instances.len() {
            self.next_instance = 0;
            self.next_function += 1;
        }
        if self.next_function == self.functions.len() {
            self.next_function = 0;
            self.next_dimension += 1;
        }

        Some(instance)
    }
}

impl Iterator for Suite {
    type Item = CocoInstance;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_instance()
    }
}

pub fn evaluate_suite(
    suite: Suite,
    configuration: Configuration<CocoInstance>,
    output_dir: &str,
) -> anyhow::Result<()> {
    let data_dir = Arc::new(PathBuf::from(output_dir));
    fs::create_dir_all(data_dir.as_ref())?;

    let runs = 1;
    let total_runs = suite.total_instances() * (runs as usize);
    let (tx, rx) = mpsc::channel();

    let eval_trigger = EvalTrigger {
        improvement: true,
        interval: None,
    };
    let iter_trigger = IterTrigger {
        improvement: false,
        interval: Some(10),
    };

    let configuration = Arc::new(configuration);
    thread::spawn(move || {
        let mut pool = SyncThreadPool::default();
        for problem in suite {
            let tx = tx.clone();
            let data_dir = data_dir.clone();
            let configuration = configuration.clone();
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
                        framework::legacy::run(&problem, logger, &configuration, None, None);
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
