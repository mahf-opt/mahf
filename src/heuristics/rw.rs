//! Random Walk

use crate::{
    framework::{components::Generation, Configuration},
    operators::*,
    problems::{LimitedVectorProblem, Problem, VectorProblem},
};

/// Random Walk
///
/// # Arguments
///
/// * mutation: The mutation method used to move in the search space.
pub fn random_walk<P>(max_iterations: u32, mutation: impl Generation<P> + 'static) -> Configuration<P>
    where
        P: Problem<Encoding=Vec<f64>> + VectorProblem<T=f64> + LimitedVectorProblem,
{
    Configuration::new(
        initialization::RandomSpread {
            initial_population_size: 1
        },
        selection::All,
        mutation,
        replacement::Generational {
            max_population_size: 1,
        },
        termination::FixedIterations { max_iterations },
    )
}

/// Random Permutation Walk
///
/// # Arguments
///
/// * mutation: The mutation method used to move in the search space.
pub fn random_permutation_walk<P>(max_iterations: u32, mutation: impl Generation<P> + 'static) -> Configuration<P>
    where
        P: Problem<Encoding=Vec<usize>> + VectorProblem<T=usize>,
{
    Configuration::new(
        initialization::RandomPermutation {
            initial_population_size: 1,
        },
        selection::All,
        mutation,
        replacement::Generational {
            max_population_size: 1,
        },
        termination::FixedIterations { max_iterations },
    )
}
