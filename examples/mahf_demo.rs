use mahf::{
    problem::Problem,
    problems::functions::BenchmarkFunction,
    threads::SyncThreadPool,
    tracking::{trigger::*, write_log, Log},
};
use std::{
    fs,
    io::{self, Write},
    path::Path,
    sync::mpsc,
    thread,
};

//                                //
//    Test Suite Configuration    //
//                                //

static DATA_DIR: &str = "data";
static HEURISTICS: &[(&str, Heuristic)] = &[
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

type Heuristic = fn(problem: &BenchmarkFunction, logger: &mut Log);

#[allow(dead_code)]
mod heuristics {
    use mahf::{
        modular::{self, operators},
        problems::functions::BenchmarkFunction,
        tracking::Log,
    };

    pub fn iwo(problem: &BenchmarkFunction, logger: &mut Log) {
        modular::heuristic::run(
            problem,
            logger,
            operators::RandomSpreadInitialization {
                initial_population_size: 25,
            },
            operators::IwoSelection {
                min_number_of_seeds: 4,
                max_number_of_seeds: 6,
            },
            operators::AdaptiveGeneration {
                initial_deviation: 0.1,
                final_deviation: 0.001,
                modulation_index: 5,
            },
            operators::FittestReplacement {
                max_population_size: 50,
            },
            operators::FixedIterationsTermination {
                max_iterations: 500,
            },
        );
    }

    pub fn es(problem: &BenchmarkFunction, logger: &mut Log) {
        let population_size = 5;
        modular::heuristic::run(
            problem,
            logger,
            operators::RandomSpreadInitialization {
                initial_population_size: population_size,
            },
            operators::EsSelection { lambda: 60 },
            operators::FixedGeneration { deviation: 0.1 },
            operators::FittestReplacement {
                max_population_size: population_size,
            },
            operators::FixedIterationsTermination {
                max_iterations: 500,
            },
        );
    }

    pub fn c1(problem: &BenchmarkFunction, logger: &mut Log) {
        let population_size = 5;
        modular::heuristic::run(
            problem,
            logger,
            operators::RandomSpreadInitialization {
                initial_population_size: population_size,
            },
            operators::EsSelection { lambda: 60 },
            operators::FixedGeneration { deviation: 0.1 },
            operators::FittestReplacement {
                max_population_size: 50,
            },
            operators::FixedIterationsTermination {
                max_iterations: 500,
            },
        );
    }

    pub fn c2(problem: &BenchmarkFunction, logger: &mut Log) {
        let population_size = 25;
        modular::heuristic::run(
            problem,
            logger,
            operators::RandomSpreadInitialization {
                initial_population_size: population_size,
            },
            operators::IwoSelection {
                min_number_of_seeds: 4,
                max_number_of_seeds: 6,
            },
            operators::AdaptiveGeneration {
                initial_deviation: 0.1,
                final_deviation: 0.001,
                modulation_index: 5,
            },
            operators::FittestReplacement {
                max_population_size: population_size,
            },
            operators::FixedIterationsTermination {
                max_iterations: 500,
            },
        );
    }

    pub fn c3(problem: &BenchmarkFunction, logger: &mut Log) {
        let population_size = 5;
        modular::heuristic::run(
            problem,
            logger,
            operators::RandomSpreadInitialization {
                initial_population_size: population_size,
            },
            operators::EsSelection { lambda: 60 },
            operators::AdaptiveGeneration {
                initial_deviation: 0.1,
                final_deviation: 0.001,
                modulation_index: 5,
            },
            operators::FittestReplacement {
                max_population_size: 50,
            },
            operators::FixedIterationsTermination {
                max_iterations: 500,
            },
        );
    }

    pub fn c4(problem: &BenchmarkFunction, logger: &mut Log) {
        let population_size = 25;
        modular::heuristic::run(
            problem,
            logger,
            operators::RandomSpreadInitialization {
                initial_population_size: population_size,
            },
            operators::IwoSelection {
                min_number_of_seeds: 4,
                max_number_of_seeds: 6,
            },
            operators::FixedGeneration { deviation: 0.1 },
            operators::FittestReplacement {
                max_population_size: 50,
            },
            operators::FixedIterationsTermination {
                max_iterations: 500,
            },
        );
    }

    pub fn c5(problem: &BenchmarkFunction, logger: &mut Log) {
        let population_size = 25;
        modular::heuristic::run(
            problem,
            logger,
            operators::RandomSpreadInitialization {
                initial_population_size: population_size,
            },
            operators::IwoSelection {
                min_number_of_seeds: 4,
                max_number_of_seeds: 6,
            },
            operators::FixedGeneration { deviation: 0.1 },
            operators::FittestReplacement {
                max_population_size: population_size,
            },
            operators::FixedIterationsTermination {
                max_iterations: 500,
            },
        );
    }

    pub fn c6(problem: &BenchmarkFunction, logger: &mut Log) {
        let population_size = 5;
        modular::heuristic::run(
            problem,
            logger,
            operators::RandomSpreadInitialization {
                initial_population_size: population_size,
            },
            operators::EsSelection { lambda: 60 },
            operators::AdaptiveGeneration {
                initial_deviation: 0.1,
                final_deviation: 0.001,
                modulation_index: 5,
            },
            operators::FittestReplacement {
                max_population_size: 50,
            },
            operators::FixedIterationsTermination {
                max_iterations: 500,
            },
        );
    }
}

//                                //
//    Test Suite Implementation   //
//                                //

fn main() -> io::Result<()> {
    if Path::new(DATA_DIR).exists() {
        println!("There already exists data from a previous run.");
        let reply = rprompt::prompt_reply_stdout("Remove old data (Y/n) ")?;

        #[allow(clippy::wildcard_in_or_patterns)]
        match reply.as_str() {
            "" | "y" | "Y" => {
                fs::remove_dir_all(DATA_DIR)?;
                println!("Old data has been removed.");
            }
            "n" | _ => {
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
        let mut pool = SyncThreadPool::new(num_cpus::get());
        for (heuristic_name, heuristic) in HEURISTICS {
            for function in FUNCTIONS {
                for &dimension in DIMENSIONS {
                    let tx = tx.clone();
                    pool.enqueue(move || {
                        let logger = &mut Log::new(eval_trigger, iter_trigger);
                        for run in 0..RUNS {
                            let problem = function(dimension);
                            let function_name = problem.name();
                            heuristic(&problem, logger);
                            let result = write_log(
                                DATA_DIR,
                                heuristic_name,
                                function_name,
                                dimension,
                                run,
                                logger,
                            );
                            tx.send(result).unwrap();
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
