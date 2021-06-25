use mahf::{
    heuristic::{self, Configuration},
    operators::*,
    problems::bmf::BenchmarkFunction,
    random::Random,
    tracking::Log,
};
use std::convert::TryFrom;

use crate::util::{print_result, ArgsIter, BaseParams};

#[derive(Debug, Default)]
struct Parameters {
    base: BaseParams,

    initial_population_size: u32,
    max_population_size: u32,
    min_number_of_seeds: u32,
    max_number_of_seeds: u32,
    initial_deviation: f64,
    final_deviation: f64,
    modulation_index: u32,
    max_iterations: u32,
}

fn parameters(base_params: BaseParams, args: &mut ArgsIter) -> Parameters {
    let mut params = Parameters::default();
    params.base = base_params;

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

pub fn run(base_params: BaseParams, args: &mut ArgsIter) {
    let params = parameters(base_params, args);

    if !(params.initial_population_size <= params.max_population_size
        && params.min_number_of_seeds <= params.max_number_of_seeds
        && params.final_deviation <= params.initial_deviation)
    {
        print_result(
            false,
            params.base.cutoff_time,
            params.base.cutoff_length,
            1000.0,
            params.base.seed,
        );
        return;
    }

    let problem = BenchmarkFunction::try_from(params.base.instance.as_str()).unwrap();

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
            max_iterations: 500,
        },
    );

    let logger = &mut Log::none();
    let rng = Random::seeded(params.base.seed);

    heuristic::run(&problem, logger, &config, Some(rng), None);

    print_result(
        false,
        params.base.cutoff_time,
        logger.final_iteration().iteration,
        logger.final_best_fx(),
        params.base.seed,
    );
}
