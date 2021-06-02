use mahf::{
    heuristic,
    heuristics::aco,
    problem::Problem,
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
    let alpha = 3.0;
    let beta = 1.0;
    let heuristic_length = tsp.evaluate(&tsp.greedy_route());
    println!("heuristic_length: {}", heuristic_length);
    let default_pheromones = (tsp.dimension as f64 * heuristic_length).powi(-1);
    println!("default_pheromones: {}", default_pheromones);

    let configs = &[
        (
            "as",
            &aco::ant_stystem(
                number_of_ants,
                alpha,
                beta,
                default_pheromones,
                0.5,
                heuristic_length,
                max_iterations,
            ),
        ),
        (
            "mmas",
            &aco::min_max_ant_stystem(
                number_of_ants,
                alpha,
                beta,
                default_pheromones,
                0.1,
                100.0,
                0.0,
                max_iterations,
            ),
        ),
    ];

    for (name, config) in configs {
        let data_dir = format!("data/aco/{}", name);
        let random = Random::seeded(0);
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
