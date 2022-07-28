use std::time::Instant;

use mahf::{
    float_eq::float_eq,
    framework::{self, Random},
    heuristics::pso,
    problems::bmf::BenchmarkFunction,
};

use crate::{
    declare_parameters,
    util::{print_result, ArgsIter, Setup},
};

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

    let config = pso::pso(
        params.population_size,
        params.a,
        params.b,
        params.c,
        params.v_max,
        setup.cutoff_length,
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
            state
                .best_objective_value::<BenchmarkFunction>()
                .unwrap()
                .value(),
            abs <= allowed_error
        ),
        runtime.as_secs_f64(),
        state.iterations(),
        state
            .best_objective_value::<BenchmarkFunction>()
            .unwrap()
            .value(),
        setup.seed,
    );
}
