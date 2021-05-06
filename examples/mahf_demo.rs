use anyhow::Context;
use mahf::{
    heuristic::{self, Configuration},
    problems::functions::BenchmarkFunction,
    threads::SyncThreadPool,
    tracking::{runtime_analysis::Experiment, trigger::*, Log},
};
use std::{fs, io::Write, path::Path, sync::mpsc, thread};

//                                //
//    Test Suite Configuration    //
//                                //

static DATA_DIR: &str = "data";
static HEURISTICS: &[(&str, ConfigBuilder)] = &[
    ("iwo", heuristics::iwo),
    ("es", heuristics::es),
    ("c1", heuristics::c1),
    ("c2", heuristics::c2),
    ("c3", heuristics::c3),
    ("c4", heuristics::c4),
    ("c5", heuristics::c5),
    ("c6", heuristics::c6),
];
static FUNCTIONS: &[fn(usize) -> BenchmarkFunction] = &[
    BenchmarkFunction::sphere,
    BenchmarkFunction::rastrigin,
    BenchmarkFunction::ackley,
];
static DIMENSIONS: &[usize] = &[10, 20, 30];
static RUNS: u32 = 50;

//                                //
//     Test Suite Heuristics      //
//                                //

type ConfigBuilder = fn() -> Configuration<BenchmarkFunction>;

#[allow(dead_code)]
mod heuristics {
    use mahf::{heuristic::Configuration, operators::*, problems::functions::BenchmarkFunction};

    pub fn iwo() -> Configuration<BenchmarkFunction> {
        Configuration::new(
            initialization::RandomSpread {
                initial_population_size: 25,
            },
            selection::Iwo {
                min_number_of_seeds: 4,
                max_number_of_seeds: 6,
            },
            generation::Adaptive {
                initial_deviation: 0.1,
                final_deviation: 0.001,
                modulation_index: 5,
            },
            replacement::Fittest {
                max_population_size: 50,
            },
            termination::FixedIterations {
                max_iterations: 500,
            },
        )
    }

    pub fn es() -> Configuration<BenchmarkFunction> {
        let population_size = 5;
        Configuration::new(
            initialization::RandomSpread {
                initial_population_size: population_size,
            },
            selection::Es { lambda: 60 },
            generation::Fixed { deviation: 0.1 },
            replacement::Fittest {
                max_population_size: population_size,
            },
            termination::FixedIterations {
                max_iterations: 500,
            },
        )
    }

    pub fn c1() -> Configuration<BenchmarkFunction> {
        let population_size = 5;
        Configuration::new(
            initialization::RandomSpread {
                initial_population_size: population_size,
            },
            selection::Es { lambda: 60 },
            generation::Fixed { deviation: 0.1 },
            replacement::Fittest {
                max_population_size: 50,
            },
            termination::FixedIterations {
                max_iterations: 500,
            },
        )
    }

    pub fn c2() -> Configuration<BenchmarkFunction> {
        let population_size = 25;
        Configuration::new(
            initialization::RandomSpread {
                initial_population_size: population_size,
            },
            selection::Iwo {
                min_number_of_seeds: 4,
                max_number_of_seeds: 6,
            },
            generation::Adaptive {
                initial_deviation: 0.1,
                final_deviation: 0.001,
                modulation_index: 5,
            },
            replacement::Fittest {
                max_population_size: population_size,
            },
            termination::FixedIterations {
                max_iterations: 500,
            },
        )
    }

    pub fn c3() -> Configuration<BenchmarkFunction> {
        let population_size = 5;
        Configuration::new(
            initialization::RandomSpread {
                initial_population_size: population_size,
            },
            selection::Es { lambda: 60 },
            generation::Adaptive {
                initial_deviation: 0.1,
                final_deviation: 0.001,
                modulation_index: 5,
            },
            replacement::Fittest {
                max_population_size: 50,
            },
            termination::FixedIterations {
                max_iterations: 500,
            },
        )
    }

    pub fn c4() -> Configuration<BenchmarkFunction> {
        let population_size = 25;
        Configuration::new(
            initialization::RandomSpread {
                initial_population_size: population_size,
            },
            selection::Iwo {
                min_number_of_seeds: 4,
                max_number_of_seeds: 6,
            },
            generation::Fixed { deviation: 0.1 },
            replacement::Fittest {
                max_population_size: 50,
            },
            termination::FixedIterations {
                max_iterations: 500,
            },
        )
    }

    pub fn c5() -> Configuration<BenchmarkFunction> {
        let population_size = 25;
        Configuration::new(
            initialization::RandomSpread {
                initial_population_size: population_size,
            },
            selection::Iwo {
                min_number_of_seeds: 4,
                max_number_of_seeds: 6,
            },
            generation::Fixed { deviation: 0.1 },
            replacement::Fittest {
                max_population_size: population_size,
            },
            termination::FixedIterations {
                max_iterations: 500,
            },
        )
    }

    pub fn c6() -> Configuration<BenchmarkFunction> {
        let population_size = 5;
        Configuration::new(
            initialization::RandomSpread {
                initial_population_size: population_size,
            },
            selection::Es { lambda: 60 },
            generation::Adaptive {
                initial_deviation: 0.1,
                final_deviation: 0.001,
                modulation_index: 5,
            },
            replacement::Fittest {
                max_population_size: 50,
            },
            termination::FixedIterations {
                max_iterations: 500,
            },
        )
    }
}

//                                //
//    Test Suite Implementation   //
//                                //

fn main() -> anyhow::Result<()> {
    let mut data_dir = DATA_DIR;
    if Path::new(data_dir).exists() {
        println!("There already exists data from a previous run.");
        println!("Options: y -> delete existing data");
        println!("         r -> rename data directory");
        println!("         n -> cancel execution");
        let reply = rprompt::prompt_reply_stdout("(Y/r/n) ")?;

        #[allow(clippy::wildcard_in_or_patterns)]
        match reply.as_str() {
            "" | "y" | "Y" => {
                fs::remove_dir_all(data_dir)?;
                println!("Old data has been removed.");
            }
            "r" | "R" => {
                let reply = rprompt::prompt_reply_stdout("New data name: ")?;
                data_dir = &*Box::leak(reply.into_boxed_str());
            }
            "n" | "N" | _ => {
                println!("Execution canceled.");
                return Ok(());
            }
        }
    }

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
                            let data_dir = Path::new(data_dir).join(experiment_desc);
                            let experiment =
                                &mut Experiment::create(data_dir, &problem, &configuration())
                                    .context("creating experiment")?;
                            for _ in 0..RUNS {
                                heuristic::run(&problem, logger, configuration());
                                experiment.log_run(logger)?;
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
