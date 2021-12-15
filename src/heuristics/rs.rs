//! Random Search

use crate::{
    framework::Configuration,
    operators::*,
    problems::{LimitedVectorProblem, Problem, VectorProblem},
};

/// Random Search
pub fn random_search<P>(max_iterations: u32) -> Configuration<P>
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem,
{
    Configuration::new(
        initialization::Noop,
        selection::None,
        generation::RandomSpread,
        replacement::MuPlusLambda {
            max_population_size: 0,
        },
        termination::FixedIterations { max_iterations },
    )
}

pub fn random_permutation_search<P>(max_iterations: u32) -> Configuration<P>
where
    P: Problem<Encoding = Vec<usize>> + VectorProblem<T = usize>,
{
    Configuration::new(
        initialization::Noop,
        selection::None,
        generation::RandomPermutation,
        replacement::MuPlusLambda {
            max_population_size: 0,
        },
        termination::FixedIterations { max_iterations },
    )
}
