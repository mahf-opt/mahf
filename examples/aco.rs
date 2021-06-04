use mahf::{
    heuristic,
    heuristics::{aco, rs},
    problem::VectorProblem,
    problems::tsp,
    random::Random,
    tracking::{
        runtime_analysis::Experiment,
        trigger::{EvalTrigger, IterTrigger},
        Log,
    },
};

fn main() -> anyhow::Result<()> {
    let tsp = &tsp::Instances::BIER127.load();

    let max_iterations = 100;
    let number_of_ants = 50;
    let alpha = 1.0;

    let configs = &[
        ("as", {
            let heuristic_length = 1.0; // tsp.evaluate(&tsp.greedy_route());
            let default_pheromones = (tsp.dimension as f64 * heuristic_length).powi(-1);

            &aco::ant_stystem(
                number_of_ants,
                alpha,
                1.0,
                default_pheromones,
                0.5,
                heuristic_length,
                max_iterations,
            )
        }),
        ("mmas", {
            let t_max = 1.0 / (0.2 * tsp.best_fitness().unwrap());
            let p_dec = (0.01f64).powf(1.0 / tsp.dimension() as f64);
            let t_min = t_max * (1.0 - p_dec) / (tsp.dimension() as f64 / 2.0) * p_dec;

            &aco::min_max_ant_stystem(
                number_of_ants,
                alpha,
                2.0,
                0.0,
                0.02,
                t_max,
                t_min,
                max_iterations,
            )
        }),
        ("rs", {
            &rs::random_permutation_search(max_iterations * number_of_ants as u32)
        }),
    ];

    for (name, config) in configs {
        let data_dir = format!("data/aco/{}", name);
        let random = Random::default();
        let mut experiment = Experiment::create(&data_dir, tsp, &random, &config)?;
        let logger = &mut Log::new(
            EvalTrigger {
                improvement: true,
                interval: Some(100),
            },
            IterTrigger {
                improvement: true,
                interval: Some(10),
            },
        );
        heuristic::run(tsp, logger, config, None, None);
        let best = logger.final_best_fx();
        println!(
            "{} reached: {:.0} of {}",
            name,
            best,
            tsp.best_fitness().unwrap()
        );
        experiment.log_run(logger)?;
    }

    Ok(())
}
