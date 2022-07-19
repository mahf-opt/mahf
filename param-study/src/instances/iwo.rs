use crate::{
    declare_parameters,
    util::{print_result, ArgsIter, Setup},
};
use mahf::{
    float_eq::float_eq, framework, heuristics::iwo, operators::termination,
    problems::bmf::BenchmarkFunction, random::Random, tracking,
};
use std::time::Instant;

declare_parameters! {
    initial_population_size: u32,
    max_population_size: u32,
    min_number_of_seeds: u32,
    max_number_of_seeds: u32,
    initial_deviation: f64,
    final_deviation: f64,
    modulation_index: u32,
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

    let config = iwo::iwo(
        iwo::Parameters {
            initial_population_size: params.initial_population_size,
            max_population_size: params.max_population_size,
            min_number_of_seeds: params.min_number_of_seeds,
            max_number_of_seeds: params.max_number_of_seeds,
            initial_deviation: params.initial_deviation,
            final_deviation: params.final_deviation,
            modulation_index: params.modulation_index,
        },
        termination::FixedIterations::new(setup.cutoff_length),
        tracking::Logger::default(),
    );

    let rng = Random::seeded(setup.seed);

    let start = Instant::now();
    let state = framework::run(&problem, &config, Some(rng));
    let end = Instant::now();
    let runtime = end - start;

    let allowed_error = match problem.name() {
        "rastrigin" => 5.0,
        "ackley" => 0.3,
        "sphere" => 0.01,
        _ => 1.0,
    };

    print_result(
        float_eq!(
            problem.known_optimum(),
            state.best_fitness().into(),
            abs <= allowed_error
        ),
        runtime.as_secs_f64(),
        state.iterations(),
        state.best_fitness().into(),
        setup.seed,
    );
}
