//! Evolutionary Strategy

use crate::{
    heuristic::Configuration,
    operators::*,
    problem::{LimitedVectorProblem, Problem, VectorProblem},
};

/// (μ+λ)-Evolutionary-Strategy
///
/// # References
/// [doi.org/10.1023/A:1015059928466](https://doi.org/10.1023/A:1015059928466)
pub fn random_search<P>(max_iterations: u32) -> Configuration<P>
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem,
{
    Configuration::new(
        initialization::Noop,
        selection::FullyRandom { offspring: 0 },
        generation::RandomSpread,
        replacement::Fittest {
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
        selection::FullyRandom { offspring: 0 },
        generation::RandomPermutation,
        replacement::Fittest {
            max_population_size: 0,
        },
        termination::FixedIterations { max_iterations },
    )
}
