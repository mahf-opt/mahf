use crate::{
    heuristic::Configuration,
    operators::*,
    problem::{LimitedVectorProblem, Problem, VectorProblem},
};

pub fn mu_plus_lambda<P>(
    population_size: u32,
    lambda: u32,
    deviation: f64,
    max_iterations: u32,
) -> Configuration<P>
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem,
{
    Configuration::new(
        initialization::RandomSpread {
            initial_population_size: population_size,
        },
        selection::Es { lambda },
        generation::Fixed { deviation },
        replacement::Fittest {
            max_population_size: population_size,
        },
        termination::FixedIterations { max_iterations },
    )
}
