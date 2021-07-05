//! Genetic Algorithm

use crate::{
    heuristic::Configuration,
    operators::*,
    problem::{LimitedVectorProblem, Problem, VectorProblem},
};

/// Genetic Algorithm
///
/// # References
/// [link.springer.com/10.1007/978-3-319-07153-4_28-1](http://link.springer.com/10.1007/978-3-319-07153-4_28-1)
pub fn ga<P>(
    population_size: u32,
    deviation: f64,
    _p_mutation: f64,
    _p_crossover: f64,
    max_iterations: u32,
) -> Configuration<P>
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem,
{
    Configuration::new(
        initialization::RandomSpread {
            initial_population_size: population_size,
        },
        selection::RouletteWheel { offspring: population_size },
        generation::FixedDeviationDelta { deviation },
        replacement::Generational {
            max_population_size: population_size,
        },
        termination::FixedIterations { max_iterations },
    )
}