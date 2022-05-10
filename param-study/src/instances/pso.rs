use crate::{
    declare_parameters,
    util::{print_result, ArgsIter, Setup},
};
use mahf::{
    float_eq::float_eq,
    framework::{self, Configuration},
    heuristics::pso,
    problems::bmf::BenchmarkFunction,
    random::Random,
};
use std::time::Instant;

declare_parameters! {
    population_size: u32,
    a: f64,
    b: f64,
    c: f64,
    v_max: f64,
}

pub fn run(setup: &Setup, args: &mut ArgsIter) {
    let params = parameters(args);

    let problem = BenchmarkFunction::try_from(setup.instance.as_str()).unwrap();

    let config = Configuration::from(pso::pso(
        params.population_size,
        params.a,
        params.b,
        params.c,
        params.v_max,
        setup.cutoff_length,
    ));

    let rng = Random::seeded(setup.seed);

    let start = Instant::now();
    let state = framework::run(&problem, &config, None, Some(rng));
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
