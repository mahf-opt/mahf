use mahf::framework::Random;
use mahf::prelude::*;

type P = problems::bmf::BenchmarkFunction;

fn main() -> anyhow::Result<()> {
    let problem = P::sphere(10);
    let v_max = 1.0 * problem.dimension() as f64;
    let config = pso::real_pso(
        pso::RealProblemParameters {
            num_particles: 30,
            weight: 0.8,
            c_one: 1.7,
            c_two: 1.7,
            v_max,
        },
        termination::FixedIterations::new(1000),
        tracking::Logger::default(),
    );

    let state = config.optimize_with(&problem, |state| state.insert(Random::seeded(0)));

    println!(
        "Found Fitness: {:?}",
        state.best_objective_value::<P>().unwrap()
    );
    println!(
        "Found Individual: {:?}",
        state.best_individual::<P>().unwrap(),
    );
    println!("Global Optimum: {}", problem.known_optimum());

    Ok(())
}
