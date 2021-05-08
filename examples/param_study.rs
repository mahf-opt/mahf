use mahf::{
    heuristic::{self},
    heuristics::es::mu_plus_lambda,
    problems::functions::BenchmarkFunction,
    threads::SyncThreadPool,
    tracking::{
        parameter_study::{Study, Summary},
        serialize::serialize_config,
        Log,
    },
};
use std::{fs, io::Write, path::Path, sync::mpsc, thread};

static DATA_DIR: &str = "data/parameter_study";

static GENERATIONS: u32 = 500;
static RUNS: u32 = 10;

static MU_OPTIONS: &[u32] = &[5, 10, 15, 20, 25];
static LAMBDA_OPTIONS: &[u32] = &[20, 30, 40, 50, 60];
static SIGMA_OPTIONS: &[f64] = &[1.0, 0.8, 0.5, 0.1, 0.01];

static FUNCTIONS: &[fn(usize) -> BenchmarkFunction] = &[
    BenchmarkFunction::sphere,
    BenchmarkFunction::rastrigin,
    BenchmarkFunction::ackley,
];
static DIMENSIONS: &[usize] = &[10, 20, 30];

#[allow(clippy::clippy::identity_op)]
static NUM_CONFIGS: usize = MU_OPTIONS.len() * LAMBDA_OPTIONS.len() * SIGMA_OPTIONS.len();

pub fn main() -> anyhow::Result<()> {
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
    let data_dir = Path::new(data_dir);
    fs::create_dir_all(&data_dir)?;

    let (tx, rx) = mpsc::channel();

    for &function in FUNCTIONS {
        for &dimension in DIMENSIONS {
            let function = function(dimension);
            let fn_name = function.name();
            let output = data_dir.join(format!("{}_{}.csv", fn_name, dimension));

            println!("Studying {} dimension {}", function.name(), dimension);

            let sample = mu_plus_lambda::<BenchmarkFunction>(0, 0, 0.0, 0);
            let mut study = Study::new(output, &sample).unwrap();

            let tx = tx.clone();
            thread::spawn(move || {
                let mut pool = SyncThreadPool::default();

                for &mu in MU_OPTIONS {
                    for &lambda in LAMBDA_OPTIONS {
                        for &sigma in SIGMA_OPTIONS {
                            let tx = tx.clone();
                            pool.enqueue(move || {
                                let logger = &mut Log::none();
                                let mut summary = Summary::new();

                                let config = mu_plus_lambda(mu, lambda, sigma, GENERATIONS);

                                for _ in 0..RUNS {
                                    heuristic::run(&function, logger, &config);
                                    summary.add_run(logger);
                                    logger.clear();
                                }

                                let config = serialize_config(&config).unwrap();
                                let _ = tx.send((config, summary));
                            });
                        }
                    }
                }
            });

            let mut finished_runs = 0;
            while finished_runs < NUM_CONFIGS {
                let (config, summary) = rx.recv().unwrap();
                study.log_run(&config, &summary)?;

                finished_runs += 1;
                print!("Configs: {}/{}\r", finished_runs, NUM_CONFIGS);
                std::io::stdout().flush().unwrap();
            }
            println!("\nDone.");
        }
    }

    Ok(())
}
