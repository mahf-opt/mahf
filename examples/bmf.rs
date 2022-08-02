use mahf::heuristics::pso;
use mahf::{framework, problems::bmf::BenchmarkFunction};

fn main() -> anyhow::Result<()> {
    let problem = BenchmarkFunction::sphere(30);
    let config = pso::pso(100, 1., 1., 1., 1., 500);

    let state = framework::run(&problem, &config, None);

    println!(
        "Found Fitness: {:?}",
        state.best_objective_value::<BenchmarkFunction>().unwrap(),
    );
    println!(
        "Found Individual: {:?}",
        state.best_individual::<BenchmarkFunction>().unwrap(),
    );
    println!("Global Optimum: {}", problem.known_optimum());

    Ok(())
}
