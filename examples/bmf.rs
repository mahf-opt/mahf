use mahf::prelude::*;

type P = problems::bmf::BenchmarkFunction;

fn main() -> anyhow::Result<()> {
    let problem = P::sphere(10);
    let config = pso::real_pso(
        pso::RealProblemParameters {
            num_particles: 100,
            weight: 1.0,
            c_one: 1.0,
            c_two: 1.0,
            v_max: 1.0,
        },
        termination::FixedIterations::new(500),
        tracking::Logger::default(),
    );

    let state = config.run(&problem, None);

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
