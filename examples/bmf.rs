use mahf::prelude::*;
use problems::bmf::BenchmarkFunction;

fn main() {
    // Specify the problem: Sphere function with 10 dimensions.
    let problem: BenchmarkFunction = BenchmarkFunction::rastrigin(/*dim: */ 10);
    // Specify the metaheuristic: Particle Swarm Optimization (pre-implemented in MAHF).
    let config: Configuration<BenchmarkFunction> = pso::real_pso(
        /*params: */
        pso::RealProblemParameters {
            num_particles: 20,
            start_weight: 0.9,
            end_weight: 0.4,
            c_one: 1.7,
            c_two: 1.7,
            v_max: 1.0,
        },
        /*termination: */
        termination::LessThanN::<state::common::Iterations>::new(/*n: */ 500)
            & termination::DistanceToOptGreaterThan::new(0.01),
    );

    // Execute the metaheuristic on the problem with a random seed.
    let state: State<BenchmarkFunction> = config.optimize(&problem);

    // Print the results.
    println!("Found Individual: {:?}", state.best_individual().unwrap());
    println!("This took {} iterations.", state.iterations());
    println!("Global Optimum: {}", problem.known_optimum());
    println!(
        "Population fitness mean: {:?}",
        state
            .populations()
            .current()
            .iter()
            .map(|i| i.objective().value())
            .sum::<f64>()
            / 20.
    )
}
