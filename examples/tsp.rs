use mahf::heuristics::aco;
use mahf::{framework, problems};

fn main() -> anyhow::Result<()> {
    let problem = problems::tsp::Instances::BERLIN52.load();
    let tau0 = 1. / problem.best_fitness().unwrap();
    let config = aco::min_max_ant_system(20, 1., 1., tau0, 0.1, 1., 0.1, 500);

    let state = framework::run(&problem, &config, None);

    println!(
        "Found Solution: {:?}",
        state.best_fitness().unwrap().value()
    );

    Ok(())
}
