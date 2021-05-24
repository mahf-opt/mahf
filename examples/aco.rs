use mahf::{heuristic, heuristics::aco, problem::Problem, problems::tsp, tracking::Log};

fn main() {
    let tsp = &tsp::Instances::BIER127.load();

    let max_iterations = 100;
    let number_of_ants = 50;
    let alpha = 1.0;
    let beta = 1.0;
    let evaporation = 0.5;
    let heuristic_length = tsp.evaluate(&tsp.greedy_route());
    let default_pheromones = (tsp.dimension as f64 * heuristic_length).powi(-1);

    let config = &aco::aco(
        number_of_ants,
        alpha,
        beta,
        default_pheromones,
        evaporation,
        heuristic_length,
        max_iterations,
    );

    let logger = &mut Log::none();
    heuristic::run(tsp, logger, config, None, None);
    let best = logger.final_best_fx();
    println!(
        "Best value reached: {} of {}",
        best,
        tsp.best_fitness().unwrap()
    );
}
