use crate::{
    framework::Configuration,
    problems::{
        coco_bound::{CocoEvaluator, CocoInstance},
        HasKnownTarget,
    },
    state::{common, State},
    tracking::{files, Log},
};
use anyhow::Context;
use coco_rs::{Suite, SuiteName};
use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::PathBuf,
    sync::mpsc,
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
    setup: impl Fn(&mut State<CocoInstance>) + Send + Sync,
) -> anyhow::Result<()> {
    #[allow(unused_variables)]
    let num_threads = 1;
    let num_threads = num_cpus::get() as u32;

    let data_dir = &PathBuf::from(output_dir);
    fs::create_dir_all(data_dir)?;

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

    let configuration = &configuration;
    let setup = &setup;

    thread::scope(move |scope| {
        scope.spawn(move || {
            let mut pool = scoped_threadpool::Pool::new(num_threads);

            loop {
                let (function_idx, dimension_idx, instance_idx) = match suite.next_problem(None) {
                    None => break,
                    Some(problem) => (
                        problem.function_index(),
                        problem.dimension_index(),
                        problem.instance_index(),
                    ),
                };

                // Create a new suite, because COCO doesn't guarantee that
                // multiple problems can be created from one suite simultaneously.
                let mut suite = suite.clone();
                let tx = tx.clone();

                pool.scoped(move |pool| {
                    pool.execute(move || {
                        let problem = suite
                            .problem_by_function_dimension_instance_index(
                                function_idx,
                                dimension_idx,
                                instance_idx,
                            )
                            .unwrap();
                        let instance = CocoInstance::from(&problem);

                        let result: anyhow::Result<_> = (move || {
                            let experiment_desc = instance.format_name();
                            let log_file = data_dir.join(format!("{}.log", experiment_desc));

                            let state = configuration.optimize_with(&instance, |state| {
                                state.insert(common::EvaluatorInstance::new(CocoEvaluator {
                                    problem,
                                }));
                                setup(state);
                            });
                            let log = state.get::<Log>();
                            files::write_log_file(log_file, log)?;

                            let target_hit = if let Some(fitness) = state.best_objective_value() {
                                instance.target_hit(*fitness)
                            } else {
                                false
                            };
                            Ok(target_hit)
                        })();

                        let _ = tx.send(result);
                    });
                });
            }
        });

        let mut finished_runs = 0;
        let mut successful_runs = 0;
        while finished_runs < total_runs {
            let hit = rx.recv().unwrap()?;
            finished_runs += 1;
            successful_runs += i32::from(hit);
            print!(
                "Runs: {}/{}/{}\r",
                successful_runs, finished_runs, total_runs
            );
            std::io::stdout().flush().unwrap();
        }
        println!("\nDone.");

        Ok(())
    })
}
