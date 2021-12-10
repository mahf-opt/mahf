//! Genetic Algorithm

use crate::{
    framework::Configuration,
    operators::*,
    problems::{LimitedVectorProblem, Problem, VectorProblem},
};

/// Genetic Algorithm
///
/// # References
/// [link.springer.com/10.1007/978-3-319-07153-4_28-1](http://link.springer.com/10.1007/978-3-319-07153-4_28-1)
pub fn ga<P>(
    population_size: u32,
    deviation: f64,
    _p_mutation: f64,
    pc: f64,
    max_iterations: u32,
) -> Configuration<P>
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem,
{
    let mut config = Configuration::new(
        initialization::RandomSpread {
            initial_population_size: population_size,
        },
        selection::FullyRandom {
            offspring: population_size,
        },
        generation::UniformCrossover { pc },
        replacement::Generational {
            max_population_size: population_size,
        },
        termination::FixedIterations { max_iterations },
    );
    config = config.add_generator(generation::FixedDeviationDelta { deviation });
    config
}
