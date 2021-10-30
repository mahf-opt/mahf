use crate::util::{print_result, ArgsIter, Setup};
use mahf::{
    float_eq::float_eq, heuristic, heuristics::pso, problems::bmf::BenchmarkFunction,
    random::Random, tracking::Log,
};
use std::time::Instant;

#[derive(Debug, Default)]
struct Parameters {
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

    let logger = &mut Log::none();
    let rng = Random::seeded(setup.seed);

    let start = Instant::now();
    heuristic::run(&problem, logger, &config, Some(rng), None);
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
            logger.final_best_fx(),
            abs <= allowed_error
        ),
        runtime.as_secs_f64(),
        logger.final_iteration().iteration,
        logger.final_best_fx(),
        setup.seed,
    );
}

fn parameters(args: &mut ArgsIter) -> Parameters {
    let mut params = Parameters::default();

    while let Some(param) = args.next() {
        let value = args.next().unwrap();

        match param.as_str() {
            "-population_size" => params.population_size = value.parse().unwrap(),
            "-a" => params.a = value.parse().unwrap(),
            "-b" => params.b = value.parse().unwrap(),
            "-c" => params.c = value.parse().unwrap(),
            "-v_max" => params.v_max = value.parse().unwrap(),
            unknown => panic!("unknown param {}", unknown),
        }
    }

    params
}
