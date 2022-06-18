use mahf::heuristics::pso;
use mahf::{framework, problems};

fn main() -> anyhow::Result<()> {
    let problem = problems::bmf::BenchmarkFunction::sphere(30);
    let config = pso::pso(100, 1., 1., 1., 1., 500);

    let state = framework::run(&problem, &config, None, None);

    println!(
        "Found Solution: {:?}",
        state.best_individual().unwrap().fitness().into()
    );
    println!("Global Optimum: {}", problem.known_optimum());

    Ok(())
}
