use mahf::prelude::*;

type P = problems::tsp::SymmetricTsp;

fn main() -> anyhow::Result<()> {
    let problem = problems::tsp::Instances::BERLIN52.load();

    let config = ils::permutation_iterated_local_search(
        ils::PermutationProblemParameters {
            local_search_params: ls::PermutationProblemParameters {
                n_neighbors: 100,
                pm: 0.9,
                n_swap: 10,
            },
            local_search_termination: termination::FixedIterations::new(100),
        },
        termination::FixedIterations::new(10),
        tracking::Logger::default(),
    )
    .into_builder()
    .assert(|state| state.population_stack::<P>().current().len() == 1)
    .single_objective_summary()
    .build();

    config.run(&problem, None);

    Ok(())
}
