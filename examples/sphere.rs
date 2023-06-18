use mahf::prelude::*;
use mahf::problems::KnownOptimumProblem;
use mahf::SingleObjective;
use std::ops::Range;

pub struct Sphere {
    pub dim: usize,
}

impl Sphere {
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }
}

impl problems::Problem for Sphere {
    type Encoding = Vec<f64>;
    type Objective = SingleObjective;

    fn name(&self) -> &str {
        "Sphere"
    }
}

impl problems::VectorProblem for Sphere {
    type Element = f64;

    fn dimension(&self) -> usize {
        self.dim
    }
}

impl problems::LimitedVectorProblem for Sphere {
    fn domain(&self) -> Vec<Range<Self::Element>> {
        std::iter::repeat(-1.0..1.0).take(self.dim).collect()
    }
}

impl problems::ObjectiveFunction for Sphere {
    fn objective(solution: &Self::Encoding) -> Self::Objective {
        solution
            .iter()
            .map(|x| x.powi(2))
            .sum::<f64>()
            .try_into()
            .unwrap()
    }
}

impl KnownOptimumProblem for Sphere {
    fn known_optimum(&self) -> SingleObjective {
        0.0.try_into().unwrap()
    }
}

fn main() {
    // Specify the problem: Sphere function with 10 dimensions.
    let problem: Sphere = Sphere::new(/*dim: */ 10);
    // Specify the metaheuristic: Particle Swarm Optimization (pre-implemented in MAHF).
    let config: Configuration<Sphere> = pso::real_pso(
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
    let state: State<Sphere> = config.optimize(&problem);

    // Print the results.
    println!("Found Individual: {:?}", state.best_individual().unwrap());
    println!("This took {} iterations.", state.iterations());
    println!("Global Optimum: {:?}", problem.known_optimum());
}
