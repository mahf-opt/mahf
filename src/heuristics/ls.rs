//! Local Search

use crate::{
    framework::{
        components::{self, Component},
        Configuration, ConfigurationBuilder,
    },
    operators::*,
    problems::{LimitedVectorProblem, Problem, VectorProblem},
};

/// Local Search
pub fn local_search<P>(
    max_iterations: u32,
    n_neighbors: u32,
    neighbors: Box<dyn Component<P>>,
) -> Configuration<P>
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem + 'static,
{
    ConfigurationBuilder::new()
        .do_(initialization::RandomSpread::new_init(1))
        .while_(
            termination::FixedIterations::new(max_iterations),
            |builder| {
                builder
                    .do_(selection::DuplicateSingle::new(n_neighbors))
                    .do_(neighbors)
                    .do_(components::SimpleEvaluator::new())
                    .do_(replacement::MuPlusLambda::new(1))
            },
        )
        .build()
}

/// Local Permutation Search
pub fn local_permutation_search<P>(
    max_iterations: u32,
    n_neighbors: u32,
    neighbors: Box<dyn Component<P>>,
) -> Configuration<P>
where
    P: Problem<Encoding = Vec<usize>> + VectorProblem<T = usize> + 'static,
{
    ConfigurationBuilder::new()
        .do_(initialization::RandomPermutation::new_init(1))
        .while_(
            termination::FixedIterations::new(max_iterations),
            |builder| {
                builder
                    .do_(selection::DuplicateSingle::new(n_neighbors))
                    .do_(neighbors)
                    .do_(components::SimpleEvaluator::new())
                    .do_(replacement::MuPlusLambda::new(1))
            },
        )
        .build()
}
