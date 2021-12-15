use anyhow::Context;
use mahf::{
    framework::{self, Configuration},
    problems::bmf::BenchmarkFunction,
    prompt,
    random::Random,
    threads::SyncThreadPool,
    tracking::{
        runtime_analysis::Experiment,
        trigger::{EvalTrigger, IterTrigger},
        Log,
    },
};
use std::{fs, io::Write, path::PathBuf, sync::mpsc, thread};

//                                 //
//    Custom Test Configuration    //
//                                 //

static DATA_DIR: &str = "data/custom_heuristic";
static HEURISTICS: &[(&str, ConfigBuilder)] = &[("diversity", heuristics::custom)];
static FUNCTIONS: &[fn(usize) -> BenchmarkFunction] = &[
    BenchmarkFunction::sphere,
    //BenchmarkFunction::rastrigin,
    //BenchmarkFunction::ackley,
];
static DIMENSIONS: &[usize] = &[10]; //, 20, 30
static RUNS: u32 = 5;

//                           //
//     Custom Heuristic      //
//                           //

type ConfigBuilder = fn() -> Configuration<BenchmarkFunction>;

#[allow(dead_code)]
mod heuristics {
    use mahf::{framework::Configuration, operators::*, problems::bmf::BenchmarkFunction};

    pub fn custom() -> Configuration<BenchmarkFunction> {
        let mut custom_config = Configuration::new_extended(
            initialization::RandomSpread {
                initial_population_size: 25,
            },
            Some(postprocesses::DiversityPostInitialization),
            selection::RouletteWheel { offspring: 25 },
            generation::UniformCrossover { pc: 0.8 },
            replacement::Generational {
                max_population_size: 25,
            },
            Some(postprocesses::DiversityPostReplacement),
            termination::FixedIterations {
                max_iterations: 500,
            },
        );
        //TODO adapt this when add_generator is improved; also we need to consider the sequence of operators!
        custom_config =
            custom_config.add_generator(generation::FixedDeviationDelta { deviation: 0.2 });
        custom_config
    }
}

//                                 //
//    Custom Test Implementation   //
//                                 //

fn main() -> anyhow::Result<()> {
    let data_dir = prompt::data_dir(DATA_DIR)?;
    if data_dir.is_none() {
        println!("Execution was canceled.");
        return Ok(());
    }
    let data_dir = PathBuf::from(data_dir.unwrap());
    fs::create_dir_all(&data_dir)?;

    let total_runs = HEURISTICS.len() * FUNCTIONS.len() * DIMENSIONS.len() * (RUNS as usize);
    let (tx, rx) = mpsc::channel();

    let eval_trigger = EvalTrigger {
        improvement: true,
        interval: None,
    };
    let iter_trigger = IterTrigger {
        improvement: false,
        interval: Some(10),
    };

    thread::spawn(move || {
        let mut pool = SyncThreadPool::default();
        for (heuristic_name, configuration) in HEURISTICS {
            for function in FUNCTIONS {
                for &dimension in DIMENSIONS {
                    let tx = tx.clone();
                    let data_dir = data_dir.clone();
                    pool.enqueue(move || {
                        let result: anyhow::Result<()> = (|| {
                            let logger = &mut Log::new(eval_trigger, iter_trigger);

                            let problem = function(dimension);
                            let experiment_desc = format!(
                                "{}_{}_{}",
                                heuristic_name,
                                problem.name(),
                                problem.dimension()
                            );

                            let data_dir = data_dir.join(experiment_desc);

                            let random = Random::default();
                            let config = configuration();
                            let experiment =
                                &mut Experiment::create(data_dir, &problem, &random, &config)
                                    .context("creating experiment")?;

                            for _ in 0..RUNS {
                                framework::run(&problem, logger, &config, None, None);
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
            }
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
