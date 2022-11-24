use mahf::framework::Random;
use mahf::prelude::*;
use mahf::problems::bmf::BenchmarkFunction;
use mahf::state::common;
use tracking::{functions, trigger};

use std::{
    fs::{self, File},
    io::{BufWriter},
    path::PathBuf,
    sync::{Arc},
};
use anyhow::Context;
use mahf::tracking::{files, Log};

type P = problems::bmf::BenchmarkFunction;

fn main() -> anyhow::Result<()> {
    let folder = "data/bmf";
    let problems = [P::sphere(10), P::ackley(10), P::rastrigin(10)];
    let output = format!("{}{}{}", folder, "/", "PSO");
    let v_max = 1.0 * 10.0 as f64;
    let config = pso::real_pso(
        pso::RealProblemParameters {
            num_particles: 30,
            weight: 0.8,
            c_one: 1.7,
            c_two: 1.7,
            v_max,
        },
        termination::FixedIterations::new(1000),
        tracking::Logger::builder()
            .log_set(
                tracking::LogSet::new()
                    .with_trigger(trigger::Iteration::new(50))
                    .with_trigger(trigger::FinalIter::new(1000))
                    .with_auto_logger::<common::Evaluations>()
                    .with_auto_logger::<common::Progress>()
                    .with_logger(functions::best_individual::<BenchmarkFunction>)
                    .with_logger(functions::best_objective_value::<BenchmarkFunction>),
            )
            .build(),
    );

    let data_dir = Arc::new(PathBuf::from(&output));
    fs::create_dir_all(data_dir.as_ref())?;

    let config_log_file = data_dir.join("configuration.ron");
    ron::ser::to_writer_pretty(
        BufWriter::new(
            File::create(config_log_file).context("failed to create configuration file")?,
        ),
        config.heuristic(),
        ron::ser::PrettyConfig::default().struct_names(true),
    )
        .context("failed to serialize configuration")?;

    for problem in problems {
        let experiment_desc = problem.name();
        let log_file = data_dir.join(format!("{}.log", experiment_desc));

        let state = config.optimize_with(&problem, |state| state.insert(Random::seeded(0)));
        let log = state.get::<Log>();
        files::write_log_file(log_file, log)?;


        println!(
            "Found Fitness: {:?}",
            state.best_objective_value::<P>().unwrap()
        );
        println!(
            "Found Individual: {:?}",
            state.best_individual::<P>().unwrap(),
        );
        println!("Global Optimum: {}", problem.known_optimum());
    }
    Ok(())
}