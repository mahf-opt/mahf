use mahf::prelude::*;
use problems::bmf::BenchmarkFunction;

fn main() {
    // Specify the problem: Sphere function with 10 dimensions.
    let problem: BenchmarkFunction = BenchmarkFunction::sphere(/*dim: */ 10);
    // Specify the metaheuristic: Particle Swarm Optimization (pre-implemented in MAHF).
    let config: Configuration<BenchmarkFunction> = pso::real_pso(
        /*params: */
        pso::RealProblemParameters {
            num_particles: 20,
            weight: 1.0,
            c_one: 1.0,
            c_two: 1.0,
            v_max: 1.0,
        },
        /*termination: */
        termination::FixedIterations::new(/*max_iterations: */ 500)
            & termination::DistanceToOpt::new(0.01),
    );

    // Execute the metaheuristic on the problem with a random seed.
    let state: State<BenchmarkFunction> = config.optimize(&problem);

    // Print the results.
    println!("Found Individual: {:?}", state.best_individual().unwrap());
    println!("This took {} iterations.", state.iterations());
    println!("Global Optimum: {}", problem.known_optimum());
}
