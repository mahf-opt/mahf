//! Random Walk

use crate::{
    framework::{
        components::{self, Component},
        Configuration, ConfigurationBuilder,
    },
    operators::*,
    problems::{LimitedVectorProblem, Problem, VectorProblem},
};

/// Random Walk
///
/// # Arguments
///
/// * mutation: The mutation method used to move in the search space.
pub fn random_walk<P>(max_iterations: u32, mutation: Box<dyn Component<P>>) -> Configuration<P>
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem + 'static,
{
    ConfigurationBuilder::new()
        .do_(generation::RandomSpread::new_init(1))
        .while_(
            termination::FixedIterations::new(max_iterations),
            |builder| {
                builder
                    .do_(archive::ElitistArchive::new(1))
                    .do_(selection::All::new())
                    .do_(mutation)
                    .do_(components::SimpleEvaluator::new())
                    .do_(replacement::Generational::new(1))
            },
        )
        .do_(archive::AddElitists::new())
        .build()
}

/// Random Permutation Walk
///
/// # Arguments
///
/// * mutation: The mutation method used to move in the search space.
pub fn random_permutation_walk<P>(
    max_iterations: u32,
    mutation: Box<dyn Component<P>>,
) -> Configuration<P>
where
    P: Problem<Encoding = Vec<usize>> + VectorProblem<T = usize> + 'static,
{
    ConfigurationBuilder::new()
        .do_(generation::RandomPermutation::new_init(1))
        .while_(
            termination::FixedIterations::new(max_iterations),
            |builder| {
                builder
                    .do_(archive::ElitistArchive::new(1))
                    .do_(selection::All::new())
                    .do_(mutation)
                    .do_(components::SimpleEvaluator::new())
                    .do_(replacement::Generational::new(1))
            },
        )
        .do_(archive::AddElitists::new())
        .build()
}
