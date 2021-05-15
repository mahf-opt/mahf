use mahf::{
    heuristic,
    heuristics::es::mu_plus_lambda,
    problems::functions::BenchmarkFunction,
    prompt,
    threads::SyncThreadPool,
    tracking::{
        parameter_study::{Study, Summary},
        Log,
    },
};
use std::{fs, io::Write, path::PathBuf, sync::mpsc, thread};

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
    let data_dir = prompt::data_dir(DATA_DIR)?;
    if data_dir.is_none() {
        println!("Execution was canceled.");
        return Ok(());
    }
    let data_dir = PathBuf::from(data_dir.unwrap());
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
                                    heuristic::run(&function, logger, &config, None, None);
                                    summary.add_run(logger);
                                    logger.clear();
                                }

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
