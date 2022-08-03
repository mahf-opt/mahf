use mahf::{framework, problems, heuristics::*, operators::*, tracking::Logger};

fn main() -> anyhow::Result<()> {
    let problem = problems::tsp::Instances::BERLIN52.load();
    // let tau0 = 1. / problem.best_fitness().unwrap();
    // let config = aco::min_max_ant_system(20, 1., 1., tau0, 0.1, 1., 0.1, 500);

    let config = ils::permutation_iterated_local_search(ils::PermutationParameters {
        local_search_params: ls::PermutationParameters {
            n_neighbors: 20,
            pm: 0.7,
            n_swap: 5
        },
        local_search_termination: termination::FixedIterations::new(100),
    }, termination::FixedIterations::new(10), Logger::default());

    let state = framework::run(&problem, &config, None);

    println!(
        "Found Solution: {:?}",
        state
            .best_objective_value::<problems::tsp::SymmetricTsp>()
            .unwrap()
    );

    Ok(())
}
