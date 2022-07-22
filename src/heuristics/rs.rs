//! Random Search

use crate::{
    framework::{components, Configuration},
    operators::*,
    problems::{LimitedVectorProblem, Problem, VectorProblem},
};

/// Random Search
pub fn random_search<P>(max_iterations: u32) -> Configuration<P>
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem + 'static,
{
    Configuration::builder()
        .do_(generation::RandomSpread::new_init(1))
        .while_(
            termination::FixedIterations::new(max_iterations),
            |builder| {
                builder
                    .do_(selection::All::new())
                    .do_(generation::RandomSpread::new_gen())
                    .do_(components::SimpleEvaluator::new())
                    .do_(replacement::MuPlusLambda::new(1))
            },
        )
        .build()
}

pub fn random_permutation_search<P>(max_iterations: u32) -> Configuration<P>
where
    P: Problem<Encoding = Vec<usize>> + VectorProblem<T = usize> + 'static,
{
    Configuration::builder()
        .do_(generation::RandomPermutation::new_init(1))
        .while_(
            termination::FixedIterations::new(max_iterations),
            |builder| {
                builder
                    .do_(selection::All::new())
                    .do_(generation::RandomPermutation::new_gen())
                    .do_(components::SimpleEvaluator::new())
                    .do_(replacement::MuPlusLambda::new(1))
            },
        )
        .build()
}
