use mahf::{
    float_eq::float_eq,
    heuristic::{self, Configuration},
    operators::*,
    problems::bmf::BenchmarkFunction,
    random::Random,
    tracking::Log,
};
use std::{convert::TryFrom, time::Instant};

use crate::util::{print_result, ArgsIter, Setup};

#[derive(Debug, Default)]
struct Parameters {
    initial_population_size: u32,
    max_population_size: u32,
    min_number_of_seeds: u32,
    max_number_of_seeds: u32,
    initial_deviation: f64,
    final_deviation: f64,
    modulation_index: u32,
    max_iterations: u32,
}

fn parameters(args: &mut ArgsIter) -> Parameters {
    let mut params = Parameters::default();

    while let Some(param) = args.next() {
        let value = args.next().unwrap();

        match param.as_str() {
            "-initial_population_size" => params.initial_population_size = value.parse().unwrap(),
            "-max_population_size" => params.max_population_size = value.parse().unwrap(),
            "-min_number_of_seeds" => params.min_number_of_seeds = value.parse().unwrap(),
            "-max_number_of_seeds" => params.max_number_of_seeds = value.parse().unwrap(),
            "-initial_deviation" => params.initial_deviation = value.parse().unwrap(),
            "-final_deviation" => params.final_deviation = value.parse().unwrap(),
            "-modulation_index" => params.modulation_index = value.parse().unwrap(),
            "-max_iterations" => params.max_iterations = value.parse().unwrap(),
            unknown => panic!("unknown param {}", unknown),
        }
    }

    params
}

pub fn run(setup: &Setup, args: &mut ArgsIter) {
    let params = parameters(args);

    if !(params.initial_population_size <= params.max_population_size
        && params.min_number_of_seeds <= params.max_number_of_seeds
        && params.final_deviation <= params.initial_deviation)
    {
        // TODO: Is there a better way to indicate "illigal configuration"?
        return;
    }

    let problem = BenchmarkFunction::try_from(setup.instance.as_str()).unwrap();

    let config = Configuration::new(
        initialization::RandomSpread {
            initial_population_size: params.initial_population_size,
        },
        selection::FitnessProportional {
            min_offspring: params.min_number_of_seeds,
            max_offspring: params.max_number_of_seeds,
        },
        generation::AdaptiveDeviationDelta {
            initial_deviation: params.initial_deviation,
            final_deviation: params.final_deviation,
            modulation_index: params.modulation_index,
        },
        replacement::Fittest {
            max_population_size: params.max_population_size,
        },
        termination::FixedIterations {
            max_iterations: setup.cutoff_length,
        },
    );

    let logger = &mut Log::none();
    let rng = Random::seeded(setup.seed);

    let start = Instant::now();
    heuristic::run(&problem, logger, &config, Some(rng), None);
    let end = Instant::now();
    let runtime = end - start;

    println!("Final value: {}", logger.final_best_fx());
    print_result(
        float_eq!(problem.known_optimum(), logger.final_best_fx(), abs <= 0.3),
        runtime.as_secs_f64(),
        logger.final_iteration().iteration,
        1.0 - (problem.known_optimum() + 10.0) / (logger.final_best_fx() + 10.0),
        setup.seed,
    );
}
