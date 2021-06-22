use mahf::{
    heuristic::{self, Configuration},
    operators::*,
    problems::bmf::BenchmarkFunction,
    random::Random,
    tracking::Log,
};
use std::convert::TryFrom;

#[derive(Debug, Default)]
struct Parameters {
    instance: String,
    instance_information: String,
    cutoff_time: f64,
    cutoff_length: u32,
    seed: u64,

    initial_population_size: u32,
    max_population_size: u32,
    min_number_of_seeds: u32,
    max_number_of_seeds: u32,
    initial_deviation: f64,
    final_deviation: f64,
    modulation_index: u32,
    max_iterations: u32,
}

fn parameters() -> Parameters {
    let mut params = Parameters::default();
    let mut args = std::env::args().skip(1);

    params.instance = args.next().unwrap();
    params.instance_information = args.next().unwrap();
    params.cutoff_time = args.next().unwrap().parse().unwrap();
    params.cutoff_length = args.next().unwrap().parse().unwrap();
    params.seed = args.next().unwrap().parse().unwrap();

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

fn print_result(sat: bool, runtime: f64, runlength: u32, best: f64, seed: u64) {
    println!(
        "Result for ParamILS: {}, {}, {}, {}, {}",
        if sat { "SAT" } else { "TIMEOUT" },
        runtime,
        runlength,
        best,
        seed
    );
}

fn main() {
    let params = parameters();

    if !(params.initial_population_size <= params.max_population_size
        && params.min_number_of_seeds <= params.max_number_of_seeds
        && params.final_deviation <= params.initial_deviation)
    {
        print_result(
            false,
            params.cutoff_time,
            params.cutoff_length,
            1000.0,
            params.seed,
        );
        return;
    }

    let problem = BenchmarkFunction::try_from(params.instance.as_str()).unwrap();

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
    let rng = Random::seeded(params.seed);

    heuristic::run(&problem, logger, &config, Some(rng), None);

    print_result(
        false,
        params.cutoff_time,
        logger.final_iteration().iteration,
        logger.final_best_fx(),
        params.seed,
    );
}
