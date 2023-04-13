use aco::ant_ops;
use mahf::prelude::*;
use problems::tsp::{self, SymmetricTsp};
use tracking::{extractor, files, trigger};

fn main() {
    // Specify the problem: TSPLIB instance Berlin52.
    let problem: SymmetricTsp = tsp::Instances::BERLIN52.load();
    // Specify the metaheuristic: Ant System.
    let config: Configuration<SymmetricTsp> = Configuration::builder()
        .do_(initialization::Empty::new())
        .while_(
            termination::FixedEvaluations::new(/*max_evaluations: */ 10_000),
            |builder| {
                builder
                    .do_(ant_ops::AcoGeneration::new(
                        /*num_ants: */ 20, /*alpha: */ 2.0, /*beta: */ 1.0,
                        /*initial_pheromones: */ 0.0,
                    ))
                    .evaluate()
                    .update_best_individual()
                    .do_(ant_ops::AsPheromoneUpdate::new(
                        /*evaporation: */ 0.2, /*decay_coefficient: */ 1.0,
                    ))
                    .do_(tracking::Logger::new())
            },
        )
        .build();

    // Execute the metaheuristic on the problem.
    let state: State<SymmetricTsp> = config.optimize_with(&problem, |state| {
        // Set the seed to 42.
        state.insert(Random::seeded(42));
        // Log the best individual every 50 iterations.
        state.insert(
            tracking::LogSet::<SymmetricTsp>::new()
                .with(trigger::Iteration::new(50), extractor::best_individual),
        );
    });

    // Save the log to file "aco_berlin52.log".
    files::write_log_file("aco_berlin52.log", state.log()).unwrap();
}
